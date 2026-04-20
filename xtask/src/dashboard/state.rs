//! Serde types for `dashboard/data/state.json`.
//!
//! The schema is the contract exposed to `dashboard/src/app.js`; keep
//! field names synchronised with the JS consumer (camelCase via
//! `#[serde(rename_all)]`).

use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct State {
    pub generated_at: String,
    pub stats: Stats,
    pub milestones: Vec<Milestone>,
    pub activity: Vec<ActivityEvent>,
}

#[derive(Serialize, Debug, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    pub open_issues: u32,
    pub in_progress: u32,
    #[serde(rename = "doneLast30d")]
    pub done_last_30d: u32,
    pub current_milestone: String,
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct Milestone {
    pub name: String,
    pub title: String,
    pub done: u32,
    pub total: u32,
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct ActivityEvent {
    pub at: String,
    pub kind: String,
    pub title: String,
    pub url: String,
}
