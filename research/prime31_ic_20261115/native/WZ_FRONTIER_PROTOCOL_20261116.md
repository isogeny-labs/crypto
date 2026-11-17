# EXP-R31-WZ-FRONTIER — pinned Rust APS versus repository Wiener–Zuccherato rho (2026-11-16)

**Status before implementation:** protocol committed prospectively; NO new runtime, success, or speedup is asserted here. All final records are producer-development `[pcc:unverified]` until independently signed and replayed.

## Literature and existing-code audit (before selection)

- [Amadori–Pintore–Sala, *On the discrete logarithm problem for prime-field elliptic curves*, FFA 51 (2018)](https://doi.org/10.1016/j.ffa.2018.01.009): published one-relation Semaev/APS approach; the current explicit S3-pair enumeration is **generic meet-in-the-middle-equivalent**, not new asymptotic/non-generic index calculus.
- [Wiener–Zuccherato, *Faster Attacks on Elliptic Curve Cryptosystems*, SAC](https://doi.org/10.1007/3-540-48892-8_15); [Bos–Kleinjung–Lenstra, ANTS 2010](https://doi.org/10.1007/978-3-642-14518-6_9); [Bernstein–Lange–Schwabe, *On the correct use of the negation map*](https://eprint.iacr.org/2011/003): negation-map collision speedups and fruitless-cycle hazards are prior art.
- Actual repository reference `src/cryptanalysis/ic_boundary.rs::{rho_reference_negation,rho_walk_with_policy,NegationClasses,RhoWalk,WalkPolicy,PrimeCurve,PrimePoint}`; compare **exact source**, not a reimplementation. `RhoWalk::negation()` includes WZ look-ahead and short-cycle escape; `WalkPolicy::v2()` adds Brent detection and counters.
- Frozen prior control in `research/prime31_ic_20261115/native/APS_COMPOSED_WIN4_PUBLICY_24Q_RESULT_20261116.md`, CI 43974039411: APS 74.433895 ms, unfolded rho32 79.833260 ms on 24 Q, but 13/24 per-target wins and paired 95% ratio interval 0.7023–1.2270; APS binary/composed 4.10% reduction missed predeclared 5% gate.
- Prior direct 48Q unfolded-rho control `APS_RHO_FOLD_SENSITIVITY_48Q_RESULT_20261116.md`: APS 109.734280 ms, unfolded rho64 99.473873 ms; overlap/parity remains.
- Reuse/follow-ons, not restarts: #4391 and #4454 (APS complete Rust), #4451 (S9 zero-fibre obstruction), #4228/#4349 (exact quotient/subresultants), #4410 (Vieta), cryptanalysis #1536 (auxiliary SIMD). Anchored m6 natural support sparse and m8 F4 censored; no invented positive full solver.
- Shared research-KB MCP and repository wrapper are not accessible from this connector-only execution, so the above primary-paper/repository review is a bounded fallback, **not** a claim of novelty.

## Exact curve and execution contract

(p=2147483647), (E:y^2=x^3+847514846x+2087444724) over (F_p), prime subgroup order (n=2147446831), cofactor 1, (G=(625037715,2100627251)). Not a deliberately weak curve. Public target Q and verifier-only k are produced by the already-merged native Rust APS program. **Neither solver may see k before returning a result.**

The root-`crypto_lib` WZ runner must accept the native APS JSON, validate every public Q and its caller-side scalar only **after** each solver arm terminates, and run these three variants for each of the same 24 Q:

1. Existing `rho_reference_negation` WZ look-ahead/short-cycle window, fixed source-randomized seed.
2. Existing `rho_walk_with_policy` with `NegationClasses`, `RhoWalk::negation()`, `WalkPolicy::v2()` Brent detection.
3. Exact arm-2 A/A repeat with identical seeds, separate fresh solver invocation.

Rotate chronological execution order modulo 3. Each arm has up to **three** charged cold attempts, distinct deterministic seeds and at most **2,000,000** rho walk steps per attempt. Preserve every unsuccessful/capped attempt and its counters, including walks, restarts, distinguished points, setup, field operations, and all wall time. On a recovered candidate, independently verify `[d]G == Q` and `d == caller_k`, where the latter is caller-only audit, never input to rho. A cap is not an evidence-free negative claim. Max attempts are not tuned on the held-out panel.

The workflow checks out the **immutable merged APS baseline** `1df714645aa683cf975bc76fcacb9842cb4780fd` in a separate directory, because the current fast-moving `main` no longer resolves the native crate manifest/source through the GitHub contents API while its reports/workflows remain present. Build its existing Rust 1.100 `--aps-composed-win4-publicy-24q` source on the **same** ubuntu-24.04 runner, then compile/run the repository WZ root-code runner on that JSON. Preserve both original JSON artifacts and the complete new WZ result. This protects against accidentally benchmarking missing/different native code.

## Predeclared gates, statistics and interpretation

- **Correctness**: caller-known `[k]G=Q` on all targets, checked rho candidate for every completed arm, no secret scalar passed to `run_arm`. Record 24/24 each for any advertised complete comparator; no all-complete claim for capped controls.
- **Cost**: sum all unsuccessful/successful cold attempts and replay, compare APS composed 24-case `cold_wall_ns` against separately timed WZ arms on identical Q and same runner. Report paired wins, per-target ratio, A/A drift; use at least 25k paired bootstrap resamples without deleting failed cases or timing outliers. If incompletes remain, mark comparison **censored**, not an apparent winner.
- **No premature speed claim**: this same-host sequential runner is *not* isolated hardware, source arithmetic differs, WZ is a tuned reference rather than established fastest *external* implementation, and the panel was previously used for APS optimization (not a new unseen panel for APS). Therefore even if a ratio is <1, no "strongest optimized rho beaten", "genuinely non-generic" or leaderboard promotion without a fresh isolated independently verified 96-Q holdout.
- **Research next steps**: (i) fresh blind 96 Q with isolated CPU and verified fastest external generic rho, (ii) target-conditioned B4/B8 original-ideal-correct S9 polynomial `f` extraction without generic half-sum enumeration; reject Dickson31 known negatives and squarefree zero-fibre counterexamples; advance to B31 end-to-end only after the extractor works, (iii) optional AVX2/512 S3-only ablation against the unchanged Rust transcript.

## Safety and evidence status

Public synthetic points only. No private keys, real-world target, paid compute, merge, leaderboard promotion or claims of full cryptanalytic breakthrough. Source test and CI status must be read from actual Actions logs; a committed workflow is not a completed experiment.

## Prospective measurement clarification before CI repair implementation (2026-11-16 UTC)

The source-exact WZ reference **already verifies** every candidate internally and includes that scalar replay in `rho.wall_ns` and its `rho.gae` group-ops accounting. The new *independent caller-side* equality `[d]G=Q` is an audit operation, analogous to the caller-side APS-known-answer replay, and therefore must be performed **after stopping each arm's cold solver timer**. The first committed runner redundantly charged that additional external replay to WZ wall time while excluding it from WZ GAE. Fix both accounting mismatches before interpreting any WZ/APS comparison; retain the first-source failure as a failed build without claiming new timings. WZ's own original in-solver scalar verification remains fully charged. No previous benchmark gate or target seed is changed.
