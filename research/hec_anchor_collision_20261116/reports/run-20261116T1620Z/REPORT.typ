#set page(paper:"a4",margin:18mm)
#set text(size:10pt)
= Joint target descent and weighted large-prime core
16 November 2026 · hec_anchor_collision_20261116 · combined two-run report

== Structural integration
A full-degree public divisor D=(u,v) gives h=v+t u. Two signed anchors with equal (y-v)/u force two norm factors; the residual Q has degreeg−1. Scan H anchor abscissae, at most2Hsheets, instead of all rational catalogue points. Genus2/3/4residuals are linear/quadratic/cubic. Repeated target support is retained. This is standard divisor arithmetic, not new mathematics.

Source and target variables join anchor logs INSIDE the existing affine weighted LP forest. Seed source variable=1. Input partials may retain two unresolved endpoints; subsequent principal relations resolve them while preserving the target coefficient. Pure core equations feed incremental elimination. Every admitted raw relation passes an original Jacobian identity; pure equations pass subgroup-certificate checks. Candidate k is checked against the original source/target and independently recomputed by sparse elimination. No expected scalar enters the solver API.

== Run1: frozen one-pencil arm
Sourcef121c5264de2, pre-run S3 checkpoint20261116T162027Z. Once-only180second watchdog, exit0/empty stderr. Six reused direct-profile inputs: g2/F251, g3/F211, g4/F61; two public targets each. Prefix H13/36/22. A single input pencil plus direct split rows, then bounded principal collection. No input replacement or target shifting in this arm.

#image("../../ONE-PENCIL.svg",width:100%)

Candidate succeeds on g2target0and bothg3targets. g2target1and bothg4targets retain missing_input_row. With no usable target equation, principal rows cannot determine its free column. Compact recovers all six controls. Raw CONTROL.tsv and main.rs are unchanged. The next arm is a separately pre-registered change after these observations, not a rerun or confirmation of this arm.

#pagebreak()
== Run2: bounded shifts, original target retained
Source6caad1753ce8, pre-run checkpoint20261116T162341Z. At most8anchor additions/input, with separate deterministic RNG, while no input row has been admitted. Exact offsets remain in every equation: collision rows add offsets; direct-split rows subtract them. Raw identities use the original source/target. Once-only180second watchdog exits0/empty stderr. All six scalars verify with independent sparse replay.

#table(columns:6,inset:4pt,
[g / p],[Target],[k],[Shift adds],[Scans],[Unused tail],
[2 / 251],[0],[8828],[0],[13],[3],
[2 / 251],[1],[5635],[1],[26],[1],
[3 / 211],[0],[660106],[0],[36],[2],
[3 / 211],[1],[420084],[0],[36],[7],
[4 / 61],[0],[2398674],[2],[66],[8],
[4 / 61],[1],[1526446],[1],[44],[1])

Four shifts total,22unused batch outputs retained. All input scans and failed earlier pencils are charged categories. Source inputs in this panel are degree-one generators; arbitrary full-degree source coverage is not established here. Prefix membership differs from Compact's target-seeded anchors. This reuses an existing panel, with no fresh-holdout claim.

#image("../../COUNTS.svg",width:100%)

Derived Compact outputs=unique tuples×2^g. More principal outputs are prepared on three inputs and fewer on three. These are finite deterministic counts, not timings, speed ratios or population estimates. Compact count vectors reproduce between its two control runs; no wall-time A/A claim follows.

#pagebreak()
== Complete-work boundary and decision
The shifted candidate is a complete source/target scalar solve on this finite panel, with direct input rows, bounded input-pencil retries, shared LP elimination, principal collection, final original verification and sparse scalar replay. Its controls recover the same six scalars. The frozen original arm's three noncompletions remain.

Both candidates cap principal search at1024unique tuples/4096draws or family exhaustion. Every selected tuple prepares the entire2^gCompact Gray batch, including unused final outputs. Duplicate draws, catalogue/table/inverse setup, input scans/shifts, norm division, roots, weighted graph/potential updates, linear algebra, allocation and final verification remain costs. Counters are selected categories, not a complete arithmetic inventory. Extra multiplier certificates and independent replays are gate work, not free production setup or a production timing comparison.

The joint forest resolves a concrete pipeline dependency: a target partial can be retained before its endpoints are resolved. Bounded shifts address missing target rows in this panel. Neither fact proves a mathematical tenfold gain. Mixed generation counts and extra scans do not supply a complete cost premise against Compact. No timing campaign, exponent claim or performance promotion follows from correctness alone. Increasing caps or comparing only to Gaussian interpolation would not establish the requested result.

Further work requires broader field/source coverage and a cost case showing which charged stage can fall enough to support the full objective. The mathematical10xgoal remains open. This integrates established Riemann–Roch/large-prime algebra without a priority claim.

== Reproducibility and scope
PROTOCOL.md and SHIFT-PROTOCOL.md define separate arms. FREEZE.sha256 identifies the original source/lock/imports/binary atf121c5264de2; adding a binary changed Cargo.toml, whose original bytes are preserved in ORIGINAL-Cargo.toml. SHIFT-FREEZE.sha256 identifies the amended pre-run versions. Both prior S3 checkpoint gates pass. CONTROL.tsv/SHIFT-CONTROL.tsv, stderr and exit files preserve every outcome. Native reduce.rs and one_plot.rs read fixed raw counters and render these graphs; no scientific Python.

Full input identities are in the preceding DESCENT-INPUTS.txt; deterministic instance-generator bytes are frozen here. Original-group raw checks include branch places; odd-prime projection kills branch2-torsion in core equations. A certified catalogue multiplier is coprime to ell, and mandatory [k]source=target remains in the original Jacobian. Shared field/group code remains a dependency; independent scalar elimination is not independent scientific validation.

Identical combined three-page reports are stored under both run ids; all pages visually inspected. No timing record, benchmark-index/leaderboard row, independent VALIDATION.json or machine-checked full group certificate. Previous canonical timing graphs/sealed reports remain unchanged. Today's shared daily log is reserved by T-6720; scoped SESSION.md preserves the handoff. Primary external PDF access gap remains explicit in THEORY.md. PR4357 remains unmerged.
