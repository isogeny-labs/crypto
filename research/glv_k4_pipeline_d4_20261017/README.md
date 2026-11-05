# The `k = 4` index calculus end to end: the `D₄` baseline (E19) and the `V₄` line (E20–E24)

Goal G2 of [`RESEARCH_GLV_INVARIANT_FACTOR_BASES.md`](../notes/index-calculus/RESEARCH_GLV_INVARIANT_FACTOR_BASES.md)
§9, 2026-10-17/18.  Every experiment was pre-registered in that note before its code (§9.5,
§9.7, §9.10, §9.13); results are §9.6, §9.8, §9.9, §9.11, §9.12 and §9.14.  Report:
[`REPORT.pdf`](REPORT.pdf) (source [`REPORT.typ`](REPORT.typ)).

## Result

On curves over `F_{p⁴}` with full rational 2-torsion, a base line stable under all three
translations (`z ↦ −z, 1/z, −1/z` in a Möbius coordinate) and `S₅` written in the invariants of
`V₄⁴_0 ⋊ S₄` (four unknowns of weights `(2, 2, 1, 1)`, quotient `64`, Macaulay degree `11`) cut
one decomposition from `1.41·10⁹` to `1.66·10⁶` `F_p` multiplications.  Measured end to end
(E24, 12 curves, `p = 101`–`521`), **the `k = 4` index calculus is `35`–`240×` cheaper than the
`D₄` baseline (E19, 10 curves) at every matched size, and the modelled handover with rho moves
from `2^{116.7}` to `2^{90.1}`.**  Every logarithm was recovered and verified.  The exponent is
unchanged (`S / rho ∝ n^{−1/4}`): **engineering, a constant.**  **Distance remaining:** every
row is still `1.3·10⁴`–`1.7·10⁵×` slower than the matched negation rho; the handover is a model
extrapolation of measured constants, not a measured crossing.  Scope: full rational 2-torsion,
rational halves of two `T`'s, `h = 4`, `n ≤ 2^{34.1}`.

| experiment | what | verdict |
|:--|:--|:--|
| E19 | `τ_T` line, weighted `D₄` solve, end to end, `p = 53`–`269` | baseline measured: `2.6`–`8.8·10⁶×` rho, handover `2^{116.7}` (accounting) |
| E20 | double large primes on the folded base, at the weighted `C` | right exponent, wrong constant: below the full base only past `2^{164}`–`2^{222}` |
| E21 | shape-trace replay on the weighted solve | net loss (`1.02×` saved, `13`–`24 %` fallbacks) |
| E22 | the `V₄` line (Stage A) and its invariant solve (Stage B) | exists, never confined; `C ≈ 1.7·10⁶` (`≈ 820×`); yield `≈ 10×` worse |
| E23 | `ψ` and `τ_T` together on `j = 0` | closed: the shared base is confined or empty |
| E24 | the `V₄` line end to end, `p = 101`–`521` | `1.3·10⁴`–`1.7·10⁵×` rho; `35`–`240×` below E19; handover `2^{90.1}` (engineering) |

## Runs and records

| run | command | records |
|:--|:--|:--|
| E19 | `experiments/27_quartic_pipeline_d4/run_all.sh` (binary `4e5810598`) | `experiments/27_quartic_pipeline_d4/` |
| E20 | `experiments/28_e20_dlp/run.sh` | `experiments/28_e20_dlp/` |
| E21 | `examples/d4_replay.rs --sizes 53,101 --seed 1 --targets 201` | `experiments/28_e21_replay/` |
| E22 A | `examples/klein_stage_a.rs --sizes 53,101,149,197,269 --seeds 2` | `experiments/28_e22_stage_a/` |
| E22 B | `examples/klein_stage_b.rs --sizes 37,41,53,101,269 --planted 6 --uniform 6` | `experiments/28_e22_stage_b/` |
| E23 | `cargo test --release --lib quartic_folds::e23_tests` | the test |
| E24 | `experiments/29_quartic_pipeline_v4/run_all.sh` (binary `854acdbc6`) | `experiments/29_quartic_pipeline_v4/` |

Tables: `cargo run --release --example glv_invariant_experiment_tables -- <records>` (`e19`,
`e20`, `e24`).  Host: one 4-vCPU cloud container, shared between runs, isolation L0; every
number is an operation count (`F_p` multiplications; a mod-`n` multiplication at `16`; GAE =
`97`), so no wall time is claimed.

## Figures

Recorded values only; source [`figures/plot.py`](figures/plot.py).

- [`figures/e24_vs_e19.svg`](figures/e24_vs_e19.svg): `S / rho` per curve, both pipelines.
- [`figures/solve_cost_both.svg`](figures/solve_cost_both.svg): median `C` per call, both solvers.
- [`figures/distance_to_rho.svg`](figures/distance_to_rho.svg): E19 against G2.12's registered band.
- [`figures/solve_cost.svg`](figures/solve_cost.svg): E19's per-call `C` against d4-s5's band (G2.11).
- [`figures/residuals_per_relation.svg`](figures/residuals_per_relation.svg): E19's `y` (G2.10).
