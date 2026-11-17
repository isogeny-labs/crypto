# Validation and remaining gates

Date: 2026-10-21. Scope: documentation, diagram and experiment planning only.

| Requirement | Evidence / state |
| --- | --- |
| Preserve five paper-derived ideas | README DGM-01 through DGM-05; all labeled proposed or awaiting reproduction |
| Identify overlap | Existing repository notes/catalog and PR pointers; folding is not presented as novel |
| Specify controls and acceptance | Per-workstream acceptance and matched single-target measurement contract |
| Record actual experiment activity | No experiments, run IDs, performance results or new certificates |
| Render the report | `make` passed with CairoSVG, Pandoc and XeLaTeX; all PDF pages and the embedded diagram inspected |
| Validate execution readiness | Not ready: fixture IDs, native harness/checker and frozen protocol are pending |

The host does not provide Cargo or Conductor. The attempted Conductor
preflight returned `conductor: command not found`. Work is isolated to a new
study directory and, in crypto, a new blocked backlog card; cryptanalysis adds
only a discoverability link to its catalog README.

The required native `cargo test --release --lib` and `backlog validate`
cannot be run on this host. Document/JSON checks do not substitute for those
gates. No native build, solver correctness test, performance benchmark or Lean
proof was run. Native validation and required CI/review remain merge gates.
The planning PR may be reviewed separately from future execution.

JSON Schema Draft 2020-12 validation of the blocked card passed against the
repository's `backlog/schema/card.v1.json`. Targeted metadata checks also
passed for directory/ID/date agreement, decision-value length, typed blockers
and zero funding. This does not replace native `backlog validate`.
