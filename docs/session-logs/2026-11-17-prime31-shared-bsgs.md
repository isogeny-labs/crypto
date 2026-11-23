# Session handoff — 2026-11-17 prime31 shared precomputation benchmark

## 2026-11-17 ~06:25 UTC
### What was done (commands, files, features)
Reviewed AGENTS.md, docs/ACTIVE.md, newest 2026-11-17 session log and PR #4513's exact-source Rust source/report, #4524/#4525 concurrent work, #4503/#4466 and prior BSGS source. Prior work confirmed: full 1024-target mixed IC 2684.798ms vs native unfolded rho32 3017.608ms (producer-only), but no shared-advice generic comparator. Created tracking branch and draft PR #4532. Literature review: Shanks BSGS reuse of baby table, published APS 2018 and repo precomputed-rho correction. KB MCP is unavailable here, fallback used.
### Decisions made (and why)
Freeze matched reusable BSGS with m=46341,262144,1048576 and same 1024 Q; original Rust Jacobian arithmetic, full setup/memory charged, no selection based on favorable timings. Generic cost frontier determines whether earlier apparent IC advantage survives a fair comparator.
### Open threads
Implement exact-source append-only BSGS suffix and CI workflow; execute checks, retain raw output, independently replay answers; report comparison and try nonenumerative RR9/S9 next.
### Errors
No Rust toolchain in this runtime; source-specific GitHub Actions required for compilation and actual measurements. No fresh result or speedup yet. [pcc:unverified]

## 2026-11-17 ~06:29 UTC — exact-source compilation defect (before measurement)
### What was done (commands, files, features)
CI [#44031421068](https://github.com/isogeny-labs/crypto/actions/runs/44031421068) completed 40/40 original native library and 3/3 reusable IC example tests, then failed BSGS suffix compilation at line 4527: local counter is named `batches` but struct initializer requested `baby_batches`. No 1024Q BSGS or IC comparison measurement ran. Source-locked native build itself succeeded.
### Decisions made (and why)
Fix only the local field binding `baby_batches: batches`; leave frozen m, Q, protocol, seeds and other arithmetic exactly unchanged. Preserve failed CI and run again on newly pinned source.
### Open threads
Source-specific BSGS tests and full 1024Q measured result, raw audit and comparison.
### Errors
Rust E0425 missing local symbol; correction is variable binding only. [pcc:unverified]

## 2026-11-17 ~06:31 UTC — passing full matched generic control and independent original-affine replay
### What was done (commands, files, features)
- Exact-source CI [#44031618724](https://github.com/isogeny-labs/crypto/actions/runs/44031618724) **SUCCESS** at frozen amended SHA `df661dee03446583f232998c95eb326f4dab5ae1`: 40/40 native library tests, 3/3 reusable IC tests, 23/23 original-source appended BSGS example tests; 1024/1024 IC, 1024/1024 each of 3 BSGS sizes and 1024/1024 each of 2 native rho arms, rho A/A 64/64. This success follows the recorded compile failure E0425 at earlier source, repaired by one local batch counter binding.
- Full charged 1024-Q costs (ms, exact same runner, p=2147483647, n=2147446831): IC setup 1315.438328 + target 906.035433 = 2221.473761; unfolded rho32 2246.126471; folded 4360.294154; shared BSGS m=46341: setup2.878276+online985.152533=988.030809; m=262144:22.911989+193.298591=216.210580; m=1048576:103.839846+66.273588=170.113434. All targets correct. BSGS/IC fully charged at 1m m=0.07658, **generic BSGS ~13x cheaper**. IC/rho32 this runner only 0.98902 vs older run 0.88971; host variability. RSS after setup BSGS 12288/27504/67924 KiB (not isolated peaks).
- Downloaded GitHub raw 3-file ZIP artifact #14305472259; ZIP SHA256 `dcf190695bfd9b939f5316b180f01db2beb20c243beed3f312923b33b4326cb2`; separately implemented original-affine EC arithmetic for arithmetic-only independent audit. Verified original generator order, [k]G=Q for all 1024 unknown targets in IC, three BSGS arms and two rho arms, 61/61 basis logs, 61 setup equations, 1024 target relation rows, 64 rho A/A, 20,000-target-resample paired intervals with setup fully charged. Audit JSON and report committed in research/prime31_ic_20261115/mixed_base/.
### Decisions made (and why)
- The previous *11.03% versus particular native rho32* multi-target advantage is real for its recorded run but **does not establish a frontier advantage**: fair generic shared-precomputed BSGS beats it by a large factor in this new same-run comparison. The index calculus is still generic meet-in-the-middle equivalent, so do not claim a novel non-generic ECDLP solver.
- Move scarce research effort to nonenumerative target-conditioned RR9/S9 extractor from public R, exact B4/B8 solution fibres and charged complete B31 cost. Also explore smaller B=28/29/30 and batched complement lookup as distinctly *generic* cost improvements, not previously missed mathematics.
### Open threads
- Signed independent PCC review, repeated isolated A/B/A hardware, RSS/space-matched advice and external precomputed rho / mature external IC benchmarks; no leaderboard promotion.
- Actual public-R polynomial elimination implementation and Rust end-to-end 2^31−1 solver with no generic half-table oracle.
### Errors
- None after repaired E0425; producer-level Rust success and independent arithmetic are not signed cryptanalytic validation. [pcc:unverified]
