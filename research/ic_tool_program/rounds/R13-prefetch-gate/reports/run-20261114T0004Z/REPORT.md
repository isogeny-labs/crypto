# R13 replacement cohort: rejected candidate

Measured on instance `i-06dc588064a36bd57`, Intel Xeon 6975P-C, with native isolation on SMT siblings 2,18, Rust 1.99.1, one Rayon thread, and canonical same-process signed-Frobenius rho. The frozen [protocol](../../PROTOCOL.md) and threshold were unchanged. This cohort is separate from all earlier hosts.

Five A/A rounds completed with 220 clean groups across 292 attempts. Five paired suite rounds produced 399 clean groups of 400 across 493 attempts. Candidate K1/F2^59 M1-T01 round 5 exhausted all three attempts under contention; it is censored, not replaced. Five holdout rounds completed with 240 clean groups across 295 attempts. All failed and contended attempts remain in `runs.tar.xz`.

Cold base/candidate ratios (geometric means and native 95% intervals; 40 pairs per primary set):

| Field degree | Suite | Holdout |
|---|---|---|
| 37 | 0.995486 [0.992043, 0.998941] | 0.999403 [0.996952, 1.001859] |
| 47 | 0.997786 [0.995150, 1.000429] | 0.998734 [0.993303, 1.004195] |
| 57 | 0.994377 [0.987760, 1.001039] | 0.990503 [0.984583, 0.996458] |

Acceptance requires every primary interval above 1.01 plus all other protocol gates. Native decision is rejected; n47 also regresses beyond its A/A band. All six interval half-widths are below 3%; no extension was called for. This is not an improvement over the frozen baseline. Table A and canonical leaderboard graphs remain unchanged because no promotion is admitted.

`analysis.json` is the native analysis. `recompute.json` independently reproduces 114 headline fields with zero disagreements. `certverify.json` passes 1,170 raw records and 2,340 scalar certificates, including contended attempts. `replay.json` reproduces four seeded records on a different local ARM build; that replay verifies outputs, not timings or exact Linux binary reproduction. Both replay aliases used the same local binary, SHA-256 707d8fcb41a8a1ca616bc94c0cdae722b0eaec154670d09f09cd57a189ebb741. This is a limited output replay, not a completed exact-arm independent validation gate. No VALIDATION.json or leaderboard promotion is issued.

Raw custody: `s3://setec-astronomy-artifacts/checkpoints/crypto/codex__ic-single-target-tournament-20261026/20261114T0504Z-measurements/runs-host-06dc5880/`. The worker was terminated after measurements completed. Native certificate verification and independent recomputation ran locally after recovery.

Next: assess a separately frozen implementation change against the retained baseline; do not retune R13 after these observations. Exact-arm replay and full publication validation remain open. Historical report `../run-20261027T1439Z/` describes a distinct incomplete cohort and is not pooled here.

Delivery verification: `cargo test --release --lib -- --test-threads=1` passed 4,849 tests, with 121 ignored. The initial parallel check and failures remain in release-lib-check.log. Post-measurement fixes distinguish explicitly weighted comparison identities, use the macOS true executable in GPU fixtures, and canonicalize checkpoint roots before containment checks. These fixes were not used to generate this frozen cohort and do not reprice its records. The unrelated source fixes are delivery prerequisites, not R13 optimizations.
