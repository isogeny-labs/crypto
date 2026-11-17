# 2026-10-18 · Cofactor-cleared factor base for prime-field index calculus

Copies for browsing. The source of truth is
[`research/isogeny_conductor_gap_round5_20261012/`](../../research/isogeny_conductor_gap_round5_20261012/README.md),
which holds the records, audited `ecbench` sessions, tables and plot scripts.

| file | what it shows |
|:--|:--|
| [`report.pdf`](report.pdf) | study report: E6, E7a, E7b and the §5a band |
| `e7a_ic_over_rho.{svg,png}` | best IC ÷ rho per curve, fiber-closed table against cofactor-cleared base |
| `e7a_h72_s_by_arm.{svg,png}`, `e7a_h4_s_by_arm.{svg,png}` | `S` per arm per curve, sessions `e7a_h72` and `e7a_h4` |
| `e6_yield_ratio.{svg,png}` | E6: quotient ÷ direct decompositions against \|H\| |
| `e7b_yield_ops.{svg,png}` | E7b: yield and F4 work per relation, default against trace-kernel base |
| `run-*.pdf` | the four run reports |

Headline: the cleared base costs 0.705× (h = 4) and 0.275× (h = 72) the
best fiber-closed arm on 16 curves of 18–30 bits, at every weight of the
0 / 0.1 / 1 pricing band. The best cell is 7.29× its matched rho
(`icv1-fp22-tm1504-967cbbde`), down from 8.43×. Rho stays ahead on every
curve, and the ratio still rises with size.
