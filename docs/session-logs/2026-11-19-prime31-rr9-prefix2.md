# 2026-11-19 UTC — Prime31 two-prefix RR9 original-curve gate (#4773)

## What was done (commands, files, features)
- Read `AGENTS.md`, `docs/ACTIVE.md`, the newest main session log, and live PRs #4692, #4728, #4770 and #4772, plus older #4548 and negative Prime31 factor-base work #4652. Reviewed `crypto-autoresearcher` SA-04/SA-18 (norm/determinant failures), SA-19 (Edwards alternative coordinate), `cryptanalysis` auxiliary prime-field IC, and published PKM 2016 / APS 2018. Shared research-KB MCP/CLI unavailable; bounded direct GitHub and paper audit only.
- Created scoped branch `automation/prime31-rr9-prefix2-cost-20261119`, committed task line to `docs/ACTIVE.md` first, and opened draft PR #4773. Froze a prospective mathematical and experimental protocol before candidate source.
- Implemented a native Rust two-signed-prefix affine elimination inside the existing original pinned Prime31 arithmetic crate, not a new independent ECC implementation: 7 original RR9 chart unknowns become 5 free parameters once two **supplied signed leaves** are fixed. The remaining six public-support norm-resultant equations have degree at most 16, and the candidate computes their exact 6×5 Jacobian. Algebraic candidate has no generic 4+4 group sum tables.
- Added a separately authored Python BigInt elliptic-curve and dual-number verifier for every returned fixture and derivative. Added a source-bound GitHub Actions workflow pinning original crate at `1df714645aa683cf975bc76fcacb9842cb4780fd`, Rust 1.100.0, and the independently replayed example tests, plus raw JSON archive.
- Local preflight (not CI or signed PCC): 16 original-curve planted signed relations per old/ternary arm, 32/32 full-chart reconstructions and norm zeros, 32/32 full-rank 6×5 Jacobians. No public unknown-index recovery.
- Exact combinatorial prefix table: at 0/1/2/3/4 known signed B8 prefixes the branch counts are 1/16/256/4096/65536 and dense degree-16 monomial counts in the remaining 7/6/5/4/3 variables are 245157/74613/20349/4845/969. This is not measured F4 runtime and not an F4 complexity lower bound.
- First source-specific CI #44432181808 failed **before candidate tests**, because sparse-pinned checkout omitted the original crate dependency `src/cryptanalysis/f4_fp_tower.rs`. Corrected workflow at `17aa927d4afd4b173c70b5b19e1cabb503d6501c`; source-specific rerun #44432429016 was queued/in progress as this entry was recorded.

## Decisions made (and why)
- This is a **conditional, algebraic** partial-guess approach and must never be called public-only unknown-index extraction. The candidate requires two signed prefix leaves; a real attack would pay for all 256 B8 guesses unless a nonenumerative finder is implemented.
- Keep comparison to prior fully charged generic-grader 1024-log controls separate from a non-generic solver result. #4770 reported ternary 9.128s vs old 11.516s on the same runner; no same-runner matched fastest BSGS/rho claim.
- No paid compute, no merged PR, no complete ECDLP advantage or signed PCC claim. Keep the report source-scoped and negative outcomes visible.

## Open threads (unfinished, pick up next time)
- Inspect rerun #44432429016, correct any Rust compilation or Python replay failures and preserve exact raw evidence.
- With a clean source-specific replay, commit a scientific report and amend this session record with code SHA, CI ID, run count, source-specific 32/32 tests and scope.
- Implement an actual public-target-only unknown-index polynomial solver with no four-sum generic lookup on the sealed 46-positive / 18-negative B8 panel, then integrate rank31, 1024 blinded target recoveries and matched cold index-calculus / shared BSGS / optimized rho comparisons.
- Measure genuine F4 degree, row/column counts, memory and full branch costs for t=0,1,2 instead of using dense monomial counts as a proxy. Run parameter ladder to assess scaling.

## Errors (what went wrong, how it was handled)
- First source-bound CI failed at original lib compilation due to sparse-checkout missing a relative-path source; patched sparse checkout to include `src/cryptanalysis/f4_fp_tower.rs`. Candidate Rust and dual-number replay had not yet been exercised by that failed execution.
- Shared KB, EKS and local Rust compiler unavailable in this tool runtime; used direct GitHub repository sources and launched the exact-source GitHub Actions checks.
