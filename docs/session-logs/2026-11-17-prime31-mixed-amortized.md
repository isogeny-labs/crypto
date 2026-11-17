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


## 2026-11-17 ~05:31 UTC — first exact-source Rust CI failure and mathematics-preserving repair

### What was done (commands, files, features)
- Initial source-specific GitHub Actions [run 44024263180](https://github.com/isogeny-labs/crypto/actions/runs/44024263180) compiled full Rust 1.100.0 and passed **40/40 library tests** and **2/3 new Rust tests**. The new reuse-basis test failed at source line 444: the imported single-target `signed_row` helper encodes the coefficient `-b` as `n-b`, assuming nonzero b. Our setup intentionally passes `b=0` and thus the helper emitted integer `n=2147446831`, not canonical zero, although they are identical mod n. The assertion (and field canonical representation) rejected the row before solving any target. No measurement of 1024 target solves is claimed.
- Patched only that target-independent zero-Q coefficient to `%= PRIME31.n`, verified it is exactly zero, then removed the zero column. Source commit `07ba1701111c0a3a2a4f729fd5e78d99516f6fe5`. Pre-registered setup/descent seeds, target list, caps and performance criteria remain unchanged.
- Earlier workflow attempt `44024509598` started from a pre-fix head; its outcome must be associated with its own SHA rather than reported as the patched source. Sparse checkout speed repair was commit `6439a498b54fb6c637c1f0438838e1ef741a2346`.

### Decisions made (and why)
- Require canonical [0,n) representation for all RREF entries. Original `signed_row` mathematical relation is unchanged; a zero coefficient is the additive identity modulo prime order. Preserve the failed CI log as an explicit implementation defect fixed before any 1024-target measurement.

### Open threads
- Run and inspect new exact-source CI at commit `07ba170`, then only report actual 1024 complete solves and their end-to-end amortized cost when it passes.

### Errors
- 40 library tests PASS, 2/3 example tests PASS, 1 test failed because `n` vs `0` noncanonical encoding; 1024 measurement did not run. Fixed in PR #4513.


## 2026-11-17 05:34 UTC — complete 1024Q source-specific index-calculus result

### What was done (commands, files, features)
- Exact-source [Actions #44024919023](https://github.com/isogeny-labs/crypto/actions/runs/44024919023) **SUCCESS** on SHA `52b977cd3bb62ca78038a9d88479051b64ee8be2`: 40/40 original Rust library tests, 3/3 new Rust example tests and **1024/1024 complete unknown-scalar `PRIME31` recoveries**. Setup full rank 61/61 and all basis point replays; solver `[k]G=Q` checked, separately caller scalar checks matched every one. Raw artifact [14302631901](https://github.com/isogeny-labs/crypto/actions/runs/44024919023/artifacts/14302631901), GitHub sha256 `707d78965e3df25ede8a4ac6e4b107ba4558fca42a0a372341707fd790d73a58`.
- Full cold once-per-curve setup **1733.483919 ms**, 1024 target descents total **1029.610760 ms**, charged complete batch **2763.094679 ms**, **1.00547926 ms per-target descent excluding setup**, **2.69833 ms per target including setup**. Full setup+descents at Q=1/64/256/512/1024 was 1734.081472 / 1813.225204 / 2008.244089 / 2246.585790 / 2763.094679 ms, all targets correct at every prefix.
- Committed full method/result report `research/prime31_ic_20261115/mixed_base/MIXED_AMORTIZED_IC_1024Q_RESULT_20261117.md`. Raw evidence preserved by exact-source CI. No separate signed PCC or matched strongest-rho yet.

### Decisions made (and why)
- Promote to **complete Rust multi-target index-calculus-derived scalar recovery** at this precise producer-development scope. Do **not** claim stronger optimized rho or external index-calculus victory: old rho32 24-Q was different host/target roster; our 4+4 group table remains generic MITM-equivalent, and amortization is classical.
- 1 cold target is ~1.734 seconds, so a lower post-setup target time must not be substituted for a single cold ECDLP result.
- The original WZ source-specific CI remains a build failure before any rho runs, not performance evidence.

### Open threads
- Same CPU and **identical 1024 Q** optimized native rho32, tuned WZ and **precomputed BSGS** comparison; include setup/memory and paired A/A statistics, preserve all failed/capped arms. This run did not run these comparators.
- Nonenumerative RR9/S9 factor extraction from public residual, not generic half-table support enumeration.
- PR review/independent signed PCC and tracked validation required for ranking.

### Errors
- After fixing `n` vs canonical `0` row, first sparse build omitted two source modules: [run 44024807315](https://github.com/isogeny-labs/crypto/actions/runs/44024807315) failed on missing `src/cryptanalysis/f4_fp_tower.rs`, not cryptanalysis. Corrected immutable checkout to include original `src/cryptanalysis` source only (SHA `52b977c`) without changing solver. Successful source-specific results above; failed runs retained.


## 2026-11-17 05:39–05:41 UTC — 1024 Q same-runner exact native rho comparison

### What was done (commands, files, features)
- Preregistered new exact-source native rho protocol in commit `a85ddac7` **before coding**; comparator suffix from source commit `7f451945` moved to non-Cargo-autodiscovered `mixed_base/prime31_amortized_rho_1024q_suffix.rsfrag`. CI `.github/workflows/prime31-mixed-amortized-rho1024.yml` at commit `533c6fd40d3bf36e3fb559255128f317e70d1aef` takes immutable actual Rust `prime31_aps_one_relation.rs`, renames only its old unused main, appends wrapper, and invokes unchanged optimized original `neg_rho_cold_policy`. Same 1024 public Q as the reusable IC run, 32-lane unfolded and folded modes, per-target execution order rotated, caps 2m rho steps, watchdog512, 64-case A/A. All cold costs charged and all incomplete cases retained.
- Source-specific [Actions #44025539872](https://github.com/isogeny-labs/crypto/actions/runs/44025539872) **SUCCESS**: 40/40 original library tests, 3/3 IC tests; **1024/1024 IC, unfolded rho32 and folded rho32 all complete, 64/64 rho32 A/A**. Raw artifact [14302284807](https://github.com/isogeny-labs/crypto/actions/runs/44025539872/artifacts/14302284807), SHA256 `a720f5f0533d384ee7526455453fae5e5dfb39e129914c924a2e3649ca098f93`.
- Actual same-run costs: IC setup **1655.699049 ms** + 1024 IC descent **1029.098499 ms** = **2684.797548 ms**; native original unfolded rho32 **3017.608361 ms**, folded negation rho32 **5625.174017 ms**. Fully charged IC / fastest tested native rho **0.8897104020**: **11.03% lower wall time** on the identical 1024-Q batch. Per-target IC descent-only paired wins 913/1024 vs unfolded and 979/1024 vs folded. A/A 64-case aggregate drift **0.4452%**.
- Independently downloaded exact GitHub ZIP, reconstructed actual first-31 and Dickson seed=3 bases and original-curve group law, verified 61 base `[ell]G` points, all 61 Q-independent relation witnesses and modular rows, all 1024 public target relation group witnesses, 1024 IC candidate logs, 1024 unfolded, 1024 folded, 64 A/A rho logs. 1024 attempted target residuals, **0 misses**, all caller k distinct. Checked ZIP SHA256 against GitHub metadata. Ran 25,000 paired resamples of 1024 targets with **fixed setup fully charged**: IC/unfolded ratio 95% percentile **[0.85818,0.92276]**, IC/folded **[0.45971,0.49584]**. Conditional resampling intervals omit host/clock/CI-run uncertainty. Independent readback is an *auditor*, not an alternative Python ECDLP solver or signed PCC.
- Committed `MIXED_IC_VS_NATIVE_RHO_1024Q_RESULT_20261117.md` (commit `7ebaee16`) and `MIXED_IC_VS_NATIVE_RHO_1024Q_INDEPENDENT_AUDIT_20261117.json` (commit `7e860c25`). Preserve raw linked artifact and exact protocols.

### Decisions made (and why)
- This **is** a documented, completely correct *multi-target* index-calculus-derived ECDLP implementation cheaper than the **specific tested native rho32 controls** in one source-specific same-runner 1024-Q batch. It is **not** faster at one cold target, a new non-generic factor extractor or strongest-rho/external-IC victory. The method is generic 4+4 MITM-equivalent and classical amortization; strongest precomputed generic BSGS is missing, as is corrected WZ and external published frontier.
- The 11.03% measured batch advantage and paired-target bootstrap are worth reporting as a restricted real result. No hardware-isolated independent PCC, no largest-scale extrapolation, no promotion.

### Open threads
- Next *decisive* benchmark: **same Q shared-precomputed BSGS** and memory/RSS, and isolated CPU repeated ABBA paired native rho/WZ run; even optimized single-query rho can lose unfairly to a cache-enabled multi-target method.
- Next *cryptanalytic* target: eliminate unknown eight-leaf `f` directly from public `R` in RR9/S9 quotient, original-ideal-certified B4/B8 then complete B31, without the MITM support table.

### Errors
- Matched CI passed. Prior unrelated repository-wide root CI and pinned WZ compilation failures are neither rho results nor algorithmic negative examples. Auto-merge connector defaulted to forbidden squash merge, so PR remains open and unmerged.
