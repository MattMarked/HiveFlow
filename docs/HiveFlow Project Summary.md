# HiveFlow Project Summary

## Overview
HiveFlow is a modern peer-to-peer file sharing system, inspired by eMule but built with contemporary technologies. The project aims to be free for users, fast in sharing files, reliable, and with minimal maintenance costs.

## Technology Stack
- Backend Core: Rust
- Frontend: Next.js (Progressive Web App)
- P2P Framework: libp2p
- Protocol: Protocol Buffers
- Network: WebRTC for NAT traversal
- Storage: Content-addressable storage
- Data Format: Protobuf for serialization

## Project Structure
```
hiveflow/
├── hiveflow-core/            # Rust P2P core
│   ├── src/
│   │   ├── swarm/            # P2P networking
│   │   ├── cell/             # Content storage
│   │   ├── crypto/           # Encryption
│   │   └── protocol/         # Protocol definitions
│   └── Cargo.toml
├── web/                      # Next.js frontend
│   ├── app/                  # App routes
│   ├── components/           # React components
│   ├── lib/                  # WebAssembly bindings
│   └── package.json
├── proto/                    # Protocol definitions
│   └── hiveflow.proto        # Message schemas
└── docs/                     # Documentation
```

## Key Features
1. Content Management
   - Content-addressed storage
   - Chunk-based file handling
   - Multiple hash algorithm support
   - File versioning
   - Categories and tagging

2. Network Features
   - WebRTC for direct peer connections
   - NAT traversal
   - Bandwidth-aware peer selection
   - DHT for peer discovery
   - Rate limiting
   - Priority-based transfers

3. Security Features
   - Built-in encryption
   - Signature verification
   - Peer reputation system
   - Multiple hash verification

4. Performance Features
   - Chunk-based streaming
   - Resume capability
   - Modern compression
   - Bandwidth management
   - Connection limits

## Protocol Overview
The protocol (.proto file) defines:
1. Core Messages:
   - FileMetadata: File information and metadata
   - ChunkInfo: Chunk-level information
   - PeerInfo: Peer capabilities and status
   
2. Operations:
   - Search functionality
   - Chunk transfer
   - Peer discovery
   - Health monitoring
   - Bandwidth management
   - Availability tracking

3. Priority Levels:
   - LOW
   - NORMAL
   - HIGH

4. Peer States:
   - UNKNOWN
   - ONLINE
   - BUSY
   - AWAY
   - OFFLINE

## Current Progress
1. Initial project structure created
2. Protocol definitions completed
3. Basic documentation in place
4. Git repository initialized both in local and Github account
5. MIT License applied
6. Rust core project setup complete
   - Project structure created (hiveflow-core)
   - Dependencies configured in Cargo.toml
   - Module structure established (swarm, cell, crypto, protocol)
   - Build system configured for protobuf compilation
   - Initial tests passing
7. Core/cell module complete with basic functionality
   - Test coverage includes basic operations and garbage collection
   - Ready for integration with other modules

## Next Steps
1. Implement Rust core modules
   - Create content storage system (cell module): in progress
      - Implement file splitting/joining functions
      - Add integrity verification
   - Develop protocol message handlers
   - Add encryption and security features (crypto module)
   - Implement swarm module for P2P networking (libp2p)


2. Create web frontend
   - Basic UI structure
   - File management interface
   - Transfer monitoring

3. Testing
   - Protocol validation
   - Network testing
   - Security audit
   - Expand test coverage for core modules

## Project Setup Instructions
1. Clone repository
2. Install protobuf compiler
3. Install Rust and cargo
4. Install Node.js and npm
5. Initialize both core and web projects

## License
The project is licensed under MIT License, allowing for open source collaboration while maintaining intellectual property rights.

## Repository Structure
- README.md: Project overview
- LICENSE: MIT License
- /docs: Detailed documentation
- /proto: Protocol definitions
- /core: Rust implementation (pending)
- /web: Next.js frontend (pending)