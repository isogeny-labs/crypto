# Prime31 weighted-slot parity continuation — 2026-11-18

## 09:31–09:45 UTC — first-source build, exact group-algebra result, and validated fixed-curve diagnostic

### What was done (commands, files, features)
- GitHub connector reviewed `AGENTS.md`, `docs/ACTIVE.md`, prior `docs/session-logs/2026-11-18.md`, existing native `lib.rs`, B31 `prime31_s9_b31_half_sum_oracle.rs`, Prime31 component records and relevant open PRs #4692/#4698/#4678/#4701. The shared research-KB MCP and wrapper command were unavailable in this connector-only runtime. Fallback was repository primary sources and pinned Bochum/published references. Novelty not cleared.
- First committed task line to `docs/ACTIVE.md`, opened draft PR #4714 before creating research artifacts, and preregistered `PRIME31_WEIGHTED_SLOT_PARITY_PROTOCOL_20261118.md`.
- Proved the exact repeated-slot characteristic-two identity `A^(2^k)=sum_[P in A][2^k P]`, the population bound `|support| <= B^(sum popcount(m_c))`, and that translating eight identical slot bases only translates the parity support. For identical original Prime31 positive B31 slots, exactly 31 of 2,147,446,831 group points have odd 8-sum parity.
- Implemented `native/examples/prime31_weighted_slot_parity.rs` importing the unchanged `prime31_algebraic_ic::{Arithmetic,Counts,Point,PRIME31,check_curve}`: fixed B={8,16,31}, eight public distinct weights (1,3,5,7,11,13,17,19), original curve group operations, exact 4+4 signed-free parity toggle, 12 deterministic public residuals, planted positive controls, exhaustive B2 mathematical checks. Candidate target queries have no caller-only target scalar.
- Source-specific pinned Rust 1.100.0 Actions #44277371478 **passed** 40 original library tests, one focused B2 test, full 36 public point rows, and independent Python BigInt replay; test summary 0/7/7 odd rows for B8/B16/B31. Producer-independent checker replayed all 36 public targets, 15 target/control witnesses and 12 full B8 parity queries. No new ECDLP unknown logs solved. Raw artifact #14414418610 ZIP sha256 `91975bcf973d0c5b64c3f522af8c13997955018eba0eacba3f7f099d7881e0a1`. Durable exact 36-target witness compact summary is `PRIME31_WEIGHTED_SLOT_PARITY_WITNESSES_20261118.json`.
- Added `verify_prime31_weighted_slot_parity.py` to rerun and independent-verify semantics of the compact frozen record, and documented the exact mathematical interpretation and next falsifying experiments in `PRIME31_WEIGHTED_SLOT_PARITY_RESULT_20261118.md`.

### Decisions made (and why)
- Preserve the original prime-order ordinary 2^31−1 curve and pin the native Rust source to `1df714645aa683cf975bc76fcacb9842cb4780fd`, avoiding duplication of the old generic B31 full solver.
- Distinct publicly known scalar weights avoid exact repeated-slot parity collapse and allow all slot relations to contribute to the same B log columns, but the current 4+4 parity oracle is *generic enumeration*, not genuine algebraic elimination.
- Stage timings (B31 setup 636 ms, 12 lookups 3414 ms) are uncalibrated, exclude base selection/target construction and cannot be compared as complete ECDLP wins. No paid host and no leaderboard claim.
- Keep PR #4714 separate from active RR9 #4692 and unverified asymptotic paper theorem #4698 to avoid duplicate ownership.

### Open threads (unfinished, pick up next time)
- Independent signed PCC validation.
- Build a truly nonenumerative F4/RR9/tensor parity oracle for the distinct weighted slot system; on B4/B8 first, prove root/witness equivalence without 4+4 table, then B31.
- Only then recover 1,024 blinded complete original-curve logs using existing rank31 pipeline with all costs and failures charged, and benchmark against matched optimized IC, shared BSGS and rho. No such solver advantage yet.
- Final CI check after frozen-summary semantic replay adjustment must pass before marking PR ready for review.

### Errors (what went wrong, how it was handled)
- Initial exact-source CI #44276734507 failed Rust E0499: nested mutable borrow in the translation-only B2 test. Split the scalar and add calls into sequential statements. Follow-up Rust CI #44276946907 passed.
- Frozen-summary exact-byte SHA rerun gate in CI #44277830348 failed because Rust's randomized HashMap hash seed selects different valid witnesses and wall times vary. The archived SHA is correct for the source artifact but **not a reproducible fresh transcript SHA**. Changed the validator to compare exact parity/operation invariants, independently recompute every archived witness against the frozen target, and retain original SHA strictly as the archived receipt. Await its corrected run.
