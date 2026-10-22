# Projective genus-2 law follow-up: frozen paired cohort

The [PDF report](REPORT.pdf), [cost graph](cost-ratios.svg),
[direct 36-cell graph](direct-price-grid.svg), and
[branch graph](branch-coverage.svg) are rendered from the standardized records
below. The experiment includes 21 destination models and the same frozen
1,260 known-answer source targets in each of two collector arms. Each raw
candidate and its same-process negation-rho control recovered a scalar that
was independently checked by source-curve multiplication: 1,260/1,260 in
each arm and control, with zero exhausted or failed rows.

The wrapper arm calls the weighted genus-2 law, then normalizes its result to
affine form at every addition. The retained-state arm keeps weighted divisors
through jump setup and walks, tests splitting in projective coordinates, and
normalizes only split candidates at the relation boundary. The latter arm
therefore has no routine group-law inversions; its 222,375,456 boundary
normalization inversions are charged in the cold counts. Both use the frozen
full factor base, 16 jumps, per-target seed and cap, and paired source rho.

| Order bits | Wrapper/rho | State/rho | State/wrapper | Wrapper/external | State/external |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 16 | 1.9818 | 1.8665 | 0.9418 | 1.6825 | 1.5846 |
| 20 | 2.5346 | 2.5111 | 0.9907 | 1.6571 | 1.6418 |
| 24 | 3.3030 | 3.4237 | 1.0365 | 1.6092 | 1.6680 |
| 28 | 3.4777 | 3.9598 | 1.1386 | 1.5860 | 1.8059 |
| 32 | 3.5856 | 3.7514 | 1.0462 | 1.6071 | 1.6815 |
| 36 | 3.9013 | 4.0603 | 1.0407 | 1.5719 | 1.6359 |
| 40 | 3.7933 | 4.2482 | 1.1199 | 1.5395 | 1.7241 |
| All 1,260 | 3.7938 | 4.1793 | 1.1016 | 1.5492 | 1.7067 |

The table uses summed full cold cost at `M+0.5S+10I` and 0.1 source-addition
equivalent (GAE) per logical operation. It includes construction, transfer,
walk initialization, relation collection, elimination, source verification,
field additions, bit operations, and native logical work. Ratios divide
summed costs on matched targets, not averages of target ratios. The 40-bit
rows use all 180 paired targets. At this price, pooled costs are 4,130,679,615
GAE for the wrapper, 4,550,428,527 for retained state, 2,666,291,894 for
the wrapped external-law collector, and 1,088,802,370 for source rho.

Across the 36 frozen price cells, pooled wrapper/rho is 2.0975–21.4887 and
state/rho is 1.4729–28.0153; neither is below rho in a pooled cell. The
direct state/wrapper range is 0.5468–1.3037: 12/36 cells favor retained
state and 24/36 favor the wrapper. The exact inputs and costs for every cell
are in [the direct paired grid](results/price-grid-state-vs-wrapper.csv).

The [cold-phase breakdown](results/cold-phase-breakdown.csv) gives arithmetic
counts and GAE for every phase, arm, comparator, bit slice, and pooled slice.
Its phase costs add to each full representative cost. For the 180-target
40-bit slice, the values in millions of GAE are:

| Method | Setup | Search and elimination | Native logical | Source verification |
| --- | ---: | ---: | ---: | ---: |
| Wrapper | 1.206 | 2,861.016 | 248.087 | 0.079 |
| Retained state | 0.961 | 3,228.833 | 253.458 | 0.079 |
| External | 0.669 | 1,771.581 | 248.087 | 0.079 |
| Rho | 0.860 | 734.608 | 84.414 | 0.079 |

The arithmetic gate checked 21 normalized Scholten models against independent
Cantor arithmetic: 24,192 weighted 1+2 cases, 26,880 sampled weighted 2+2
pairs, 2,310 targeted weighted 2+2 exceptional cases, 1,680 weighted doubles,
and 21,504 consecutive weighted-state additions. Its records explicitly
include inverse, shared-support, coincident-support, degree-drop, and doubling
branches, with zero unchecked weighted cases. The [exact p=739, W=3
regression](results/n6-w-power-regression.json) retains the transcribed
author-code `N6` expression, the homogeneous `W²` variant, both resulting
divisors, and the independent Cantor divisor. The comparison is scoped to
the transcribed Magma code at
[ozbayelif/jac-on-jac](https://github.com/ozbayelif/jac-on-jac), rather than
an unshown equation in the [published paper](https://link.springer.com/article/10.1007/s13389-024-00351-7).
Finite branch checks do not establish universal correctness for every curve
or input.

The [protocol](PROTOCOL.md) defines the frozen input and accounting.
[DATA_DICTIONARY.md](DATA_DICTIONARY.md) defines raw JSONL, per-target
normalized records, cost grids, and failure fields. The independent Rust
validator recomputed all 576 arm/slice/price cells and exact cold-phase sums;
its [certificate](results/independent-validation.json), source replay counts,
and [validation notes](VALIDATION_NOTES.md) are retained. The representative
ratios are operation-count prices, not isolated wall-time claims.
