# Prospective independent signed-pair review

Recorded before opening SignedPair.lean or Controls.lean. Reviewer: portfolio_audit; producer: separate Kummer agent. Scope: producer status describes three declarations proving signed-pair uniqueness conditional on quotient fibers and a non-two-torsion anchor. No geometric fiber correctness, xadd closure, native curve run or cost claim is requested.

Prior: the conditional abelian-group statement should hold. Blind derivation: equal first projections imply E=D or E=-D. In the latter case equal neighbor projections imply E+G=D+G (which already forces E=D), or E+G=-(D+G). Substituting E=-D in the second equation yields G=-G, contradicting 2G nonzero. This is signed uniqueness, not existence or completeness of a recovery candidate enumeration.

One reviewer owns the algebra/source-filter joint. Worked attack: remove the non-two-torsion premise and choose a nonzero anchor of order2 in a cyclic group where D and -D differ; both pairs coincide modulo sign. Also use a degenerate constant projection to show that forward sign invariance alone does not imply two-element fibers. Inspect whether source uses both checks, whether it counts distinct signed candidates, and whether it can return None despite uniqueness.

Fresh isolated replay will copy exact producer .lean files, compile the three declared modules with pinned Lean, add reviewer controls separate from producer controls, and print axioms for every main/control declaration. Only standard propext/Classical.choice/Quot.sound permitted; no native_decide, sorry or custom axioms. Dependency cache remains a disclosed trust boundary. Snapshot sources and source Rust before/after; preserve every failure.

Input is a private producer snapshot rather than a committed publication. Bind its exact hashes; root must bind publication before treating the full formal-review workflow complete. No producer repairs, scientific executions or new target enumeration.
