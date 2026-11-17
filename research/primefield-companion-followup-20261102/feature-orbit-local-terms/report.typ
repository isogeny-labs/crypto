#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= Regularized Miller updates on the subgroup
Native component and producer derivation • T-1784 • 2026-11-07

The evaluator retains the order and leading coefficient of each rational function. A numerator and denominator may both vanish without losing their quotient's local value. The binary Miller chain remains normalized at infinity. All arithmetic uses the original prime field.

#image("flow.svg", width: 100%)

== Constructed update
For nonzero subgroup points S, T, and T+S, define c(T)=2y(T)K′(x(T)). The preceding translation identity now yields
#align(center)[c(T+S) = c(S)c(T) / f(r,−S)(T).]
This is a local-leading-term identity, not pointwise division of kernel zeros. It permits paired ratios on the subgroup without outside points. At the Miller function's own zero, the leading term in the translated invariant parameter is −c(S)² times parameter to power r. The square is available without constructing K; the oriented sign is not supplied by that square.

== Implementation
Finite local coordinates use u=x−x(T), with three curve-expansion coefficients sufficient for line contact. Infinity uses −x/y. Product and quotient operations combine integer orders and nonzero coefficients. Finite y=0 evaluation is rejected. Prime p and prime r are caller-certified; membership, nonsingularity and r-torsion are checked. The backend is fixed-width u64 with u128 products, not a cryptographic-size implementation.

== Recorded software validation
Four native test methods pass in validation-01, with no compiler warnings. A kernel-derivative reference checks all nonzero source and target points on prime-field subgroups of orders 3, 7, 13, plus fixed selections at order479. Other controls cover the explicit cubic Miller function over F₅, tangent contact orders2 and3, cancellation of simultaneous zeros and rejected domains. The reference shares field/group arithmetic: no independent verification is claimed. PCC-20261107-b635c0 remains unsigned/unverified.

== Next bridge
Derive the divisor of the existing partial geometry residual and seek a paired update with matching support cancellation and preserved odd-weight sign. Do not identify these kernel coefficients with the existing division-jet seed without proving their relationship. Original cohort unchanged; no cloud experiment or timing comparison.

== Sources
Miller1986, Algorithm1: https://crypto.stanford.edu/miller/miller.pdf. Lynn's chord recurrence: https://crypto.stanford.edu/pbc/notes/elliptic/tate.html. Prior orbit translation and native division-jet derivations. KB unavailable. Full derivation, API domain, commands and hashes accompany this report.
