# Four follow-ups: cross-lead review packet

This packet joins the priced results available from the frozen 21-model,
1,260-target public cohort. The [graph](cost-ratios.svg) shows candidate cost
divided by matched rho at the representative declared price. The
[standardized price grid](results/standard-cost-grid.csv) retains all 36 price
cells for each measured arm and size; the [schema](DATA_DICTIONARY.md) defines
empty fields and the source grid for every row. The [PDF](REPORT.pdf) presents
the same status and selected values. These are operation-count prices, not
isolated wall-time claims.

The [standardized target index](results/standard-targets.csv) joins 10,080
target-arm rows on the exact public workload key. It retains per-target
representative costs, source verification, DLP verification, and Kummer's
seven label outcomes per target. Its builder checks that all eight currently
integrated arms agree on the 1,260 workload IDs; blank DLP fields mean that
the coordinate gate did not run a collector.

| Lead and arm | Source-verified candidate / rho solves | All-size candidate/rho | All-size candidate/external | Current scope |
| --- | ---: | ---: | ---: | --- |
| Factor base, full 16 jumps | 1,260 / 1,260 | 2.448830 | 1.000000 | Five frozen factor-base and jump arms completed |
| Factor base, full 8 jumps | 1,260 / 1,260 | 2.447173 | 0.999323 | Same targets and price |
| Factor base, full 32 jumps | 1,260 / 1,260 | 2.453278 | 1.001816 | Same targets and price |
| Factor base, half base 16 jumps | 1,260 / 1,260 | 3.017732 | 1.232316 | Same targets and price |
| Factor base, quarter base 16 jumps | 1,260 / 1,260 | 3.919193 | 1.600435 | Same targets and price |
| Projective law, normalized wrapper | 1,260 / 1,260 | 3.793783 | 1.549223 | Full cold collector and rho |
| Projective law, retained weighted state | 1,260 / 1,260 | 4.179297 | 1.706651 | Full cold collector and rho |
| Point-driven pencil | 541 / 541 distinct in integrated online pilot | See separate nine-curve and one-target prices | null | 719 original targets remain; sparse-rank continuation in progress |
| Kummer coordinate walk | 0 / 0 DLP solves; 1,260 source targets and 8,820 labels checked | null | null | Fast recovery and priced collector remain open |

The representative cell is `M + 0.5S + 10I` with 0.1 source-addition
equivalent (GAE) for each declared native logical operation. All-size ratios
divide summed candidate cost by summed comparator cost. A per-size line may
cross another even if the all-size ratio differs; inspect the
[raw normalized grid](results/standard-cost-grid.csv), not just this table.
The [factor-base evidence](../four_followups_20261021/REPORT.md) includes
6,300 candidate and 6,300 matched rho source-verified scalars. The
[projective-law evidence](../projective_law_followup_20261022/REPORT.md)
includes two 1,260-target arms, 576 independently recomputed arm/size/price
cells, and branch records. Those finite checks do not prove universal
completeness of the formula on every curve.

The [Kummer coordinate evidence](../kummer_coordinates_20261022/REPORT.md)
reports 8,813 matching nonzero xADD outputs and seven zero outputs on the
8,820 checked target labels, with no wrong nonzero output. Its source labels
and scalar identities passed. No full Kummer DLP target was solved by a
coordinate-only collector, so its operation price and ratio are null.
The [pencil pilot](../four_followups_20261021/REPORT.md) prices nine complete
60-target small-curve slices and a separately replayed 28-bit target. Its
original 1,260-target requirement remains open; additional sparse-rank
measurements are retained separately when available.

This review packet maps source results without modifying their frozen raw
records. `build.py` checks imported arithmetic ratios and generates the graph
and standardized files. Consult each source study's independent validator,
manifest, and hash list for source-scalar and replay evidence. The unresolved
fields are explicit rather than inferred from a phase or coordinate gate.
