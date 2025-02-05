pub mod swarm;
pub mod cell;
pub mod crypto;
pub mod protocol;

use anyhow::Result;

/// Core structure for the HiveFlow P2P system
pub struct HiveFlow {
    // Will be expanded as we implement features
}

impl HiveFlow {
    /// Create a new instance of HiveFlow
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }

    /// Start the HiveFlow node
    pub async fn start(&self) -> Result<()> {
        // Will be implemented later
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_hiveflow() {
        let hive = HiveFlow::new().unwrap();
        assert!(hive.start().await.is_ok());
    }
}