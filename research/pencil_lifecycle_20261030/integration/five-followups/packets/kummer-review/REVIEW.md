# Independent Kummer source review

The plan was saved after reading result.json and before reading REPORT.md or the patch. No patch changes or arithmetic trials were made. Exact inputs are in inputs.json and were freshly hashed.

## Closure: one missing explicit hypothesis
For signed R=aG+bQ, the unreduced relation 2R+cG=(2a+c)G+(2b)Q is valid in every abelian group, and the selected neighbours differ by G. Reducing labels modulo r additionally requires rG=rQ=0. REPORT.md begins with arbitrary abelian J and never states this assumption. Add it explicitly to the mathematical claim; it is not an objection to the intended prime-subgroup setting.

Exact counterexample to the literal unrestricted reduction: J=Z/11Z, G=1, Q=0, r=5, a=4, b=0, c=1. The correct successor is 9, while reduced a'=9 mod5=4 represents4. This is symbolic arithmetic, not an experiment. Source-level labels also require r>0 and sufficient range for 2a+c and 2b (e.g. canonical labels and r<=2^63). The source patch itself does not alter label arithmetic, so these are existing conditional-correctness obligations rather than a pruning regression.

## Pruning: selected values and fallback hold on the stated normal-execution domain
Read main_labelled.rs step, advance, signed_fallback, and both collector step call sites; read walk_impl_uncached.rs branch, arithmetic helpers, xadd and xadd_repaired. branch(k,p) is evaluated before either variant's arithmetic and returns0or1. Choice0 keeps kd then km; choice1 keeps km then kpp. Repaired arithmetic reads immutable arrays/actions and mutates only thread-local OPS counters. It never branches on those counters. Removing the unused double therefore preserves selected values, including None, provided no panic or external interruption occurs. The retained calls keep their relative order. advance invokes signed_fallback exactly when selected pair is None. The collector reads roots only to count successful repair entries; it never uses them to choose continuation. Validation-enabled expected projections also depend on values, not OPS.

The changes are not observational identity: unused roots become None and operation totals decrease. None already means both un-repaired success and repair failure in existing slot semantics; the branch identifies the new unevaluated slot, but any new export should explicitly document or tag evaluation status. This does not currently alter fallback.

The stated canonical prime domain p<=2^32 gives every product at most(p-1)^2<2^64. add/sub temporaries fit; canonical nonzero t makes inv's assertion safe. Sufficient eager OPS headroom is essential. Outside it, skipping a counter increment can remove a panic. Timing/deadline outcomes are deliberately excluded. This review does not prove recovery formulas, validate all runtime input bounds, or establish compilation of the proposed patch.

## Literature and evidence boundary
Reviewed producer literature summary and historical N19 validation note as background only. The primary-paper claim is not used to prove implementation equivalence: conditional group algebra and exact local call graph suffice. No novelty, new cost ratio, signed-recovery totality, or new cohort claim is supported. Parent should require the literal missing group/label hypotheses before integrating the report and perform bounded arithmetic qualification before any measurement.
