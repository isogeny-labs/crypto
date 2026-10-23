# Four follow-ups: cross-lead review packet

This packet joins the priced results available from the frozen 21-model,
1,260-target public cohort. The [graph](cost-ratios.svg) shows candidate cost
divided by matched rho at the representative declared price. The
[paired pencil graph](pencil-ratios.svg) shows the 36-cell peel/online range
and the native-price point on each measured curve. The
[pencil coverage graph](pencil-coverage.svg) shows the indexed outcome on
each of the 1,260 frozen targets by order size. The
[standardized price grid](results/standard-cost-grid.csv) retains all 36 price
cells for each measured arm and size; the [schema](DATA_DICTIONARY.md) defines
empty fields and the source grid for every row. The [PDF](REPORT.pdf) presents
the same status, selected values, and all three graphs. These are operation-count prices, not
isolated wall-time claims.

The [standardized target index](results/standard-targets.csv) joins 12,850
target-arm rows on the exact public workload key. It retains per-target
representative costs, source verification, DLP verification, and Kummer's
seven label outcomes and signed-Recover method per target. Its builder checks
that all included arms agree on the 1,260 workload IDs;
blank DLP fields apply only to Kummer's separate coordinate-only gate.
The [pencil coverage matrix](results/pencil-coverage.csv) has one row for
every frozen workload key. It records 543 keys with a verified candidate,
one capped key without a verified candidate, and 716 keys with no pencil
record in the integrated studies. The last category is an absence of indexed
evidence, not a measured candidate failure. The 543 verified keys comprise
180 each at 16, 20, and 24 bits and three at 28 bits.

| Lead and arm | Source-verified candidate / rho solves | All-size candidate/rho | All-size candidate/external | Current scope |
| --- | ---: | ---: | ---: | --- |
| Factor base, full 16 jumps | 1,260 / 1,260 | 2.448830 | 1.000000 | Five frozen factor-base and jump arms completed |
| Factor base, full 8 jumps | 1,260 / 1,260 | 2.447173 | 0.999323 | Same targets and price |
| Factor base, full 32 jumps | 1,260 / 1,260 | 2.453278 | 1.001816 | Same targets and price |
| Factor base, half base 16 jumps | 1,260 / 1,260 | 3.017732 | 1.232316 | Same targets and price |
| Factor base, quarter base 16 jumps | 1,260 / 1,260 | 3.919193 | 1.600435 | Same targets and price |
| Projective law, normalized wrapper | 1,260 / 1,260 | 3.793783 | 1.549223 | Full cold collector and rho |
| Projective law, retained weighted state | 1,260 / 1,260 | 4.179297 | 1.706651 | Full cold collector and rho |
| Point-driven pencil | 543 distinct candidate solves; 544 rho solves | See paired grid and graph | null | 717 original targets have no candidate solve; one p=186,379 target reached four caps across three solver methods |
| Kummer differential collector, uncached | 1,260 / 1,260 | 14.356662 | null | Same-process matched rho; 36-cell cold ratio 8.959687–55.935424 |
| Kummer coordinate/recovery gate | 0 / 0 DLP solves; 1,260 signed recoveries and 8,820 labels checked | null | null | Separate arithmetic gate with no collector cost |

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
completeness of the formula on every curve. A later [42-model proof audit](../projective_law_followup_20261022/PROOF_AUDIT.md)
checked another 1,224,000 finite weighted additions against Cantor with zero
mismatches. An exact integer-polynomial certificate and written divisor
argument establish the generic weighted 1+2 output identity for valid
inputs with `u(x) != 0`. A separate exact certificate covers exceptional
1+2 cancellation, the coincident-positive branch when `x4 != x`, and
tripling (`x4 = x`). Separate exact certificates and written divisor
arguments cover coprime 2+2 with `B != 0` (including `C = 0` degree
drop), same-`u` mixed composition and doubling, and `B = 0` shared
support. This covers the implemented 1+1, 1+2, and 2+2 branch map for
smooth odd-characteristic curves, valid reduced inputs, and nonzero
weights. A faithful mechanized theorem and independent mathematical
review have not been recorded.

The [Kummer coordinate evidence](../kummer_coordinates_20261022/REPORT.md)
reports 8,813 matching nonzero xADD outputs and seven zero outputs on the
8,820 checked target labels, with no wrong nonzero output. Its source labels
and scalar identities passed. The [signed-Recover gate](../kummer_recover_20261022/REPORT.md)
repaired 175/175 recorded quotient zero cases and recovered all 1,260 signed
targets: 1,257 generic and three bounded exceptional lifts. It retains literal
printed-equation failures and an exact witness. The [matched collector](../kummer_differential_collector_20261022/MATCHED_REPORT.md)
solved all 1,260 DLP targets with 1,260 fresh source-verified rho controls
in the same process. Its full-cold ratio is 14.356662 at the representative
price, 8.959687–55.935424 across the 36-cell band, with zero cohort cells
below rho. The separately frozen cached-inverse arm also solved all 1,260
targets and costs 12.945664 times archived same-source rho at the
representative price; that cached comparison uses a separate rho process
and is kept in the [source study](../kummer_differential_collector_20261022/REPORT.md),
outside the matched normalized grid.
The [validated pencil continuation](../pencil_sparse_20261022/REPORT.md)
prices nine paired 60-target slices and a separately replayed 28-bit target
across all 36 cells. Degree peeling cost less than online identity in 24/36
cells and more in 12/36 on every paired slice; all 360 degree-peel/rho cells
were above one. At the representative price, peel/online spans 0.233555 to
0.898244 and peel/rho spans 11.615357 to 22,644.843832 across these slices.
The 28-bit endpoint is one target; it is not a 60-target average. The
[target index](results/standard-targets.csv) retains all 1,449 pencil
arm-target rows in that study, including three unpriced cap failures and all
1,446 verified solves. The separate [Markowitz-priority continuation](../pencil_markowitz_20261022/REPORT.md)
has 60 source-verified p=739 solves and one replayed p=186,379 cap with null
candidate ratio. Its representative Markowitz/peel ratio is 1.204855 and
Markowitz/rho is 13.994822; zero of 36 cells cost less than peel or rho.
The combined target index has 1,510 pencil arm-target rows, including four
unpriced cap attempts across methods. The original 1,260-distinct-target
requirement remains open.

This review packet maps source results without modifying their frozen raw
records. The native Rust converter checks imported arithmetic ratios and generates the graph
and standardized files. Consult each source study's independent validator,
manifest, and hash list for source-scalar and replay evidence. The unresolved
fields are explicit rather than inferred from a phase or coordinate gate.
