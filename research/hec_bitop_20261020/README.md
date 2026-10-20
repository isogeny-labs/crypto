# The genus-4 sub-rho row, re-priced in bitop/v1

**Report:** [`REPORT.pdf`](REPORT.pdf) (source [`REPORT.typ`](REPORT.typ)).
**Protocol:** [`PROTOCOL.md`](PROTOCOL.md), frozen 2026-10-20 before data, with
three deviations and the owner's interpretation amendment.
**Run report:** [`reports/run-20261020T0713Z/`](reports/run-20261020T0713Z/).

Step 4 of the bitop/v1 follow-ups. The scoreboard's only end-to-end
measurement below rho was index calculus on a genus-4 hyperelliptic Jacobian
over `F_61` (`N = 16,790,591 ≈ 2^24`) at **0.24× rho**. That figure has two
weaknesses:
- **Wall-clock conversion.** It is in the harness's in-situ unit, where the
  linear algebra and the smoothness oracle enter through a wall-clock
  calibration.
- **Slow rho reference.** It was measured against a rho whose 8-bit branch
  coefficients slowed the walk on the largest rows.

This study counts every field, mod-`N`, hash and word operation of both arms
where it executes. It prices both arms in `bitop/v1`, an accounting convention
with frozen gate-equivalent weights (`docs/units/BITOPS.md`), not a physical
gate count. It uses the full-width rho walk, on the same 18 round-seven rows
with the same seeds:
- index calculus: 5 runs per row;
- rho: 40 runs per row.

Every logarithm verified.

## Results (`analysis/analysis.json`, `examples/hec_bitop_analyse.rs`)

**Genus 4, p = 61:** IC/rho = **0.114 [0.096, 0.138]** (95% bootstrap).
- **Verdict:** by the preregistered rule, the row **survives**: its upper
  bound is below 1.
- **Variants:** the ratio stays near 0.13 under every reported variant:
  - excluding validation work: 0.132;
  - post hoc, without rho's fixed precomputation: 0.134;
  - post hoc, the IC oracle charged 2×: 0.134;
  - the in-situ unit with full-width rho: 0.132.

Slope of `log₂(IC/rho)` against `log₂ N`, bitop/v1, 95% two-stage bootstrap:

| genus | rows | bitop/v1 slope | round seven (in-situ) | verdict |
|--:|--:|:--|:--|:--|
| 4 | 5 | −0.232 [−0.320, −0.082] | −0.29 [−0.42, −0.16] | advance **holds** (upper bound < 0) |
| 3 | 7 | +0.095 [+0.050, +0.148] | −0.01 [−0.07, +0.05] | rising; above round seven's −0.17..+0.07 band |
| 2 | 6 | +0.269 [+0.138, +0.523] | +0.21 [0.00, +0.41] | rising |

**What qualifies the small rows (post hoc).** Rho's reference precomputes
16 full-width jump points, each a scalar multiplication. At small `N` that
fixed cost dominates: 68% of rho's bitops at genus 2, `p = 251`. Without it,
every genus-2 row and the small genus-3 and genus-4 rows read above 1. So
ratios below 1 at small `N` say more about rho's set-up than about index
calculus. At genus 4, `p = 61`, the precomputation is 14% of rho, and the row
reads 0.134 without it.

**The code is not round seven's.** In the in-situ unit with the full-width
rho, today's genus-4 ratios are 1.22, 0.87, 0.61, 0.62 and 0.13. Round seven's
were 2.47, 1.79, 1.21, 1.16 and 0.15. So the move from 0.24× to 0.11× combines
three things:
- the algorithm changes made since round seven;
- the corrected rho walk;
- the change of unit.

The study separates the unit from the rest (0.132 in-situ against 0.114
bitop/v1 at the same head). It does not attribute the code change.

**Class: accounting.** This is a re-pricing; no algorithm changed. It confirms
the sub-rho row and the genus-4 exponent under one accounting convention for
both arms. It is not an improvement of index calculus.

## Scope and limits

- **What is measured:** toy hyperelliptic Jacobians. No deployed system is
  hyperelliptic. Counts only; wall time is not used.
- **Coverage.**
  - The Callgrind diagnostic put the two arms within 1.16× in instructions per
    counted gate. The IC oracle alone ran 1.7–2× rho's, and the 2× sensitivity
    covers that.
  - Not counted: memory movement, generator internals beyond one word
    operation per draw, and loop-control tests. Both arms do these.
  - The diagnostic does not prove coverage is complete (see the interpretation
    amendment in `PROTOCOL.md`).
- **Sample.** One curve per row, as in round seven. Genus 4 has five sizes,
  and the 0.114 row is the largest.

## Files

| path | what |
|:--|:--|
| `PROTOCOL.md` | the frozen protocol, its deviations and the owner's amendment |
| `run.sh` | the runner: 18 rows, three genera in parallel |
| `raw/` | `hyperelliptic_ic_vs_rho_bitops --json` output per genus, and logs |
| `cards/` | task card, receipt, executor return and director assessment |
| `analysis/` | `analysis.json` and `tables.md` from `examples/hec_bitop_analyse.rs` |
| `figures/` | SVG/PNG figures and `plots.py` (plots recorded values only) |
