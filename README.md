# Yellowstone gRPC Starter

A gRPC server implementation using Tonic and Rust for Yellowstone gRPC projects.

## Features

- gRPC server implementation using Tonic
- Protocol Buffer definition for Ping service
- Async/await support with Tokio
- Client example for testing

## Project Structure

```
yellowstone_grpc_starter/
├── proto/
│   └── geyser.proto          # Protocol Buffer definitions
├── src/
│   ├── main.rs               # Server implementation
│   └── client.rs             # Client example
├── build.rs                  # Build script for proto compilation
├── Cargo.toml               # Rust dependencies
└── README.md                # This file
```

## Prerequisites

- Rust (latest stable version)
- Cargo

## Building and Running

### Build the project
```bash
cargo build
```

### Run the server
```bash
cargo run
```

The server will start on `[::1]:50051` (IPv6 localhost, port 50051).

### Test with the client
In a separate terminal, test the server using the client:

```bash
cargo run --bin client
```

## Protocol Buffer Definition

The service is defined in `proto/geyser.proto`:

```protobuf
syntax = "proto3";

package solana.geyser;

message PingRequest {}
message PingResponse {}

service Geyser {
    rpc Ping (PingRequest) returns (PingResponse);
}
```

## Dependencies

- `tonic`: gRPC framework for Rust
- `prost`: Protocol Buffer implementation
- `tokio`: Async runtime
- `tonic-build`: Build-time dependency for proto compilation

## Usage

1. Start the server: `cargo run`
2. Test connections: `cargo run --bin client`

The client will test both local and external gRPC endpoints. 