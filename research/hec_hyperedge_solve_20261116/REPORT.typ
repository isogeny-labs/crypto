#set page(paper:"a4",margin:20mm)
#set text(size:10.5pt)
= Complete higher-arity supplied-target solves
Genus 2, 3 and 4 · 15 November 2026

== Target recovery survives the higher-arity construction
Keep source and target descent as catalogue equations. Fixed anchors do not have to absorb descent support. Degree-g interpolation through g+1 signed anchors generates principal equations; admit up to g off-anchor places and incrementally solve for the target scalar.

All 20 cases terminate consistently with an independent sparse solver. Twelve recover and verify, including all ten highest-arity configurations. Eight lower-arity controls exhaust or reach their tuple cap. Caller checks known expected scalars outside the solver API. No case is replaced.

#table(columns:(0.9fr,0.4fr,0.4fr,1.6fr,1.6fr),[g / p],[H],[LP],[Target 113: status / class],[Target 136: status / class],
[2 / 1009],[12],[2],[Verified / 813],[Verified / 813],
[2 / 1009],[20],[2],[Verified / 837],[Verified / 837],
[3 / 503],[14],[2],[Exhausted / 8008],[Exhausted / 8008],
[3 / 503],[14],[3],[Verified / 2068],[Verified / 1660],
[4 / 127],[10],[2],[Exhausted / 4032],[Exhausted / 4032],
[4 / 127],[10],[3],[Exhausted / 4032],[Exhausted / 4032],
[4 / 127],[10],[4],[Verified / 2139],[Verified / 2139],
[4 / 127],[16],[2],[Tuple cap / 16384],[Tuple cap / 16384],
[4 / 127],[16],[3],[Verified / 3304],[Verified / 3304],
[4 / 127],[16],[4],[Verified / 1504],[Verified / 1504])

== Target determination is stronger than anchor rank
For principal rows R, source descent a and target descent b, projected logs satisfy Rx=0, ax=1, k=bx. The scalar is determined exactly when b belongs to the row space of R together with a. A representation b=k a+lambda R is a dual certificate for the scalar.

At genus 3, H14, anchor rank completes at class 1447, but the supplied targets require 2068 and 1660. Their descent support extends beyond fixed anchors. At genus 4, support 31 / 20 exceeds H10 and H16, yet catalogue equations retain it correctly.

== Verification boundary
Fresh setup / descent per call. Exact source and target descent identities, full-group principal identities, catalogue annihilator projection certificate, nonzero projected source, integer Hasse bound, final original-source scalar check. All twenty terminal results, including None, agree with solve_fast on the inserted equations. Reused curves and known-scalar controls; no fresh holdout claim.

== Decision
Higher arity closes a real finite relation-space gap. It does not establish novelty, a new exponent or a tenfold gain. Complete existing controls on the next page remove the large apparent advantage inferred from an incomplete restricted policy. Keep this prototype; require a new complete cost premise before a timing campaign.

#pagebreak()
= Existing complete controls and the cost premise
All 78 existing control calls recover and verify their targets: six unique inputs times compact principal solver plus twelve native / table variants. Every arm remains in CONTROLS.txt. No elapsed-time comparison is reported.

#table(columns:(0.9fr,0.5fr,0.7fr,1fr,0.9fr,1fr),[g / p],[Target index],[Compact H],[Compact outputs],[Candidate classes],[Native / table b1 trials],
[2 / 1009],[0],[21],[672],[813],[1038],
[2 / 1009],[1],[21],[736],[813],[1045],
[3 / 503],[0],[64],[2744],[2068],[1503],
[3 / 503],[1],[64],[2336],[1660],[1660],
[4 / 127],[0],[38],[1888],[1504],[1489],
[4 / 127],[1],[38],[2112],[1504],[1657])

Candidate uses H12 / LP2 for genus 2, H14 / LP3 for genus 3, H16 / LP4 for genus 4 in this illustration. All other candidate policies remain on page one. Table is not a fastest-arm selection.

== Counts are not time ratios
Compact uses an optimized signed-batch kernel and includes unused batch tails in constructed outputs. Candidate uses fresh Newton interpolation per sign class. Native / table trials include a group operation and oracle. Compact draw counts can also include tuple retries. These events have different costs; no ratio or elapsed ranking is inferred. Branches64 and other oracle / solver variants remain in complete raw output.

The contrast is sufficient to reject promoting the earlier incomplete-policy comparison into a tenfold claim. Setup, source/target support, actual construction kernels and elimination still need a common production boundary. Genus 2 has no extra residual arity to admit.

== Next family question
Consider whether osculatory rational interpolation with repeated anchor support changes useful function capacity at small H. The cited rational interpolation paper already treats osculatory interpolation, so this is prior art to review, not a novelty claim. Primitive cancellation, multiplicity and independent rank/cost gates are needed before implementation. No such constructor is part of this report.

== Custody and retained preparation failure
Candidate freeze 01ac15121e29, S3 checkpoint 20261116T050035Z. Twenty cases ran once, exit zero / empty stderr.

Historical table driver initially failed on a changed solver API. A shell without fail-fast then recorded an incomplete manifest lacking the binary; it is explicitly invalid for execution custody and retained. No scientific control ran at that stage. A local compatibility copy adds only the step-column argument; original source stays unchanged.

Correct successful control freeze 93c779a4673e, checkpoint 20261116T050440Z. All 78 calls then ran once, exit zero / empty stderr. Source, lock, imported modules, native arithmetic and binary hashes are retained. No timing record or wall-time claim. Both pages rendered and inspected; PR remains unmerged.
