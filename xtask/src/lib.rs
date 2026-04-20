//! Library entry — exposes dev-task modules so integration tests under
//! `xtask/tests/` can exercise the pipeline directly. The `xtask` binary
//! in `src/main.rs` is a thin CLI wrapper over these modules.

pub mod dashboard;
