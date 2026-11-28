# Prime31 linear-chord APS experiment — 2026-11-17 UTC

## 2026-11-17 UTC — prospective Rust S3 batch-denominator experiment [pcc:unverified]

### What was done (commands, files, features)
- Read `AGENTS.md`, shared `docs/session-logs/2026-11-17.md`, prime31 old complete-solver source, `APS_S3_BUFFER_ABBA_RESULT_20261117.md`, PR #4484 and past prime31 original-curve proof and rho controls; audited original-curve coefficients/generator/group order. Main and pinned `1df714645aa683cf975bc76fcacb9842cb4780fd` had identical baseline APS source blob `dd9c747e72b43662bad0ffabea7ceebc8dc513ae`. This work is disjoint from the open scratch-buffer PR #4484.
- Reviewed prior research: merged #4391/#4454 APS, #4451/#4466 RR9/Hermite supplied-leaf predicate and norm, #4228/#4349 quotient/selected-root, #4373 censored F4, #4410 Vieta, crypto-autoresearcher #3301/#3305, crypto #4108/#4363, cryptanalysis #1536, #4468 pinned WZ comparator build blockers. Published APS 2018 DOI 10.1016/j.ffa.2018.01.009 and PKM 2016 DOI 10.1007/978-3-662-49387-8_1 are established methodology. Research-KB MCP tools and local wrapper were unavailable in this connector-only session; no novelty clearance.
- Established the classical chord identity `x(P±Q)=((y_P∓y_Q)/(x_P-x_Q))²-x_P-x_Q` and used it to replace batched inverses of *squared* x-differences by batched inverses of unsquared x-differences in a compile-time specialized Rust APS S3 oracle. The factorized S3 algorithm remains a generic-MITM-equivalent explicit pair enumerator; no novel non-generic claim.
- Pre-registered the protocol (commit `f401170141d1ed46fce52704888b414bf1d77a26`) **before** implementation and before any timings. Added candidate Rust source (commit `d7ae92905312b45b723a28629bfaf5d3e70afee1`), a separate independent-affine Rust JSON replay auditor, and source-pinned Rust-1.100.0 GitHub Actions for 24 caller-only fresh public targets × 5 fully cold arms: incumbent APS / candidate APS / candidate repeat / unfolded rho32 / unfolded rho64.
- Source-specific workflow overlays only the two new Rust example files onto immutable `1df714...` native dependencies. It retains all raw JSON including negative speed results, and reports the frozen 5% development gate without treating it as verified speedup.

### Decisions made (and why)
- Keep the baseline AES? No: this is the existing **APS** method, not a new S9 extractor. The trial is engineering and does not claim better asymptotics or faster-than-optimized-WZ.
- Preserve original-curve arithmetic, caller-only known scalars, identical source walks/profile and public Q for all arms. Reject any signed transcript divergence; count failed rounds and cold setup. Do not mark an advantage on 24 producer-host cases without independent signed PCC and stronger external comparisons.
- Stop allocation-only S3 buffer reuse as a performance lead after #4484's preregistered negative ~0.17% result; carry its evidence forward rather than repeat.

### Open threads (unfinished, pick up next time)
- Run and inspect source-specific Rust CI compile/test, 120/120 independently replayed affine scalars, and cold candidate vs original/controls; repair real failures with full receipts.
- Independently audit raw artifacts and compute paired uncertainty; repeat on isolated hardware and separately tuned Wiener–Zuccherato rho, batch rho, and fastest mature external index-calculus baseline.
- Novel-algorithm objective remains target-conditioned RR9/S9 extraction of U,V and eight-leaf polynomial from public R *without* support enumeration; certify exact positive/negative B4/B8 fibres first, then full B31 unknown logarithms and charged costs.

### Errors (what went wrong, how it was handled)
- No local Rust toolchain or accessible independent compute runner is provisioned here; therefore no local source compilation or new measured runtime is claimed. CI work is pending.
- Research KB and Conductor interfaces are not exposed in this connector runtime; GitHub search/source review and literature citations are recorded as bounded fallback. No paid compute launched.


## 2026-11-17 ~03:28Z — exact-source Rust audit catches identity bug, repaired before any benchmark [pcc:unverified]

### What was done (commands, files, features)
- GitHub Actions [run #44010310623](https://github.com/isogeny-labs/crypto/actions/runs/44010310623) successfully compiled the original 40 native library tests and all 23 APS example tests, including the new 256-pair linear chord equality/S3-root test. The separately implemented affine audit's first unit test failed (0/1): `scalar(1,G)` returned the identity instead of G because the independent `add` function used Option `?` on the inputs rather than handling O+P and P+O.
- Patched only `native/examples/prime31_linear_chord_audit.rs` (commit `42352dc4421563d122d9d681987f47801c309389`): both identity branches now return the other point and nonidentity pairs use the unchanged affine field formulas. CI rerun is required before benchmarking.
- Sparse source checkouts were introduced for efficiency (commit `b12c27fe35fd1f965a504a74a60f7d950477fc9e`) while preserving immutable 1df714 native code, compiler, curve and all target seeds. One preregistered 24Q panel remained **unexecuted** at this point.

### Decisions made (and why)
- Retain the failed CI receipt and repair the independent checker before any performance data; neither modify the preregistered speed threshold nor the candidate solver after observing zero measured outcomes.
- Global workflow-syntax check for the previous source head failed because unrelated base `.github/workflows/curve-covers.yml` duplicates a `run` key at lines 225–226. Treat it separately; do not alter unrelated workflow to mask this result.

### Open threads (unfinished, pick up next time)
- Await exact-source rerun of the fixed Rust auditor, then execute/inspect full cold 24Q panel; preserve the failed-identity-auditor log and all resulting new performance measurements.

### Errors (what went wrong, how it was handled)
- Independent affine identity treatment was incorrect. Failure was caught by its Rust unit test before any benchmark; patched the group identity case and retained the original evidence. No new ECDLP timing or speedup was available from this failed CI.


## 2026-11-17 03:30 UTC — Rust complete-cold panel finished, frozen speed gate failed [pcc:unverified]

### What was done (commands, files, features)
- Exact-source CI [#44010498590](https://github.com/isogeny-labs/crypto/actions/runs/44010498590) passed 40 library, 23 APS and 1 independent-affine-audit tests, then 24 fresh public Q × five cold solvers. The separate Rust auditor replayed 120/120 recovered affine scalar equations, checked all signed APS congruences, compared 48 paired APS attempt sequences and independently recomputed totals and speed decision.
- Frozen full-cold totals: old APS 81,680,616 ns; linear-chord APS 81,006,788 ns; candidate repeat 81,015,446 ns; unfolded rho32 85,208,157 ns; rho64 83,519,404 ns. **Only 13/24** linear-chord wins; candidate/old = **0.991750454**, candidate A/A drift 0.01069%. Frozen 5% + 18/24 gate **FAILED**. S3 oracle itself regressed from 42,964,022 to 43,787,310 ns; setup varied down from 29,777,554 to 28,376,568 ns. No mathematical-oracle advantage credited.
- Raw JSON and source hashes are in GitHub [artifact #14295409872](https://github.com/isogeny-labs/crypto/actions/runs/44010498590/artifacts/14295409872) (30-day retention). The permanent Git report is `APS_LINEAR_CHORD_24Q_RESULT_20261117.md`; independent Rust audit aggregate JSON is `APS_LINEAR_CHORD_24Q_AUDIT_20261117.json`; PR #4486 updated. The run is complete at producer-CI level only, [pcc:unverified].

### Decisions made (and why)
- Deprioritize linear-difference batch inversions: the hypothesized one field multiplication per pair saving did not yield preregistered cold benefit and was not manifested in faster S3-oracle totals. Don't select only favorable target orderings or alter frozen thresholds.
- Keep new identity-regression Rust tests and full comparative harness, but promote no fastest-ECDLP or non-generic claim. Treat unfolded rho32/rho64 as controls, not tuned WZ reference.

### Open threads (unfinished, pick up next time)
- Pursue genuinely nonenumerative public-R RR9/S9 extraction at B4/B8, then B31 complete log recoveries and costed external comparison. Archive raw artifact before 30-day expiry if permanent raw custody is required.
- Investigate #4468 WZ exact-source comparator and perform independent PCC-signed result validation before leaderboard promotion.

### Errors (what went wrong, how it was handled)
- Expected speed gate failed despite all complete logarithms being correct; negative conclusion is reported without retuning. Independent PCC and strongest tuned WZ/external IC remain missing.
