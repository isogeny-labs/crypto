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

## 2026-11-18T22:38Z — successful pinned native 1,024-target result

### What was done (commands, files, features)

- After the original Rust E0499 repair at `ffb5d899`, checked source-specific GitHub Actions [#44412897116](https://github.com/isogeny-labs/crypto/actions/runs/44412897116) and [#44412887094](https://github.com/isogeny-labs/crypto/actions/runs/44412887094): both **success**. Original 40/40 library tests, 2/2 focused Rust tests, actual 31/31 rank and 1,024/1,024 unknown public-log recoveries, and separately authored BigInt verifier PASS.
- Downloaded raw artifact [#14478900939](https://github.com/isogeny-labs/crypto/actions/runs/44412897116/artifacts/14478900939), checked ZIP SHA256 `cd76ee0008805d1f53ce057aa7d225866d52f2dba9393ed2df9ed1869fb30db8`, parsed exact JSON: training 8+8+10+11=**37 natural** queries and 31 independent relations; 1,312 shifted target queries (288 misses) to solve 1024 targets, full count **44,516,323 EC additions** (560,630 setup, 1,036,373 training, 42,919,320 online).
- Charged full original-Rust source-specific time: **14,098.140071 ms** including setup 182.507865ms, training/replay 326.206728ms, online 13,589.425478ms. Total field costs: **2,848,160,879 multiplications, 44,500,854 inversions**. All producer data are `[pcc:unverified]`; independent signed PCC not yet available.
- Committed durable `W8_WINDOWED_B8_RANK_SUMMARY_20261118.json` and appended source-specific CI verdict and failure disclosure to scientific report.

### Decisions made (and why)

- Full 31-log / 1024-target Rust pipeline **works** using four small B8 windows and reuses the original 2^31-1 curve. Four-window architecture supports pursuing only degree≤16 per-slot resultants versus degree≤62 at B31.
- This implementation uses explicit **generic 4+4 EC group-sum tables**. No non-generic public-only RR9 coefficient extractor was implemented or timed. The 14.10s cold batch is slower, not faster, than historical shared-BSGS 144.049ms result (different runner, so numerical comparison is indicative only). Do not call it a cheaper prime-field attack.
- The raw full transcript is in GitHub Actions with 30-day retention (expires 2026-12-18), while frozen summary/protocol/source/validator persist in PR. Preserve raw artifact before expiry if institutional record demands it.

### Open threads (unfinished, pick up next time)

- Existing RR9 public-only algebraic lane #4692: substitute real B8 RR9/F4 unknown-coordinate selection with zero generic enumeration, independently grade unseen public positive/negative residuals, then feed unchanged four-window rank31 + 1024-log Rust pipeline and measure complete costs vs optimized mixed IC, shared BSGS and negation-map rho.
- Signed PCC independent validation of current structural control remains open. Do not merge this draft PR until non-generic gate or permitted scope change.

### Errors (what went wrong, how it was handled)

- Initial source-bound Rust E0499 failed in run #44412198556; corrected in `ffb5d899` and verified in two succeeding source-bound complete workflows.
