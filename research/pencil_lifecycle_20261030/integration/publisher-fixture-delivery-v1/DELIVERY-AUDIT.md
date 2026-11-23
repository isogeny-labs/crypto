# Read-only scoped delivery audit

Three concrete build/delivery blockers found: fresh-owner Cargo.lock ignored/untracked despite --locked build; workflow build artifact upload omitted fresh-owner binary; tested-source minimal checkout omitted fresh-owner package. Parent has acknowledged and is addressing these. A fourth is the existing positive publisher fixture, which lacked the newly mandatory owner artifact/hash. This private patch fixes that fixture and adds owner sidecar and mutation coverage. Mock fixture passes; no actual AWS/Docker process is called because local mock executables override them.

The source checkout issue is supported by workflow materialization paths, scripts/sparse-checkout.sh CORE excluding taskq, and no root Cargo target/include supplying fresh-owner. Parent owns the separate workflow edit. Source V10 remained frozen; no builds or root edits were performed by this reviewer.

PR-NOTES correctly distinguishes V8 success, V10 still running, publication/container/protocol gates and incomplete science. Existing fresh native observation/same-UID, polling/deallocation and evidence-closure boundaries remain; this audit does not remove them. Final regression and exact PR CI remain required as already recorded. No additional concrete correctness blocker identified in the inspected packaging, schema, custody and campaign cutoff paths; this is scoped review, not an exhaustive proof of the whole runner diff.

Initial private fixture attempt lacked sparse l2-validation.json; exact HEAD bytes were restored privately, failure log retained, and v2 passed after the written software validation plan. No experiment protocol, timing result, cloud invocation, deployment or new mathematical claim was produced.
