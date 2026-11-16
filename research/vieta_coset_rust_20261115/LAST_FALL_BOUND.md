# A degree bound for the quadratic power-chain lift

**Hand argument, awaiting independent/formal validation [pcc:unverified].**
This concerns the exact two-term lift below over the prime field. It is not a
bound for the joint three-term system, a Boolean encoding, or an ECDLP running
 time. No novelty claim is made.

## Statement

Let p>3, m=2^d divide p-1, and c be nonzero in F_p. For a finite target
R=(t,r), use the Vieta equations

\[
 L^2-x_1-x_2-t=0,\qquad
 (x_1-t)(x_2-t)-3t^2-A-2rL=0.
\]

For each j=1,2, put u_(j,0)=x_j and impose

\[
 u_{j,i}^2-u_{j,i+1}=0\quad(0\le i<d-1),\qquad
 u_{j,d-1}^2-c^m=0.
\]

Call this generating system F. For d>=1 every input equation is quadratic,
and there are 2d+1 variables. Under the standard constructible-space definition
of last fall degree,

\[
 \boxed{d_F\le 2d+2=2\log_2(m)+2.}
\]

The argument includes r=0 using quadratic quotient algebras, without assuming
that the ideal is radical. For m=1, use x_1-c=x_2-c=0; the same formula gives 2.
Identity targets have a separate enumerating branch and are outside this lift.

The definition used here is Huang–Kosters–Yang–Yeo, *On the last fall degree of
zero-dimensional Weil descent systems*, definitions 2.1 and 2.5:
https://arxiv.org/html/1505.02532. V_D contains the input equations of degree at
most D and is closed under multiplication whenever the **actual product degree**
stays at most D. The last fall degree is the least T for which every f in (F)
belongs to V_max(T,deg f). The primary source was read directly; KB access was
unavailable. Direct ePrint PDF retrieval returned 403; the arXiv source worked.

## 1. Compressed projectors

Let C=c mu_m, a set of m distinct field elements. For a in C write
 a_i=a^(2^i) and, for one chain U=(u_0,...,u_(d-1)), define

\[
 E_a(U)=\frac1m\prod_{i=0}^{d-1}(1+u_i/a_i).
\]

This has total degree d **in the lifted variables**, rather than m-1 in x.
Moreover, sum_(a in C) E_a(U)=1 as an exact polynomial identity, without using
any gate equation. Expanding the product, each subset of indices has weight
k=sum 2^i between 0 and m-1. Its coefficient sums to zero over C when k is
nonzero, by the multiplicative-group character sum; the constant coefficient
sums to one. On a valid chain with initial value b in C, E_a evaluates to the
Kronecker delta at a=b.

There is an explicit low-degree certificate for every coordinate action:

\[
 (u_j-a_j)E_a(U)\in V_{d+1}.
\]

Normalize v_i=u_i/a_i and P_i=1+v_i. The normalized gates are
v_i^2-v_(i+1), with last gate v_(d-1)^2-1. The identity

\[
 (v_j-1)\prod_{i=0}^{d-1}P_i
 =\left(\prod_{i<j}P_i\right)
 \left[\sum_{k=j}^{d-2}(v_k^2-v_{k+1})
                   \prod_{i=k+1}^{d-1}P_i
       +(v_{d-1}^2-1)\right]
\]

proves the claim: every gate multiple on the right has degree at most d+1.
Empty sums and products cover j=d-1 and d=1. Rescaling gates and coefficients
uses only nonzero field constants.

For two chains set e_(a,b)=E_a(U_1)E_b(U_2). These degree-2d projectors sum
exactly to one, and every chain-coordinate relation

\[
 e_{a,b}(u_{j,i}-a_j^{2^i})\in V_{2d+1},\qquad(a_1,a_2)=(a,b),
\]

follows by multiplying a one-chain certificate by the other projector.

## 2. Local slope algebras

Put T=2d+2, q_(a,b)=a+b+t, and
K_(a,b)=(a-t)(b-t)-3t^2-A. Multiply the two Vieta equations by e_(a,b), and
reduce x_1,x_2 using the preceding coordinate relations. All products have
degree at most T. This gives

\[
 e(L^2-q_{a,b}),\quad e(2rL-K_{a,b})\ \in V_T.
\]

If r is nonzero, let ell=K/(2r). Then e(L-ell) is in V_T and has actual
degree at most 2d+1. Multiplying by L+ell remains within T. Consequently
(ell^2-q)e is in V_T. If ell^2 differs from q, e itself is in V_T and that
block is discarded. Otherwise its local algebra is F_p with slope ell.

If r=0, a block is discarded when K differs from zero. Every remaining block
is F_p[L]/(L^2-q), with basis 1,L. This includes nonsplit quadratic fields and
nonreduced blocks when q=0. Thus radicality or rationality of all geometric
solutions must **not** be asserted for the r=0 case.

## 3. Degree-preserving normal-form induction

Map the full polynomial ring to the direct product of the remaining local
algebras: chain coordinates take their fixed values and L takes the local
slope class. Every original generator maps to zero. Let W be the span of e
for the linear blocks, or e and Le for the quadratic blocks. The map restricted
to W is injective, and W gives unique representatives of the product algebra.
Every element of W has degree at most 2d+1=T-1.

Write NF for the resulting normal-form map. The discarded-projector relations
give 1-NF(1) in V_T. For each basis element w of W and each coordinate z,

\[
 zw-\operatorname{NF}(zw)\in V_T.
\]

For chain coordinates this is the coordinate-projector identity, optionally
multiplied by L; its actual degree stays at most T. For L it is the local
linear or quadratic slope relation. This also covers nonreduced local algebras.

Induct on the degree j of a monomial M. Suppose
M-NF(M) is in V_max(T,j). Its actual degree is at most max(j,T-1), so multiplying
it by the next variable stays within max(T,j+1). Add a linear combination of
the basis multiplication relations above. This proves

\[
 M-\operatorname{NF}(M)\in V_{\max(T,\deg M)}.
\]

Linearity gives the same result for every polynomial f. If f belongs to (F),
its image in the product algebra is zero. Injectivity on W forces NF(f)=0.
Therefore f belongs to V_max(T,deg f), which is exactly the claimed bound.
The empty-product-algebra case gives 1 in V_T and is covered too. For m=1,
linear membership relations replace the chain projectors and the argument uses
T=2 directly.

## Interpretation and limits

At m=8192 the bound is 28. The native extractor nevertheless manipulates
univariate elimination polynomials with degrees growing with m. These are
compatible facts: last fall degree is about the lifted generating system,
not the degree of a lexicographic elimination polynomial.

The proof uses m^2 projectors as **existence witnesses**. The executable solver
constructs no such family and performs no interpolation: it uses composed
quadratic remainders, gcds and finite-field roots. This degree bound does not
price a Macaulay computation or imply that it is competitive with lookup.
For example, the full degree-28 space in 27 variables is already very large.

The measured implementation uses binary power chains. General polynomial or
rational/isogeny towers need their own precise projector certificates and
hypotheses; they are not covered by this statement. Independent review or a
formal proof remains required for signed theorem standing.
