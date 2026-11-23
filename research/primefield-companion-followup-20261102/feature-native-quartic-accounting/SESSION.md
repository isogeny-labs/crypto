## 2026-11-05 — Quartic compiler and adaptive source accounting (T-1687)

### What was done (commands, files, features)

Instrumented eleven modules with exclusive compilation, calibration, preparation, initial geometry/seed/character, decoder, extension and verification stages. Three accounting tests pass; 120 inherited tests were filtered. All 20 public-point recoveries agree with the frozen source and pass forward multiplication. Source-map checks accept eleven forward/reverse transforms and reject 22 corruptions. Thirty-six source/data hashes bind the build. [pcc:unverified]

### Decisions made (and why)

Count modular-product helpers, field/scalar inverse calls, Euclidean iterations, popcounts, rule comparisons, characters and allocator events separately. Do not assign time weights or call this complete machine-work accounting. Filed unsigned execution claim PCC-20261105-a67f89; local verdict not_covered. Sources checked (KB): unavailable MCP and wrapper executable; used the previous accounting code, current source and Rust GlobalAlloc contract. [pcc:unverified]

### Open threads (unfinished, pick up next time)

Implement checked batch reciprocals for initial-window value-division initialization, retaining all added products and validation. Public-field enumeration and dense synthesis remain representation work. Original 4,563-case cohort unchanged. No timing panel or cloud job. Current-head CI/review pending.

### Errors (what went wrong, how it was handled)

First generator attempt rejected a duplicate fragment before native execution; deduplicated replacement matches and retained the failure note. Fourteen compiler warnings preserved. No native failure. Packaging initially omitted the first counter row because the test name prefixes it; corrected the parser, retained the initial projection and pinned derivation, and added DERIVATION-CORRECTED.md. Upstream journal merge conflict resolved by retaining main's entries and the previously published recovery entry.
