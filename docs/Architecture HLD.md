Architecture HLD

Backend:

Rust for core P2P functionality (high performance, memory safety)
libp2p framework for P2P networking (proven, maintained by Protocol Labs)
Content-addressable storage using IPFS-style content IDs
DHT for decentralized peer discovery and content routing
WebRTC for NAT traversal and direct peer connections
Protobuf for efficient message serialization

Frontend:

Progressive Web App using Next.js
WebAssembly module (compiled from Rust) for heavy lifting
WebRTC for direct browser-to-browser transfers
Local-first architecture with IndexedDB for file/metadata caching

Key improvements over eMule:

Browser-based (no installation needed)
WebRTC for better NAT traversal
Content-addressed deduplication
Modern compression (zstd)
Built-in encryption
Bandwidth-aware peer selection
Reputation system using proof-of-work
Chunk-based streaming

This design offers:

Zero server costs (fully P2P)
Fast transfers (WebRTC + modern protocols)
High reliability (content verification)
Easy deployment (web-based)
Cross-platform support