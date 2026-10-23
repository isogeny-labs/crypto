//! isobench: a benchmark harness and frontier for isogeny problems, after ecbench (the ECDLP
//! harness of the parent repository): problems with deterministic instances and independent
//! checkers, a counted unit per problem, frozen paired specifications, sealed records, replay
//! audits, sealed bounds (cost = C size^alpha with bootstrap intervals), challenges with
//! verdicts, and a frontier page. See `docs/isobench/README.md`.
pub mod bounds;
pub mod counted;
pub mod harness;
pub mod problems;
