# 🦀 FeroxDB

**FeroxDB** – _Fast. Persistent. In-memory. Key-Value Store in Rust_  
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
│   │   └── fox.rs          # Main CLI entry point
│   │   └── server.rs       # gRPC server entry point
│   ├── grpc/               # generated code from proto
│   │   └── feroxdb.rs      # generated code from proto
│   │   └── mod.rs          # grpc module
│   ├── cache.rs            # In-memory cache implementation
│   ├── cli.rs              # CLI parser using clap
│   ├── lib.rs              # Core library interface
│   ├── persistance.rs      # save/ load logic
│   ├── storage.rs          # File-backed persistence logic
│   └── types.rs            # Shared types and TTL structs
├── build.rs
├── Cargo.toml
└── README.md
```


## 🔧 Basic Architecture

```objective

┌────────────┐     CLI Command      ┌────────────┐
│  fox CLI   │ ───────────────────▶ │ FeroxDB    │
│ (client)   │      via gRPC        │ Server     │
└────────────┘                      └────────────┘

```
