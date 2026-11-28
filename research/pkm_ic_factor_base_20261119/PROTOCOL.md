# EXP-PKM-IC-001 — a Kummer-coset factor base in the repository's prime-field index calculus

Frozen before any row (2026-11-19). Owner's choice of next step after EXP-PKM-RESULTANT-001.

## Question

The PKM studies found that a Kummer-coset `V` (abscissae `g·μ_S`, the degree-2 tower of PKM) costs the same as a
random `V` in every decomposition oracle measured [pcc:unverified]. Does that hold in the repository's actual
prime-field index calculus at `m = 2` (pair-sum table, `ec_index_calculus_dlp_staged_*`), for the whole solve —
relation collection, linear algebra, verification — and how far is that solve from Pollard rho?

## Instrument

- `ec_index_calculus::build_factor_base_from_xs` (new): the factor base whose abscissae are a given list, `y`
  canonicalised as in `build_factor_base`. `ec_index_calculus_dlp_staged_seeded_with_factor_base` (new) runs the
  unchanged staged pipeline on it (`FactorBaseChoice::Given`; the two existing choices are unchanged).
- Driver `examples/pkm_ic_factor_base.rs`, pair-sum table forced (`IC_SUM_TABLE=1`), extra relations 8,
  1,000,000 trials per attempt, 64 attempts (the a = −3 ladder's policy).
- Curves `curves.json` (`find_curves.py`, a = −3, prime order, 2-adic valuation of `p − 1` ≥ 14):
  20-bit `p = 1032193`, 24-bit `p = 16760833`, 28-bit `p = 268369921`. The driver re-checks `[n]G = O`.

## Arms (same curve, same targets, same relation-sampler seed; only the factor base differs)

- **kummer**: abscissae `V = pkm_smooth_tower::kummer_tower(p, [2]·t, None, 5).v` (`|V| = S = 2^t`, the PKM
  construction), lifted; `M` = the number of liftable points.
- **small-x**: `build_factor_base(curve, M)` (the repository default), the same `M`.
- **random-x**: distinct uniformly random abscissae (seeded), lifted until `M` points.

Cells: (20-bit, `t = 8, 9`), (24-bit, `t = 9, 10`), (28-bit, `t = 9, 10`); 6 targets per cell, target scalar
`k_i` from `StdRng(20261119 + i)`, relation-sampler seed `7 + i`, identical across arms.

## Metrics

Per solve: correctness (`[k]G = Q` and the returned log equals the sidecar scalar), charged group operations
(`group_additions + group_doublings` from the stage report), trials, wall times per stage. Linear algebra is not a
group operation in the harness's unit; it is reported as wall time and named, not charged. Rho reference: the
expected `√(πn/2)` group operations of Pollard rho (theoretical, not a measured matched rho), so the IC ÷ rho
ratio is a distance, not a validated comparison.

## Predictions

- **K0 (validity)**: every solve returns the correct logarithm. Any wrong answer invalidates the study.
- **K1**: median charged group operations, kummer ÷ small-x, in `(0.8, 1.25)` in every cell; random-x ÷ small-x
  likewise.
- **K2**: median trials per relation, kummer ÷ small-x, in `(0.8, 1.25)` in every cell.
- **K3 (distance, reported)**: median charged group operations ÷ `√(πn/2)` `> 1` in every cell (the prior: `m = 2`
  index calculus costs about `n^{2/3}` and loses to rho; the ratio is not expected to fall with `n`).

A solve that does not finish in 30 min wall is censored and reported, not imputed.
Nothing here concerns cryptographic sizes.

## Sources checked

`research-kb` unavailable in this runtime (no executable). Repository: `src/cryptanalysis/ec_index_calculus.rs`
(pair-sum table, staged pipeline), `examples/a3_stage_ladder.rs` (policy), `src/cryptanalysis/ic_boundary.rs`
(`PrimeCurve::prime_order`, the method `find_curves.py` mirrors), `research/pkm_*` (earlier PKM studies).
