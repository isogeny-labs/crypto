# Candidate-base effects across a toy relation flow

Run 20261118T0159Z uses Singular 4.4.1 on Linux x86-64. The protocol starts
from the same single-variable pin and finite-difference definitions as the
[original coloring run](../run-20261118T0135Z/). The sequential toy flow has
membership in a five-element set, a sum relation (x+y+z=t), and a mixed
relation (y(x+z)=s), all over F₁₀₁.

The matched candidate bases are centered ({-2,-1,0,1,2}), consecutive
({0,1,2,3,4}), and spread ({0,1,2,17,43}). Count ordered triples
across the entire (101 	imes 101) target grid and record every nonempty
bin. Empty bins are exactly the complement. The shared target ((3,2))
comes from ((0,1,2)), which belongs to every base. The experiment stops
after three complete 125-tuple sweeps and a Gröbner staircase verification
of each nonempty bin. The basis count must equal exhaustive enumeration at
every stage. This is a deterministic fixture with exact counts and no
runtime comparisons.

Across the full grid, the spread base reached 29 distinct sums and 60
distinct full target pairs. Each compact base reached 13 sums and 35 full
pairs. All three bases accounted for 125 triples across their full target
pairs. The median observed degree over reachable full pairs was 2 for
centered, 3 for consecutive, and 2 for spread. The maxima were 5, 5, and 3,
respectively. [pcc:unverified]

| Base | Triples at membership → sum → mixed | Observed degree at the three stages | Degrees after pinning x, y, z |
| --- | ---: | ---: | ---: |
| Centered | 125 → 10 → 7 | 13 → 4 → 4 | 2, 3, 2 |
| Consecutive | 125 → 10 → 5 | 13 → 4 → 3 | 2, 3, 1 |
| Spread | 125 → 7 → 5 | 13 → 4 → 3 | 2, 3, 1 |

The table is for the common target ((3,2)). Pinning uses the corresponding
coordinate of ((0,1,2)). Degree is the first empty level in the graded
reverse lexicographic Gröbner staircase. The direct diffusion count detects
a change in symbolic dependence on the second variable under a unit change
in the first, as in the earlier figure. At the mixed stage, (x)–(y) and
(y)–(z) each couple in one of five equations. The (x)–(z) count is
zero throughout. The linear sum still reduces the candidate count and
changes the degree. Its finite difference is constant, so it adds zero
direct coupling under this definition. [pcc:unverified]

[Figure (SVG)](base_flow.svg) · [PDF report](report.pdf) ·
[Measurements (CSV)](base_flow.csv)

To reproduce from this directory, run

```sh
Singular -q --no-rc ../run-20261118T0135Z/measure_variable_effects.sing > /tmp/variable-effects-replay.csv
diff -u ../run-20261118T0135Z/variable_effects.csv /tmp/variable-effects-replay.csv
Singular -q --no-rc measure_base_flow.sing > /tmp/base-flow-replay.csv
diff -u base_flow.csv /tmp/base-flow-replay.csv
python3 plot_base_flow.py
typst compile report.typ
```
