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