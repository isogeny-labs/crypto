# Session log — Prime31 B8 overlapping-window rank control — 2026-11-18

## What was done (commands, files, features)

- Reviewed `AGENTS.md`, today's active task queue and latest 2026-11-18 log; inspected current #4728, #4692, #4714, #4722, #4746 and source Rust B8 signed grader/weighted RR9 coefficient encoding. Searched GitHub-indexed crypto, crypto-autoresearcher and cryptanalysis projects, including related APS window4 and Dickson31 lanes; no claim of exhaustive novelty. Research-KB server/wrapper unavailable in this execution.
- Reviewed primary prior art: Petit–Kosters–Messeng PKC 2016 (DOI 10.1007/978-3-662-49387-8_1), Amadori–Pintore–Sala FFA 2018 (DOI 10.1016/j.ffa.2018.01.009).
- Preregistered `research/prime31_ic_20261115/rr9/W8_WINDOWED_B8_RANK_PROTOCOL_20261118.md` at commit `1c51f606ad951879620df2237ba5c5452d84258a`, **before code/results**.
- Derived four factor-base B8 windows `0..7,8..15,16..23,23..30` covering all 31 columns with exactly one shared point, and exact mean ordered witness counts `16^8/n=2.000034...` for B8 versus `62^8/n=101674.2777...` for B31.
- Committed native reused-Rust group-arithmetic + generic B8 4+4 **grader** and incremental modular rank31/full-target loop `research/prime31_ic_20261115/native/examples/prime31_w8_windowed_rank.rs`; independent Python BigInt checker, source-bound CI workflow, and full scientific report on existing PR #4728.
- Independently reproduced the structural hypothesis in a separately coded local Python original-curve pilot: window training 8,8,10,11 public trials, 8+8+8+7 new independent rows (global rank 31/31 in **37** natural trials), each of 31 point logs checked by `[log]G=P`, plus **32/32** new shifted public target logs checked. All four right-half support tables measured 64,737 group points. Local 32-target Python diagnostic total ~20.17s; this is **not** a native Rust wall-time claim.
- Exact-source initial GitHub Action [#44412198556](https://github.com/isogeny-labs/crypto/actions/runs/44412198556) passed original library 40/40 but failed to compile focused new example due to Rust E0499 nested mutable borrow `ar.add(..., ar.scalar(...))`. Corrected by a separate `shift_point` local in commit `ffb5d89969abfa2c67901373d001f0dd5bf06028`. Follow-up native CI still requires observed verdict; DO NOT claim 1,024-source test PASS before checking.

## Decisions made (and why)

- Test the *index-calculus architecture* by B8 windows rather than one B31-wide resultant: four low-degree B8 selectors can in principle yield all 31 unknown factor logs while avoiding high multiplicity and degree-62-per-slot B31 resultants. B8-only supports have exact group-average ordered witnesses near two.
- Keep generic 4+4 enumeration explicit as **grader-only**; it is not the requested non-generic relation extractor and should not enter any security advantage claim.
- Keep existing #4692 the owner of the public-only unknown-index RR9/F4 solver. This lane #4728 provides the original-crate rank and target test harness and data.
- Preserve every failed CI run and all initial measurements at their true scope. No paid compute, merge, or new PR.

## Open threads (unfinished, pick up next time)

- Read exact-source CI for `ffb5d899`; correct any errors and archive full Rust 1024-target JSON + independent BigInt verification, adding actual costs and source hashes to report.
- Replace the four generic grading tables with true public-only nonenumerative B8 algebraic unknown-chart extractors; test natural positive and negative fibres in every window, then exact rank31 and 1024 recovered logs.
- Only on that success, repeat same-runner matched complete cost test against fastest existing Rust mixed index calculus, cached/cold shared BSGS, and optimized rho. Signed independent PCC remains absent.

## Errors (what went wrong, how it was handled)

- Assumption that all alternative windows have exactly 64,737 right sums was removed from Rust assertion before native checks; the count belongs to a measured original control, not a general theorem. Local Python pilot happened to find 64,737 each.
- GitHub-hosted CI reported Rust E0499 in `ar.add(PRIME31,q,ar.scalar(...))`; fixed with a separate mutable call. Previous failure retained in run history; local Python pilot cannot replace native CI.
