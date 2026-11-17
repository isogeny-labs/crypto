# R6.2 results: pooled multi-target rho on an isogeny class, and path-finding cost

Toy instances only; every number is a count (group operations, field `M`/`I`,
neighbour enumerations); wall times are contended notes and support no claim.
No sub-rho claim and nothing about a deployed curve. Preregistration:
[`../PROGRAM.md`](../PROGRAM.md), "R6.2".

Status: batch half as first merged (#2230, unchanged); **path half corrected**
(schema `r6_2.paths/v2`, section "Correction" below); the pooled method is now
registered prime-only.

## Hypothesis, falsifier, verdict

*Hypothesis.* The transported batch costs B/C in [0.97, 1.03] of single-curve
L-target rho for L <= 256; path finding between two curves of a toy class costs
O(sqrt(class size)) Phi_l-root evaluations and is instance-independent.
*Falsifier.* B/C outside [0.95, 1.05] with the 95% interval excluding 1 at any
L; or a path-finding cost that scales with r.

*Verdict: open.* The harness computed no paired interval for B against C, so the
B/C falsifier was not evaluated and no difference or equivalence claim is made.
For paths, the O(sqrt(class size)) part is not shown by the data (corrected or
not); the cost grows with the component size and stays on its order. No
dependence of the cost on r is visible at 18 + 18 classes of 24 to 32 bit p (the
cost tracks the component size; no formal test was run, so the r-scaling
falsifier is not evaluated either).

## Batch (measured, `batch.json`; unchanged by the correction)

One base curve p = 11018534707, r = 2^28.04, h = 40, a class of 257 curves
(l in {2, 3} only: every l = 5, 7 edge lacks a rational-kernel dual because
p = 2 mod 5 and p = 5 mod 7), 20 trials per L. S = total counted group
operations / sqrt(r), mean +- SE.

| L | B (transported) | C (same-curve pooled control) | B/C (ratio of means, derived) |
|--:|--:|--:|--:|
| 1 | 1.084 +- 0.153 | 0.843 +- 0.106 | 1.287 |
| 4 | 2.089 +- 0.107 | 2.285 +- 0.092 | 0.914 |
| 16 | 4.272 +- 0.097 | 4.424 +- 0.130 | 0.966 |
| 64 | 9.395 +- 0.093 | 9.266 +- 0.112 | 1.014 |
| 256 | 20.165 +- 0.138 | 20.170 +- 0.086 | 0.9998 |

The means of B and C lie within about 2 SE of each other. Both stay above the
Kuhn-Struik reference (15.992 at L = 256) and far below L times the
single-target mean (1.042 +- 0.120 at L = 256, about 267, derived). Transport
costs at most 163.0 M + 18.1 I per target at L = 256. 13 740 recovered
logarithms were verified on their original curves. Run report:
[`reports/run-20261018T2219Z/report.pdf`](reports/run-20261018T2219Z/report.pdf).

## Correction (paths v2)

**What was wrong.** The v1 path search (`paths.json`, schema `r6_2.paths/v1`,
kept frozen and reproducible as `--mode paths-legacy`) expanded outgoing
neighbours from both the source a and the target b. An outgoing edge u -> v of b
is a predecessor edge v -> b only if v has a degree-l edge back to u. That holds
for l = 2, 3 and, for l = 5, 7, only when p = +-1 mod l (`SmallIsogeny::dual`
needs a rational root of the l-division polynomial on the codomain). Otherwise
a meeting a -> c <- b, or a hop read backwards, is not an executable route a -> b,
and the reported length is not the length of any route. So the v1 lengths and
enumeration counts, and the v1 component sizes (outgoing reachability sets),
are not path-finding costs of an executable route.

**Second defect.** `rho.multitarget_pooled` was registered `Applies::Any` but
its dispatch handles only `Instance::Prime`; it is now `Applies::PrimeOnly`,
refused by the planner (`applicable`, so a spec naming it on a binary curve
fails at plan time) and again by `solve`, with negative controls (Koblitz and
random binary curves) and a positive control (a prime curve) in
`pooled_multitarget_rho_is_prime_only_and_refuses_binary_instances`, and the
spec-level tests `prime_only_arm_on_a_binary_curve_is_refused_at_plan_time` and
`koblitz_only_arm_on_a_prime_curve_is_refused`.

**The fix.** The graph keeps an edge u -> v only when `SmallIsogeny::dual`
certifies v -> u (symmetric certified edge set); each certification is counted.
Every returned route is re-validated from a edge by edge in the forward
direction, re-enumerating the kernels of the chained curves (counted). The cost
per pair is search enumerations + certifications + validation enumerations. The
pre-fix search runs on the same pairs as the `legacy` control.

**Re-run.** `--mode paths` (fresh classes, defaults, seed 7:
`v2/paths_v2.json`) and `--mode paths --replay paths.json` (the 18 frozen v1
start curves: `v2/paths_v2_replay.json`), 10 pairs per class. Logs
`v2/paths_v2.log`, `v2/paths_v2_replay.log`. Run report with tables and graphs:
[`reports/run-20261025T0156Z/report.pdf`](reports/run-20261025T0156Z/report.pdf)
(source `report.typ`, plots `plot_v2.py`).

| quantity | fresh (18 classes, 180 pairs) | replay (18 v1 curves, 180 pairs) |
|:--|--:|--:|
| corrected routes validated forward | 180 of 180 | 180 of 180 |
| pre-fix route not executable (on these pairs) | 19 | 4 |
| pre-fix length shorter than the executable one | 27 | 9 |
| certified component smaller than the v1 component | n/a | 8 of 18 |
| mean total enumerations per pair, components > 1000 curves | 3 044 to 12 905 | 4 132 to 10 158 |
| total / component size, components > 1000 curves (derived) | 2.7 to 5.1 | 3.0 to 4.3 |

What changes. (1) The v1 numbers are not executable-route costs; quote v2. (2)
For p not = +-1 mod 5, 7 the one-way l = 5, 7 edges are gone and components
shrink (in the replay, 480 to 6, 844 to 4, 240 to 4, 2424 to 8, 1024 to 16, 692
to 2, 620 to 2, 647 to 1); for the other classes the component is unchanged. (3)
The pre-fix counts of non-executable routes (19 and 4 of 180) are lower bounds
on the v1 defect, because v1 drew its pairs from the larger outgoing components.
(4) What does not change: the cost still grows with the component size and
stays on its order (total per pair 2.7 to 5.1 times the component size above
1000 curves), so the O(sqrt(class size)) hypothesis is not shown here either;
the correction adds the certification and validation work the v1 count omitted.

Figures (same-curve links from v1 to v2 in the first):
[`reports/run-20261025T0156Z/paths_v2_enumerations_vs_component.svg`](reports/run-20261025T0156Z/paths_v2_enumerations_vs_component.svg),
[`reports/run-20261025T0156Z/paths_v2_legacy_not_executable.svg`](reports/run-20261025T0156Z/paths_v2_legacy_not_executable.svg).
Graphs checked and refreshed: the paths graph (superseded in
`reports/run-20261018T2219Z/` by the v2 figure; the v1 copy stays as the record of
what was first committed); the batch graph is unchanged.

## Controls (native, release build)

In `examples/r6_2_pooled_class_rho.rs`: a degree-5 edge without a reverse edge is
excluded from the certified set; the pre-fix search returns non-executable
routes on p not = +-1 mod 5, 7 and the corrected search does not; on p = +-1 mod
5 and 7 both edge sets validate; the refactored outgoing search reproduces the
frozen v1 lengths. In `src/cryptanalysis/ecbench/{methods,spec}.rs`: the
prime-only contract tests above.

## Limits

Only r = 2^28 of the planned {2^28, 2^32, 2^36}; the batch class uses l in
{2, 3} only; no isolated or independent-runner measurement, no ecbench session
or paired bootstrap, no replay certificate; the path pairs are drawn uniformly
from components under the cap 3000 (selection toward small components), and a
bidirectional breadth-first search on a toy class is not an Elkies walk.
