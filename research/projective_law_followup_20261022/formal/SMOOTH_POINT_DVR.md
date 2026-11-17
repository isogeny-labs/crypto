# Rational-point Jacobian condition and the actual point-stalk DVR

The checked `pointwise_jacobian_iff` identifies `2*y ≠ 0 ∨ f'(a) ≠ 0` with the disjunction that at least one partial derivative of `y²-f(x)` is nonzero at `(a,y)`. No assumption on the characteristic is needed for this algebraic equivalence. The source uses the actual monic quintic `X⁵+a₃X³+a₂X²+a₁X+a₀`.

Under the rational point equation `y²=f(a)` and separability of that quintic, `point_local_dvr` and `projective_point_dvr` combine the two previously checked local-parameter proofs. `projective_point_dvr_of_jacobian` states the same DVR conclusion from the nonzero-partial-derivative premise. These statements concern the actual weighted-Proj point stalk. They do not formalize an equivalence between this differential condition and a scheme-theoretic `Smooth` predicate; that identification remains open.

Review artifacts: [PDF note](SMOOTH_POINT_DVR.pdf), [SVG dependency graph](smooth-point-dvr.svg), [CSV status table](smooth-point-dvr.csv), [Lean source](WeightedProjSmoothPointDVR.lean), [direct Lean/axiom log](smooth-point-dvr-lean.log), [TeX source](SMOOTH_POINT_DVR.tex), and [vector figure](smooth-point-dvr-figure.pdf). Source SHA-256: `b662ea86f06b6ff9066d283272304f83df49aee3a3d8fdaad19a4740f471c283`.

Direct pinned Lean 4.34.1 compilation exited zero against Mathlib `d13f23b723b8a846827a245b89c10fc7d3f11612`. Five axiom audits list only `propext`, `Classical.choice`, and `Quot.sound`; no `sorryAx`. Global Pic0 transport, source-style order allocation in characteristic two, and compiled Rust semantics remain open. No target or cost result is claimed.
