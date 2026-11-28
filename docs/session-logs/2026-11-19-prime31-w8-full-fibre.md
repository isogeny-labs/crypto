# Prime31 W8 full-fibre session log — 2026-11-19 UTC

## 13:16–13:30 UTC — Exact witness multiplicity diagnostic

### What was done (commands, files, features)

- Read AGENTS.md, main docs/ACTIVE.md, source original native README/RESUME, the previous #4807 study, the RR9/F4 owner #4692, ternary #4770, generic pipeline #4728, and past #4805/#4803/#4801/#4779/#4793/#4532. Reviewed open crypto-autoresearcher leads and Diem/PKM/APS literature. research-KB MCP/local wrapper were unavailable in this connector runtime.
- Continued the existing #4807 research lane instead of opening a competing PR to #4692.
- Pre-registered research/prime31_ic_20261115/rr9/W8_FULL_FIBRE_PROTOCOL_20261119.md in commit 50028eda452cd198d48c268d2a35f44e96d39627 BEFORE experimentation; seed 5748385f48455432, 1,024 public targets and four arms OLD, POW2, NEAR127 and TERNARY.
- Implemented original-crate Rust example native/examples/prime31_w8_full_fibre.rs with exact full ordered-witness counting via two generic four-slot EC multisets. Grader receives only public points. Added separately authored Python BigInt affine replay validator, a source-specific GitHub Actions workflow and all raw output checks.
- Source-specific GitHub Actions PASS: https://github.com/isogeny-labs/crypto/actions/runs/44533134056; source c099bbb821472986805ebb427c2987a9284cfca1. Original Rust 40/40 tests; focused 3/3; 4 × 1,024 public target tests; 16/16 independent affine signed-witness certificates; independent histogram, second-moment and paired comparison reconstruction. Raw artifact https://github.com/isogeny-labs/crypto/actions/runs/44533134056/artifacts/14534683345, raw JSON SHA256 5725afe43a35d6150a61748329a11a790de59cbf655b92d773327ad4b0af43f0.
- Measured OLD 795 hits / 2,116 ordered witnesses, POW2 877/2,131, NEAR127 873/2,018 and TERNARY 892/2,056 on matched 1,024 targets. OLD variance 4.814 and max 28; POW2 2.231/max8; NEAR127 2.036/max8; TERNARY 1.955/max7. Breaking scale-16 right-half symmetry with NEAR127 removed the W1 40 right-half collisions but did NOT improve public hit yield (4 fewer than POW2; 128 candidate-only vs 132 POW2-only).
- Committed scientific report W8_FULL_FIBRE_RESULT_20261119.md, machine-readable W8_FULL_FIBRE_SUMMARY_20261119.json, and this log.

### Decisions made (and why)

- Treat this as exact generic-table grading evidence ONLY, not public-only non-generic extraction or whole-solver advantage; do not promote to leaderboard or claim rho/BSGS parity.
- Deprioritize NEAR127 yield-only weight perturbation; retain TERNARY and POW2 as comparable arms for genuine nonenumerative F4.
- Proposed next source-scoped algebraic bridge: POW2 weighted eight slots satisfy U+[16]V=R. Four x-only doubling constraints are 4(x_i^3+a x_i+b)x_(i+1)=x_i^4-2a x_i^2-8b x_i+a^2, coupled to Semaev f3(x(U),x([16]V),x(R)). Odd prime group means no rational 2-torsion, hence no rational denominator-zero x. x-only roots can fail square-root liftability; test actual EC lift and signs. Private Python checked the known point identity on 128 points × four steps (512 checks), not a source-bound F4 benchmark.

### Open threads

- Implement genuine public-only sign-free weighted B8 RR9/F4 unknown-index extraction in current owner #4692; use frozen 46-positive/18-negative B8 corpus, zero generic sum-table access, full signed EC replay, recorded F4 degree/memory/failed queries.
- Integrate only a qualifying extractor into the original Rust rank31/1,024 complete solver, then compare same-runner cold and amortized costs against strongest index calculus, cached BSGS and optimized Pollard rho.
- Obtain independent signed PCC validation and keep all per-target raw results/source hashes. No complete non-generic Prime31 solver advantage was demonstrated.

### Errors (what went wrong, how it was handled)

- Fixed a nested mutable Rust arithmetic borrow and typed serde_json comparisons before CI; corrected source compiled and all tests passed.
- research-KB unavailable; used bounded GitHub code, PR and published literature review, no novelty-by-absence claim.
- No paid compute launched. Only GitHub Actions source-bound CI was used.
