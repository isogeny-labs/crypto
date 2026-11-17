# Auxiliary elliptic cosets remove the modulus congruence restriction

2026-10-30 UTC. Supporting construction and probability bounds, not a relation
solver. The auxiliary factor base and low-degree isogeny tower are already in
PKM2016 sections3.4-3.5; they are not claimed new. This note transfers the
present rationality/coverage analysis and charges a conservative construction
bound. It does not claim an efficient implicit solver, a measured result or
an implemented auxiliary-curve pipeline.

Use a nonsingular main E/Fp of odd whole order r; direct logarithm recovery
assumes r prime. Let d=2^t>=8 and p>=256d^4. No divisibility of p-1 or p+1
by d is assumed. The restriction on the main curve's order remains explicit.

## 1. Finding a rational cyclic auxiliary subgroup, with setup charged

The [primary-source counting review](../reviews/modular-curve-counting.md)
verifies the following classical facts. For d>=8 and p odd, X_1(d)/Fp is
smooth, proper and geometrically connected. Its noncuspidal points classify
actual pairs (A/Fp,T in A(Fp)) with T of exact order d: the fine-moduli
condition removes a possible twist/descent ambiguity. Conservative bounds
are genus<=1+d^2 and at most2d^2 geometric cusps. Hasse-Weil gives

    #Y_1(d)(Fp)>=p+1-2(1+d^2)sqrt(p)-2d^2>=p/2.

For each Fp-isomorphism class of A, at most d^2 pointed classes occur.
Each class of short Weierstrass models has at least(p-1)/6 coefficient
pairs. Thus a uniform coefficient pair defines a nonsingular curve with
a rational exact-order-d point with probability at least1/(24d^2).
This is not the stronger, heuristic near1/d distribution used by PKM.

An explicit randomized search is therefore:

1. Draw a,b uniformly; reject a singular auxiliary curve A.
2. Count N=#A(Fp) by the classical polynomial-time point-counting algorithm.
3. Write N=2^s u with u odd. Draw T uniformly in A(Fp), put S=[u]T,
   and determine its exact2-power order by repeated doubling.
4. If ord(S)=2^e with e>=t, output T_H=[2^(e-t)]S and H=<T_H>.
   Otherwise draw another auxiliary curve.

If A contains an order-d point, the uniform2-primary component has order
at least d with probability at least1/2: in C_(2^a) times C_(2^b), a>=t,
the elements killed by2^(t-1) form at most half the group. Thus the
expected number of curve draws is at most48d^2, with polynomial(log p)
work per draw. Divisibility of N by d alone is never used as a cyclicity
certificate. Exact doubling checks certify the returned point.

Uniform point sampling also has expected polynomial cost. For example draw
a uniform integer from0 through2p: the last value proposes O; the other
values propose x and one of two canonical square-root signs. Reject
nonsquares and one duplicate proposal at y=0. Every point then has exactly
one proposal, so the first accepted point is uniform. Acceptance probability
N/(2p+1) is bounded below in this range. Ordinary finite-field square-root
and point-counting costs are charged. Geometric sampling tails can be capped
and resampled with their cost and constant success factors retained.

This is a mathematical construction bound, not an implementation or timing.
The source review supplies the fine-moduli and point-counting references.

## 2. The base and its exact trace-collision test

Sample G uniformly in A(Fp), and use abscissas

    x_h(G)=x_A(G+h),  h in H.

Exclude2G in H. There are at most4d such rational G, because[2] has
degree4. Outside this event every abscissa is finite and they are distinct:
equal x values give G+h=-(G+h') and hence2G in H.

Let phi:A->A/H be the quotient isogeny with kernel H. It is a composition
of t rational2-isogenies, constructed from the known subgroup with Velu's
formulae. The test2G in H is exactly[2]phi(G)=O and costs polynomial work
in t,log p. The weaker test[2d]G=O is not substituted: it can reject many
points outside this kernel and would require a different error bound.

For trace-good G, put c=x(phi(G)). The numerator of phi_x(X)-c is a
degree-d split separable polynomial F_G. Its roots are exactly the d
abscissas above. At every intermediate tower stage the denominators are
nonzero on those roots: a pole would map to final O, whereas phi(G) is
not2-torsion. The rational signed main-curve base has B<=2d points.

## 3. Why the six branch sets are private

Let alpha_i be the three roots of the main cubic f. They are a Frobenius
three-cycle. The auxiliary cubic is reducible, because A has rational
nonzero2-torsion; it cannot share a root with the irreducible main cubic.
There are consequently six distinct points Z on A with x_A(Z)=alpha_i.
They lie in A(Fp6). Put sigma=Frobenius^2. Then sigma^3 Z=Z, and sigma
cycles their three x values. Frobenius fixes a nonzero point of A[2], so
its action on A[2] has order at most2; sigma fixes all of A[2].

Suppose two different branch points differ by h in H. If they have the
same x value, they are inverses and2Z is in H. Then delta=sigma Z-Z
is in A[2] and fixed by sigma. The identity sigma^3 Z=Z gives3delta=0,
so delta=0, contradicting the degree-three x value over Fp2.

If their x values differ, orient them so the second is sigma of the first.
The points are either Z+h=sigma Z or Z+h=-sigma Z. In the first case
sigma^3 Z=Z gives3h=0; H has2-power order, so h=0, a contradiction.
In the second case sigma^2 Z=Z, again contradicting the x three-cycle.

Thus the translates of these six branch points by different h are disjoint.
Each f(x_A(G+h)) has six private simple zeros and a pole of order6 at-h.
The poles have even order. These functions have independent geometric
squareclasses; their k-fold multiquadratic cover is connected of degree2^k.

## 4. Density, rationality and selected-chart coverage

Write M_A=#A(Fp). Define the individual quadratic character at a pole by
its normalized local leading unit, which is1 for the short model and monic
main cubic. Thus its extended value is+1 there. At a product's pole the
extended character is this value times the other factor's character; the
product is not automatically+1. All poles are in the separately rejected
trace-bad event, but this extension makes the unconditional moment precise.

The off-diagonal double cover over A has twelve simple branch zeros and
even-order poles. Riemann-Hurwitz gives genus7. The crude Hasse-Weil
difference bound is16sqrt(p), without needing a sharper Prym estimate.
For the index-weighted signed count B_tilde this yields

    E(B_tilde-d)^2<=d+d(d-1)*16sqrt(p)/M_A,
    epsilon_den=4/d+64sqrt(p)/M_A,
    epsilon_trace=4d/M_A.

On trace-good G this weighted count equals actual B. On trace-bad G it is
not identified with a count of distinct points.

The substituted s-point summation polynomial is nonzero by a one-square-root
Galois flip. Each x_A(G+h) has pole divisor of degree2; the degree of the
composed pole divisor is at most2s2^(s-2). Therefore

    Delta_A=M_A^(-1) sum_(s=3..m-2) binom(d,s)*2s*2^(s-2)

bounds the short-zero-sum event. Exclude trace-bad G as well. The previous
mixed-twist argument then makes every selected-chart geometric norm model's
core coefficients rational, simultaneously for every nonzero rational target.

Each projection from the k-cover to the main E has degree2^k. The same
deck involutions and theorem of square give

    deg(sum_i c_i P_i)=2^k sum_i c_i^2.

Consequently the torus collision and chart-loss proof transfers with p+1
replaced by M_A. For p>=37, Hasse gives r/M_A<=2. With odd m>=3,
d>=8m^2 and kappa=d^m/(m!r)>=2^m, use

    epsilon_chart=4*4^m(2m^2+3m)/M_A,
    rho0=1/(320m16^m).

A fraction at least

    3/4-epsilon_den-epsilon_trace-epsilon_chart-Delta_A

of bases simultaneously have rationality and selected-chart coverage at
least rho0. These are union and second-moment bounds, not independence or
representation-uniformity assumptions. Index-labelled tuples are counted
on all G, then identified with actual distinct-abscissa tuples only outside
the trace-bad event.

## 5. Radical membership can stay implicit

For each degree-two rational tower step N_j/D_j retain the residue V_j
modulo monic U. Enforce

    D_j(V_(j-1))*V_j-N_j(V_(j-1))=0 mod U

and a bounded Bezout inverse for D_j(V_(j-1)) modulo U. Start V_0=X and
end V_t=c. Monic quotient equations implement the remainders. All these
coefficient equations can be kept quadratic with O(tm) coefficients and
O(tm^2) terms, up to fixed constants. Inverses and residues are unique graph
coordinates on the split selected chart. They do not introduce new models.
The norm equation and its chart constraints are the same as before.

This is a specified encoding construction; it has not been implemented or
run in the current native prototype. An efficient solution method for this
implicit system remains precisely the missing step.

## 6. What this does and does not remove

Set n=log2 r, choose odd m near sqrt(n)/2 and the least power of two
d>=2(m!r)^(1/m). Then log2 d=n/m+O(log m). Eventually d>=8m^2,
p>=256d^4, and all probability errors above tend to zero. At least half
the bases are good. These inequalities now hold for every sufficiently
large prime p with a main odd-prime-whole-order input, rather than only
when d divides p-1 or p+1. No new prime-distribution assumption is used.

The auxiliary setup is exp(O(sqrt(n))) expected cold work. At the stated
balanced choice it is at most2^(4sqrt(n)+O(log n)); base construction is
smaller. The earlier conditional query count2^(4sqrt(n)+O(log n)) and
dense linear-algebra bound2^(6sqrt(n)+O(log n)) retain their exponent
classes. The [selective oracle contract](selective-oracle-contract.md)
applies without requiring fast refutation of all uncovered targets.

This removes a modulus congruence restriction from the supporting reduction
on paper. It does not remove the odd prime whole-order assumption, construct
a fast relation finder, bound its capped verified yield, implement the
auxiliary pipeline, establish novelty, or meet the user's sub-exponential
algorithm objective. There is still no achieved ECDLP breakthrough.
