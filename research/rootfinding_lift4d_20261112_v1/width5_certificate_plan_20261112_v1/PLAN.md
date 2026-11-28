# Five-dimensional discovery for the width-relevant coefficients

UNEXECUTED changed hypothesis. This plan does not modify any frozen4D/7D source, input, manifest or raw evidence, and adds no experiment to the current three-experiment session. There is no novelty, original-global-root, measured speed or binary/extension chart claim.

## Why four dimensions can lose pruning

At a fixed pivot r the4D seed controls the coefficient B(r) of S in the exact conditional quadratic J. In the full u chart,

    B(u) = -2 lambda0 u² - lambda1 u + lambda3.

Small B(r) alone says nothing about lambda1, hence nothing about B(u) away from r: B(u)-B(r)=-2lambda0(u²-r²)-lambda1(u-r). A coefficient of order p times S of order M makes an interval much wider than p, defeating interval exclusion. Independently centering the seven lifted coefficients also need not preserve the seed's small integer B(r). These are algebraic failure mechanisms to check against actual recorded masks, not a claim that the current4D experiment has failed before it runs. A longer identical LLL budget does not target either missing constraint.

## Keep every coefficient that controls per-u width

The two field charts have coefficient lists a0..a5,b0..b5 for
F=((u²-S)²,(u²-S)u,u²,S,u,1). Consider integer vectors
w=(lambda0,lambda1,lambda3,gamma,delta) satisfying THREE congruences

    lambda0 = gamma*a0 + delta*b0,
    lambda1 = gamma*a1 + delta*b1,
    lambda3 = gamma*a3 + delta*b3                 modulo p.

The congruence map to Fp³ is onto via the three lambda coordinates. Thus its kernel lattice has index p³. An explicit five-row basis is p times the first three coordinate unit vectors, then (a0,a1,a3,1,0) and (b0,b1,b3,0,1), with coefficient representatives fixed before reduction. Its determinant is p³. Membership plus absolute determinant p³ independently proves full-basis equality. Exact unimodular row operations with exact unscale preserve the lattice.

Use five exact LLL basis rows plus all twenty signed pair sums, a FIXED25-vector pool. For A=3M choose positive diagonal discovery weights

    max(1, A²+2AH²), max(1, AH), max(1,A),
    max(1,3M²), max(1,M³).

These reflect a safe per-u variation bound. Weight quality is a hypothesis, not a correctness requirement. Gamma/delta may be centered after seed verification, leaving lambda0/lambda1/lambda3 as their short seed representatives; modular membership is preserved, but a centered set must not be mistaken for an unimodular basis change.

For each candidate derive the omitted coefficients lambda2=center(gamma*a2+delta*b2), lambda4=center(gamma*a4+delta*b4), and c=center(gamma*a5+delta*b5). These coefficients have NOT disappeared from the equations. They influence the constant phase at fixed u, but none multiplies S, T or U in that fixed-u range. The exact signed expression is

    J_u(S,T,U) = lambda0 S²
                  + (-2lambda0u²-lambda1u+lambda3) S
                  + C(u) - gamma T - delta U,
    C(u) = lambda0u⁴+lambda1u³+lambda2u²+lambda4u+c.

At every genuine tuple it is divisible by p. For each u, replace C(u) by its centered residue if desired; this subtracts an integer multiple of p and cannot change whether the exact interval contains a multiple of p. Constant recentering can reduce arithmetic size but is NOT an additional pruning gain. Equivalently obtain the phase directly as center(gamma*T_constant(u)+delta*U_constant(u)), with the public quadratics' constant terms independently evaluated modulo p. Never set omitted coefficients or their phases to zero without this identity.

The range width over S in[0,A],T in[0,3M²],U in[0,M³] is determined entirely by lambda0, B(u), gamma and delta. Coefficients lambda2/lambda4 affect the interval's location relative to pZ, not its width. Therefore the5D discovery targets all width-relevant large coefficients while the old7D discovery also optimizes two phase-only coefficients. This is the falsifiable reason5D might retain useful7D pruning at lower discovery cost. It does not prove pruning quality or a speedup.

Compute exact quadratic extrema over every declared u and the full integer S interval, then add signed T/U extrema. A range missing pZ certifies that complete u fiber empty. Every other fiber remains UNKNOWN until the complete primal recovery finishes. Independently replay all modular identities and ranges. The union of all certificates and every completed surviving fiber is exactly the same conditional ordered root union as the all-h baseline. Multiplicities, permutations and literal norm k reconstruction remain mandatory. Caps and deadlines cannot become empty roots.

## Next bounded experiment

Implement a NEW versioned5D variant and an independent25-pool/membership/determinant/per-u phase audit. Before any timing freeze all dependencies, midpoint/weight policy, coefficients, fields, complete source/input hashes and budgets. Do not add5D as a post-result faster arm to a completed4D comparison.

Controls: p5/7/11/13/17,M2,H2, all405original ordered triples/u and all three complete conditional unions; exact corrupted-index and wrong-lambda/phase rejection; negative and repeated-source cases; signed phase shifts by several multiples ofp must preserve masks. Include the nonvacuous simple certificate p19,M1 with T=9 and U=0, which has J=9-T in[6,9], excluding every u. Include a cap/deadline branch returning unresolved. At most200000 declared independent checks,90seconds,4GiB per-process address space and one core.

For the first cost diagnostic reuse the explicitly declared shifted five-prime TRAINING placements [0,1,3,5,7] relative to public planted h, one per NIST field, rather than all-five planted pivots. Compare NEW5D, prior7D and all-h, using three fixed alternating-order repetitions and the same complete output. The targets remain exposed and source-dependent; do not call this holdout/hardness evidence. Freeze any broader unopened panel under a separate selection rule before timing it.

Charge public construction, discovery, native lifetime, seed/per-u certificate replay, all survivor solves, independent Fraction/Sturm/norm checking and complete serialization. Record whole-worker time, per-process RSS, common compilation and wrapper exclusions, and all capped/unresolved arms. Never choose only the fields on which masks succeed. Promote only if identical complete unions and lower paid cost than7D; a5D slowdown or insufficient pruning refutes this specific cost hypothesis.

If5D fails, inspect coefficient-width and phase feasibility before choosing the next mechanism. Candidates include fixed correlated two-pivot constraints or compact simultaneous phase optimization; state an actual changed assumption. The user8x/genuine-matched-F4/F5/F6 gate remains entirely separate, including20hard completed inputs, original root sets, cold repetitions/isolation/A-A and requestedGF(2^83)/binary/Koblitz/odd-extension coverage.

## Geometry diagnostic, not a performance or counting bound

On the current small-width training blocks only, A is much larger than H², so the5D diagonal-weight product scales as M⁹H up to fixed constants. With determinant p³, a volume-based discovery scale is roughly (p³M⁹H)^(1/5); the7D counterpart is roughly (p⁵M⁹H⁴)^(1/7). These are geometry heuristics, not guaranteed useful certificate widths, rejection probabilities, bit gains or running times. LLL approximation and norm conversion constants, phases and surviving roots still decide measured usefulness. Once H² is comparable to or exceeds A, the stated small-width scaling no longer applies; use the actual A²+2AH² weights. The global h domain still has p elements (about ceil(p/8) width8 blocks). This component does not remove that global enumeration bottleneck or predict a tractable full all-roots search.
