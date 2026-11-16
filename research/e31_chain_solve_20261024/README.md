# E31: the chain solve for the `k = 4` `V₄`-line decomposition

Goal G2 of [`RESEARCH_GLV_INVARIANT_FACTOR_BASES.md`](../notes/index-calculus/RESEARCH_GLV_INVARIANT_FACTOR_BASES.md),
2026-10-24.  Pre-registered in §9.22 (G2.85–G2.94) before any solver code; results in §9.23.
Report: [`REPORT.pdf`](REPORT.pdf) (source [`REPORT.typ`](REPORT.typ), which reads every number
from [`summary.json`](summary.json)).  **Result class: engineering (phase speed-up).**

## Result

A second solver for one decomposition, working in the quotient as a 12-generator module over
`F_p[P₂]` (A), and A with sparsest pivot rows (B), returns the existing solver's decompositions on
all 10,000 Stage A calls at `2.62`–`2.69×` (A) and `4.99`–`5.32×` (B) fewer counted `F_p`
multiplications per call (median `C`).  End to end on E25's ten curves, both arms, the same target
streams, every relation, log and walk is unchanged and `S` falls `2.50`–`2.56×` (A) and
`4.53`–`4.76×` (B) at 0 GAE; `1.39`–`1.43×` / `1.59`–`1.64×` at 0.1 GAE and
`1.034`–`1.053×` / `1.051`–`1.071×` at 1 GAE (one GAE = 97 `F_p` multiplications per Macaulay entry
and per walk step).  E31 is below E25 on every row at every weight.

**Distance remaining:** the matched `⟨±ψ⟩` rho is ahead of every row at every weight.  B's rows
are `2.39·10³`–`1.52·10⁵×` rho at 0 GAE (best `2.39·10³×`, `p = 397` seed 1, arm B; E25's best
`1.13·10⁴×`), `1.31·10⁴`–`8.55·10⁵×` at 0.1 GAE, `7.21·10⁴`–`5.14·10⁶×` at 1 GAE.  Exponents
unchanged.  Scope: E25's GLS twists, `p ≤ 397`, the degree-11 solve.  The validation gate's files
for these rows are prepared separately in `benchmarks/20261024-e31-chain-solve/`.

The README's numbers are copied from `summary.json`; regenerate both with the command below.

## Records

| run | command | records |
|:--|:--|:--|
| Stage A | `examples/e31_stage_a.rs` (binary `a8f52422d5`), one thread, 1,000 calls per curve | `experiments/31_solve_cost/stage_a.json` |
| end to end | `experiments/31_solve_cost/run_e2e.sh` (`SOLVER=chain`, `SOLVER=chain-sparsest`) | `experiments/31_solve_cost/e2e_{chain,chain-sparsest}_p*s*.json` |
| baseline | E25, `experiments/30_gls_v4/run_all.sh` | `experiments/30_gls_v4/p*s*.json` |

Tables: `experiments/31_solve_cost/tables.md` (`examples/e31_tables.rs`).  Host: one shared 4-vCPU
container, isolation L0; operation counts only, wall time a practicality note.

## Figures

Recorded values only; source [`figures/plot.py`](figures/plot.py), which also writes `summary.json`:

```sh
python3 research/e31_chain_solve_20261024/figures/plot.py   # needs matplotlib
python3 -c "import typst; typst.compile('research/e31_chain_solve_20261024/REPORT.typ', output='research/e31_chain_solve_20261024/REPORT.pdf')"
```

- [`figures/per_call_cost.svg`](figures/per_call_cost.svg): Stage A median and mean `C` per call, existing vs A vs B.
- [`figures/phase_breakdown.svg`](figures/phase_breakdown.svg): Stage A phase means per call, three solvers.
- [`figures/s_by_curve.svg`](figures/s_by_curve.svg): `S` per curve, E25 vs A vs B, both arms.
- [`figures/distance_to_rho.svg`](figures/distance_to_rho.svg): `S / rho` at 0, 0.1, 1 GAE, E25 vs A vs B, both arms.

Copies of the PDF, SVGs and PNGs: [`benchmarks/20261024-e31-chain-solve/`](../../benchmarks/20261024-e31-chain-solve/).
