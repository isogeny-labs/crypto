# Session log: 2026-11-17 — reusable mixed-base prime31 index calculus

## 2026-11-17 05:22 UTC — literature and existing evidence before implementation

### What was done (commands, files, features)
- Queried connected GitHub original `AGENTS.md`, `docs/ACTIVE.md`, newest `docs/session-logs/2026-11-17.md`, PRs #4503, #4505, #4506, #4466, #4454, #4486 and #4468, native `prime31_mixed_base_ic.rs`, actual RR9 interpolation source, prime31 mixed support theorem and autoresearcher `SUMMARY.md`/`ERRATA.md`. Checked publisher records for PKM 2016 (DOI 10.1007/978-3-662-49387-8_1) and APS 2018 (DOI 10.1016/j.ffa.2018.01.009). KB unavailable: no research-KB MCP action or container repo wrapper in this runtime; fallback is bounded GitHub/publisher reading.
- Inspected latest WZ Actions #43994897230; it **failed before either solver** on unrelated pinned `src/cryptanalysis/sat.rs` unclosed-delimiter syntax, so it offers no strongest-rho timing. Earlier failed repairs must not be reported as measured controls.
- Exact source prior #4506: 2/2 complete mixed IC logs, setup ~2050ms out of ~2120ms per cold target; rebuilding the same tables and relearning all 61 basis logs for every Q wastes substantial work.

### Decisions made (and why)
- New prospective experiment shares BOTH mixed four-sum tables and independently recovered 61 factor-base logarithms across many unknown targets. After 61-column full rank, each fresh Q requires only one verified signed relation with nonzero b; derive k from its modular row. This is a genuine **index-calculus-derived complete solver** and tests an amortization hypothesis, but its relation oracle remains **generic MITM-equivalent** and is not a novel non-generic extractor.
- Freeze 1024 deterministic fresh targets, setup 256 attempt cap, per-Q 8 residual cap, full original-curve signed replay, basis log point checks, exact per-target k check, raw failure recording and all setup/descent costs before code. Historical rho32 85.208ms/24Q is unmatched and cannot authorize a faster-solver claim.

### Open threads
- Source-specific Rust 1.100 CI needed for `prime31_mixed_base_amortized_ic.rs`: 61/61 basis logs, 1024/1024 target recovery, all signed relation and scalar original-curve checks, uncensored raw JSON and prefix total costs. Fix any compilation/correctness defects transparently, without changing frozen target seed/gates.
- After valid result, add a same-host 1024-Q tuned WZ / external IC control; the root WZ benchmark is blocked by unrelated syntax and should be isolated instead of sequentially repairing large unrelated code. For single target report full cold cost, not per-target amortized cost.
- Separate scientific lead: nonenumerative RR9/S9 polynomial extraction directly from public R without four-sum enumeration, on actual Dickson base with previous negatives.

### Errors
- No Rust compile or runtime test has been observed in this session yet. CI is the first test source; no measured speedup claimed. Local execution environment lacks `cargo`; pinned source-specific GitHub Actions performs the code verification.

## 2026-11-17 ~05:30 UTC — publish frozen protocol and initial Rust implementation

### What was done (commands, files, features)
- Created unique branch `research/prime31-mixed-ic-amortized-20261117`. Frozen protocol in commit `9c0a8fa1c4565707933224257b5382667bb218f0` **before source**.
- New Rust example derived directly from existing `prime31_mixed_base_ic.rs`, preserving `HalfTable`, signed row replay, RREF and first-x/Dickson base code. Commit `ed7e7c66dfce73e34e4772fe3fe5288c7ee27dc4`. Added reusable 61-column setup, full original-curve basis-log point checks, blind 1024-target descents, charged prefixes and small test.
- Source-specific CI on immutable merged native source `1df714645aa683cf975bc76fcacb9842cb4780fd`, pinned Rust 1.100.0 and raw artifact retention in commit `a705ca41c6b660fa0ff29be1f5d03b57c4b7d9b0`. This is checked-in source only until actions complete.

### Decisions made (and why)
- No claims of statistical or scientific performance advantage before CI and a matched optimized rho. Preserve failures and zero user secrets.

### Open threads
- Observe source-specific CI; fix errors in PR only after recording exact log. Inspect actual Rust setup/descent wall time, 1024 correctness and break-even against matched controls.

### Errors
- None from GitHub connector writes. CI pending; no local Rust toolchain.
