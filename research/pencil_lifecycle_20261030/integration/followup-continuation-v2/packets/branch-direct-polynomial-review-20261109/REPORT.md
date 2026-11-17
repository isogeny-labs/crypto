# branch-direct-polynomial-review-20261109

Source-only independent review. No payload or compiler was run.

The actual coefficientData theorem hypotheses and conclusion are byte-identical to the previous target statement. Namespace, imports and proof route change; nonzero factors remain conclusions rather than new assumptions. All source-bindings hashes and sizes matched.

Independent derivation: product support zero and F(a)=0 force N(a)^2=0, hence N(a)=0 in a field. Differentiating P=N^2-b^2 F gives P'(a)=-b^2 F'(a), nonzero from b!=0 and a simple root. Therefore P has multiplicity exactly one. Product multiplicity yields the three-factor sum and nonzero factors.

The pinned Mathlib one_lt_rootMultiplicity_iff_isRoot needs only nonzero P and returns the root and derivative-root conjunction used here; rootMultiplicity_mul receives separately derived nonzero product hypotheses. The source specialization uses the exact existing source_norm_identity and separability-to-simple-root declarations.

The rational actual-source fixture retains F=X^5-X, distinct supports X^2+1 and X^2-1, b=-4, residual=-16X, divisibility and explicit Bezout separability proof. It demonstrates simultaneous satisfaction of the actual-source premises at a=0.

Three additional generic controls retain the norm identity while removing one premise: b=0 permits sum2; nonsimple F=X^2 permits sum2; missing support for F=X permits sum0. These are generic identity controls, correctly not claimed to realize coefficientData. The separate wrong-root expected failure remains at a=2.

Eighteen named readbacks cover the generic theorem, exact source specialization and all controls. Source inspection found no weakened statement, circular multiplicity premise or extra axiom. Elaborator and kernel acceptance remain unmeasured.

Remaining: Bind the successful Nonbranch artifact and its transitive custody into one finite four-call replay; coordinator allocates and publishes that concrete runner.
