# Fixed-offset genus-two Kummer quotient and oriented-coupling obstruction

2026-11-24 UTC (November 23 Pacific). Draft PR #5064. [pcc:unverified] Written algebra, frozen input 9259dc59d472f5a84fe9d73152050a7bd86c4596; parent genus-five note blob 2af393d391b192265a0e9cf294e18663b0f8d616 and geometry receipt 0d0924c1c1abc0b7ea5238b3b6afbadeab2778e0. Same hypotheses: p>3, nonsingular E:y²=f(x)=x³+ax+b, odd full E(Fp), nonendpoint trace delta of a 2-power torus element. No extension to even-order source curves. No production changes.

## Explicit genus-two quotient

On the signed unordered pair cover Y_delta of genus 5, simultaneous elliptic negation j sends {P,Q} to {-P,-Q}. Write s=x1+x2, u=x1*x2, v=y1*y2, and

    u=(s²+delta²-4)/(delta+2).
    L=s²-4u=(delta-2)(s²-4(delta+2))/(delta+2).
    A=s(u+a)+2b.
    W=u³+a*u*(s²-2u)+b*(s³-3us)+a²*u+ab*s+b².

Then H_delta: v²=W(s,u(s)) is the smooth projective model of the quotient Y_delta/<j>. Indeed s determines the unordered abscissae via Z²-sZ+u. Generically, v selects the relative y sign, leaving precisely the simultaneous-negation orbit. Equivalently the indicated field is the invariant field in the ordered cover under the commuting swap and simultaneous negation, a group of order 4. No claim that all rational points of the quotient lift rationally.

For each root alpha of the cubic f over the algebraic closure,

    W = product_(alpha) (u-alpha*s+alpha²).

The numerator quadratic after multiplying each factor by delta+2 has discriminant (delta²-4)(alpha²-4), nonzero by irreducibility of f and the offset hypotheses. No two factors share a root: a common root would produce a conic point whose two abscissae are distinct roots of f, excluded by the parent private-zero lemma (it would force delta=-1). Thus W is a squarefree sextic of leading coefficient (delta+2)^(-3). The double cover of P1 branched at exactly six finite roots has genus 2 by tame Riemann-Hurwitz. This proof is uniform in all 31 nonendpoint offsets at N=64.

## Exact projected sum and its exceptional points

On L!=0 the sum abscissa is

    r=(A-2v)/L.

The norm identity is

    (A-rL)²-4W = L*[ L*r²-2A*r+(u-a)²-4bs ].

The bracket is precisely the symmetric S3 in the pair variables. Substitution u(s) makes its leading coefficient in s equal to (delta+2)^(-2), independent of r. Hence a monic quartic controls generic projected recovery with no extraneous norm sign after setting v=(A-rL)/2. The map H_delta -> P1_r has degree four and is separable. One direct check: the two zeros of L correspond to x1=x2=x with x²=delta+2 and f(x)!=0. At each there are two points v=±f(x); the opposite-sign branch gives a simple pole, while the equal-sign branch has removable singularity and yields the tangent doubling abscissa. At each of the two geometric points at infinity, v has pole order 3, s has pole order 1, and A-2v has nonzero leading coefficient (the square of A's leading coefficient differs from that of 2v since delta!=2). L has pole order 2, so each contributes one more simple pole. Four simple poles establish the claimed degree and separability. These are geometric counts, not rational output counts.

For actual leaf membership the split quadratic in Z and the eligible base are mandatory; check square roots for y1 and enforce y2=v/y1. Odd full group means f has no rational roots, so rational nonidentity lifts have y1!=0. Simultaneous negation supplies the other subtotal sign; use the actual target orientation for replay. L=0, endpoint offsets delta=±2, and O retain separate tangent/cancellation guards. A quotient point with nonsplit quadratic or nonsquare f(x1) is not a valid relation. The preceding midpoint parity selector can be written D_(N/2)(s²/(delta+2)-2)=e*c, degree N in s, e=(-1)^d, and must still be charged or replaced by exact actual-M filtering. This is a representation, not a faster root solver.

## Why a second quotient does not halve the coupled geometric degree

For R!=O with 2R!=O, consider the correspondence C_R with coordinates (r1,r2)=(x(S),x(R-S)). Its normalization is E: generically x(S) and x(R-S) determine S uniquely, since x(R-S)=x(R+S) would imply 2S=O or 2R=O. This also follows from the standard S3 quadratic criterion; exceptional infinity/2-torsion fibres are separate. Over a generic point of C_R, each of H_delta and H_epsilon supplies four geometric preimages. The coupled product therefore has degree 16 over C_R, exactly as the earlier genus-five oriented product over E, not degree 8 or 4. Over the r1 line its degree is 32, since C_R -> P1_r1 has degree 2. Generically the orientations of the two unordered pairs are uniquely restored by the fixed intermediate S and R-S; no independent sign factor can be discarded for the coupled target. The one-dimensional pre-selector obstruction remains.

## Natural Jacobian transfer vanishes

Let q:Y_delta -> H_delta be the negation quotient and t:Y_delta -> E the oriented sum. On Jacobians, t_* q^*:J(H_delta)->E is zero. Since q j=q and t j=-t, the image of q^* is j-invariant whereas t_* is j-anti-invariant; hence 2(t_*q^*)=0. An abelian-variety homomorphism from a connected variety with image in finite E[2] is zero. This only rejects the natural pullback/sum correspondence; it does not rule out unrelated homomorphisms or Prym constructions. In particular genus two here is not an established DLP-preserving transfer.

## Costs, validation and handoff

Constant-degree geometric models use O(1) field coefficients per offset; O(N) across offsets. The quartic is solvable per fixed subtotal and offset, but costs root isolation, actual-M lookup, rationality, sign replay and output. Enumerating both independent offset sets costs Theta(N²) before coupled midpoint solving; using all actual signed pairs retains the prior O(B²) setup and O(D) expected lookup per miss. Neither the degree-four quotient nor its genus reduces this miss exponent. The growing-family and useful-rank assumptions stay exactly as previously scoped. No claim of novelty for symmetric S3, hyperelliptic quotients or Jacobian functoriality; see Semaev ePrint 2015/310 and Stacks 0C1B for standard background. Repository problem and prior selector/swap notes were checked; shared KB and native worker slots unavailable.

Coordinator symbolic checks: expand W(x+y,xy)-f(x)f(y); expand (A-rL)^2-4W-L*F; factor L after substitution; confirm sextic and quartic degrees/leading coefficients. All checks matched (see receipt). These are formal symbolic identities, not original-Rust cryptographic executions or independent signatures. No benchmark or source repair launched. Rust worker unavailable and no eligible unclaimed implementation selected; validation preparation consists of comparing exact rational lifts and tangent/O exceptions with the frozen signed pair control, and the generic degree/transfer proofs above. Next gate: a charged implicit index or simultaneous midpoint filter that actually avoids the offset Cartesian product, not another genus-only contraction.
