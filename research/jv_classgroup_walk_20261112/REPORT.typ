#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
= A class-group isogeny walk into the JV weak class
_Kernel-polynomial odd isogenies; protocol frozen before the runs, two disclosed amendments. Every result is an unverified assertion [pcc:unverified]._

`research/jv_weak_coverage_20261111` showed the weak-class walk reached only about 1% of curves at p = 1009, because its odd steps used only ℓ-isogenies with a _rational_ kernel point. This study implements the full set of rational odd ℓ-isogenies (degrees 3, 5, 7) by Vélu through the _kernel polynomial_ — a Galois-stable cyclic ℓ-subgroup whose points need not be rational.

#figure(image("reach.svg", width: 100%), caption: [Left: fraction of admitted classes reaching a weak curve, by prime (Wilson 95 percent), class-group vs rational-root walk. Right: fraction of successful class-group walks that use a new odd kernel-polynomial step.])

*Construction* (`src/cryptanalysis/jv_isogeny_walk.rs`). `kernel_polys` factors the ℓ-division polynomial over F#sub[q³] and keeps degree-(ℓ−1)/2 factors whose roots are doubling-closed (a cyclic ℓ-subgroup); `velu_image_through_h` and `map_odd_h` evaluate the isogeny by symmetric functions, self-certified as a degree-ℓ map. Correctness gates C1–C5 all pass: C1 symmetric Vélu equals the validated per-root Vélu on split kernels; C2/C4 certified isogenies are real-point homomorphisms including non-rational kernels; C3 the doubling-closure is the decisive filter; C5 valid subgroup counts and completeness.

*Result* (small p, cap 10,000, 60 walks).
- p = 37: class-group 41/60 = 68 percent vs rational-root 24/60 = 40 percent (non-overlapping intervals); 93 percent of found walks use a new odd step.
- p = 53: class-group 17/25 = 68 percent (partial) vs 25/60 = 42 percent; 94 percent use a new odd step.

*Large p.* At p = 1009, 1511, 1823, paired at cap 3,000, both walks reach the weak class rarely: the nearest weak curve is beyond a feasible 2-, 3-, 5-, 7-isogeny search radius. The class-group walk improves connectivity within the explored region, which helps when the target is in range (small p), not when it is beyond any feasible cap.

*Not a cost result.* The validated sieve speedup applies once a curve is _in_ the weak class; this widens the set of curves that can reach the class at small p. No leaderboard row moves.
