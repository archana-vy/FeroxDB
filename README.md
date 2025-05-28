# 🦀 FeroxDB

**FeroxDB** – *Fast. Persistent. In-memory. Key-Value Store in Rust*  
A fierce hybrid key-value store written in Rust. Combines the blazing speed of in-memory storage with the durability of file-backed persistence.

## 🚀 Features

- ⚡ **Fast**: In-memory caching for ultra-low-latency reads/writes
- 💾 **Persistent**: File-backed storage ensures data durability
- ⏱️ **TTL Support**: Optional time-to-live for ephemeral keys
- 🔐 **Thread-safe**: Safe concurrency using `parking_lot` mutexes
- 📦 **Serializable**: Uses `serde` + `bincode` for efficient serialization
- 🧩 **Modular**: Clean, extensible design for future upgrades (e.g., gRPC)

## 🗂 Project Structure

```bash
FeroxDB/
├── src/
│   ├── bin/
│   │   └── feroxdb.rs      # Main CLI entry point
│   ├── cli.rs              # CLI parser using clap
│   ├── lib.rs              # Core library interface
│   ├── cache.rs            # In-memory cache implementation
│   ├── storage.rs          # File-backed persistence logic
│   └── types.rs            # Shared types and TTL structs
├── Cargo.toml
└── README.md

