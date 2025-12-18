Project : Raft distributed key-value consensus engine.
Tools : Rust, Docker.
Scope : Single-cluster key-value store (no-sharing)
Guarantees : Linearizable reads, fault tolerance (suvives <=(N-1)/2 failures)
OutScope : multi-datacenter, complex queries (just `get/pull/delete`)

---

- Single-Node KV Store :
    - In-memory `HashMap` + disk persistence (e.g. `rocksdb`).
    - CLI for `get/pull/delete`.

- Raft Core :
    - Leader election, log replication, and safety checks.
    - Chaos testing (kill node(s), partition network).

- Networking
    - gRCP (`tonic`) or raw TCP with `tokio`.
    - 3-node cluster testing with automated leader failover.

- Persistance + Snapshotting
    - Log compaction to avoid OOM
    - Node restarting and state recovery testing

- Benchmark
    - Latency/throughput vs `etcd` (baseline).

---

Language : Rust
Async : `tokio`
Networking : `tonic` (gRCP) or `quinn` (QUIC)
Persistence : `rocksdb` or `sled`
Serialization : `prost` (Protobuf) or `bincode`
Logging : `tracing` + `tracing-subscriber`
Testing : `tokio-test` + custom chaos tests
CLI : `clap`

---

Rust : Safety for consensus-critical code
`tonic`/`quinn` : Production-grade networking
`rocksdb` : Battle-tested for KV stores (used in `etcd`)
`tracing` : Debugging distributed systems is hard, good logs are vital
