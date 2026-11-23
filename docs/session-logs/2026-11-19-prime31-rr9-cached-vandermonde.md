# 2026-11-19 Prime31 RR9 cached Vandermonde chart-building experiment

## 2026-11-19T05:30Z — objective and source publication

### What was done (commands, files, features)
- Read AGENTS.md, current docs/ACTIVE.md, latest 2026-11-19 RR9 rowupdate report and session log, current #4783 source, #4780 graph-chart report, and active #4692 extractor PR.
- Audited #4776 exterior, #4779 Pluecker span, #4773 prefix, #4728 four-window generic solver, #4774 matched BSGS, autoresearcher negative determinant/norm leads and the PKM 2016 / APS 2018 primary papers.
- Added task to docs/ACTIVE.md on existing PR #4783 lane and **published preregistered** RR9_CACHED_VANDERMONDE_PROTOCOL_20261119.md **before** source/benchmark.
- Implemented original-Rust reusable target-independent unsigned Vandermonde 4-row cache in prime31_rr9_exterior4.rs, B8 wrapper prime31_rr9_cached_vandermonde.rs, independent Python BigInt verifier, source-specific GitHub Actions workflow. GitHub Actions run #44465566138 started on commit `936d12053c64b65f41b74f6a1ab8e7604218fdaa`; status was in-progress when this entry was created.

### Decisions made (and why)
- Key identity: A=diag(x-r)V, B=diag(y+s)V, so M=A^-1 B=V^-1 diag((y+s)/(x-r)) V. Because x(Q)=x(-Q), there are only 8^4 unsigned 4-tuples per half (not 16^4 signed), each reusable across all public targets. Cache four rank-one outer products per unsigned tuple and only 64 target denominator inversions per full two-half target.
- Restrict to exact full B8 table tests (two public residuals, both halves) against Gaussian chart, independently re-check 128 exported half charts + 64 full 8x8 pairs in Python, and preregister paired ABBA chart-only measurements. Fail closed for denominator-zero or colliding x.
- This is an algorithmic **precomputation optimization** of a classical identity, not unknown-index extraction and not a claimed ECDLP speedup. Do not overlap with #4692, which owns the nonenumerative extractor.

### Open threads
- Read exact-head source-bound CI #44465566138, fix any Rust or Python errors, freeze accurate benchmark results and update PR #4783.
- Continue public-only unknown-index extraction on frozen 46-positive/18-negative B8 corpus via active #4692, then 31-rank/1024 complete Rust index calculus comparison if it succeeds.

### Errors
- Container shell has no GitHub DNS, so accessed GitHub using linked connector. Shared research-KB connector and wrapper CLI unavailable; audit bounded to cited PRs/repositories and primary literature, not an exhaustive novelty check.


## 2026-11-19T05:35Z — exact source CI PASS and measured result

### What was done (commands, files, features)
- Candidate source-specific GitHub [run #44465566138](https://github.com/isogeny-labs/crypto/actions/runs/44465566138) passed. Pinned original Rust library **40/40** tests; included example **8/8** tests (including complete 262,144-entry B8 comparison). Independently authored Python BigInt replay passed **128** original-curve charts plus **64** full original-curve 8x8 determinants. Raw-artifact separate Python replay independently passed the same checks.
- Downloaded [artifact #14502390564](https://github.com/isogeny-labs/crypto/actions/runs/44465566138/artifacts/14502390564); ZIP sha256 `3e64baefd91c74cc4c0405ee5129c833a310c8293234a224129bfb5af7f3bea7`; inner raw JSON sha256 `38a283660e2b80eb4109e3c7747d6da704357b6a56401f30c5a6c46244942aa1`. Recorded exact source commit `936d12053c64b65f41b74f6a1ab8e7604218fdaa` and pinned unchanged Prime31 arithmetic.
- ABBA B8 chart-only paired complete 262,144-entry two-target arm: fresh A1 175.625104 ms and A2 166.112302 ms, cached B1 25.700974 ms and B2 25.721692 ms; means 170.868703 and 25.711333 ms (6.645657× warm speedup). Cache setup independently measured 7.953809 ms, giving 5.075538× ratio when charged once to the paired two-target workload. All four resulting output digests 1790301562.
- Published exact-source evidence report `RR9_CACHED_VANDERMONDE_RESULT_20261119.md`, machine summary JSON and Rust/Python artifacts. Marked the ACTIVE.md item complete.

### Decisions made (and why)
- This chart-table build optimization reduces target-specific field inversions from 524,288 original Gaussian pivot operations per target to 64 target leaf denominators, after paying for 8,192 unsigned Vandermonde cache entries. Relevant to *algebraic chart construction* only; no generic EC sum table enters the candidate, but subsequent exhaustive 16^8 graph-pair comparisons are completely unchanged.
- Follow up on Lagrange pair-difference inverse caching if amortization warrants it, while treating #4692 nonenumerative public-only extraction as the main solver frontier; never turn 6.646× local timing into a complete ECDLP speedup claim.
- Independent **signed PCC** still [pcc:unverified]. Do not promote evidence to complete-solver advantage.

### Open threads
- Nonenumerative B8 unknown-index extractor on #4692 frozen 46-positive/18-negative natural targets; feed verified relation costs into #4728 rank31/1024 Rust whole solver. Cost comparisons against the strongest IC, BSGS, rho on the same runner.
- Independent signed PCC review, and parent-first PR review/landing: #4783 remains stacked on #4780.

### Errors
- None in the source-bound Rust/Python gates or separate raw-artifact replay.
