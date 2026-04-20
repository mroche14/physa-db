# Feature Matrix Aggregate

> Menu of features seen across competitors. Attribution-free.

| AFM-NNN | Feature | Category | Seen in N | Typical implementation | Disposition |
|---------|---------|----------|-----------|------------------------|-------------|
| AFM-001 | Native property graph storage | Storage | 1 | Linked-list record format on disk, page-cache dependent | redesign |
| AFM-002 | Declarative pattern-matching language | Query | 1 | Logical plan to volcano/pipelined execution | adopt |
| AFM-003 | Raft-based metadata and read replicas | Cluster | 1 | Raft for primaries, async replication for reads | adopt |
| AFM-004 | Vector similarity search index | Query | 1 | Embedded HNSW in memory / Lucene | redesign |
| AFM-005 | Fine-grained RBAC | Tenancy | 1 | Enforced at query planning level | adopt |