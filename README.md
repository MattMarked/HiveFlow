# HiveFlow

> Oh eMule, dear donkey of downloads past,
> With your ED2K links that seemed to last
> Forever loading at 56k,
> While I hoped my Windows wouldn't break.
> 
> Your chunk system taught me patience true,
> As I watched that progress bar crawl through.
> High ID or Low, you didn't care,
> Sharing rare files with peers who'd dare.
> 
> Through Kad and servers, you'd bray and seek,
> Making dial-up modems wheeze and squeak.
> A stubborn beast, but faithful too,
> Ancient p2p, we remember you!
> - Metchio

A modern peer-to-peer file sharing system built with contemporary technologies. The project aims to be free for users, fast in sharing files, reliable, and with minimal maintenance costs.

## Technology Stack
- Backend Core: Rust
- Frontend: Next.js (Progressive Web App)
- P2P Framework: libp2p
- Protocol: Protocol Buffers
- Network: WebRTC for NAT traversal
- Storage: Content-addressable storage
- Data Format: Protobuf for serialization

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

## Project Structure
```
hiveflow/
├── core/                      # Rust P2P core
├── web/                      # Next.js frontend
├── proto/                    # Protocol definitions
└── docs/                     # Documentation
```

## Getting Started

### Prerequisites
- Protobuf compiler
- Rust and cargo
- Node.js and npm

### Installation
[Coming soon]

## Documentation
For more detailed information, check the [docs](./docs) directory.

## Contributing
[Coming soon]

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.