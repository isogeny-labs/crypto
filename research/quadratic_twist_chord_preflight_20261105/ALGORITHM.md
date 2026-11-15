# Target-aware chord: exact proof and implementation obligations

Everything below is a proposed algebraic construction, not a checked identity, coverage theorem or measured advantage. Statement record `PCC-20261105-7179e4` has no signed verification. No Lean elaboration, native polynomial check or IDC1 creation has run. [pcc:unverified]

## Domain and original-source binding

The development fixture is E/F37: y²=x³+7x+19 and its nonsquare d=35 monic twist T: y²=x³+28x+33, with historical P=(1,8), U=(32,29) and order r=19. Use the actual coordinate Phi and inverse I=[10]D on their cyclic subgroups, with current source bindings to `research/quadratic_twist_evidence_20261021/lean-cm8-37/CM8ThirtySeven.lean`. These are historical inputs to recheck, not a new theorem, registry alias or generic rational main-to-twist map. The coefficient field is F19; the curve field is F37. [pcc:unverified]

For the general algebraic lemmas, state a field of characteristic other than 2 or 3, a nonsingular short model, actual target R=(a,b) satisfying b²=a³+Aa+B, and a base of three distinct abscissae with actual nonidentity point representatives. Do not infer those hypotheses from a model label. [pcc:unverified]

## Residual quadratic and cubic membership

For each finite slope l, propose

    L_l(X)=l*(X-a)-b
    s=l²-a
    p=a*l²+2*b*l+a²+A
    q_l(X)=X²-s*X+p.

The first proof target is the following proposed identity. [pcc:unverified]

    X³+A*X+B-L_l(X)²=(X-a)*q_l(X).

Let F_B(X)=X³-c1*X²+c2*X-c3 be the exact expanded product of the supplied base roots. The remainder identity is another proof target. [pcc:unverified]

    F_B(X)=(X+s-c1)*q_l(X)+U*X+V
    U=s²-p-c1*s+c2
    V=(c1-s)*p-c3.

For implementation, the cancellation W=V+a*U is proposed as

    U=l⁴-(3*a+c1)*l²-2*b*l+c1*a+c2-A
    W=-2*b*l³-(3*a²+A)*l²+2*b*c1*l
      +a³+2*c1*a²+c1*A+a*c2-c3.

Prove these expressions before using their degrees as a certificate. W may have lower actual degree on special inputs; record the degree actually obtained, never force a nonzero leading coefficient. [pcc:unverified]

## Three mandatory branches

**Distinct residual roots.** Compute the univariate gcd of U and W using native polynomial arithmetic, retaining division/Bezout records. Extract its F37 roots algebraically, reconstruct q_l, require two distinct actual base roots, and set y_i=L_l(x_i). Check the actual curve equation and P_i+P_j=R. Record both ordered coefficient rows, then deduplicate only at the declared row-space boundary. A separate native reference enumerates point pairs; its answers never feed candidate gcd, stopping or root extraction. [pcc:unverified]

**Diagonal/tangent.** Do not require q_l to divide the square-free F_B. Instead use

    Delta=s²-4*p=0
    F_B(s/2)=0,

with 2 inverted in the stated field. Reconstruct x=s/2 and y=L_l(x), then require the actual curve and doubling equation 2*(x,y)=R. The implementation may clear the fixed powers of 2, but must record the identity and nonzero denominator used. Prove that this covers repeated roots, including coincident target intersections. A branch cannot be omitted because the off-diagonal gcd has no roots. [pcc:unverified]

**Identity query.** R=O has no affine (a,b). Enumerate the supplied base representatives with their negatives as the separately charged identity branch and retain all ordered zero-sum pairs. Prove why nonidentity pair decompositions do not need a vertical-line branch; do not silently apply an affine chart to O. Zero-sum coefficient rows stay in the raw record even when they add no rank. [pcc:unverified]

Each accepted signed lift uses the supplied column order and point orientation. Record i,j, both signs, the actual point equation and integer coefficient row before reduction modulo19. Column permutations/sign transport must be explicit in matched controls. The actual source inverse transports aggregate relations; global Phi injectivity is not assumed. [pcc:unverified]

## Degree and work are separate records

Record actual U/W/Delta/diagonal-membership degrees, polynomial divisions, quotient degrees, Bezout witnesses, root-extraction work, construction, failed branches, sign/point checks, map and rank work. A gcd algorithm has no Macaulay matrix degree: set that field to null with reason `not_applicable_univariate_gcd`, never zero. If later comparing the retained Macaulay solver, run the same original ideal and record its matrix and expanded original-generator witness degrees separately; a quartic eliminant is not automatically a lower solving degree for that algorithm. [pcc:unverified]

No degree or counter comparison is a complete cost claim. Native field add/mul/inverse and group/map work have different costs and may be nested. Retain their phases and units without an equal-weight sum. Root finding and optimized source-side univariate elimination belong in the matched baseline. The general membership degree grows with the base size; this cubic case does not claim constant-degree scalable membership. [pcc:unverified]

## Sources and novelty

[Semaev, 2004/031](https://eprint.iacr.org/2004/031) supplies the summation-polynomial background; [Kosters–Yeo, section 2 and sections 4–5](https://arxiv.org/html/1503.08001v2) distinguish summation equations, first-fall degree and regularity. The current lead atlas lists transported bases, eliminants/subresultants and algebraic membership as existing mechanism families. This experiment tests complete branches and delivery cost on an actual transferable fixture, not invention or an asymptotic novelty claim. [pcc:unverified]

The current atlas match also identifies `LEAD-ICP-split-graph-secant-presentation` and `LEAD-ICP-shared-coefficient-membership`. Read the sibling [SECANT_GRAPH.md](../prime_factor_base_degree_20261031/SECANT_GRAPH.md) for its existing exact secant equations, four signs and tangent/identity treatment, and [its execution handoff](../prime_factor_base_degree_20261031/parallel_20261031/HANDOFF.md) for retained extraction caps and unpriced work. Reuse those antecedents and offer their applicable optimized source-side presentation/extraction as a later cost baseline. Their prior theorems, saved-basis checks and counts are historical, not new or signed results of this experiment. [pcc:unverified]
