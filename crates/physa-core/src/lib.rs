//! # physa-core
//!
//! Core storage engine and graph data structures for physa-db.
//!
//! This crate is intentionally empty at M0. It will grow to contain:
//! - The property graph data model (nodes, relationships, labels, properties).
//! - The on-disk storage engine (WAL, page cache, B-tree / columnar adjacency).
//! - The transactional layer (MVCC).
//! - The index manager.
//!
//! See `docs/architecture/` for design.
