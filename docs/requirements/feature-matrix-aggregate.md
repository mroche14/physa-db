# Feature Matrix Aggregate

> Menu of features seen across competitors. Attribution-free.

| AFM-NNN | Feature | Category | Seen in N | Typical implementation | Disposition |
|---------|---------|----------|-----------|------------------------|-------------|
| AFM-001 | Native property graph storage | Storage | 1 | Linked-list record format on disk, page-cache dependent | redesign |
| AFM-002 | Declarative pattern-matching language | Query | 2 | Logical plan to volcano/pipelined execution | adopt |
| AFM-003 | Raft-based metadata and read replicas | Cluster | 1 | Raft for primaries, async replication for reads | adopt |
| AFM-004 | Vector similarity search index | Query | 1 | Embedded HNSW in memory / Lucene | redesign |
| AFM-005 | Fine-grained RBAC | Tenancy | 1 | Enforced at query planning level | adopt |
| AFM-006 | Distributed Massively Parallel Processing (MPP) | Cluster | 1 | Custom distributed C++ engine | adopt |
| AFM-007 | Compiled Query Language | Query | 1 | Compiled down to C++ | redesign |
| AFM-008 | High-velocity Bulk Ingest | Ops | 1 | Distributed loading into memory structures | redesign |
| AFM-009 | Aggressive Data Compression | Storage | 1 | 2x-10x compression | adopt |
| AFM-010 | In-Memory C++ Engine | Storage | 1 | In-memory structures with snapshot isolation | redesign |
| AFM-011 | Analytical Storage Mode | Storage | 1 | Drops transaction guarantees for raw speed | non-goal |
| AFM-012 | Streaming First Integration | Ops | 1 | Deep integrations with Kafka/Pulsar | adopt |
| AFM-013 | Decoupled Compute and Storage | Cluster | 1 | Stateless query nodes, stateful storage nodes | redesign |
| AFM-014 | Generic KV-Store Backend | Storage | 1 | LSM-tree (RocksDB) | non-goal |
| AFM-015 | Hard Memory Tracker Aborts | Ops | 1 | Aborts queries when hitting memory high watermark | non-goal |
| AFM-016 | Proprietary Query Language | Query | 3 | Custom language alongside standards | non-goal |
| AFM-017 | Native Multi-Model Storage | Storage | 1 | Handles documents, graphs, and KV in one engine | non-goal |
| AFM-018 | Embedded JavaScript Microservices | Ops | 1 | V8 engine runs within the DB process | non-goal |
| AFM-019 | Native GraphQL Interface | Query | 1 | Parses GraphQL directly to internal query plans | non-goal |
| AFM-020 | Sparse Matrix Algebra Backend | Query | 1 | GraphBLAS matrix multiplication traversals | non-goal |
| AFM-021 | In-Memory Dual-Representation Storage | Storage | 1 | Optimized in-memory structures | redesign |
| AFM-022 | Sub-millisecond Cold Starts | Ops | 1 | Fast startup for serverless | adopt |
| AFM-023 | Zero-ETL Virtualization Layer | Storage | 1 | Translates graph queries to remote SQL/Data Lakes | non-goal |
| AFM-024 | Vectorized In-Memory Compute | Query | 1 | Apache Arrow based execution | adopt |
| AFM-025 | Local Storage Cache | Storage | 1 | Caches remote data locally | redesign |
| AFM-025 | Local Storage Cache | Storage | 1 | Caches remote data locally | redesign |
| AFM-026 | Worst-Case Optimal Joins (WCOJ) | Query | 1 | Leapfrog triejoin variants | adopt |
| AFM-027 | Factorized Execution | Query | 1 | Compressed intermediate query representations | adopt |
| AFM-028 | Embedded Columnar Graph Engine | Storage | 1 | DuckDB-style embedded OLAP graph | adopt |
| AFM-029 | Pure OLAP-only Focus | Ops | 1 | Sacrifices OLTP for pure OLAP speed | non-goal |
