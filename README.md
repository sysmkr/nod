```
                  __     
                 /\ \    
  ___     ___    \_\ \   
/' _ `\  / __`\  /'_` \  
/\ \/\ \/\ \L\ \/\ \L\ \ 
\ \_\ \_\ \____/\ \___,_\
 \/_/\/_/\/___/  \/__,_ /
```

Status : ongoing.  
Note : this document has been redacted right after setting project boundaries.  
Some things may figure while not exisiting yet or being changed or removed.  
Everything will be updated in due time, at least right before publishing the first version.  

Nod is a Raft-based distributed key-value store implemented in Rust. It focuses on providing linearizable reads and fault tolerance within a single cluster. The project aims to build a robust, scalable, and reliable key-value store that can survive node failures and network partitions.

## Features
```
- Single-Node Key-Value Store: In-memory `HashMap` with disk persistence.
- Raft Consensus: Leader election, log replication, and safety checks.
- Networking: gRPC or QUIC for inter-node communication.
- Persistence and Snapshotting:** Log compaction to avoid OOM and node restarting with state recovery.
- Chaos Testing: Kill nodes and partition networks to test fault tolerance.
- Benchmarking: Latency and throughput benchmarking against `etcd`.
```

## Tools and Technologies
```
- Language: Rust
- Async Runtime: `tokio`
- Networking: `tonic` (gRPC) or `quinn` (QUIC)
- Persistence: `rocksdb` or `sled`
- Serialization: `prost` (Protobuf) or `bincode`
- Logging: `tracing` + `tracing-subscriber`
- Testing: `tokio-test` + custom chaos tests
- CLI: `clap`
```

## Project Structure
```
The project is structured to ensure modularity and separation of concerns:
- Single-Node KV Store: Foundation for the distributed system with basic operations (`get`, `pull`, `delete`).
- Raft Core:** Consensus algorithm implementation.
- Networking Layer: Communication between nodes.
- Persistence Layer: Disk storage and snapshotting.
- CLI: Command-line interface for user interaction.
```

## Milestones
```
Single-Node Key-Value Store
- Implement basic key-value operations with in-memory storage and disk persistence.
- Ensure the node can start, stop, and recover its state.

Raft Integration
- Extend the node to participate in Raft consensus.
- Implement leader election, log replication, and safety checks.

Networking
- Use gRPC or QUIC for node-to-node communication.
- Test with a 3-node cluster and automated leader failover.

Testing
- Unit tests for individual components.
- Integration tests for end-to-end behavior.
- Chaos tests to simulate node failures and network partitions.

Benchmarking
- Benching against `etcd` to measure latency and throughput.
```
