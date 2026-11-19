# Session log — 2026-11-19 Prime31 original-Rust signed shared BSGS comparison

## 2026-11-19 UTC — preregistered source experiment in PR #4774

### What was done (commands, files, features)
- Read AGENTS.md, docs/ACTIVE.md, #4770 and #4728 Rust source and frozen results, the new #4773 and #4772 algebraic components, #4692 extractor draft, #4548 negative public Macaulay, latest dated Prime31 logs. Confirmed research KB MCP was unavailable and used repo/literature fallback; checked PKM (2016) and APS (2018) bibliographic primary records.
- Created branch automation/prime31-shared-bsgs-paired-20261119 stacked on #4770 at f74968435f1a278e3c91469b990b978b6e299deb; first commit only tracked the task in docs/ACTIVE.md; created draft PR #4774.
- Registered W8_MATCHED_SIGNED_BSGS_PROTOCOL_20261119.md before source/benchmark. Implemented signed cached BSGS in native examples/prime31_shared_signed_bsgs.rs with original Arithmetic, 32768 x-keyed babies, signed ± residue, 1024 Q, counted setup and online, complete verification.
- Added independently authored Python BigInt verifier with exact same-source target-stream check against both #4770 W8 arms; instrumented original Rust tests, equality of repeated BSGS answer/count transcripts, six-arm old-ternary-bsgs-bsgs-ternary-old same-runner execution, raw artifact.
- Used sparse pinned checkouts in source-specific GitHub Actions to avoid 10GB repository overhead and include the original Rust library F4 dependencies.
- Source-specific workflow launches at commit 12f7906e9c0703dd838a1b1b614b31d4b99e46fc; do not record a passing benchmark until the final Rust and independently executed Python results are fetched.

### Decisions made (and why)
- Reuse #4770's original EC arithmetic and identical target stream, rather than substitute an unrelated fast ECC library. The signed BSGS is **generic control** and intentionally not promoted as non-generic IC; a qualified benchmark against fastest BSGS/rho still requires separate implementations.
- Distinguish exact signed BSGS coverage theorem from empirical speed measurement.
- Do not run paid EKS work or merge; source-specific GitHub CI and producer-side independent replay can inform but not replace signed PCC.

### Open threads (unfinished, pick up next time)
- Inspect exact CI, failures and raw artifact, and commit frozen paired measurements only after full replay.
- Continue public-residual-only RR9/F4 unknown-index extraction on natural B8 controls; integrate into the existing verified four-window rank31/1024 solver if successful.

### Errors (what went wrong, how it was handled)
- Before CI fixed a likely Rust nested immutable/mutable borrow in construction of the stride; use separate scalar then negation.
- Earlier #4728 transcript has a different deterministic target seed from #4770; verify against *the actual #4770 arm* in the same six-arm execution, not against #4728's distinct fixture stream.


## 2026-11-19 01:27 UTC — exact-source paired six-arm CI PASS and durable results

### What was done (commands, files, features)
- Final source commit \`12f7906e9c0703dd838a1b1b614b31d4b99e46fc\`, GitHub Actions [#44436754627](https://github.com/isogeny-labs/crypto/actions/runs/44436754627): success. Original Rust 40/40 library, new BSGS 2/2, original W8 3/3 tests passed. Independent BigInt Python replay passed 1024/1024 per arm across two signed-BSGS and four legacy/ternary-W8 complete runs, with identical public Q and 16,768,762 giant probes in both BSGS repetitions.
- All phase costs counted on one Ubuntu 24.04 runner: OLD W8 9,122.589ms, TERNARY W8 7,834.933ms, signed shared BSGS 3,342.793ms across two paired means, all 1024 complete logs. BSGS 16,940,250 EC additions, ternary 35,150,980 and old 44,344,469. Ratios of W8 full time over BSGS: old 2.729x, ternary 2.344x. Same-runner ternary/old 1.164x, differing from earlier #4770 1.262x on another runner.
- Preserved durable six-arm JSON summary in \`research/prime31_ic_20261115/rr9/W8_MATCHED_SIGNED_BSGS_SUMMARY_20261119.json\`, [full report](../../research/prime31_ic_20261115/rr9/W8_MATCHED_SIGNED_BSGS_RESULT_20261119.md), raw seven-file artifact [#14488208293](https://github.com/isogeny-labs/crypto/actions/runs/44436754627/artifacts/14488208293), SHA256 360a761d92aa41b18ca3f99e1709ac280c69d867e1c1284281158b9.
- For the future experiment queue reviewed existing #4692 public-only RR9/F4, #4772 H/Jacobian, #4773 conditional prefix, #4548 negative Macaulay, #4728 windowed rank31 and #4532 historical BSGS. No duplicate proposed weight screen.

### Decisions made (and why)
- Stop spending the main effort on generic 4+4 weight refinements: even this not-frontier-optimized, signed x-key shared BSGS is 2.344x faster than ternary on matched Q/EC code; true target remains a nonenumerative F4 solver.
- Scope the result as same-affine-arithmetic native reference only, not fastest standalone optimized shared BSGS, not a non-generic algorithm, and not signed PCC validation. Source-specific replay is complete; next independent attestation remains open.

### Open threads (unfinished, pick up next time)
- On #4692, implement public-only signed-index RR9/F4 extraction against frozen 46/18 B8 natural panel and explicit false positives/timeouts; use the native rank31+1024 complete harness only upon success.
- Run matched, **optimized** rho and stronger BSGS reference plus existing best index calculus with full wall time and group/field costs on same runner; growing prime ladder for scaling.

### Errors (what went wrong, how it was handled)
- One earlier full-checkout workflow run preceded the sparse checkout improvement and ran redundant source-specific validation; selected the source-specific sparse run #44436754627 as the authoritative completed evidence.
- No source-specific compile/test/verification failure in #44436754627; existing weight example emits a dead_code warning in non-test builds, not a failed gate.
