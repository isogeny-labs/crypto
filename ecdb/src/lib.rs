//! ecdb: a Postgres catalogue of elliptic curves, higher-genus curves,
//! Jacobians and the maps between them, with an HTTP API for agents.
//!
//! The schema is in `migrations/` and explained in `docs/SCHEMA.md`; the API
//! is described by `openapi.yaml`. Identity conventions are the repository's:
//! ICV1 (`docs/curves/ICV1.md`) names a curve model, EC1
//! (`docs/curve-identities.md`) a representation with subgroup and generator.

pub mod api;
pub mod db;
pub mod field;
pub mod import;
pub mod json;
pub mod lambda;
pub mod math;
pub mod migrate;
