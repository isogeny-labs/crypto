# Frozen exact Markowitz-priority pencil continuation

The [protocol](PROTOCOL.md) fixes a pivot-priority change to the
point-driven pencil relation solver, with the same archived source targets,
transfer, accepted relation checks, and same-process source-negation-rho
control. The [exactness argument](PROOF.md) explains why the priority rule
does not alter the linear identities. The first gate is all 60 frozen p=739
targets. Every candidate and rho scalar verified by source multiplication;
the independent replay matched all non-wall fields. The [native validator](results/validation.json)
checked 240 source scalars, 122 raw/replay rows, 4,320 target-level price
cells, and the 36 paired aggregate cells.

| p=739, 60 targets | Peel control | Markowitz priority |
| --- | ---: | ---: |
| Modular row updates | 1,336,284 | 1,307,655 |
| Logical index operations | 9,475,326 | 17,513,708 |
| Representative cold cost / matched rho | 11.615357 | 13.994822 |

The representative Markowitz/peel complete cold-cost ratio is 1.204855.
Across the 36-cell price grid, 0 cells are below peel and 0 are below rho;
the paired Markowitz/peel range is 1.000000–1.248952. The
[paired CSV](results/p739-markowitz-vs-peel.csv) retains each price cell,
including equality and higher prices. The [PDF](reports/run-20261022T0732Z/RUN_REPORT.pdf),
[graph](reports/run-20261022T0732Z/p739-markowitz-paired.svg),
[raw target file](results/p739-markowitz.jsonl), and
[fresh replay](results/replay/p739-markowitz.jsonl) make the gate reviewable.
The [native stream audit](results/p739-stream-audit.json) matched the
frozen inputs, rho counters, and observed walk/candidate/accepted-relation
counts for all 60 paired targets. Those counts do not claim a bytewise
relation trace; the collector code is shared outside the pivot branch.

The separately frozen p=186,379 target-0 primary reached the
25,000,000-coefficient guard with **25,000,069** peak stored coefficients,
rank 85,463, 95,648,928 modular row updates, and 36,545,252,445 logical
index operations. Its failure code is `sparse_matrix_cap`, candidate scalar
is null, and candidate/rho cost ratio is null in all 36 pricing cells. The
same-process source-negation rho returned 34,864,137 and its source
multiplication check passed. The [upper raw file](results/p186379-markowitz-t0.jsonl),
[process status](results/p186379-markowitz-t0.status.json), and
[rho-only priced grid](results/price-grid.csv) retain the failure. The
[stream audit](results/p186379-stream-audit.json) matched the archived
workload, source-rho counters and observed 3 walks, 93,308 candidate
lambdas, and 93,307 accepted relations against the frozen 25m peeling
control. This is an aggregate-prefix check, not a bytewise trace.

The [upper-size independent process replay](results/replay/p186379-markowitz-t0.jsonl)
matched the primary non-wall record, including the cap, rho scalar and
operation counters; the [process replay record](results/upper-process-replay.json)
pins the matched outcome. The [native validator](results/validation.json)
checked 241 source scalars and 124 raw/replay rows, recomputed 4,320
priced cells, and confirmed 36 null candidate-price cells for this failed
target. The [cohort status](results/status.json) remains 543 of 1,260
distinct original targets solved, with 717 open. The p=739 gate repeats
60 previously solved targets.

The [standardized review record](results/lead-result.json),
[final PDF](reports/final-20261022/RUN_REPORT.pdf),
[graph](reports/final-20261022/markowitz-pricing-and-upper-guard.svg),
[manifest](results/manifest.json), and [validation gate](VALIDATION.json)
are the review packet. The upper-size failure does not have a candidate
price or cost ratio; the representative rho-only price is 220,794.228571
GAE. The graph shows operation counts rather than a cost ratio for that
failed target. Wall time is retained as an L0 diagnostic and excluded
from the cost ratios.
