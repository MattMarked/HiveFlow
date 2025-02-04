# HiveFlow Project Structure

```
hiveflow/
├── core/                      # Rust P2P core
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
└── proto/                    # Protocol definitions
    └── hiveflow.proto        # Message schemas
```

## Components

1. Core (Rust)
   - libp2p network stack
   - DHT implementation
   - WebRTC transport
   - Content storage
   - Protobuf handlers

2. Web Frontend
   - File management UI
   - Transfer monitoring
   - Peer management
   - Search interface

3. Protocol
   - File discovery
   - Chunk transfer
   - Peer reputation
   - Network messages