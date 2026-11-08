# Closed oriented relations and native point actions

2026-11-08. P-192 is the primary target. Native construction now supplies P-192/P-224/P-256, alongside the independently replayed historical P-384/P-521 certificates. Source: construct.cpp, relation_verify.hpp, orbit_verify.hpp. The combined native-v17 replay passes 150 point/window cases and three signed-orbit fixtures per curve; native-v18 rejects six certificate mutations at the intended check, including a sign error that preserves the kernel x action but reverses its y action.

| Curve | ell | selected rho | marked order s | nonzero signed orbit | norm bits | residual degree bits |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| P-192 | 13 | 2 | 5 | 10 | 382 | 378 |
| P-224 | 11 | 10 | 17 | 34 | 451 | 447 |
| P-256 | 11 | 8 | 3 | 6 | 516 | 513 |
| P-384 | 19 | 9 | 7 | 14 | 769 | 765 |
| P-521 | 7 | 6 | 7 | 14 | 1044 | 1041 |

For each distinct-split edge, the constructor computes the full ell-division polynomial and extracts the selected kernel as the gcd of its x and signed-y Frobenius constraints. This identifies the complete eigenline, not just an abscissa. The native verifier checks that its monic kernel polynomial squares to the map denominator, is squarefree, divides the division polynomial and has the specified signed Frobenius action.

The normalized Velu maps phi:E->E1 and eta:E1->E2 have endpoint E2 with coefficients (ell^4*a,ell^6*b). The explicit closing isomorphism is sigma(x,y)=(x/ell^2,y/ell^3). Full rational-polynomial composition against the independently formed division-polynomial scalar map proves sigma eta phi=[ell]; the positive differential ell fixes the sign. Both complete curve-map identities are checked. Standard kernel and Velu formulas are given in [Sutherland's MIT lecture 5](https://math.mit.edu/classes/18.783/2023/LectureSlides5.pdf), slides 11-16; the construction is an implementation of those tools, not a novelty claim.

Choose the preregistered nonidentity prime-order projection mu and a centered CRT lift c satisfying c=rho modulo ell and c=1-mu modulo n. Then alpha=pi-[c]=a+b*pi with a=-c,b=1, norm c^2-c*t+p, and subgroup label a+b=mu modulo n. Since alpha kills the signed kernel, it factors uniquely as psi phi with endpoint exactly E. The residual degree is norm/ell. Its global polynomials are not expanded. On E(Fp), an explicit evaluator of the closed action is [k] sigma eta phi, with k=mu/ell modulo n. This has the stated subgroup action; its global degree differs from the noninteger quotient. The distinction is retained in the certificates and costs.

The canonicalizer enumerates +/-mu^j P and returns the least affine (x,y) and its scalar transporter. Exact prime order s, odd prime n and mu!=1 give 2s distinct points for nonzero P; O is handled separately. The native relation-driven canonicalizer matches a separately generated OpenSSL orbit, including its saved representative and transporter. Multiplying the original input by the transporter checks every generated action. This is the object the orbit method can now consume.

# Exact costs of the current native evaluator

These are deterministic source-specific group counts, not timings. The fixed six-width scan selected the minimum unweighted D+A separately for display. The maps arm additionally has the explicit field-formula costs in the last column; backend point conversion and setup remain separate.

| Curve | direct D,A,N (width) | fused D,A,N (width) | explicit-map correction D,A,N (width) | two maps + closure explicit M,A,I |
| --- | --- | --- | --- | --- |
| P-192 | 190,39,19 (4) | 195,43,23 (4) | 191,41,22 (4) | 114,104,2 |
| P-224 | 218,42,14 (5) | 227,49,24 (4) | 223,47,22 (4) | 98,88,2 |
| P-256 | 254,46,23 (5) | 259,52,27 (5) | 255,50,25 (5) | 98,88,2 |
| P-384 | 380,70,34 (5) | 387,72,25 (6) | 383,70,24 (6) | 162,152,2 |
| P-521 | 518,88,31 (6) | 524,89,40 (6) | 521,88,39 (6) | 66,56,2 |

The fused route computes [ell] followed by [mu/ell], justified by the checked full closure. It includes the small-scalar point operations. In these schedules the direct route uses fewer doubles and additions; negations are reported independently. The explicit-map formula cost follows from actual coefficient-array lengths in Map.apply: each map evaluates N, D and H using full Horner from zero, then performs four further multiplications and one inverse. The two closing scales add two multiplications. This yields (8ell+10)M +8ell A +2I. It is a source-level formula-call count; it does not include backend field operations within group arithmetic or coordinate conversions.

Raw evidence: qualification/structural-v2/rows.jsonl, with 90 verified rows and clocks disabled. The current correction still requires a full-size public scalar. The next algebraic improvement must reduce that correcting-scalar work or provide a different directly evaluable closed relation; changing the class-number interpretation alone does not change these recorded costs.

# Timing, scope and preserved failures

Native comparison, isolation, analysis and independent output-replay programs exist. The initial P-384/P-521 timing draft passes schema validation but is unfrozen; extending mathematical fixture coverage did not silently extend the timing panel. The full cold accounting boundary includes runtime/table/map setup, closure, correction, verification and destruction. No cryptographic timings were admitted: local preflight twice exceeded its quietness limit, and the direct EKS live read failed role assumption. The exact blocker and exit conditions are in [the problem record](../../problems/cm-marked-scalar-measurement-admission.md). No job/controller was restarted or submitted; the native Diem receipt remains deadline-expired.

Preserved development failures include the type-macro compile error, unavailable P-192 named group, unsuccessful initial scalar projections, the explicit correction of V2's overstatement of the observed P-192 failure, and earlier native verifier/validator build attempts. Scope amendments preceded their affected constructions. Native-v17 is producer-side exact software evidence, not an independent signed scientific review or a Lean formalization. The requested end-to-end runtime improvement remains open.
