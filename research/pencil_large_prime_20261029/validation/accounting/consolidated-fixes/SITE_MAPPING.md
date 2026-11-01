# Consolidated finite-event corrections

This patch is relative to the root files copied into before/. source-manifest.json binds each changed file before/after. Source changes are confined to the isolated checkout. This is implementation/test evidence, not independent complete coverage approval.

| Site | Before | After |
|---|---|---|
| engine Ring::new | Unobserved is_prime(r) | Same deterministic check once via scalar_constructor_prime(r), before unchanged && r>2; separate scalar-domain counts |
| geometry Snapshot | Base constructor fields only | Four scalar_constructor_* fields included in ZERO, checked_since, checked_add, serialization and existing partition/replay attachment |
| Sparse get/remove/first_key_value | Excluded map_calls only | One map_entry_visits for each returned existing entry, none on miss |
| Sparse insert | Comparison/call only | Fresh retained entry: one collection initialization; replacement returning old value: one visit, no initialization |
| Set insert/extend/from_iter | Comparison/call only | Exact fresh retained-entry count; insert boolean / size difference / final size; original std bulk algorithm unchanged; no membership prelookup |
| accumulate existing coefficient and graph recovered coefficient | Explicit copied() unobserved | One copied-element event only for Some; borrowed get retains separate entry visit |
| engine pdiv | Empty and length guards unobserved | One denominator-nonempty predicate and actual loop !empty/length predicates, preserving && short circuit |
| engine pexact | Remainder-empty assertion unobserved | One predicate event |
| residual cap checks | Working/retained length predicates unobserved | One event only when each original short-circuit branch evaluates its length comparison |

Scalar constructor M and S are expanded noninverse scalar events. They must be priced at native scalar weight, not base M/S weights. Calls are diagnostic; standalone reductions counted once; embedded product reductions not charged again. Separate importer/analyzer correction is owned by four_lead_state. The adapter existing engine geometry partition and accounting replay digest serialize these new fields automatically; tests verify separated valid/rejected constructor segments and their checked sum. No additional constructor call is introduced, no retry/cap algorithm is changed.

Validation (CARGO_TARGET_DIR=/tmp/pencil-primality-target):

- cargo test --manifest-path research/pencil_large_prime_20261029/Cargo.toml --features producer --lib: 48 passed,0 failed,1 ignored. producer-tests.log.
- cargo test --manifest-path research/five_leads_e2e_20261019/native/Cargo.toml --lib coverage_consolidated_tests: 2 passed,0 failed,21 filtered. engine-tests.log.

Exact regressions cover successful/missing/replaced/removed map entries; duplicate and bulk set insertion; explicit coefficient copy; working/retained cap short circuits; actual valid/rejected Ring constructor counts and disjoint base domain; phase partition serialization; empty polynomial guards. Final changed files rustfmt-formatted; three_leads used skip_children to avoid unrelated child edits. Constructor primality module has formatting-only changes from earlier geometry rustfmt; mathematical helper unchanged.

The initial generated Snapshot ZERO/test literal had a repeated-field typo, causing a local compile failure before any test ran. Corrected it; raw failure retained separately as initial-compile-failure.log. Final logs are unmodified bytes. No experiments, cloud work, root edits, or finished coverage receipt were produced.
