# Prime31 Dickson31 adaptive holdout — 2026-11-17

## What was done (commands, files, features)
- Inspected current AGENTS.md, docs/ACTIVE.md and prior Prime31 records and GitHub PRs #4612, #4596, #4604, #4532, #4513, #4581, #4550, cross-repo autoresearcher #3301/#3305 and cryptanalysis #1536. Verified exact prior B31 4A membership oracle and Dickson depth-six semantics in native Rust. KB MCP / repository wrapper not available in this connector execution; bounded repo and primary-paper fallback only.
- Created task branch automation/prime31-dickson31-adaptive-holdout-20261117, registered one active item, and opened draft PR #4615 before research writes.
- Prospectively froze DICKSON31_ADAPTIVE_HOLDOUT_PROTOCOL_20261117.md: original p=2147483647 cofactor-one curve, 96 new caller-blinded deterministic public Q from seed 0x533942484f4c4431, public shifts a=0..8, exact 4A+4A signed Dickson31 membership including certified exhaustive negatives, plus two frozen legacy no-support Q.
- Appended new isolated Rust test module to the existing prime31_s9_b31_half_sum_oracle.rs rather than rewriting arithmetic or altering historical main/test fixtures. The worker receives Q and public shifts, never caller-known k. Raw test JSON includes each attempted residual, membership, witness, probe count, original-curve operation counters, diagnostic wall ns, and unresolved cap status.
- Added source-specific Actions workflow using original native Rust pinned source commit 1df714645aa683cf975bc76fcacb9842cb4780fd, Rust 1.100, same original library suite, and a raw transcript artifact. Observed GitHub run #44169385150 as in progress at time of log; no pass/measurement claim yet.

## Decisions made (and why)
- This is a **generic oracle baseline and falsification control**, not an index-calculus decomposition algorithm. A classical 4A group table cannot be relabelled as non-generic. The real P0 research gate is recovering the RR9/S9 factor polynomial and signs from public R using Rust quotient / F4 with NO 4A point-sum enumeration.
- Frozen legacy Dickson31 24-Q oracle recorded 22 positives and two exhaustive negatives; need independent new synthetic Q, actual adaptive transform cost and separate non-support controls before extrapolating.
- The historical merged #4532 same-1024Q fully charged comparison is mixed IC 2221.474ms vs shared BSGS 170.113ms (producer, [pcc:unverified]); do not reframe the older rho-only 11.03% comparison #4513 as an IC-frontier victory.
- Separately owned #4596 owns complete batched IC vs shared BSGS benchmark and #4604 owns full sparse D9 rank; do not duplicate them. External papers: Petit-Kosters-Messeng PKC 2016 DOI 10.1007/978-3-662-49387-8_1; Amadori-Pintore-Sala FFA 2018 DOI 10.1016/j.ffa.2018.01.009.

## Open threads (unfinished, pick up next time)
- Read exact-source Actions #44169385150 verdict and native log. If compilation/test fails, repair **source defects only** without changing frozen seed, 96 Q count, nine shift cap or Dickson base. Record failed predecessor and preserve raw artefact.
- Once green, inspect all 96 and two negative controls from raw JSON, check exact summary counts, timestamp/source SHA and independent arithmetic / PCC scope. Then mark PR ready, do not infer a complete novel solver. If still queued or failing, leave draft and record exact blocker.
- Implement separate public-target-conditioned Dickson31 S9/K961 selected-root construction in existing Rust #4228/#4349 machinery, with repeated-root/identity exception branches. Follow with complete blinded k recovery, charged unsuccessful searches, modular LA and matched shared BSGS versus published IC baseline. No paid compute or ledger promotion in this task.

## Errors (what went wrong, how it was handled)
- GitHub connector can write/read files but local cargo/rustc and authenticated EKS/conductor/Research-KB MCP unavailable in this execution. Scheduled source-specific GitHub Actions rather than claiming local compilation. Independent signed PCC not available; all measured/correctness claims here remain [pcc:unverified].
- Existing main contains unrelated simultaneous compiler collisions/CI changes per PRs #4611/#4608; pinned native Rust source prevents silently shifting the test toolchain.


## Completed follow-up: pinned GitHub execution and frozen outcome

### What was done (commands, files, features)
- Exact-source workflow #44169385150 passed native Rust 40/40 and new holdout 1/1 on initial source; then amended only JSON accounting to capture the existing 4A build time/additions, table capacity and whole-cohort wall. The cohort, seed, public shifts, algorithms, targets, witnesses, probe counts and operation counts were unchanged; verified record-for-record identical across runs.
- Final exact-source pinned CI **#44169834979 PASS** on source 4dcbbce6e3480c55d97e115d802833062955c1d1. 40/40 native tests, 1/1 prospective holdout; raw artifact #14366792594 with GitHub-reported digest sha256:1d957459b4e1b55230b7fc8ad96ee9cf133a6564556305573140c40dd43027a5 (not independently downloaded/rehashed).
- All 96 fresh Q had a bounded 8A representation: 88 at a=0, seven at a=1, one at a=2, zero unresolved; 105 total membership attempts, nine exhaustive misses. Both legacy known a=0 misses were certified absent and succeeded at a=1. All 98 cases: 109 attempts, 98 positives, 11 exhaustive misses. Setup was 296.826437ms and 678,993 native group additions; full 98-Q diagnostic study wall 5,182.860432ms; 18,049,427 complement probes.
- Independent affine mathematical readback reconstructed all 31 Dickson points from seed=3, replayed 98/98 public Q=[k]G, all 98 signed eight-leaf witnesses, and all 109 public residual shifts. Zero mismatches. This is still producer-side readback, not signed PCC.
- Checked the complete raw 98-case/109-attempt JSON (94,917 UTF-8 characters) into the PR as DICKSON31_ADAPTIVE_HOLDOUT_RAW_20261117.json and wrote DICKSON31_ADAPTIVE_HOLDOUT_RESULT_20261117.md with measured scope, theoretical occupancy heuristic, prior-lead exclusions and next real algebraic gate.

### Decisions made (and why)
- Holdout gate is complete. Do not pretend that 4A+4A is a non-generic or index-calculus speed advance, or that it recovered any unknown scalar; no full-cost direct BSGS comparator was run for these 98 targets.
- The only still-priority mathematical route is B31 sign-free target-conditioned root extraction followed by native modular LA and fully charged complete 1024-Q solve against shared BSGS. Retain exact 98-Q witness transcript as oracle ground truth, and require a new holdout for independent scientific evaluation.
- Report data as producer-only [pcc:unverified]. Preserve raw historic CI and corrected instrumentation in GitHub; no leaderboard promotion.

### Open threads (unfinished, pick up next time)
- Implement actual target-conditioned signed S9/K961 or RR9 full-system root extraction from public residual only using original Rust certified quotient/F4 machinery. Prohibit 4A group table calls in candidate, require B31 exact positive/negative/repeated-root and identity branches.
- Once true algebraic relations exist, complete unknown-k scalar recovery from factor-base log rank and public relation, with all setup/failures/memory charged. Compare against fastest same-source 1024-Q BSGS and relevant external index calculus; independently sign PCC before promoting.

### Errors (what went wrong, how it was handled)
- Whole-cohort setup and 4A table capacity were absent from the first correct raw transcript, although already required by frozen protocol. Added **only additional instrumentation**, reran pinned source in CI, and verified that all outcomes and operation counters were unchanged. No altered seed/targets/shift cap or hidden scalar.
- Workflow-files-parse is an unrelated shared CI failure at the store-data checkout before parse tests; source-specific pinned Native Rust workflow passed. No claims of whole-repo CI green.
