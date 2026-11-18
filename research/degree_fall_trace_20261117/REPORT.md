# Degree-fall trace: how relations drive the degree of regularity

**Date:** 2026-11-17 · **Branch:** `claude/nifty-heisenberg-khbw4f` ·
**Run:** `reports/run-20261117T1511Z/`

## Question

Visualize, for ecbench's new measured degree-fall diagnostic, how adding
relations to a polynomial system drives down its degree of regularity — the
"coloring to trace out how the relations affect degree of Reg" the owner asked
for.

## Method

- Instrument: the post-hoc degree-fall diagnostic added to the F4 engine
  `src/cryptanalysis/f4_fp.rs` (`F4Options::record_degree_falls`). At each
  working degree it records the semi-regular (Bardet) Hilbert expectation
  `[∏(1−t^{d_i})/(1−t)^n]_+` and the realized standard-monomial count of the
  computed Gröbner basis, and derives first-fall degree, last-fall degree and
  the observed degree of regularity. Diagnostic only: never priced into `S`,
  outside the replay comparison, never a stopping rule.
- Population: a family over F_101 in 3 variables — three fixed quadrics, then
  4, 5, 6 relations, every quadric forced through a common root so the
  over-determined systems stay consistent (an inconsistent system has no
  degree-of-regularity story). Deterministic (LCG-seeded) and reproducible.
- Data recorded by `src/cryptanalysis/f4_fp.rs::tests::emit_degree_fall_dataset`
  into `reports/run-20261117T1511Z/degree_fall_dataset.json`
  (`ecbench.degree-fall-dataset/v1`). The figure (`plot_degree_fall_trace.py`)
  only lays recorded values out; it computes nothing.

## Finding (measured)

See `reports/run-20261117T1511Z/degree_fall_trace.svg` / `.png` and
`report.pdf`. With the three quadrics the system is a regular sequence: realized
Hilbert = semi-regular expectation at every degree, no fall, degree of
regularity at the generic bound. Each added relation pushes the realized counts
below the expectation (coloured cells / outlined falls) and lowers the observed
degree of regularity. First-fall / last-fall degree and the observed degree of
regularity are read directly from the recorded dataset.

Every value is **measured** by the Rust engine, whose definitions are
unit-tested (`f4_fp::tests`). This is a harness/diagnostic figure on a
constructed family — not an ECDLP result, speedup, or rho comparison.

## Status / scope

- Proposed: none. Derived: the per-degree deviation shown as colour is
  `expected − realized`, both recorded. Measured: all degree/Hilbert values.
- Reachability: the same diagnostic is recorded on real ecbench runs for the
  elliptic summation-polynomial IC family (APS); see PR #4538. Covers,
  genus-2+, and sparse-LA families are tracked separately.

## Growth rate (measured)

The companion growth-rate analysis `src/cryptanalysis/ecbench/growth.rs` fits a
measured metric against an instance-size parameter three ways (linear, power,
exponential; ranked by R²) and reports the top fit's slope with a 95% interval.
For the regular sequence `{x_i^2 - 1}` over F_101 swept across n = 2..6
variables, the measured degree of regularity is `n + 1`, so the growth is
**linear with slope 1.000 (95% CI 1.00..1.00, R² 1.000)** — the expected
baseline (see `reports/run-20261117T1511Z/growth.svg` / `growth.png`, dataset
`growth_dataset.json` = `ecbench.growth-dataset/v1`). A family whose solving
degree or degree of regularity grew sub-linearly or logarithmically in the
instance size is what would signal an index-calculus advantage; the harness can
now measure and report that slope. The fit functions are unit-tested
(`growth::tests`) to recover known linear, power and exponential slopes exactly.

## Variable effects and direct diffusion

The companion figure colors each variable's effect on observed degree of
regularity and the strength of its direct coupling to other variables.
[Figure (SVG)](reports/run-20261118T0135Z/variable_effects.svg) ·
[PDF report](reports/run-20261118T0135Z/report.pdf) ·
[Measurements (CSV)](reports/run-20261118T0135Z/variable_effects.csv)

Each regularity cell compares the original system with that same system plus
one equation pinning a variable at its shared-root value of 1. The observed
degree is the first degree with no standard monomials in the graded reverse
lexicographic Gröbner staircase. Each diffusion cell counts the fraction of
relations where changing the source coordinate from 1 to 2 changes the
polynomial dependence on the destination coordinate, with the remaining
coordinates symbolic. This measures direct coupling at the chosen anchor.

For the first three dense quadrics, each pin changes the observed degree from
4 to 1. For the larger prefixes the observed degree stays at 1. All the dense
relations couple every pair, so this family gives identical scores for all
three variables. The diagonal control distinguishes (x,y) from the already
fixed (z) and has no cross-variable coupling. The chain control exhibits
only the (x)–(y) and (y)–(z) links. [pcc:unverified]

The inputs come from the original
[fixture generator](https://github.com/isogeny-labs/crypto/blob/7db7d3d4e9381446796c38f91f13b666be5127d3/src/cryptanalysis/f4_fp.rs#L5876).
Singular 4.4.1 replay checks the dense Hilbert counts against the
[original Rust dataset](reports/run-20261117T1511Z/degree_fall_dataset.json).
To reproduce the companion values and figures from this directory, run

```sh
cd reports/run-20261118T0135Z
Singular -q --no-rc measure_variable_effects.sing > /tmp/variable_effects_replay.csv
diff -u variable_effects.csv /tmp/variable_effects_replay.csv
python3 plot_variable_effects.py
typst compile report.typ
```
