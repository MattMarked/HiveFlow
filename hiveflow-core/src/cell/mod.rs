use blake3;
use hex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use tokio::sync::RwLock;
use walkdir::WalkDir;

/// Represents a chunk of file data in the content-addressable storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    hash_bytes: Vec<u8>, // Store hash as raw bytes instead of Hash type
    size: usize,
    index: u32,
    file_id: String,
}

impl Chunk {
    /// Convert stored bytes back to Hash when needed
    pub fn hash(&self) -> blake3::Hash {
        blake3::Hash::from_bytes(self.hash_bytes.as_slice().try_into().unwrap())
    }
}

/// Represents metadata about a stored file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    file_id: String,
    total_size: u64,
    chunk_hashes: Vec<Vec<u8>>, // Store hashes as raw bytes
    mime_type: String,
    original_name: String,
    created_at: u64,
    checksums: HashMap<String, Vec<u8>>,
}

impl FileInfo {
    /// Convert stored bytes back to Hash objects when needed
    pub fn chunk_hashes_as_hash(&self) -> Vec<blake3::Hash> {
        self.chunk_hashes
            .iter()
            .map(|bytes| blake3::Hash::from_bytes(bytes.as_slice().try_into().unwrap()))
            .collect()
    }
}

/// The main storage manager for the content-addressable system
pub struct Cell {
    /// Base directory for all stored content
    base_path: PathBuf,
    /// In-memory cache of file metadata
    file_index: RwLock<HashMap<String, FileInfo>>,
    /// Default chunk size for new files
    default_chunk_size: u32,
    /// Reference count for chunks
    chunk_refs: RwLock<HashMap<Vec<u8>, u32>>,
}

impl Cell {
    /// Creates a new Cell instance with the specified base directory
    pub async fn new(base_path: PathBuf, chunk_size: u32) -> io::Result<Self> {
        // Ensure the base directory exists
        std::fs::create_dir_all(&base_path)?;

        // Create subdirectories for chunks and metadata
        let chunks_dir = base_path.join("chunks");
        let meta_dir = base_path.join("metadata");
        std::fs::create_dir_all(&chunks_dir)?;
        std::fs::create_dir_all(&meta_dir)?;

        Ok(Cell {
            base_path,
            file_index: RwLock::new(HashMap::new()),
            default_chunk_size: chunk_size,
            chunk_refs: RwLock::new(HashMap::new()),
        })
    }

    /// Stores a new chunk in the content-addressable storage
    pub async fn store_chunk(&self, data: &[u8], file_id: &str, index: u32) -> io::Result<Chunk> {
        let hash = blake3::hash(data);
        let chunk = Chunk {
            hash_bytes: hash.as_bytes().to_vec(),
            size: data.len(),
            index,
            file_id: file_id.to_string(),
        };

        // Store the chunk data
        let chunk_path = self.get_chunk_path(&hash);
        // Ensure parent directory exists
        if let Some(parent) = chunk_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        if !chunk_path.exists() {
            let mut file = std::fs::File::create(chunk_path)?;
            file.write_all(data)?;
        }

        // Increment reference count
        self.inc_chunk_ref(&chunk.hash_bytes).await;

        Ok(chunk)
    }

    /// Retrieves a chunk by its hash
    pub async fn get_chunk(&self, hash_bytes: &[u8]) -> io::Result<Vec<u8>> {
        let hash = blake3::Hash::from_bytes(hash_bytes.try_into().unwrap());
        let chunk_path = self.get_chunk_path(&hash);
        let mut file = std::fs::File::open(chunk_path)?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;
        Ok(data)
    }

    /// Stores file metadata
    pub async fn store_file_info(&self, info: FileInfo) -> io::Result<()> {
        let mut index = self.file_index.write().await;
        index.insert(info.file_id.clone(), info.clone());

        // Persist metadata to disk
        let meta_path = self.get_metadata_path(&info.file_id);
        let meta_json = serde_json::to_string(&info)?;
        std::fs::write(meta_path, meta_json)?;

        Ok(())
    }

    /// Retrieves file metadata by ID
    pub async fn get_file_info(&self, file_id: &str) -> io::Result<Option<FileInfo>> {
        let index = self.file_index.read().await;
        if let Some(info) = index.get(file_id) {
            return Ok(Some(info.clone()));
        }

        // Try loading from disk if not in memory
        let meta_path = self.get_metadata_path(file_id);
        if meta_path.exists() {
            let meta_json = std::fs::read_to_string(meta_path)?;
            let info: FileInfo = serde_json::from_str(&meta_json)?;
            Ok(Some(info))
        } else {
            Ok(None)
        }
    }

    /// Returns the path where a chunk should be stored
    fn get_chunk_path(&self, hash: &blake3::Hash) -> PathBuf {
        let hex = hash.to_hex();
        // Use first 4 chars as directory name for better distribution
        self.base_path.join("chunks").join(&hex[0..4]).join(&hex)
    }

    /// Returns the path where file metadata should be stored
    fn get_metadata_path(&self, file_id: &str) -> PathBuf {
        self.base_path
            .join("metadata")
            .join(format!("{}.json", file_id))
    }

    /// Returns the default chunk size for this cell
    pub fn default_chunk_size(&self) -> u32 {
        self.default_chunk_size
    }

    /// Increment reference count for a chunk
    async fn inc_chunk_ref(&self, hash_bytes: &[u8]) {
        let mut refs = self.chunk_refs.write().await;
        *refs.entry(hash_bytes.to_vec()).or_insert(0) += 1;
    }

    /// Decrement reference count for a chunk
    async fn dec_chunk_ref(&self, hash_bytes: &[u8]) -> io::Result<()> {
        let mut refs = self.chunk_refs.write().await;
        if let Some(count) = refs.get_mut(hash_bytes) {
            *count -= 1;
            if *count == 0 {
                refs.remove(hash_bytes);
                // Delete the chunk file
                let hash = blake3::Hash::from_bytes(hash_bytes.try_into().unwrap());
                let path = self.get_chunk_path(&hash);
                if path.exists() {
                    std::fs::remove_file(path)?;
                }
            }
        }
        Ok(())
    }

    /// Remove unreferenced chunks
    pub async fn collect_garbage(&self) -> io::Result<usize> {
        let mut removed = 0;
        let chunks_dir = self.base_path.join("chunks");

        // Get all referenced chunks
        let refs = self.chunk_refs.read().await;
        let referenced: std::collections::HashSet<_> = refs.keys().cloned().collect();

        // Walk through chunk directory
        for entry in WalkDir::new(&chunks_dir)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
        {
            if let Some(file_name) = entry.file_name().to_str() {
                // Convert filename (hex) to bytes
                if let Ok(hash_bytes) = hex::decode(file_name) {
                    if !referenced.contains(&hash_bytes) {
                        std::fs::remove_file(entry.path())?;
                        removed += 1;
                    }
                }
            }
        }

        Ok(removed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_store_and_retrieve_chunk() {
        let temp_dir = tempdir().unwrap();
        let cell = Cell::new(temp_dir.path().to_path_buf(), 1024 * 1024)
            .await
            .unwrap();

        let test_data = b"Hello, World!";
        let chunk = cell.store_chunk(test_data, "test_file", 0).await.unwrap();

        let retrieved = cell.get_chunk(&chunk.hash_bytes).await.unwrap();
        assert_eq!(retrieved, test_data);
    }

    #[tokio::test]
    async fn test_store_and_retrieve_file_info() {
        let temp_dir = tempdir().unwrap();
        let cell = Cell::new(temp_dir.path().to_path_buf(), 1024 * 1024)
            .await
            .unwrap();

        let info = FileInfo {
            file_id: "test123".to_string(),
            total_size: 1000,
            chunk_hashes: vec![],
            mime_type: "text/plain".to_string(),
            original_name: "test.txt".to_string(),
            created_at: 12345,
            checksums: HashMap::new(),
        };

        cell.store_file_info(info.clone()).await.unwrap();

        let retrieved = cell.get_file_info("test123").await.unwrap().unwrap();
        assert_eq!(retrieved.file_id, info.file_id);
        assert_eq!(retrieved.total_size, info.total_size);
    }

    #[tokio::test]
    async fn test_garbage_collection() {
        let temp_dir = tempdir().unwrap();
        let cell = Cell::new(temp_dir.path().to_path_buf(), 1024 * 1024)
            .await
            .unwrap();

        // Store two chunks
        let chunk1 = cell.store_chunk(b"data1", "file1", 0).await.unwrap();
        let chunk2 = cell.store_chunk(b"data2", "file1", 1).await.unwrap();

        // Zero out reference count for chunk1 (simulating deletion)
        {
            let mut refs = cell.chunk_refs.write().await;
            refs.remove(&chunk1.hash_bytes);
        }

        // Run garbage collection
        let removed = cell.collect_garbage().await.unwrap();
        assert_eq!(removed, 1, "Should remove one unreferenced chunk");

        // Verify chunk1 is gone and chunk2 remains
        assert!(cell.get_chunk(&chunk1.hash_bytes).await.is_err());
        assert!(cell.get_chunk(&chunk2.hash_bytes).await.is_ok());
    }
}
