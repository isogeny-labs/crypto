# Three proposed twist mechanisms

All constructions and expected benefits below are unverified assertions. Their PCC records are `statement` records with no scientific proofs; they do not certify novelty or execution. [pcc:unverified]

## 1. Complementary degree-limited certificates

Record: `PCC-20261105-1f629a`. Fix an actual source subgroup G=<P> of odd prime order r, an actual target subgroup and a scalar-preserving map phi with an explicit inverse on G. Fix ONE ordered source factor base B_i and transported target points phi(B_i). Inputs are actual Q in G; recovery may not receive its scalar label. Coefficient arithmetic is over F_r, distinct from the curve field. [pcc:unverified]

Let R_E(D,Q) and R_T(D,phi(Q)) be coefficient rows admitted by the respective algebraic solvers under a declared matrix bound AND expanded-original-input witness bound. Map both into the same columns. The proposed criterion at state L is strict additional rank of the combined span over both standalone spans, with setup, query generation, map operations, unsuccessful solves and linear algebra included in a common resource/work contract. [pcc:unverified]

A complete-or-unknown oracle can release identical full rows on both models, or release none. The plan must test that case explicitly: do not present its per-query union as partial-certificate complementarity. Separately inspect row-space growth across frozen query prefixes and degree-limited schedules. Introducing partial solutions would require a new solver contract and individual certificates; enumerating pairs is a separately labelled generic reference. [pcc:unverified]

Try fixed main-only, twist-only, alternating and dual-model schedules. A dual-model call costs both calls. Freeze any routing features before held-out targets; a post-hoc best-chart oracle is diagnostic only. Equalize total work, not the number of high-level calls. If shared setup cannot determine legitimate base logs, every dependent target remains blocked. Do not import synthetic base logs or unlimited solver answers.

Source obligation: accepted target rows must transport back to sum c_i B_i=Q through the actual subgroup inverse. Reuse a prior theorem only within its written hypotheses, with current source/claim bindings. Then derive k=sum c_i*l_i mod r from legitimately obtained PUBLIC base logs. No full-group injectivity assumption. [pcc:unverified]

## 2. Full-target chord chart and membership

Record: `PCC-20261105-585d85`. On an elliptic short model y^2=x^3+A*x+B in characteristic other than 2 or 3, take a nonidentity actual target R=(a,b). Candidate lines through -R are y=lambda*(x-a)-b. Proposed residual quadratic:

    q_lambda(X)=X^2-s*X+p
    s=lambda^2-a
    p=a*lambda^2+2*b*lambda+a^2+A

The first Lean obligation is the exact polynomial factorization of the curve minus line-square by (X-a)*q_lambda, using b^2=a^3+A*a+B. No statement is claimed proved here. [pcc:unverified]

For a three-abscissa factor-base polynomial F_B(X)=X^3-c1*X^2+c2*X-c3, the candidate remainder is U*X+V, with U=s^2-p-c1*s+c2 and V=(c1-s)*p-c3. Distinct-root completeness requires both remainder coefficients to vanish, actual splitting over the chosen field, and valid point/sign lifts. A proposed cancellation V+a*U gives a quartic/cubic gcd starting chart. This is an unverified formulation, not a certified degree bound or a scalability claim for growing factor bases. [pcc:unverified]

Keep the diagonal/tangent branch separately; a square-free F_B need not be divisible by a repeated-root quadratic. Include origin, vertical lines, target intersections, characteristic/denominator exceptions and all ordered sign lifts. Prove coverage before using any reduced system to admit rows. Record matrix degree, expanded original-input witness degree, weighted/pulled-back degree, field work, root extraction and group work separately. [pcc:unverified]

Give the same chord encoding to the source, twist and transported-control arms. Compare against the strongest applicable source solver and an univariate elimination reference, not just an intentionally expensive old implementation. No pair table may silently supply stopping or answers. The chord law and summation-polynomial ingredients are established literature; the proposed combination and engineering benefit require testing.

## 3. Twist variable whose trace vanishes

Record: `PCC-20261105-cfd582`. Let K=F_(q^2), sigma be q-Frobenius and tau(X)=X+sigma(X). Use an actual additive twist embedding psi whose image is the full Frobenius-minus subgroup. Let source B_i,Q lie in an explicit cyclic subgroup G of odd order r. Choose public h with 2*h=1 modulo r, take C_i=psi(T_i), and form A_i=[h]B_i+C_i and A_Q=[h]Q. [pcc:unverified]

Target proof obligation:

    (exists T, sum c_i*A_i = A_Q + psi(T))
        iff (sum c_i*B_i = Q).

The forward proof plan applies tau; the reverse constructs the residual twist point using the supplied T_i and additivity. This is a proposed faithful theorem on actual groups, not an already checked abstract replacement. Q remains a point input, not a logarithm label. Rational source/twist groups need not be assumed globally isomorphic. [pcc:unverified]

Retain T as an auxiliary variable during solving and test whether the extension formulation has cheaper intermediate constraints. Charge all added variables and field/group/map work, keep expanded source-witness degree, compare with a source-side auxiliary-variable encoding, and collapse all witnesses projecting to the same coefficient row before rank. The target is cheaper admitted useful relations; extra witnesses alone are not extra useful rows. [pcc:unverified]

## Sources and novelty review

- Semaev, [Summation polynomials and the discrete logarithm problem on elliptic curves](https://eprint.iacr.org/2004/031). Primary metadata read; PDF retrieval was inaccessible in the prior review. Use [Kosters--Yeo](https://arxiv.org/pdf/1503.08001), section 2, for the accessible general summation-polynomial definitions and exceptional-case context.
- Kosters--Yeo, [Notes on summation polynomials](https://arxiv.org/pdf/1503.08001), sections 4--5: first-fall degree and solving/regularity behavior require separate evidence; lower input degree is not a solver-cost certificate.
- Gorla--Massierer, [Index calculus in the trace zero variety](https://arxiv.org/pdf/1405.1059), definitions and relation-generation sections: standard trace/Frobenius machinery and actual lift checks. The paper does not establish the proposed source-trace auxiliary solver.
- Galbraith--Granger--Merz--Petit, [On Index Calculus Algorithms for Subfield Curves](https://sacworkshop.org/SAC20/files/preproceedings/18-IndexCalculus.pdf): Frobenius-compatible bases and their field/instance assumptions. Do not treat conjugate targets as independent rank.
- Lead atlas reviewed at `774611edb0a58861c2991372b81e49be0a0644b4`: LEAD-ICE-prime-field-semaev-solving-degree, LEAD-ICP-isogeny-transported-relation-yield, LEAD-ICP-algebraic-membership-factor-base and LEAD-ICB-frobenius-search-symmetry. Plain transported-base yield and subresultant searches are existing mechanisms; no new disposition or novelty claim is made.

Prior program context: [F37 upper-cap study](../quadratic_twist_evidence_20261021/reports/cm8-37-b3-high-20261030/README.md), [actual F37 source-map proof map](../quadratic_twist_evidence_20261021/lean-cm8-37/README.md), [matched-row bridge](../quadratic_twist_evidence_20261021/lean-row-yield/README.md). These are historical records, not new signed PCC verdicts. Rebind assumptions and inputs before any new scientific use. [pcc:unverified]
