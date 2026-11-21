# Explicit high-genus covers and the full-factor-base cost

15 November2026. This is a source review and hand derivation for a different
route: transfer the elliptic subgroup into a low-degree plane-curve Jacobian,
then apply the published Enge--Gaudry--Thome algorithm. No curve search,
Jacobian solver, native experiment, formal geometry proof or novelty claim is
made. The conclusions concern its explicit full factor base, rather than all
possible algorithms on covers. [pcc:unverified]

## Prior work actually checked

KB unavailable: no research-kb MCP tools are loaded and the repository wrapper
returns127 (no executable). The fallback review reads the existing
[prime-field cover/norm report](../enge_primefield_cover_20261024/REPORT.md),
the [cover-transfer assessment](../cover_transfer_assessment_20261013/README.md)
and its conditional group theorem, and the prime-field barrier/closure notes.
The October24 report already constructs hyperelliptic covers and compares
projected relations with their elliptic norms. It leaves general higher-degree
places and projected-norm cost open. Those prior constructions are reused as
context, not presented as absent or new. [pcc:unverified]

Primary text opened: Enge, Gaudry and Thome, *An L(1/3) Discrete Logarithm
Algorithm for Low Degree Curves*, arXiv:0905.2177v2, December2009,
https://arxiv.org/abs/0905.2177 . Read the relevant discussion in sections2--5, especially Proposition3,
Heuristics5/6, the precise factor base on page9, Theorem7 and the non-cyclic
case. Theorem7 requires g>=(log p)^rho with rho>=2, not merely a small genus
or a small elliptic factor. It computes a full factor base before individual
descent; its complexity analysis is heuristic. [pcc:unverified]

Milne, *Jacobian Varieties*,
https://www.jmilne.org/math/xnotes/JVs.pdf, section2/Proposition2.3 and
section6/Corollary6.3, supplies the Abel--Jacobi embedding and induced-map
context. The degree composition below is derived directly on divisors.
The PDF hashes and repository blob bindings are in cover-review/sources.txt.
The standard Hasse--Weil point bound is used explicitly, without a new proof
of that theorem. No exhaustive literature-search claim. [pcc:unverified]

## A concrete cover family meets the geometric hypotheses

Let E/Fp:y²=f(x)=x³+A*x+B be nonsingular, p>3, and let its whole rational
group have prime order r comparable to p. Set k=ceil(log2 p), let u be the
least power of two at least k, and v=u+1. Choose s with f(s)!=0; among
0,1,2,3 at least one works because a nonzero cubic has at most three roots.
For sufficiently large p, p>v. Define the smooth projective model of

    C: Y^(2u) = f(X^v+s),      pi(X,Y)=(X^v+s,Y^u).

All data are over Fp and the displayed equation has only a constant number
of nonzero monomials after expansion. This is a mathematical construction,
not an implementation of arbitrary-divisor arithmetic. [pcc:unverified]

The polynomial H(X)=f(X^v+s) has degree3v and is squarefree. A root has
X!=0 because f(s)!=0; its derivative is v*X^(v-1)*f'(X^v+s), nonzero because
p does not divide v and f has simple roots. A simple zero of H also proves
Y^(2u)-H absolutely irreducible by the valuation/Eisenstein criterion over
the rational function field (algebraic-closure(Fp))(X). The affine curve is nonsingular: when Y!=0
its Y derivative is nonzero; at Y=0 its X derivative is nonzero.
[pcc:unverified]

Since2u is a power of two and3v is odd, gcd(2u,3v)=1. Tame Kummer
ramification has index2u at all3v finite zeros and at infinity. The latter
is one Fp-rational place. Riemann--Hurwitz gives

    N_Y=2u,  D_X=3(u+1),
    g=((2u-1)(3u+2))/2 = 3u²+u/2-1,
    delta=deg(pi)=u(u+1).

For the map degree, [Fp(C):Fp(X)]=2u and [Fp(X):Fp(x)]=v, while
[Fp(E):Fp(x)]=2. The field tower therefore gives degree uv. The infinity
place maps to O on E. These are hand geometric arguments, not Lean or
native program certificates. [pcc:unverified]

Thus g=Theta(k²), N_Y,D_X=Theta(k), delta=Theta(k²), and
N_Y*D_X/g<=3 for u>=2. In the notation of Theorem7,
M=log(g log p)/log p=Theta(log k/k). Both

    N_Y/(g/M)^(1/3), D_X/(g/M)^(1/3)

grow like (log k)^(1/3). Consequently this family meets all three geometric/
growth conditions of Theorem7, with fixed kappa=3 and rho=2, eventually.
Merely taking N_Y about g^(1/3) would not suffice here: M tends to zero as
p grows, and the precise inequalities matter. Heuristics5/6 remain separate
assumptions; they are not established for this family by the construction.
[pcc:unverified]

Known-order/generator handling and the non-cyclic reconstruction in the paper
also require charged algorithms. No Jacobian arithmetic, order computation,
factor-base implementation or target descent is supplied by this hand review.

## Subgroup transfer is available algebraically

For a finite degree-delta map of smooth projective curves, pullback of a
closed point Q is the sum of its preimages with ramification multiplicities;
pushforward weights each preimage by its residue-field degree. The sum of
these products is delta. Therefore pi_*pi^*(Q)=delta*Q, extending by
linearity and passing to divisor classes to

    pi_* o pi^* = [delta] on E=Jac(E).

Pullback and pushforward respect principal divisors. Since delta<r eventually,
the prime r is coprime to delta, and pullback is injective on E(Fp)[r].
The pullback divisor may have non-rational support; fully rational fibres
are not required for this algebraic reduction. Construction, representation,
descent and recovery costs still belong in a complete algorithm.
[pcc:unverified]

The existing theorem CoverTransfer.pull_injective_on_subgroup has a pinned
October22 Lean4.34.1/mathlib replay with only propext, Classical.choice and
Quot.sound. It formalizes the conditional group implication once additive
maps, cardinality, coprimality and the composition identity are supplied.
That historical receipt is reused at exactly that scope; no fresh kernel
build or formal construction of pi and its divisor maps is claimed here.
[pcc:unverified]

## Two separate cost statements

First, substituting g=Theta(k²), log p=Theta(k) in the published
L_(p^g)(1/3,c+o(1)) guarantee gives

    log(T_bound) = Theta(k*(log k)^(2/3)).

This upper-bound expression does not certify exp(o(k)) work for the original
elliptic input. An upper bound of that size is not a lower bound on the
actual running time of every specialized algorithm. [pcc:unverified]

Second, there is a direct lower bound for the explicit factor-base stage of
this route. The paper's factor base contains affine non-inert places of
degree<=mu, where mu>=1 eventually. Every affine Fp-rational point has the
form (X-x,Y-y), so has inertia degree one and is included, apart from any
finite exceptional bookkeeping. Hasse--Weil gives

    #C(Fp) >= p+1-2g*sqrt(p),
    #C_affine(Fp) >= p-2g*sqrt(p) = p*(1-o(1)).

Distinct rational points give distinct Abel--Jacobi classes for g>0
(Milne Proposition2.3). Materializing the full source base therefore takes
Omega(p) records, which is exponential in k. This assertion concerns the
specified explicit-factor-base procedure, not an information lower bound
against an implicitly represented base. [pcc:unverified]

Projection and sign folding do not remove the exponent in this family.
Each rational point of E has at most delta distinct geometric preimages,
so projecting the rational source points yields at least
(p-2g*sqrt(p))/delta distinct elliptic points. Identifying P with -P costs
at most a factor of two. For sufficiently large p, an explicit projected
base consequently has at least

    p/(4 delta) = exp(k*log(2)-O(log k))

signed columns. This bound charges the full projected base even after
duplicate images are merged. No relation-generation timing or random-rank
heuristic is needed for that counting obstruction. [pcc:unverified]

## Disposition

The curve and subgroup-transfer ingredients exist algebraically; the
published full-factor-base Jacobian procedure does not meet the requested
subexponential prime-field criterion. No high-genus native benchmark is
launched merely to rediscover that materialization cost. A smaller implicit
base, compressed projected columns, or a different norm-generation/descent
method would be another algorithm with its own useful-yield and total-cost
obligations. This review does not rule those out or claim a universal lower
bound for cover-based ECDLP algorithms. [pcc:unverified]
