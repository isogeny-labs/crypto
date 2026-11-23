---
title: "A nonregular Edwards descent with nonzero equations"
fontsize: 10pt
geometry: margin=17mm
mainfont: DejaVu Serif
---

Lean checks a concrete twisted-Edwards point-decomposition input over F25:
both descended equations are nonzero quadratics, the affine coordinate
algebra has dimension two over F5 and is Artinian, yet their highest
homogeneous parts are not regular. Whole gate: 55 declarations, zero failures and zero
trusted-native exceptions. Signed standing remains unverified.
[pcc:unverified]

Take $K=\mathbb F_5[\beta]$, $\beta^2=2$, and
$a=\beta$, $d=4$, $R=(1,1+\beta)$ on $ax^2+y^2=1+dx^2y^2$.
The parameters are valid, the target is smooth and its y-coordinate and
a are outside F5. Actual factor-base points
$P=(-(2+3\beta),2)$ and $Q=(2+3\beta,3)$ add to R with nonzero Edwards
denominators. Their explicit images are actual Mathlib points on
$Y^2=X^3+(2+3\beta)X^2+(3+2\beta)X$; Lean checks their group sum and
R≠O directly. [pcc:unverified]

The exact FGHR2014 printed page 604 polynomial is used, including a/d and
the cross coefficient 2(d−a)/d. Universal symmetrization with e1=y1+y2,
e2=y1*y2 and actual coefficient extraction in the basis (1,beta) give

$$f_0=2(e_1^2+e_2^2)+2e_2+1,\qquad f_1=2(e_1^2+e_2^2)+3.$$

Their two highest homogeneous parts coincide and are nonzero. Mathlib
`RingTheory.Sequence.IsRegular` is directly disproved for those parts,
which is the affine-regularity definition in the paper. [pcc:unverified]

Exact ideal certificates and an actual algebra equivalence prove the following.
[pcc:unverified]

$$ (f_0,f_1)=(e_1^2,e_2-1),\qquad
\mathbb F_5[e_1,e_2]/(f_0,f_1)\simeq\mathbb F_5[T]/(T^2). $$

Thus the affine quotient is finite-dimensional/Artinian and has length 2.
Its F5 zero set is exactly {(0,1)}. Multiplicity and support differ.
The source's generic degree 4 is not a universal equality.
[pcc:unverified]

This disproves the literal universal n = 2 reading of Hypothesis3 with the
paper's Edwards representation and nonzero/zero-dimensional affine inputs.
It strengthens the prior short-Weierstrass [f,0] witness. A generic-input
or n ≥ 3 reading, conditional Theorem 4.1, prime-order sampling distribution
and Joux–Vitse Assumption 2 remain separate. No native solver outcome,
generic refutation or novelty claim is asserted. [pcc:unverified]

In namespace `EdwardsDescentAudit`, the closed binding is
`actual_edwards_descent_counterexample`.
Lean 4.35.0 and Mathlib 56a00585 use only propext, Classical.choice and Quot.sound:
no holes, custom axioms, native_decide, waivers or raised elaboration limits.
All failed attempts and complete gate receipts are retained losslessly.
Original FGHR §§1–4/Hypothesis3 and Joux–Vitse pp4–5/7/9–10 are recorded
with exact hashes/read scopes. KB unavailable (MCP absent/wrapper127);
bounded primary/repository/Mathlib fallback. No paid compute or storage
upload. Exact packet/source receipts accompany this report.
[pcc:unverified]
