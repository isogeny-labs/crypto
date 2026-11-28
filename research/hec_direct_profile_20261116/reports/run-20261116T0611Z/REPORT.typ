#set page(paper:"a4",margin:18mm)
#set text(size:10pt)
= Direct weighted-Cantor rows in a complete IC solver
15 November 2026 · Record 20261116T0611Z-hec-direct-profile

== Constructive result and correctness
The interpolation-to-Cantor row identity now drives a complete supplied-target solver. Cold multiple caches and reflected Gray traversal generate sign batches, stored in the same integer sign order as Gaussian interpolation. Prepared batch tails are charged. Source/target descent remains catalogue equations.

On fields disjoint from profile selection, 18 paired inputs / profiles give 72 solver calls: 64 verify; eight genus-2 polynomial controls exhaust. All selected profiles recover. Gaussian/Direct and correctness/production paths consume exactly the same original equations and match scalar, status, work counts, rank and descent. Up to 63 unused prepared outputs are retained in the gate.

Production omits the extra catalogue-annihilator witness under the explicit prime-order and ell-squared Hasse bound argument in STRUCTURE.md; it retains initial validity / torsion checks and final original-source verification. The correctness path retains full principal/group checks and an independent scalar solver. Standard group/divisor algebra is not a novelty claim.

== Measured boundary
Algorithm-cold single-target wall seconds: fresh tables/catalogue, caches, descent, all relation construction including unused outputs, filtering, linear algebra, allocation/result drops and final group verification. Common input selection/order admission and known-target construction are excluded equally. Inputs/code/allocator/OS remain warm in one process; no fresh-child or CPU-isolation claim.

Seventeen arms on each of six inputs, five batches × sixteen fresh algorithm setups: 510 cells / 8160 calls, all verified. Apple M4 Pro, 14 logical CPUs, macOS 27.2 ARM, Rust 1.99.1. Shared local L0 producer observations. Native/table caps 200000; all controls complete.

== Complete-solve observations (wall_s)
#table(columns:(0.9fr,0.35fr,1fr,1fr,1fr,0.8fr),[Genus / p],[Target],[Direct],[Gaussian],[Best existing],[Existing / Direct],
[2 / 251],[0],[0.000696510],[0.001472089],[0.000104872],[0.150568],
[2 / 251],[1],[0.000591815],[0.001285661],[0.000120727],[0.203994],
[3 / 211],[0],[0.001412172],[0.003201302],[0.000385766],[0.273172],
[3 / 211],[1],[0.001108880],[0.003202747],[0.000367826],[0.331709],
[4 / 61],[0],[0.000979958],[0.002315776],[0.000323951],[0.330576],
[4 / 61],[1],[0.000929977],[0.002324175],[0.000299070],[0.321589])

Measured median of five batch means; derived ratio. Best existing is Compact on every input, selected by minimum median among thirteen complete implementations. Gaussian is a same-family constructor reference, not the primary incumbent. No pooled target / host statistic or fastest-single-cell selection.

== Frozen qualification fails
Twenty of 102 arm/input spread ratios exceed 1.25; worst 1.745383804 at g2 target1 n1fr. Direct A/A ratio 1.222747831 at g3 target0 exceeds 1.10. Table A/A passes. Every cell remains; no outlier removal, retuned threshold or rerun. Numerical tenfold criterion also fails on all six inputs. No qualified speedup or mathematical breakthrough follows from this cohort.

#pagebreak()
= Visual evidence and frozen decision
#image("../../timing/COST.svg",width:100%)

Five batches of sixteen fresh setups per point. Median with min/max bars; log scale in wall seconds per solve. Figure shows Direct, Gaussian and the best measured existing control on all six inputs. The next page retains every arm. Native Rust produces all values; Matplotlib only lays out the recorded plot table.

== Interpretation at the measured scope
Direct has lower observed cost than the Gaussian reference (derived ratios 2.114–2.888), but the measured existing Compact implementation is lower than Direct in every case. Stability qualification fails, so these are unqualified local observations, not accepted gains or formal regression estimates. The new constructor does not supply a tenfold total-cost case against the measured incumbents.

Direct still uses generic Cantor additions and full-catalogue incremental echelon. Individual phase shares were not measured. Further tuning needs a new complete cost premise; multiplying component ratios from different cohorts or comparing to incomplete policies cannot establish the goal. Engineering implementation is complete; the mathematical tenfold objective remains open.

== Exact workload and custody
Inputs are g2 / p251 / c11 / ell61667 with H8 and parts4+1+1; g3 / p211 / c12 / ell4620611 with H9 and parts2+2+1+1+1; g4 / p61 / c2 / ell16790591 with H10 and parts2+2+2+2. Targets are ell/7+19 and ell/11+29; public Mumford coordinates and polynomial models are in INPUTS.txt. These hyperelliptic models are not assigned fabricated elliptic ICV1 names.

Scientific source 5efdf3e7ec5d7f15a10fd521c7a840a09b5c2f69, checkpoint 20261116T061139Z. Record registration 0200efe908a6, checkpoint 20261116T061305Z precedes execution. Command exits zero, all cells verify, no scientific/build failure. The dirty flag refers to the unrelated preexisting resume edit. Source/lock/import/native/binary hashes and raw cells accompany the record; raw objects are uploaded to content-addressed S3.

No independent signature, VALIDATION.json or leaderboard promotion. Existing canonical IC graphs are unchanged because no admitted result is added; benchmark progress is regenerated from records. Whole historical ledger issues are separate from per-record validation. PR remains unmerged.

#pagebreak()
= Every measured arm: wall_s and ratio
Each cell below is median wall seconds per solve, followed by the derived best-existing / arm ratio. Five batch means per cell, sixteen fresh setups per batch. All calls verify. Values are descriptive; global spread and A/A qualification failed.

#include "MATRIX.typ"

== Arm definitions
Direct and DirectAA are identical selected-profile weighted-Cantor kernels; Gaussian uses the same profile stream with confluent construction. Compact is the existing bounded-anchor signed kernel. Native n1 / n64 uses one or 64 walk branches; f / h denotes Fast / Hybrid; r / d denotes reference or alternate native oracle. Table t1 / t64 uses the table oracle; f / h denotes Fast / Hybrid. t1faa is the identical t1f A/A control. A/A controls and both new kernels are excluded from the thirteen-arm incumbent minimum.

These are per-input observations; no curve/target pooling. Spreads and raw batch values are in STATS.tsv / CELLS.tsv. ALL-ARMS.md retains the full precision table with correctness and ratios. Primary qualified tenfold result is false; none of the six numerical primary ratios reaches ten.
