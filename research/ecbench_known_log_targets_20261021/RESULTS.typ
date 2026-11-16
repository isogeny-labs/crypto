#set page(paper: "a4", margin: 17mm)
#set text(font: "DejaVu Sans", size: 10pt)
#set heading(numbering: none)
= Arbitrary-target known-log recovery
2026-10-21 · Frozen synthetic prime-field study · Same-host validation

The target-blind known-log two-sum method recovered public target logs without
receiving their values. Its cold counted cost is lower than the specified
abscissa IC pipeline at the three larger sizes in both primary and holdout
panels. *Negation rho remains cheaper at every size.* The smallest comparison
changes ordering across the native-cost band. Independent validation and
complete native-cost calibration remain open.

== Cold measured costs
Eight public targets per curve, five measured rounds and one warmup per arm;
32 unsigned base pairs (64 signed points). Setup is rebuilt and charged for
each target and round. S = GAE / √r, with GAE counting group additions and
doublings. The figure displays native per-curve means; it fits no exponent.

#image("reports/run-20261021T0636Z-primary/cost.svg", width: 100%)

#table(columns: (1fr, 1fr, 1fr, 1fr, 2fr), inset: 5pt,
  [log₂ r], [Abscissa S(0)], [Known-log S(0)], [Rho S(0)], [Abscissa / known-log at prices 0, 0.1, 1],
  [11.807], [27.7819], [25.0571], [2.9177], [1.1087, 1.0488, 0.7550],
  [15.697], [11.1004], [7.5688], [1.7587], [1.4666, 1.4472, 1.3476],
  [19.101], [12.0860], [2.9616], [1.3624], [4.0810, 4.2953, 5.3357],
  [23.974], [42.2267], [2.6984], [1.1584], [15.6487, 15.9587, 16.9821],
)

At 24 bits the holdout abscissa / known-log ratios are 16.6589, 17.0190 and
18.2168. The primary price-zero candidate / baseline ratio is 0.063903, with
95% target-cluster bootstrap interval [0.044228, 0.087908]. This interval is
from ecbench; it is distinct from the pricing-sensitivity band. The matched
rho / candidate S ratios are 0.4293, 0.3998 and 0.3025, so rho remains cheaper.

The comparator is the unchanged prime-abscissa B32 two-sum MITM pipeline with
incremental Gaussian elimination. It is the specified baseline, not an
established globally best IC configuration. The candidate is generic table
search with known base logs, not evidence of a new algebraic trapdoor.

Source: #link("https://github.com/aburan28/crypto/tree/main/research/ecbench_known_log_targets_20261021")[study README, protocol, sealed sessions, native summaries and paired comparisons].
The README lists exact registered curve identities, costs and uncertainty.

#pagebreak()
== Controls, collisions and validation

Preparation accepts no target. A checked full-point decomposition gives one
useful equation in the unknown target log; unknown base logs and matrix work
are zero. The runner independently checks the answer. Baseline final matrix
rank, dependent rows and elimination work are retained; a rank-growth time
series is not implemented.

Generator panels use G, [2]G and [7]G. Transporting the base by the inverse
multiplier preserves its point-set hash exactly. A fixed-query-seed exhaustive
test checks scalar conversion. Cross-generator harness seeds differ, so those
timings establish no generator-dependent hardness change.

#image("reports/run-20261021T0643Z-cofactor-v2/cost.svg", width: 100%)

The corrected cofactor panel fixes p = 65519. With 1,024 nonidentity pair
attempts, h = 2, 4 and 8 yield 993, 957 and 911 distinct signed sum classes,
and 31, 67 and 113 colliding attempts. The respective subgroup orders are
32,843, 16,411 and 8,231. Changing both the subgroup and curve model prevents
a causal cofactor conclusion. Negation is folded; other symmetry contributions
are not isolated. These are not NIST P-192/P-224 recovery results.

Across eight sessions, 4,484 executions comprise 4,340 verified recoveries and
144 retained projected-control setup errors. All 3,610 verified measured
replays are identical on the measuring host. Totals include warmups, A/A and
the corrected rerun; they are not counts of distinct targets. The original
cofactor control rejected scan_cap = 65536 at p = 65519. The separately frozen
version-two control uses 65518; no failed record was replaced.

Counted A/A ratios equal 1 with interval [1, 1]. Wall comparisons admit no L2
pairs. The host is virtual Linux x86_64 on AMD EPYC 9V74, five visible logical
CPUs; one thread, shared CPU, 120-second timeout and 4 GiB address-space limit.
Raw RSS and wall values are descriptive. Pricing all emitted native operations
at 0, 0.1 and 1 GAE is a sensitivity model, not complete machine calibration.

Library validation: 4,478 tests passed, 119 ignored; 17 ecbench integration
tests and three example tests passed. CI-toolchain Rust 1.98 clippy passed.
Independent host replay, independent headline recomputation, strongest external
comparison and actual multi-target amortization are still missing. No validated
leaderboard or benchmark-index row is added, and no asymptotic claim is made.

Next: independently replay and validate these frozen records, calibrate native
work, then compare a predeclared B sweep with optimized negation BSGS and the
strongest rho on fresh holdouts. See REQUIREMENTS.md, verification/study-audits.json,
and problems/known-log-recovery-cost-and-validation.md for the exact open gates.
