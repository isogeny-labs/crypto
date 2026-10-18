# GLS-enabled index calculus at `k = 4`: the GLS endomorphism on the `V₄` line (E25)

[`RESEARCH_GLV_INVARIANT_FACTOR_BASES.md`](../notes/index-calculus/RESEARCH_GLV_INVARIANT_FACTOR_BASES.md)
§9.15 (registered before any code, commit `40b2973f4`) and §9.16 (results), 2026-10-18.  Report:
[`REPORT.pdf`](REPORT.pdf) (source [`REPORT.typ`](REPORT.typ)); figures in [`figures/`](figures/)
(`plot.py` lays out recorded values only); leaderboard summary [`summary.json`](summary.json)
(`glv_invariant_experiment_tables --gls-summary`).

## Result

On GLS twists over `F_{p⁴}` of curves over `F_{p²}` whose 2-torsion the twist splits, the GLS
endomorphism `ψ` (`ψ² = −1`) keeps E24's `V₄` base line and acts on it as `z ↦ ±i/z`, so the base
folds by `⟨V₄, ψ⟩` with 8 points a column instead of 4.  Measured end to end on one curve, one base
and one target stream (10 curves, `p = 101`–`397`): **the fold cuts the `k = 4` index calculus by
`1.35`–`2.67×` (median `1.86×`), and against the matched `⟨±ψ⟩` walk the ψ arm gains `1.26`–`1.97×`
over the control at every `p ≥ 197`**; the modelled handover on these curves moves from `2^{93.2}`
to `2^{88.0}`.  Every logarithm on both arms and all 960 walks verified.  **Engineering, a
constant** (fitted exponent `−0.29` against `−0.30`), on this family only.

**Distance remaining:** every row is `1.1·10⁴`–`2.7·10⁵×` slower than the matched rho.  **What it
does not do:** every curve of this construction has `h = 16` (E24's had `4`), which makes rho `2×`
cheaper at a given `p` and absorbs most of the fold at matched `n` (arm B's handover `2^{88.0}`
against E24's own `2^{90.1}`, different families).  Two registered bands were falsified per curve
(G2.34 yield, G2.35 `S` ratio, on two curves each, from the noise of `5`–`60` relations a curve) and
G2.36's improvement band was exceeded at `p ≤ 149`, where walk set-up dominates.

## Runs and records

| run | command | records |
|:--|:--|:--|
| Stage A | `examples/gls_stage_a.rs --sizes 29,37,41,53,61,89,97,101,109,113,149,197,269,397 --seeds 4` | `experiments/30_gls_v4/stage_a.{json,log}` |
| End to end | `experiments/30_gls_v4/run_all.sh` (binary `46202dc97`) | `experiments/30_gls_v4/p*s*.json`, `run.log`, `tables.md` |
| Unit tests | `cargo test --release --lib quartic_folds` (GLS: coefficients against true logs at `p = 37, 41`; the `⟨±ψ⟩` walk), `-- --ignored the_gls_pipeline` (both arms at `p = 37`) | the tests |

Host: one shared 4-vCPU container, L0; operation counts only, no wall-time claim.

**E26 addendum (note §9.17–§9.18):** a census of 560 GLS twists shows no GLS curve carries the `V₄` line with `h = 4` — `t ≡ 2 (mod 4)` (the only way to `h = 4`) and the halves condition never meet; `h = 16` is the minimum. Records: `experiments/31_gls_cofactor/`; figure `figures/census.svg`.
