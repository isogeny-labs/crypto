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
