# Matched-level bivariate jets do recover the origin

2026-10-30 UTC. Pure mathematical review of the pinned Oct25 manuscript,
D.4.4, and `/workspace/audit/multivariate-code-tradeoff-review.md`.
No implementation, numerical experiment or ECDLP evaluation. This proves a
recovery lemma only. It does not prove a packet-occupancy, mode-count or
contraction algorithm for a proposed multivariate deformation.

## Contract and result

Let `L=2^a`, `S={-L,...,L-1}`, `alpha=2^(a+1)`, and

    S_i={t in S: v2(t)=i},   |S_i|=n_i=L/2^i,   0<=i<u<=a.

Let F(X,Y) have bidegree at most

    (d,d),   d=M(2L-1),   M>=u.

Work in the source's coefficient field K and truncated coefficient ring
`A=O[V]/(V^ell)`, coefficientwise in V. Assume `F(0,0) in A` and that its
local series are integral at the sampled pairs. At each `(t1,t2) in S_i^2`,
the supplied rectangular jet consists of coefficients

    [z1^j z2^k] F(t1+alpha*z1,t2+alpha*z2),   0<=j,k<q_i.

With the *original* source choices

    h=ceil(3M/u),   q_i=h(u-i),
    p_i=p+8hD_i+8M(a+1),   D_i=L/2^i,

these jets modulo `2^p_i` suffice to recover `F(0,0) modulo 2^p`.
For the main large-u regime, only diagonal pairs `(t,t)` and the triangular
part `j+k<q_i` of their rectangular jets are needed. The proof is below.

A shorter conservative corollary follows immediately from the source's
Lemma D.5: use `M'=2M`, `h'=ceil(6M/u)`, multiplicities `h'(u-i)`, and precision
`p+8h'D_i+16M(a+1)`. This has the same asymptotic `O(MD_i/u)` precision under
the source's stated large-u recursive hypotheses. No new integrality or
denominator lemma is needed for that corollary.

## 1. Diagonal restriction for u>=3

Set

    G(T)=F(T,T),   deg G<=2d=2M(2L-1).

At a diagonal node `(t,t)` of level i,

    [z^s]G(t+alpha*z)
      = sum_(j+k=s) [z1^j z2^k]F(t+alpha*z1,t+alpha*z2).

For `s<q_i` every summand is in the given rectangle. Summing integral
coefficients preserves integrality and loses no two-adic precision. In
particular, the diagonal restriction is not a claim that arbitrary Boolean
entry code can be evaluated on non-Boolean arguments; it is an operation on
the stated, already supplied local jets.

The exact total univariate Hermite multiplicity is

    W=sum_(i=0)^(u-1) n_i q_i
     =hL(2u-2+2^(1-u)).

For every `u>=3`, using `h>=3M/u` gives

    W>4ML>2d.

Thus the univariate Hermite construction uniquely recovers G, including
`G(0)=F(0,0)`. D.5 used only the odd nodes for its simpler sufficient degree
estimate; the full multiplicity sum permits this doubled diagonal degree.

The precision proof can be reused without changing h or the original
displayed p_i. Its Hermite basis factor E_t obeys exactly the source bound

    v2(E_t(0))>=-6hD_i.

Also `q_i<=hu<=3M+u<=4M`. Evaluation of the truncated quotient at
`-t/alpha` therefore loses at most `4M(a+1)` further powers of two. The
reciprocal local series for E_t is integral with constant term one, exactly
as in D.5. Consequently the total loss is bounded by

    6hD_i+4M(a+1)<8hD_i+8M(a+1).

The doubled degree changes neither this valuation estimate nor the
coefficient-evaluation bound after W exceeds the degree. Rational
denominators are charged as in the source: each basis multiplier has
polynomially many rational factors of O(a) bits, and the recovery performs
exact rational combination and cancellation before reduction modulo `2^p`.

## 2. Full polynomial reconstruction is a different, easier exact question

Even the odd matched grid `S_0 x S_0` with its rectangular jets is exactly
unisolvent for the full bidegree-(d,d) polynomial. Indeed

    |S_0|q_0=Lhu>=3ML>d.

The univariate odd-node Hermite map is injective on degree at most d in
either variable. Taking its tensor product is therefore injective on the
bivariate space. Equivalently, first fix a derivative order and an odd X
node; its polynomial in Y has enough zero jets to vanish identically. Then
apply the same argument to each coefficient polynomial in X.

Consequently there cannot be a nonzero polynomial of this bidegree whose
all exact matched-level rectangle jets vanish. In particular there cannot
be one with nonzero value at the origin. This exact fact alone, however,
does not establish the source's inexpensive finite-precision recovery:
odd-node-only Hermite denominators can have valuation proportional to ML.
The diagonal/tapered argument above supplies the needed precision claim
for the origin functional in the large-u regime, without promising equally
cheap recovery of every bivariate coefficient.

## 3. The small cases u=1,2 also fit the original precision

These cases are not needed in the source's asymptotic active-call regime,
but can be checked without invoking an extrapolation of that regime.

Write `q=q_0=hu` and

    A_0(T)=product_(t odd in S) (T-t).

At an odd node `t=-L+1+2j`, where `0<=j<L`, the derivative is, up to sign,

    A_0'(t)=2^(L-1) j! (L-1-j)!.

Since `L=2^a`, the binary digits of `j` and `L-1-j` complement each other.
The factorial valuation formula gives

    v2(A_0'(t))=2L-a-2.

In the odd-node Hermite formula with multiplicity q, the one-variable basis
factor at zero consequently has valuation `-q(2L-a-2)`. Its local reciprocal
is integral after the scale alpha, because every difference of distinct odd
nodes has valuation at most a. Tensor-product Hermite at the origin loses
at most

    2q(2L-a-2)+2(q-1)(a+1)
      =4qL-2q-2(a+1).

For `u<=2`, `8hL>=4qL`, so the source's odd-node precision
`p_0=p+8hL+8M(a+1)` covers this loss. Together with Section 1 this establishes
the stated origin-recovery claim for all `1<=u<=a`, under the specified
integral local-jet assumptions.

## 4. Arbitrary constants in a proposed taper still need checking

The notation `q_i ~ (M/u)(u-i)` hides constants that matter for
unisolvence. The preceding result is for the stated source constant and
the conservative doubled-M variant, not every positive constant.

For example define

    A_i(T)=product_(t in S_i)(T-t),   N_i=n_i q_i.

If a partition J of the levels satisfies

    sum_(i in J) N_i<=d,
    sum_(i notin J) N_i<=d,

then the explicit polynomial

    R_J(X,Y)=product_(i in J) A_i(X)^q_i
             *product_(i notin J) A_i(Y)^q_i

has allowed bidegree and all matched-level rectangular jets zero, yet
`R_J(0,0)!=0` in characteristic zero because none of the sample nodes is
zero. Such a partition is therefore an exact obstruction to origin
recovery over the field for those smaller multiplicities. This is a
sufficient construction of a kernel element, not a complete criterion for
all possible multiplicity choices. Scaling its origin value to a unit can
change local integrality and precision, which must be analyzed separately
if a modulo-2^p claim is intended.

## Scope

Matched-level sampling is not itself an algebraic obstruction to recovering
the origin. Diagonal restriction resolves that particular question while
preserving the source's asymptotic precision scale. This does not prove that
simultaneous-level packets have the desired occupancy, that their weighted
crossing-mode counts improve, or that the transformed entry descriptions
obey a recursive evaluation invariant. Those are separate proof obligations.
