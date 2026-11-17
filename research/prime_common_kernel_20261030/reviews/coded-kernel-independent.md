# Independent review of the syndrome-coded operator kernel

2026-10-30 UTC. Written mathematical review, with no experiments or timing
claims. Reviewed `/workspace/audit/coded-operator-kernel-20261030.md`, including
the added weighted-mode consequence. The reviewed version before minor
wording/final-check updates had SHA256
`b7c3519a45cec8529c9223685fad0ebfd7b1dbcf0553b6f00655240406a1fa48`.
Primary framework checked: the pinned Oct25 manuscript Appendix D.4.3--D.4.6
and the current common-kernel problem. No new novelty or priority claim is
made by this review.

## Exact projection identity: passes

With H of rank r and distinct nonzero columns s_j, the map (x,y) -> H(x+y)
has rank r, so its zero fiber C has 2^(2a-r) elements. For each fixed x its
allowed partners y form x+ker(H), of size 2^(a-r). Including the identity
error label with syndrome zero makes all error-label syndromes distinct.

Applying E_j^(1) E_k^(2) to a vector in the zero-syndrome coordinate
subspace changes the syndrome by s_j+s_k. This vanishes exactly when j=k
in the binary construction. The two coordinate projectors therefore kill
every mismatched term of A(T) tensor A(T), leaving exactly

    W(T)=f_*(T)^2 I+sum_j f_j(T)^2 U_j.

This is a positive common-matrix identity, not merely common-origin data.
It does not extend without hypotheses to a general Cauchy operator family.
The original endpoint tensors remain arbitrary Boolean tensors.

One minor wording recommendation was sent to the author: existence of H
should explicitly select the r basis columns along with other distinct
nonzero columns. Alternatively, the first a positive binary integers do
include all powers 2^j for j<r because 2^(r-1)<=a. The phrase 'because
a>=r' alone does not specify why an arbitrary ordering's first a columns
span, though the required construction certainly exists.

## Lagrange charts and odd-node valuation: passes

The grid S is a complete residue system modulo alpha=2^(a+1). For each q,
the valuation of C0'(q) is the same. Consequently replacing C0'(0) by
C0'(q) changes the source's factored chart only by a two-adic unit. Local
integrality and charged coefficient evaluation are justified.

For an odd t and even q, the exact formula

    ell_q(t+alpha*z)
      =alpha*C0'(t)/(C0'(q)*(t-q))*z
         *product_(s!=t,q)(1+alpha*z/(t-s))

follows by separating its factor at t and evaluating the others' constants.
Every ratio alpha/(t-s) is integral and divisible by two. At z=1 the product
is a unit, and t-q is odd. Thus each such ell_q(t+alpha) has valuation
exactly a+1, with odd residue after dividing by 2^(a+1).

In particular f*=ell_0 has valuation exactly a+1. For every j>=1, the
selector's q set has 2^(a-j) elements, an even number, so its normalized
sum is even and its valuation is at least a+2. The exceptional q=-L
is assigned to f0 and does not affect these crossing-selector bounds.
At an active node the cardinal values also correctly give W(t)=U_i.

## Crossing block and truncated-ring mode consequence: passes

Fix first-register orientation bit zero. There are K/2 code slots on each
side, because the first-register orientations are uniform. U0 flips this
bit and is local with the stated opposite endpoint convention. The identity
and Uj for j>=1 preserve the bit and form the crossing block. Consequently,
with w=2a+2,

    B=2^w(cI+4D), c a two-adic unit, D integral.

The parenthesized matrix is invertible over the two-adic integers. The
Smith elementary-divisor orders of B are all w, including after an
unramified scalar extension. Unlike a rational-rank-only argument, this
also yields a necessary mode count modulo 2^p whenever p>w.

Here is an explicit proof of that last implication. Suppose B=UV modulo
2^p with U of size n-by-r and r<n=K/2. Lift U to a matrix over the DVR and
put that lift into Smith form by invertible row/column operations. Its last
n-r rows are zero. Applying its invertible row operation to B makes those
rows zero modulo 2^p. After dividing by 2^w, the corresponding rows of an
invertible matrix must then be zero modulo 2^(p-w), hence zero modulo two.
This contradicts invertibility modulo two. Thus a sum of rank-one modes
needs at least n terms. This argument does not try to invert a nonunit in
the truncated ring. If nilpotent coefficient variables are present,
specializing them to zero supplies the same necessary condition.

For p=w+1, a uniform constant number of modes per nonnegative pair (b1,b2)
with coefficient valuation at least b1+b2+2 provides at most

    O(sum_(s=2)^w(s-1))=O(a^2)

nonvanishing rank-one terms. But n=2^(2a-r-1) exceeds every fixed multiple
of a^2 as a grows. This disproves that particular per-packet weighted linear
mode template for this selector. Lower-weight cancellations cannot increase
the number of intermediate coordinates in a proposed rank-one factorization.
It proves no lower bound for nonlinear encodings, better kernels, arbitrary
contraction algorithms or ECDLP.

## Idealized density and precision envelope: passes with the stated scope

The exact capacity is K=L^2/2^r and a+1<=2^r<2(a+1), so even perfectly dense
feasible packets require an order-2^r factor in packet count relative to
unrestricted L^2 slots. This is a capacity observation, not an implemented
graph-packing theorem.

Under the explicitly *granted* bound A_i=O(2^r M/D_i^2) and precision
p_i=O(MD_i/u), the plane-partition envelope is

    exp(O(A_i^(1/3) p_i^(2/3)))
      =exp(O(M*(2^r)^(1/3)/u^(2/3))).

The inherited root schedule a=Theta(u) therefore gives
exp(O(M/u^(1/3))), which does not certify an improvement on
exp(O(M/sqrt(u))). This is an upper-envelope calculation under optimistic
hypotheses, not a runtime lower bound. The actual selector already fails
the narrower weighted linear-mode condition above.

The review correctly charges degree 4L-2 for W and M(4L-2) for the Boolean
deformation, and notes that a new packet vertex may have 2K rather than 2L
ports. No complete recursive invariant or runtime is asserted.

## Formalization scope and verdict

The written general projection identity is consistent with the described
six-declaration Lean bridge. I did not independently rebuild that separate
Lean project in this review, and do not certify its build from this written
inspection. The main note explicitly leaves code cardinality, all Lagrange
valuation and density/rank arguments outside its formalized scope.

No material mathematical defect found in the stated construction or its
scoped unsuccessful-mode conclusion. The common-origin matrix identity is
useful positive progress; its explicit selector does not supply an improved
contraction or ECDLP algorithm.
