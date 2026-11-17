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
