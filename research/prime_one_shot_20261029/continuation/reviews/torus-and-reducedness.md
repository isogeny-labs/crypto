# Independent review: norm-one torus extension and reduced norm scheme

Reviewer: `/root/theory_review`, 2026-10-30 UTC. This is an independent on-paper review of the proposed p+1/norm-one extension and the additional reducedness claim sent by `/root`. No solver run, numerical experiment, runtime measurement, formal proof certificate, or novelty claim is supplied here.

## Verdict

The proposed norm-one torus extension is supported under the stated odd-order prime-field curve and 2-power subgroup assumptions. The six branch points have the required disjointness, the projection and collision degrees double relative to the scalar Kummer case, and the off-diagonal character cover has genus five. Trace collisions need explicit exclusion, but that exclusion costs only a small probability and an efficient test in the proposed sparse regime.

The full selected norm/remainder scheme is also finite and geometrically reduced at fixed input parameters, under the stated separability and chart conditions. This is stronger encoding correctness than reduced membership alone. It does not imply a small solving degree or an efficient solver.

## 1. Setup and six branch roots

Let p>3, E/Fp: y^2=f(x)=x^3+ax+b be nonsingular, and #E(Fp) odd. Let T(Fp) be the norm-one subgroup of Fp2*, of order p+1. Let H=mu_d be its subgroup of order d=2^t, with d|p+1. For g in T(Fp), define

    x_w(g)=g*w+(g*w)^(-1),  w in H.

These abscissas lie in Fp, because Frobenius sends every norm-one element to its inverse. Using an auxiliary Fp2 torus does not change the source ECDLP into an extension-field curve problem: E and the recovered points remain over Fp.

Odd curve order makes f irreducible cubic, with three roots alpha_i in Fp3. None equals +2 or -2, since those are rational. The six roots of

    h(Z)=Z^3 f(Z+Z^(-1))

are the two distinct roots of Z^2-alpha_i Z+1 for each alpha_i. They are all distinct and nonzero. Each lies in Fp6, because it is at most quadratic over Fp3. In particular, for sigma=Frobenius^2,

    sigma^3(z)=z.

This removes the need for a possible inversion alternative after sigma^3 in the informal root argument. Over K=Fp2, each alpha_i still has degree three and sigma cycles them in a three-cycle.

## 2. No ratio of distinct branch roots lies in H

Suppose z_j/z_i=zeta in H. Since H is in Fp2, sigma fixes zeta.

If z_i,z_j have the same trace alpha_i, distinctness makes z_j=z_i^(-1). Thus z_i^2=zeta^(-1) lies in Fp2. This would put z_i in an extension of degree at most two over Fp2, while its trace alpha_i has degree three over Fp2. Contradiction. Equivalently one can use the 2-power torsion degree argument, but the degree-three argument is shorter.

If the traces differ, interchange the pair and invert zeta if necessary so alpha_j=sigma(alpha_i). Then exactly one of the following holds:

- z_j=sigma(z_i). Combining sigma(z_i)=zeta z_i with sigma^3(z_i)=z_i gives zeta^3=1. Its order also divides d, so zeta=1, contradicting different traces.
- z_j=sigma(z_i)^(-1). Then z_i*sigma(z_i)=zeta^(-1). Applying sigma gives sigma(z_i)*sigma^2(z_i)=zeta^(-1), hence sigma^2(z_i)=z_i. Its trace would be fixed by sigma^2, contradicting a three-cycle.

Therefore no ratio of two distinct roots belongs to H. The six branch-root sets for f(wG+(wG)^(-1)), as w varies through H, are disjoint: a shared zero would give precisely such a ratio. Their private simple finite zero valuations make the corresponding rational-function squareclasses independent over Fpbar(G).

The common poles at G=0 and infinity do not invalidate independence; the private finite zeros already detect every nonempty product. The proof uses that H has no order-three element and lies in Fp2. It must not be extended automatically to a subgroup containing three-torsion, even whole-curve order, or unrelated auxiliary isogeny towers.

## 3. Trace collisions and the correct sample space

For w!=v,

    x_w(g)-x_v(g)=(w-v)*(g-g^(-1)/(wv)).

Thus trace equality is equivalent to g^2=(wv)^(-1), which lies in H. Excluding every g with g^2 in H is a sufficient global way to make all d abscissas distinct. There are at most 2d such norm-one elements, so their unconditional probability is at most 2d/(p+1). The predicate is efficiently testable as g^(2d)=1.

If g^2 is not in H, the cosets gH and g^(-1)H are distinct but produce the same abscissa set under inversion. This is only a representation multiplicity in sampling bases, not a free relation-yield factor. Uniform g still defines a valid distribution, and all probability bounds can be stated in that distribution.

For clarity, a second-moment proof may either:

- Set the tuple count to zero on the trace-bad g and bound only trace-good fibers; or
- Count index-labelled w-tuples on all g, then identify those with actual distinct-abscissa supports only on trace-good g.

One must not use B(g)=d+sum_w chi(f(x_w(g))) as the actual distinct-point base size on trace-bad g, where traces repeat. That expression is a multiplicity-weighted count there. On trace-good g it is exactly the signed point-base size.

If one samples uniformly from the already filtered trace-good set, unconditional failure bounds must be divided by its probability. It is simpler to sample uniform g from T(Fp), include the <=2d/(p+1) event in the union bound, and treat a failed cheap test as a fully charged resample.

## 4. Projection degrees and collision constants

Let C_K be the smooth projective model of the independent k-square-root cover

    y_i^2=f(w_i G+(w_i G)^(-1)).

Its degree over the G-line is 2^k. The map x_i(G) to the x-line has degree two. The elliptic x map has degree two as well. Therefore, for the projection P_i:C_K->E,

    deg(x_E o P_i)=2^(k+1)=2*deg(P_i),
    deg(P_i)=2^k.

This is twice the scalar-Kummer projection degree. The same deck-involution/parallelogram proof then gives

    deg(sum_i c_i P_i)=2^k sum_i c_i^2.

All nonzero coefficient maps are nonconstant. Total degree bounds fiber point counts regardless of separability, so inseparable sum maps are not an exception to the upper bound.

For two m-subsets I,J with overlap h and union size k=2m-h, the same relative-sign classification applies. If j shared signs disagree, the coefficient-square sum is 2(m-h)+4j. Summing the degree over all relative signs gives

    2m*4^m.

Only the identical-support, identical-sign diagonal has zero map; it is counted separately. Thus the scalar-Kummer second-moment off-diagonal constant doubles. The weighted chart-loss degrees also double. These are constant changes in the exp(O(m)) coverage argument, not a change in exponent class.

## 5. Density: why the exact genus-five bound applies

Compactify the norm-one torus to a projective line over Fp. Its two missing geometric points correspond over Fpbar to G=0 and infinity; they are conjugate and are **not Fp-rational**, because this is the nonsplit norm-one torus.

For distinct w,v, the product

    F(G)=f(x_w(G))*f(x_v(G))

has twelve distinct simple finite zeros by branch disjointness. It has poles of order six at each of the two missing geometric torus points. Those pole orders are even, so the associated quadratic cover is branched at exactly twelve points and has genus five.

The product function is Fp-rational on the torus, because translation by w,v and the trace maps are Fp-rational. At every g in T(Fp), it is finite and nonzero: the poles are nonrational and f has no rational root. Thus each rational fiber of the quadratic cover has 1+chi(F(g)) rational points. Every rational point of the cover maps to a rational point on the compactified torus, and all such base points belong to T(Fp). Hasse-Weil therefore gives the exact convenient bound

    |sum_(g in T(Fp)) chi(F(g))| <=10 sqrt(p).

There is no extra missing-affine-point term here, because the sample space has p+1 projective rational points and neither common pole is rational. A rational-parameter derivation that uses only p affine parameters must account for its remaining projective point instead of silently dropping it.

Let B_tilde(g)=d+S(g), S=sum_w chi(f(x_w(g))). The second moment satisfies

    E S^2 <= d+d(d-1)*10 sqrt(p)/(p+1).

Consequently

    Pr[B_tilde<d/2] <=4/d+40 sqrt(p)/(p+1).

Adding the trace-collision event gives a valid bound on actual insufficient base density outside the excluded sample points:

    Pr[trace-bad or actual B<d/2]
      <=2d/(p+1)+4/d+40 sqrt(p)/(p+1).

This has the same qualitative decay in the sparse admissible regime as the scalar-Kummer density bound.

## 6. Rationality and short-sum degree bounds

The independent squareclasses imply S_s(x_(w1)(G),...,x_(ws)(G)) is a nonzero rational function, by the same one-square-root-flip contradiction. Its arguments are Laurent polynomials with exponents +/-1. A conservative numerator degree after clearing denominators is at most

    2s*2^(s-2).

Indeed the per-variable Semaev bound gives total degree at most D=s*2^(s-2), and multiplying by G^D moves its Laurent exponent range [-D,D] into [0,2D]. Counting its roots among the p+1 norm-one elements is then bounded by that polynomial degree. Coefficients being in Fp2 rather than Fp does not affect a root count over the algebraic closure.

Therefore a valid conservative torus short-relation failure bound is

    Delta_T <= [sum_(s=3..m-2) binomial(d,s)*2s*2^(s-2)]/(p+1),

capped at one, with the trace-bad event handled separately. Signs are already included by Semaev and must not be multiplied again.

Once actual residual abscissas are distinct and rational, the earlier mixed-twist rationality proof is unchanged: residuals lie in E(Fp2), Frobenius negates the nonrational ones, odd #E(Fp2) forces their sum to O, and cardinalities 1,2,m-1,m are excluded by the same chart and nonzero-target conditions. Absence of short zero sums therefore makes all geometric core coefficients rational on the good event.

The doubled numerator degree and doubled collision/chart constants do not change the earlier leading conservative asymptotic inequality 3m-2n/m+O(log m); they add only constant terms. The sparse admissible choice with m=c sqrt(n), c<sqrt(2/3), still gives high-probability rationality and exp(-O(m)) selected-chart coverage after the torus constants and the <=2d/(p+1) trace event are included.

## 7. Actual Dickson membership and field-family limits

On a trace-good coset, the d distinct abscissas are exactly the roots of

    D_d(X)-[g^d+g^(-d)],

where D_2(X)=X^2-2 and D_d is its t-fold composition. This follows from D_d(z+z^(-1))=z^d+z^(-d). The terminal scalar is in Fp and is not +/-2 on a trace-good coset. It may be zero; the scalar-Kummer requirement of a nonzero terminal c must not be copied blindly to a future Dickson implementation.

The trace-good condition makes this degree-d polynomial separable and fully split. Shared quotient/remainder membership can therefore use the same radical divisor interface, with each squaring update replaced by V_j^2-2. Denominators arise only in the proof's G parametrization, not in this Dickson polynomial membership circuit.

The norm-one group and its elements can be constructed with standard Fp2 arithmetic, and random torus elements can be sampled uniformly, for example through a rational parametrization or the surjective map z->z^(p-1) on uniformly sampled nonzero Fp2 elements. An exact-order-d element can be found through its 2-adic part without full factorization of p+1. These are polynomial-overhead base-construction mechanisms, not decomposition solvers.

P-256 has v_2(p+1)=96, so this torus family is concretely compatible with substantial tower depths at that fixed field. That does not supply a solver or an attack, and a fixed field is not an asymptotic family. Arbitrary primes need not have a suitably large 2-power divisor in either p-1 or p+1. The extension therefore broadens the admissible family but does not establish an all-prime-field algorithm.

## 8. Full selected norm/remainder scheme is reduced at fixed input

Fix the curve, a trace-good split membership polynomial F, and a rational target R, and use the selected chart. The monic degree-m divisor U of separable F varies in a finite geometrically reduced divisor scheme. Hence a tangent vector in that membership scheme has delta U=0.

Put N=(X-x_R)U. Target separation and squarefree U make N squarefree. Every root of N is rational and f has no rational root, so gcd(N,f)=1. At any geometric norm solution

    A^2-f C^2=N,

these facts imply gcd(A,C)=1: a common factor would square-divide N. They also imply gcd(A,f)=1: a common root would make N and f share a root. Thus gcd(A,fC)=1.

A tangent variation with fixed monic leading coefficient of A and fixed N satisfies

    A*delta A=f*C*delta C,
    deg(delta A)<h, deg(delta C)<=h-2.

Because gcd(A,fC)=1, A divides delta C. Its degree is h, so delta C=0, and then delta A=0. Characteristic is odd, so dividing the differentiated norm equation by two is legitimate.

Every geometric point of the fixed-N norm scheme therefore has zero tangent space. For a finite-type affine scheme over a field, this forces zero-dimensional reduced local rings at those points; equivalently its finite geometric fibers are reduced. Finiteness can also be seen directly: for each root of squarefree N, the function selects one of the two curve points above it, leaving only finitely many possible zero divisors, and the monic normalization gives at most one function for each divisor.

The target linear equation selects components of that finite reduced scheme; it does not create nilpotents. The retained quotient/remainder intermediates are uniquely determined by monic division. Uniquely bounded polynomial Bezout witnesses and nonvanishing inverse coordinates are graph extensions as well, so they preserve this reducedness. Over a finite field the resulting finite geometrically reduced scheme is finite etale. If arbitrary redundant free slack variables were added, this last assertion would need to be revisited.

This argument applies to scalar-Kummer and trace-good Dickson split membership alike. It does not carry over directly to terminal Graeffe membership, whose original divisor coefficient scheme can be nonreduced before the norm equations are imposed; the final norm intersection would need its own analysis there. Reducedness also does not mean all points are Fp-rational: outside the short-relation-free event, a reduced scheme can contain genuine separable extension-field models, as the retained F17 mixed-twist regression demonstrates.

Reducedness is an encoding property, not a computational complexity bound. A finite radical ideal can still have enormous degree, expensive Groebner computations and difficult point extraction. None of the preceding reasoning proves the missing uniform sub-exponential solver cost.
