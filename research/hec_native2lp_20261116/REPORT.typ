#set page(paper:"a4",margin:22mm)
#set text(size:10.5pt)
= Native signed large-prime graph: three-genera controls
Genus 2, 3 and 4 · 15 November 2026

#table(columns:(1fr,1fr),[Gate],[Observed],[Planted affine constraints],[9000],[Reduced rows checked],[8712],[Vertex invariants],[Every vertex after every edge],[Curve reference/candidate cases],[12 pass],[Signs and nonunit coefficients],[Explicit controls pass],[Corrupted relation],[Nonzero residual detected])

== Correct signed-cycle algebra
Partial relation → affine component potential → balanced cycle: full relation → unbalanced cycle: anchor component → later anchored cycle: full relation.

The dummy root represents zero. Multiplicities and signs are kept as field coefficients. Not every graph cycle eliminates all large primes. Sparse-potential storage is capped, including compression updates.

== Integration against the existing native reference
The existing bounded native module already supports zero, one and two large primes with dense elimination. That module was missed in the initial audit; the correction precedes curve execution. New work replaces its combiner, preserving catalogue/order projection, decomposition and final scalar verification. Every touched potential and emitted full relation is independently checked in the projected actual Jacobian.

#table(columns:(1fr,1fr,1fr,1fr),[Genus / prime],[Smallbase],[1LP trials, two seeds],[2LP trials, two seeds],[2 / 101],[23],[103, 157],[86, 113],[3 / 61],[16],[421, 311],[260, 231],[4 / 31],[8],[866, 1343],[536, 761])

Reference and candidate agree on stopping trial counts, rows and recovered scalar in all twelve cases. These are internally generated known-scalar controls, not arbitrary public targets. Trial counts are not wall-time ratios. Sparse graph counters differ from dense baseline counters and are not mixed.

== Custody and remaining objective
Graph freeze0ef6b25b0942/checkpoint20261116T041713Z; curve freezea9fd49abdc48/checkpoint20261116T042221Z. Each gate executes once, exit zero, empty stderr. No timing run. Initial integration compile failure is retained and fixed before the freeze.

The graph follows known large-prime mathematics, not a novelty claim. Dense input assembly and catalogue/projection costs remain. Next: arbitrary supplied targets, sparse assembly and a full-cost same-instance comparison. Published complexity, genus-wide applicability and the mathematical tenfold objective remain unestablished. Earlier F631results and compiled-inverse proof obligations are unchanged.
