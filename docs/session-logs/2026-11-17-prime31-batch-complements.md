# 2026-11-17 — Prime31 Dickson31 batched-complement IC control

### What was done (commands, files, features)
- Inspected AGENTS.md, current active queue, latest source and frozen 98-Q report in #4615, current active #4596/#4604/#4614 and 2026-11-17 portfolio audit #4594.
- Searched existing repository batch inversion work and relevant PKM/APS prime-field references and Montgomery's trick. Branch `automation/prime31-dickson31-batched-complement-20261117`, draft PR #4618 stacked on #4615, first commit tracked the task in docs/ACTIVE.md.
- Frozen protocol written before candidate measurement. The proposed public target and base remain identical to the previous 98-Q Rust control.

### Decisions made (and why)
- Isolate an arithmetic-only batch-inversion optimization; report it as a component benchmark, not a new non-generic ECDLP algorithm. Reuse existing native Rust and avoid competing with full solver #4596 or public D9 #4604.
- Preserve all failed shift attempts and same-coordinate exceptional branches. Require matching witnesses and per-operation counts.

### Open threads (unfinished, pick up next time)
- Implement exact batched `R-L` affine complement in source-specific Rust, run pinned CI, read full JSON log, document measured outcome.
- Then pursue sign-independent, target-conditioned B31 algebraic extractor, full modular rank, unknown scalar recovery and matched IC/BSGS/rho frontier.

### Errors (what went wrong, how it was handled)
- GitHub repository cannot be locally cloned from the isolated container; source-specific GitHub Actions must compile and measure. No CI result claimed yet.


## Verified follow-up — 2026-11-17 source-specific Rust measurements

### What was done (commands, files, features)
- Implemented `batch_complement_tests` in existing `research/prime31_ic_20261115/native/examples/prime31_s9_b31_half_sum_oracle.rs`; frozen 512-way Montgomery simultaneous inversion, native fallbacks, exact same-sign witness and probe-order matching, 98-Q/109-attempt sample; added pinned Rust workflow `prime31-dickson31-batch-complements.yml`.
- Three successful source-bound workflows: #44179136645 and #44179154585 at `bec8c11`, and #44179525293 at `f062125` after additional common setup and residual instrumentation. Each passed 40/40 original library tests, 1/1 new example tests, unchanged 98 positives, 11 exhaustive misses and 18,049,427 incumbent probes.
- Persisted three complete JSON records at `research/prime31_ic_20261115/native/DICKSON31_BATCH_COMPLEMENTS_RAW_20261117.json`; wrote `DICKSON31_BATCH_COMPLEMENTS_RESULT_20261117.md` with 3 source-specific raw receipts, cost accounting and prior-lead assessment.

### Decisions made (and why)
- Measured incumbent attempts 5,944.999/5,824.300/6,916.734 ms vs optimized 1,337.701/1,271.526/1,070.154 ms: scoped 4.44–6.46x gains across three separate hosted runs, not calibrated cross-host speed.
- Final run charges 351.360 ms for original curve/base/4A setup, and 0.0318 ms for shared public residual generation: partial full common-phase costs 7,268.126 ms incumbent vs 1,421.546 ms candidate (5.11x). Target construction/test harness and all index-calculus linear algebra and unknown-logarithm recovery remain unmeasured.
- 512-way batching reduced complement arithmetic inversions from 18,049,990 to 35,979 and multiplications from 1,155,199,372 to 110,651,079 while processing 26,404 extra terminal-batch lanes. The classical Montgomery trick is prior art and this entire 4A+4A oracle remains generic equivalent; no non-generic exponent or cheaper complete solver established.
- Mark the scoped component task complete, leave PR #4618 stacked on unmerged parent #4615 for review, and do not change any scoreboard or paid compute.

### Open threads (unfinished, pick up next time)
- The next mathematical gate is **nonenumerative, public-target-conditioned signed RR9/S9 or K961/S5 extraction** for Dickson31 without building 4A group tables. Test positive/negative, exceptional/repeated-root and identity branches against the 98-case oracle transcript. Reuse existing F4/quotient/rootfinding Rust.
- Only after non-generic relations exist: independently scored relation rank and target logarithm recovery for 1,024 blinded Q, with all failures, setup and memory cost, against matched BSGS, optimized rho and external IC. Coordinate with #4596 and #4604 to avoid duplication.

### Errors (what went wrong, how it was handled)
- No compiler or test failures in the three focused Actions runs. First source-bound tests omitted charged shared setup timing, so an instrumentation-only edit measured it without changing curve, targets, shift cap, oracle or batch width. Separate GitHub reporting is producer-only [pcc:unverified]; artifacts were not independently rehashed.
