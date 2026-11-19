# 2026-11-19 Prime31 RR9 Grassmann chart session

## 2026-11-19T03:28:00Z — preregistration and prior evidence

### What was done (commands, files, features)
- Read root AGENTS.md, docs/ACTIVE.md and the PR #4776 2026-11-19 session log; reviewed prior PR #4466, #4692, #4728, #4772, #4773, #4774, #4776 and concurrent #4779.
- Checked the previous native exterior-4 Rust code and source-specific workflow. Inspected published prime-field index-calculus prior art PKM 2016, APS 2018 and Kudo et al. hybrid elimination summary. Bounded GitHub searches for Grassmann chart and Schur-complement RR9 found no exact prior implementation.
- Created task branch and draft PR #4780, recorded task in docs/ACTIVE.md and preregistered the prospective experiment before code/measurement.

### Decisions made (and why)
- Test a fixed four-plane graph-chart Schur complement: normalized 4x4 blocks need 16 coordinates per half, and six 2x2-minor Laplace terms need 30 field multiplications per pair, versus 70-term exterior-4 contractions.
- Keep exact all-pairs exhaustive B4 negative fixture and identify this as a relation-predicate component, not a nonenumerative unknown-index solver.
- Avoid duplicating the separate #4779 fixed-linear Plucker span audit and #4692 unknown-index extractor.

### Open threads (unfinished, pick up next time)
- Implement native Rust chart computation, synthetic and real elliptic-curve identities, B4 positive/negative extraction, independent Python BigInt verification and source-pinned GitHub Actions.
- Inspect CI; record actual operation counts, elapsed and failures, then update PR and research protocol outcome.
- Signed independent PCC remains pending.

### Errors (what went wrong, how it was handled)
- Shared research-KB connector/CLI not present, so used bounded repo search and primary literature.
- Main did not have docs/session-logs/2026-11-19.md; recorded this task in a dedicated dated session log to avoid overwriting other branches.

## 2026-11-19T03:40:00Z — source-specific paired benchmark completed

### What was done (commands, files, features)
- Appended the exact finite-field 4-plane graph chart, 30-field-mul 2+2 Laplace determinant check, fail-closed singular-chart status and B4 extractor to the existing Rust RR9 exterior module; added Rust example wrapper and independent Python BigInt verifier with 62 original-curve determinant-residue identities.
- Added source-specific GitHub CI with pinned unchanged native Prime31 arithmetic 1df714645aa683cf975bc76fcacb9842cb4780fd. Initial sparse checkout omitted src/cryptanalysis/f4_fp_tower.rs and failed original lib compilation; fixed pinned checkout to include src/cryptanalysis. Successful source-bound CI #44452361338 passed 40 original library + 4 focused tests and independent Python verification.
- Expanded CI to two complete paired executions per algebraic method, in E-G-G-E order. Source-bound execution #44452737502 succeeded at source 122f6d9c830d4111c0c3261eb2209fc24743858c with 40/40 library, 4/4 example and independent Python B4 witnesses, full negative and 62 original-curve residue checks.
- Downloaded original raw artifact #14496176710 (SHA256 e3fe6eeee64d2b9bf21b913bcd6173788ed9b6b7daa164ff58a8d65c3b7cd1fa). B4 fully-exhaustive negative R=G: graph chart 1444.170124 and 1446.774604 ms; exterior4 2025.929563 and 2022.829508 ms. Mean graph 1445.472364 vs exterior 2024.379536 ms; 1.4005x component speedup/28.60% less elapsed. All 16,777,216 negative pairs fully scanned in each run. Positive all-zero first-pair fixture: graph 5.426871 and 5.352730 ms, exterior 289.090905 and 289.880874 ms; deliberately not a natural-target yield measurement. Independent oracle returned correct witness/negative for all arms.
- Froze full machine JSON RR9_GRAPH_CHART_SUMMARY_20261119.json and mathematical scientific report RR9_GRAPH_CHART_RESULT_20261119.md with exact B8 product barrier and prior-lead references.

### Decisions made (and why)
- This algebraic 30-product graph predicate improves B4 candidate negative-scan wall time and vector memory, but retains O(B^8) pair search and cannot substitute for public-only nonenumerative unknown-index selection in #4692. Do not scale naive B8: 128,849,018,880 Fp pairwise products per exhaustive negative query, before preprocessing and inversions. Do not claim full ECDLP speedup or novel Schur identity.
- Investigate the rank-one graph update on one signed leaf as a prospective mathematical lever; coordinate with #4692 and avoid duplicating #4779's fixed-linear Plücker-rank scope.
- Keep signed PCC distinct from independent Python development oracle; [pcc:unverified].

### Open threads (unfinished, pick up next time)
- Unseen B8 public-target-only nonenumerative signed-factor-base recovery on frozen 46 positives and 18 negatives, fully costed, remains with #4692; test rank-one updates only if they produce an actual reduced search algorithm.
- Once this succeeds, integrate with the existing four-B8-window rank31/1024 Rust solver #4728 and same-runner strongest index calculus, shared BSGS and rho controls. Signed PCC validation still pending.

### Errors (what went wrong, how it was handled)
- First pinned sparse checkout omitted original shared tower F4 module `src/cryptanalysis/f4_fp_tower.rs`; repaired the checkout paths and reexecuted CI. Earlier CI evidence remains recorded as failure, not treated as a valid benchmark.
- No errors in final source-bound A-B-B-A (exterior-graph-graph-exterior) execution. Shared research KB not accessible; bounded literature/cross-repository code and PR search used instead.
