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
