# 2026-11-17 — Prime31 RR9 public B8 barycentric chart controls and evidence

### What was done (commands, files, features)

- At 15:20–15:31 UTC, checked current owner AGENTS.md, docs/ACTIVE.md, shared session log, #4550 D9 theorem, #4532 1024Q cached BSGS frontier and related #4539/#4541/#4545/#4546/#4547/#4548/#4549, #3301/#3305, #1536.
- Research-KB MCP tool not present in this runtime; repository wrapper unavailable without checkout (GitHub connector fallback). Verified public primary-paper records for PKM 2016, APS 2018, Kudo et al. 2018; no novelty claim for barycentric interpolation.
- Created track-first draft [#4556](https://github.com/isogeny-labs/crypto/pull/4556), then implemented new `prime31_rr9_barycentric_core.rs` nested under original `prime31_rr9_public_macaulay.rs` and frozen `prime31_rr9_barycentric_public.rs` example. Reused original elliptic curve arithmetic. Created frozen protocol and pinned `.github/workflows/prime31-rr9-public-barycentric.yml`.
- Derived exact B8 barycentric four-moment criterion for existence of a degree-three A at each of 256 signed orientations; Rust RREF solves three unknown V coefficients, Newton interpolates A, independent original curve and all coefficient identities replay. No secret or supplied mask enters the extractor, but it exhaustively constructs public masks.
- GitHub Actions 44114922651: **PASS** native Rust 1.100.0, 40/40 original library and 2/2 example tests, release JSON under 1 GiB virtual-memory cap. Downloaded artifact 14341594010, independently matched ZIP SHA256 `c8e9034540686b598153b702e0a133a2ef09152ed7741d33f5c14a1a744c18ff` to recorded GitHub digest.
- Actual positive `R=(1911579686,548820096)`: one chart, 140161 ns; coefficient tuple `455835358,518652820,754716693,932383545,62510441,1884059825,1142049404`, original-curve replay true. Negative `R=(625037715,2100627251)` zero charts/135655 ns; `R=(1923055775,307731458)` zero charts/135685 ns. Candidate visits 256 masks per R.
- #4556 merged automatically by repository integration at 15:30:40Z (merged SHA a7b92b0c29e6c64575e04785aca9e75fdf63c168) before final result markdown could be pushed; original source/protocol/workflow are on main. Created evidence follow-up [PR #4558](https://github.com/isogeny-labs/crypto/pull/4558) and committed `RR9_PUBLIC_BARYCENTRIC_B8_RESULT_20261117.md` with timings, scope, anti-duplication and prioritized queue.

### Decisions made (and why)

- Preserve explicitly enumerative B8 solver as a **correctness oracle**, not a non-generic method or speedup. Its 140 microsecond phase is *not* a complete unknown-log runtime. The 2^31−1 1024Q full cost frontier remains historical cached BSGS 170.113 ms versus IC 2221.474 ms from #4532.
- Prioritize a native 4+4 MITM same-host B8 comparator next, then a public-only seven-quadrics quotient/D9 sparse certification with no supplied sign choices. Only proceed to B31 complete unknown-log measurement after a genuinely nonenumerative relation extractor exists.
- Report an illustrative ~18-minute per B31 residual enumeration extrapolation for naive C(31,8)=7,888,725 B8 subsets, explicitly **not** a measurement or lower bound. Do not promote to leaderboard or claim independent signed PCC.

### Open threads (unfinished, pick up next time)

- Native Rust 4+4 MITM comparison using identical three public R; include cached setup and search operations.
- Measure seven-equation quotient dimension and saturation on B8 with Rust sparse F4; replay positive chart and negative unit certificates without enumerating signs.
- B31 repeated-root and exceptional chart coverage, blinded complete ECDLP with fully charged setup/misses/LA and independent signed validation.

### Errors (what went wrong, how it was handled)

- Container has neither network access to github.com nor rustc/cargo; code was written with GitHub connector and compiled via pinned exact-source GitHub Actions. Initial compile/test run passed; no source correction was needed.
- After first source PR #4556 merged and its feature branch was removed, an attempted result-file commit to the deleted branch returned 404. Recovered by verifying merge and main content, creating separate tracked PR #4558 for the frozen result and handoff. No source or test was lost.
