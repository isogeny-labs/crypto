# Norm-one/Dickson extension with explicit coverage constants

2026-10-30 UTC. This is a supporting mathematical extension, not an efficient
solver. The independent [branch and scheme review](../reviews/torus-and-reducedness.md)
and [constant review](../reviews/final-theory.md) check the arguments. The
Dickson/norm-one factor base predates this continuation in the repository's
P-256 factor-base work; no novelty is claimed for it.

## Input and membership

Let p>3, E/Fp: y^2=f(x)=x^3+ax+b be nonsingular, and r=#E(Fp) odd.
For direct discrete-log recovery assume r prime. Let T be the norm-one group
in Fp2*, of size p+1, and H its subgroup of order d=2^t dividing p+1.
Sample g uniformly in T and set

    x_w(g)=gw+(gw)^(-1),  w in H.

All these abscissas lie in Fp. For w!=v their equality is equivalent to
g^2=(wv)^(-1). Reject every g with g^2 in H; there are at most2d such
elements. The test g^(2d)=1 is polynomial in the input bit length. This
exclusion is sufficient, including the small-d cases where it need not be
necessary. Charge every sample and rejection.

On the retained coset there are d distinct abscissas, and their membership
polynomial is

    F_g(X)=D_d(X)-c,  c=g^d+g^(-d),
    D_2(X)=X^2-2, D_(2d)=D_2 o D_d.

It is split and separable. Its terminal c is not +/-2, but it can be zero.
Thus U|F_g can be imposed by the radical shared remainder circuit, replacing
each squaring update by square-minus-two. This statement describes an
encoding; no unknown-coefficient Dickson solver has been implemented here.

The actual rational signed base includes both signs above each usable x.
Its size B is at most2d. Uniform sampling and constructing H, Fp2 and the base
cost d*poly(log p), using ordinary finite-field arithmetic and square roots;
no logarithms or full factorization of p+1 are required. An exact-order-d
generator is obtained from the 2-adic part with rejection.

## Why the covers remain connected

Odd r implies f is irreducible cubic. Its roots alpha_i form a Frobenius
three-cycle. The six distinct nonzero roots of Z^3 f(Z+Z^(-1)) satisfy
Z^2-alpha_i Z+1=0 and lie in Fp6. Put sigma=Frobenius^2, which fixes H and
satisfies sigma^3(z)=z on those roots.

Two distinct branch roots cannot have ratio zeta in H. If their traces are
equal, they are inverses, so z^2 lies in Fp2. This contradicts the trace's
degree3 over Fp2. If their traces differ, orient them so the second trace is
sigma of the first. Their roots then satisfy either sigma(z)=zeta*z,
forcing zeta^3=1 and hence zeta=1, or z*sigma(z)=zeta^(-1), forcing
sigma^2(z)=z and contradicting the trace three-cycle.

Consequently the six finite branch zeros for each w are private to that w.
The rational-function squareclasses f(wG+(wG)^(-1)) are geometrically
independent, even though they share poles. Their multiquadratic covers are
connected. This uses that H has no three-torsion; it does not establish the
same claim for arbitrary subgroup orders.

## Density and rationality errors

Use uniform g in all of T for the probability calculations. Define the
index-weighted count

    B_tilde(g)=d+sum_(w in H) chi(f(x_w(g))).

On trace-good g this is exactly B; on trace-bad g it counts multiplicities
and must not be treated as a count of distinct base points.

For w!=v the product character cover has twelve simple zeros and two poles
of even order, hence genus5. On the Fp compactification the two common
poles are nonrational. The sample space is all p+1 rational points; none is
omitted. The quadratic-character sum is bounded in absolute value by
10 sqrt(p). Therefore

    E(B_tilde-d)^2 <=d+d(d-1)*10 sqrt(p)/(p+1),
    epsilon_den=4/d+40 sqrt(p)/(p+1),
    Pr[B_tilde<d/2] <=epsilon_den,
    epsilon_trace=2d/(p+1).

Private branch zeros also make each substituted s-point summation polynomial
nonzero, by changing only one square-root sign in the generic cover.
Its arguments have Laurent degrees +/-1. Clearing denominators doubles the
scalar-Kummer degree bound, giving

    Delta_T=(p+1)^(-1) sum_(s=3..m-2) binom(d,s)*2s*2^(s-2).

Cap probability upper bounds at one where necessary. Signs are already
included by the summation polynomial. Outside this short-zero-sum event
and the trace-bad event, the prior rationality proof applies unchanged:
nonrational residual points are anti-Frobenius points; their sum is zero
because E(Fp2) has odd order. Distinctness and target separation exclude all
cardinalities except3 through m-2, which the short-relation event forbids.
Thus every selected-chart geometric norm solution has rational core
coefficients for every nonzero rational target, on this good event.

## Coverage: an explicit simultaneous statement

Let M=binom(d,m), with tuple and chart conventions as in [coverage.md](coverage.md).
For k indices the cover has degree2^k over G, and x_w has degree2.
Consequently the elliptic projection has degree2^k, twice the Kummer value.
The deck involutions and theorem of square give exactly

    deg(sum_i c_i P_i)=2^k sum_i c_i^2.

The same relative-sign count for pairs of m-supports now sums to2m4^m.
Using index-labelled tuples also on trace-bad g makes the unconditional
count valid. Only on trace-good g do we identify them with actual distinct
abscissa tuples. The second-moment and chart-loss inequalities become

    sum_(g,R) V_g(R)^2 <=sum_g T_g+2m4^m M^2,
    sum_g E_g <=M 2^m(2m^2+3m).

With p>=17, d>=8m^2, and kappa=d^m/(m!r)>=2^m, the density and binomial
argument still gives T_g/r>=kappa/2^(m+1). Markov's inequality bounds the
event E_g>kappa*r/2^(m+2) by

    epsilon_chart=4*4^m(2m^2+3m)/(p+1).

The averaged second moment is at most

    K_T=2^m M/r+2m4^m M^2/((p+1)r).

Since r/(p+1)<=2, its normalized value satisfies
K_T/kappa^2<=1+4m4^m<=5m4^m. Restrict also to the event nu_g<=4K_T,
which fails on probability at most1/4. Cauchy-Schwarz then gives actual
selected-chart coverage

    rho_g>=1/(320m16^m).

All the preceding events hold simultaneously on bases of probability at least

    3/4-epsilon_den-epsilon_chart-epsilon_trace-Delta_T.

No independence of these events is assumed. No extra conversion factor for
conditioned sampling is hidden: the distribution throughout is uniform g in
T, and trace-bad samples trigger charged rejection.

For odd m=c0 sqrt(n)+O(1), n=log2 r, c0<sqrt(2/3), and d the least
compatible power of two above2(m!r)^(1/m), these errors tend to zero along
an admissible input family. This is a family condition, not a theorem that
every prime supplies the required d. At least half of bases eventually
satisfy rationality and this coverage bound. The conditional reduction has
the same exp(O(sqrt(n))) non-solver overhead as in the scalar case.

## Reducedness is separate from efficiency

The [independent review](../reviews/torus-and-reducedness.md#8-full-selected-normremainder-scheme-is-reduced-at-fixed-input)
also proves the full selected norm/remainder scheme finite and geometrically
reduced at fixed input. Briefly: U|F_g is finite etale; fixed
N=(X-x_R)U is squarefree and coprime to f. At a norm solution
A^2-fC^2=N, gcd(A,fC)=1. Its tangent equation A*deltaA=fC*deltaC and the
degree bounds force deltaC=deltaA=0. The bounded quotient/Bezout witnesses
are uniquely determined graph extensions. The target equation selects
components. None of this bounds Groebner solving degree or construction
cost. A reduced ideal can still be expensive to solve.

P-256 has v2(p+1)=96, as already used in the prior Dickson-base research.
This verifies an applicable membership parameter structure at that fixed
prime. It does not give an efficient decomposition oracle, a P-256 attack,
an arbitrary-prime algorithm, or an asymptotic conclusion from one field.
