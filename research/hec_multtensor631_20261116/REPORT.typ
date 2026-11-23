#set page(paper:"a4",margin:24mm)
#set text(size:11pt)
= Compiled quotient multiplication and dual maps
Genus 3 · 15 November 2026

#table(columns:(1fr,1fr),[Correctness evidence],[Observed],[Compiled degree pairs],[66],[Sparse structure entries],[20132],[Exact product comparisons],[283914 pass],[Exact colon nullspaces],[2665 pass],[Recovered original classes],[Both unchanged],[Conditional Lean identity],[Checked; exit zero])

== Algebraic representation
Actual quartic normal forms → sparse multiplication tensor → dual annihilator matrices → colon nullspaces → unchanged Jacobian operations.

The product evaluator uses compiled basis structure coefficients. For a colon call, each annihilator row contracts with that tensor once, producing a bilinear matrix; applying it to supplied forms produces the same linear constraints by finite-sum interchange. This removes repeated inner monomial lookups and repeated construction of unit-polynomial products.

Every product coefficient and every resulting canonical colon nullspace matches the reference in actual original-input reconstruction. Public adjacent-sign and corruption controls pass. The degree-two fixed-base cancellation control is retained. Counters include reference-check operations and are not production operation counts.

== Formal scope
Dual.lean proves the contraction rearrangement for arbitrary commutative rings and finite index sets. It uses standard axioms propext, Classical.choice and Quot.sound. The normal-form table, Rust compiler, field arithmetic, nullspace routine and global geometric correspondence are not formalized by this lemma.

== Custody and claim boundary
Sourcefe390c895615 and checkpoint20261116T034008Z precede one execution. Exit zero; stderr empty. Cache construction is in-run and stays below the frozen storage cap.

This comparison build runs reference work and cannot measure speedup. A future production comparison must charge compilation and dual-map construction and use stable same-host controls/phase attribution. Earlier1.621xand drift-affected0.930cohorts remain separate. No novelty, tenfold gain, generic complexity or genus-two/four mechanism is established.
