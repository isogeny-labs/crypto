# Shared canonical variable: exact, modest structural reduction

Source02af681212a6, checkpoint20261116T003824Z preceded one standalone scientific execution in run/. Exit0, stderr empty. No rerun, discarded case or scientific failure. Candidate uses only runtime F127quartic and fixed48factorbase. It introduces canonical-log variable z in basecolumn47, shifting only LPindices; checks augmented, defining and all original unlifted relations. All129point logs projectively match the independent dense reference,314originalresiduals pass, and twelve planted kernel/corruption controls pass.

| Structural count | Original sparse V2 | Shared canonical variable |
|---|---:|---:|
| Input rows |314|315|
| Input nonzero coefficients |2378|1558|
| Maximum input row support |8|5|
| Reduced rows |232|234|
| Reduced columns |47|48|
| Reduced nonzero coefficients |2413|2155|
| LP elimination operation counter |14455|13801|
| LP elimination entry counter |9480|7703|
| Eliminated LP columns |81|81|
| Sparse-kernel attempt |0|0|

Reference structural counts come from frozen prior run-v2/stdout.txt, not a new timing sample. Input coefficients fall34.5%, reduced nonzeros10.7%, LPoperations4.5%, LPentrycounter18.7%; dimensions grow. These are implementation counters, not CPU/wall gain or full-solver work. Candidate retains original sparse rows for verification in addition to augmented rows. Thus input coefficient reduction is NOT a measured total-memory reduction. No fullcold deployment or tenfold claim is made from this modest result.

## Exact algebra and proof boundary

For original equations (Hi−h)x=0, introduce Hi x−z=0 and hx−z=0. The lift x↦(x,hx) and projection are inverse on kernels. Subtracting the defining row from every other augmented row yields the original matrix plus a final row with a unique nonzero auxiliary coefficient; hence rank increases1and nullity is preserved over any coefficient field. The canonical variable must stay in the small core; immediately eliminating it undoes this representation.

SharedCanonical.lean checks equation equivalence and uniqueness of the lifted coordinate in the same pinned Lean4.19/mathlib environment as the earlier centering proof. This formal check does not prove sparse implementation correctness, matrix-rank formula, plane-curve geometry, termination or runtime. LEAN.txt/EXIT-LEAN.txt retain outcome/axioms. LEAN-PRODUCER.txt retains an initial incorrect-working-directory/filemissing attempt; source scientific execution was not repeated.

## Decision relative to the actual objective

Keep as a possible solver representation; don't promote to a mathematical breakthrough or10xgain. It applies to shared divisor subtraction in any genus but the finite check covers only this F127quartic. No timing comparison was collected. The full executed cold default remains sparseV2, whose source/destination recovery is already verified. The next decisive work must address field/factorbase scaling and construction cost, rather than continue repeating fullcold checks of modest count changes. Genus2/4and the fullmathematical10xobjective remain open; the earlier failed charged coldcost result is unchanged.
