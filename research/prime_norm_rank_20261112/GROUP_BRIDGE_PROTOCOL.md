# Actual norm-to-elliptic-group bridge: prospective contract

2026-11-13. The broader objective remains a general-prime-field, non-generic,
exp(o(log r)) complete bit-cost algorithm. This component connects an actual
norm certificate to the actual elliptic point group; it does not find branches.

Question: for a Weierstrass coordinate ring R over a field K, distinct x_i,
nonsingular actual points (x_i,y_i), and a nonzero h in R that belongs to all
their point ideals, does degree(norm h)=number of points force the product of
their point ideals to equal (h), and hence their actual elliptic sum to be zero?
Specialize to short Weierstrass h=A-BY and the existing full norm c*G.
Do not assume an ideal factorization, a divisor theorem, a group relation or
an oracle supplying a scalar. Expose all nonsingularity/degree/nonzero guards.

Approach selected from pinned Mathlib and Angdinata-Xu ITP2023 sections3.1–3.3:
the actual coordinate ring is free of rank2 over K[X]. Its principal quotient
has dimension equal to the degree of the algebra norm. Distinct point ideals
are coprime, their quotients are K, and CRT gives the product quotient's
dimension. The surjective quotient map between equal finite dimensions is
injective, identifying the actual ideals. Apply the existing additive,
injective Point.toClass map. No projective divisor library is assumed.
This is established mathematics used as a formal foundation, with no novelty
claim. Import the explicit public signed-Hankel certificate only after the
norm-to-group specialization is complete; never assume the original W-jet
family equals this public branch family.

Lean acceptance: faithful statements on actual CoordinateRing and W.Point,
no sorry/admit/custom axioms, explicit theorem bindings and standard-axiom
audit, current whole-study gate and import-inclusive fresh replay. Preserve
all failed builds including the external initial helper prototype001–003.
Conductor remains unavailable in this runtime; only the named study, journal,
new claims and append-only ledger are reserved. No unrelated edits.

Native correctness contract, no timing/yield/sign-search campaign: reuse the
frozen F17 curve y^2=x^3+2x+2 and the same six actual public point branches.
Using native Rust modular affine group arithmetic, check each input point,
sum the norm-positive branches, flip only the existing x=13 branch and sum
again. Predict the positive sum is infinity, while the flipped sum is nonzero
and differs by minus twice the original point at13. Independently check the
group relation against the existing norm decisions. A negative control that
adds an off-curve continuous repair must be rejected before group arithmetic.
If a scalar-label example is feasible, construct its labels from public base
point addition and verify the recovered label by multiplication; it is a
known-answer certificate, not a target solver or order/scaling result.

Freeze native source before one write-once control artifact invocation. Retain
versions, source hashes, commands, raw logs/exits and rejected inputs. Fixed
development correctness only; no paid/EKS launch, random sampling, scaling,
new attack solver or performance claim. A future algorithm campaign needs
its own actual generator, frozen protocol and live finite resource admission.

Sources checked (KB): MCP absent and wrapper previously exits127 (no catalog
executable); bounded fallback is the pinned Mathlib Point.lean, FreeModule/
Norm.lean, FreeModule/IdealQuotient.lean, Ideal/Operations.lean and quotient
operations, plus Angdinata-Xu DOI10.4230/LIPIcs.ITP.2023.6. No freshness,
exhaustiveness or source-only-as-proof inference. Results need source-bound
unsigned PCC claims, readable proof map and linked diagram/SVG/PDF.
