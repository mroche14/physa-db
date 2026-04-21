# Feature Matrix Aggregate

> Menu of features seen across competitors. Attribution-free.

| AFM-NNN | Feature | Category | Seen in N | Typical implementation | Disposition |
|---------|---------|----------|-----------|------------------------|-------------|
| AFM-001 | Native property-graph storage | Storage | 2 | Custom graph-native store optimized for adjacency-local reads | adopt |
| AFM-002 | In-memory-first graph storage | Storage | 1 | RAM-resident graph structures with durability layered underneath | redesign |
| AFM-003 | Embedded / library deployment mode | DX | 1 | Linkable database library with local file-backed state | adopt |
| AFM-004 | Generic KV-backed graph storage | Storage | 3 | Graph entities encoded over an LSM or KV engine | non-goal |
| AFM-005 | Sparse-matrix / GraphBLAS graph representation | Query | 1 | Adjacency stored and queried as sparse matrices and algebraic expressions | non-goal |
| AFM-006 | Columnar analytical graph storage | Storage | 1 | Disk-backed columnar property storage with CSR-style adjacency | adopt |
| AFM-007 | Native multi-model document+graph+KV engine | Storage | 1 | Single engine spans document, graph, and key-value workloads | non-goal |
| AFM-008 | Zero-ETL graph virtualization | Storage | 1 | Query graph structure over external systems without importing data | non-goal |
| AFM-009 | Local cache over external data | Storage | 1 | Materialize or cache remote graph-shaped data close to execution | non-goal |
| AFM-010 | WAL + snapshot durability | Storage | 2 | Persist mutations through a write-ahead log plus periodic durable checkpoints | adopt |
| AFM-011 | Consensus-backed cluster coordination | Cluster | 4 | Replicated cluster state with leader election and durable quorum updates | adopt |
| AFM-012 | Read-scale secondaries / replicas | Cluster | 1 | Route read workloads to non-writer replicas or secondaries | adopt |
| AFM-013 | Shared-nothing MPP query execution | Query | 1 | Move graph computation across cluster workers instead of centralizing it | redesign |
| AFM-014 | Separated compute and storage | Cluster | 2 | Independent query and storage tiers with networked data access | redesign |
| AFM-015 | ACID transactions | Storage | 6 | Transactional reads and writes with durable commit semantics | adopt |
| AFM-017 | Fine-grained RBAC | Tenancy | 4 | Privileges scoped to databases, labels, operations, or objects | adopt |
| AFM-018 | Logical multi-tenancy / namespaces / multi-database | Tenancy | 6 | Separate databases or namespaces within one cluster or process | adopt |
| AFM-019 | SSO / LDAP / enterprise authentication | Tenancy | 1 | External identity providers and centralized auth integration | adopt |
| AFM-020 | openCypher compatibility | Query | 6 | Cypher parser, planner, and runtime compatibility for migration ease | adopt |
| AFM-021 | GQL support | Query | 1 | Native ISO GQL surface or credible rollout toward it | adopt |
| AFM-022 | Proprietary graph query language | Query | 3 | Vendor-specific graph DSL or language extensions as primary surface | non-goal |
| AFM-023 | GraphQL-native API surface | Query | 1 | Database directly exposes GraphQL schema and operations | non-goal |
| AFM-024 | Vector similarity index | Query | 5 | Built-in ANN structure such as HNSW for embedding retrieval | adopt |
| AFM-025 | Hybrid graph + vector query composition | Query | 4 | Compose ANN retrieval with graph expansion and filtering in one plan | adopt |
| AFM-026 | Full-text search | Query | 5 | Integrated text index and ranking inside the database query surface | adopt |
| AFM-027 | Geospatial search | Query | 3 | Native geo types or geo indexes in graph-aware queries | adopt |
| AFM-028 | Classical graph algorithms library | Query | 7 | Centrality, community-detection, pathfinding, and connectivity algorithms run over the stored graph or a tightly coupled analytics surface | adopt |
| AFM-029 | Streaming ingest / CDC connectors | Ops | 2 | Native ingestion from message buses or change streams | adopt |
| AFM-030 | High-speed bulk import | Ops | 5 | Dedicated large-scale loader separate from transactional writes | adopt |
| AFM-031 | Backup / restore / disaster recovery | Ops | 3 | Operational tooling for snapshot, restore, and recovery workflows | adopt |
| AFM-032 | Managed cloud service | Ops | 4 | Vendor-operated hosted deployment path | adopt |
| AFM-033 | Operator / cloud-native cluster control plane | Ops | 3 | Kubernetes operator or comparable cluster automation surface | adopt |
| AFM-034 | Schema-flexible start / optional schema enforcement | Storage | 2 | Allow writes before full schema lock-in, then layer stronger checks | redesign |
| AFM-035 | Strongly typed / schema-driven storage layout | Storage | 1 | Physical encoding depends directly on declared schema metadata | redesign |
| AFM-036 | Custom procedures / UDF / module system | Query | 1 | Extend query execution with user-defined native or scripted code | adopt |
| AFM-037 | Query compilation / generated code | Query | 1 | Compile query logic into generated or native code paths | adopt |
| AFM-038 | Factorized execution | Query | 1 | Compressed intermediate/result representations reduce blow-up | adopt |
| AFM-039 | Worst-case-optimal / novel join algorithms | Query | 1 | Join engine optimized beyond traditional binary join trees | adopt |
| AFM-040 | Vectorized / pipeline-parallel analytical execution | Query | 1 | Pipelines, morsels, and vectorized operators for analytical scans | adopt |
| AFM-041 | Hard memory-abort guardrails | Ops | 1 | Protect process stability by aborting work at memory thresholds | non-goal |
| AFM-043 | Embedded application runtime inside DB | Ops | 1 | Run user application code inside the database process | non-goal |
| AFM-044 | Native vector / embedding value type | Query | 2 | First-class typed embedding values instead of raw numeric lists | adopt |
| AFM-045 | AI / agent integration surface | Tooling | 2 | Schema introspection, MCP endpoints, or agent-facing database tools | adopt |
| AFM-047 | Workload-isolated analytical workspaces | Cluster | 1 | Separate read-write and read-only or analytic execution surfaces | redesign |
| AFM-048 | Pricing or licence gating of production features | Packaging | 3 | Clustering, security, or ops limited to paid editions | non-goal |
| AFM-049 | Browser / visual studio / schema designer tooling | Tooling | 5 | Integrated GUI for querying, modeling, and exploration | adopt |
| AFM-050 | Permissive OSS licensing | Packaging | 3 | Apache/MIT-style licensing for production use and redistribution | adopt |
| AFM-051 | Restrictive source-available / fair-code licensing | Packaging | 4 | SSPL, BSL, non-commercial, or similar restrictions | non-goal |
| AFM-053 | Arrow-native / columnar in-memory compute | Query | 1 | Execution layer leans on Arrow-style columnar batches | redesign |
| AFM-054 | Redis-module / protocol adjacency | Packaging | 1 | Graph engine runs as a module or adjacent surface over Redis | non-goal |
| AFM-055 | Gremlin compatibility | Query | 1 | Support Gremlin as an additional graph query language | non-goal |
| AFM-056 | Graph over external SQL / lakehouse tables | Storage | 1 | Map graph abstractions onto existing SQL or lakehouse tables | non-goal |
| AFM-057 | Analytical graph projection / graph catalog workspace | Query | 2 | Project a filtered graph into an in-memory or temporary analytical workspace before running algorithms | redesign |
| AFM-058 | Node / graph embedding algorithms | Query | 5 | Built-in embedding procedures such as node2vec or FastRP write vectors back into the graph | open |
| AFM-059 | Supervised graph ML pipelines | Tooling | 4 | Train node-classification or link-prediction models over graph-derived features and graph structure | open |
| AFM-060 | Python / notebook graph-data-science workflow | Tooling | 4 | Vendor-maintained Python or notebook APIs orchestrate projections, algorithms, and ML jobs | open |
| AFM-061 | Managed or isolated graph analytics workbench / sessions | Tooling | 3 | Separate analytics sessions or workbenches isolate heavy graph-ML jobs from the primary OLTP surface | open |
| AFM-062 | Third-party graph ML / analytics framework integration | Tooling | 4 | Adapters or data loaders bridge the graph store into PyG, DGL, NetworkX, cuGraph, Spark, or GraphX | open |
| AFM-063 | Graph ML model catalog / inference APIs | Tooling | 3 | Persist trained graph models and expose prediction or model-inspection APIs inside the DS surface | open |
