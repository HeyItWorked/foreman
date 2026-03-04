//! # foreman
//!
//! A mini Celery-style async task queue.
//!
//! ## Architecture
//!
//! ```text
//! ┌──────────────────────────────────────────────┐
//! │                   foreman                    │
//! │                                              │
//! │  POST /jobs ──► JobStore ──► WorkerPool      │
//! │                    │             │           │
//! │  GET  /jobs/:id    │         tokio tasks     │
//! │         ▲          │        (bounded by      │
//! │         └──────────┘         Semaphore)      │
//! └──────────────────────────────────────────────┘
//! ```
//!
//! - `types`  — Job, JobStatus, HTTP request/response shapes
//! - `store`  — thread-safe in-memory store (Arc<DashMap>)
//! - `worker` — worker pool, handler trait, retry logic
//! - `api`    — Axum router, handlers, error types

pub mod api;
pub mod store;
pub mod types;
pub mod worker;
