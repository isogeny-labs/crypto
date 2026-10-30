# Common-kernel review: Boolean gauges, physical port maps and charged costs

2026-10-30 UTC. This is a mathematical follow-up to the common-kernel
problem, not a contraction algorithm, benchmark or ECDLP solution. The
explicit finite counterexample has a faithful Lean proof; the general
gauge/invariant and cost arguments below remain written proofs.

## Sources checked before selecting the argument

- The repository problem record
  `/workspace/crypto/problems/correlated-packet-common-kernel.md` and
  `research/prime_tensor_variants_20261030/reviews/matched-level.md`.
- The pinned Oct25 primary manuscript in
  `/workspace/audit/tensor-primary/openai-oct25.txt`, D.4.1–D.4.7,
  especially the physical-variable kernel (D.8), its actual affine factors,
  immutable creation precision, and the entry-description argument.
  The source PDF and its pinned provenance remain in the original study.
- Local atlas metadata search over
  `/workspace/crypto-autoresearcher/knowledge/leads/ecdlp/` for basis,
  tensor, kernel, gauge, Fourier and Cauchy. This did not locate a
  dedicated common-kernel gauge construction. That is a bounded search,
  not a novelty finding or an exhaustive history review.
- Al-Bashabsheh and Mao, *Normal Factor Graphs and Holographic
  Transformations*, arXiv:1004.3833v3 (2011), §2.3–2.4 and Theorem1:
  <https://arxiv.org/pdf/1004.3833>. Its inverse finite-index transformations
  give the relevant existing framework. Cai–Fu–Guo–Williams, *FKT is Not
  Universal*, Theorem2.2, provides an additional statement of the Boolean
  Holant identity: <https://doi.org/10.1007/s00224-021-10032-1>.

The algebra below distinguishes that established gauge identity from an
invalid inference about the special physical-port deformation. It makes
no priority claim for either the standard identity or this elementary
counterexample.

## 1. A physical-coordinate substitution is not a general Boolean gauge

For an n-by-n matrix W over a commutative coefficient ring define the
2^n-by-2^n matrix

    K(W)[Y,X] = product_j ell_(Y_j)((WX)_j),
    ell_0(z)=1-z, ell_1(z)=z, X,Y in {0,1}^n.

This is exactly the Boolean-index matrix used by the manuscript's
multilinear-extension deformation. Its contraction against arbitrary
row and column tensors is bilinear in those tensors. It is not a
representation W -> K(W) of general physical-coordinate maps.

Take integer matrices

    W = [[1,1],[0,1]],   V = [[1,0],[1,1]].

Index the Boolean cube in order 00,01,10,11, first listed bit first
coordinate. We have V(1,1)=(1,2), hence

    K(V)[-,11]=(0,0,-1,2)^T.

The last row of K(W) has values K(W)[11,10]=0 and K(W)[11,11]=2.
Therefore

    (K(W)K(V))[11,11]=4.

But WV=[[2,1],[1,1]], so WV(1,1)=(3,2) and

    K(WV)[11,11]=6.

The difference is nonzero over the integers and modulo4. It disappears
modulo2, which would be an insufficient control for the manuscript's
growing-precision algorithm. Negative intermediate weights are permitted
in the coefficient ring; they are not probabilities.

More generally, summation over an intermediate Boolean assignment
multilinearizes each positive power Z_j^k to Z_j. Multiplication of the
physical matrices does not. For distinct output rows r,s and z=VX,
the difference in the corresponding second moment is

    sum_j W[r,j]*W[s,j]*z_j*(1-z_j).

Thus the shortcut

    K(P W P^(-1)) = K(P) K(W) K(P)^(-1)

cannot be assumed. K(P^(-1)) is not generally the inverse of K(P), either.
A consistent permutation of actual Boolean indices is valid; that
restricted renaming does not justify arbitrary linear port mixing.
The displayed W,V are a counterexample to unrestricted functoriality,
not a proof that every special Cauchy matrix or every useful gauge fails.

## 2. True gauges preserve one common family pointwise

Let H(T) denote the full Boolean-index kernel tensor, possibly K(W(T)).
For each original vertex v with d_v ports, its index space E_v has
dimension 2^(d_v). Choose an invertible operator G_v on that space.
Apply the tensor product G of these operators to H, and apply the
inverse dual operators to its attached tensors. Every inverse pair
cancels in the complete contraction, so the value is exactly unchanged
for every choice of original tensors and every T.

The G_v may depend on the interpolation node or level. If the complete
operators and inverse changes are actually included, this gives a
genuine common-family identity pointwise, stronger than merely sharing
the origin. However, the transformed object is the actual Boolean-index
operator G H. It is not automatically K of a newly named physical
matrix. Nor does the identity prove that this transformed object has
the desired targeted two-axis mode representation.

Independent one-bit 2-by-2 gauges are a useful compact subclass:

    G_v = tensor product_(e incident to v) H_e.

A general linear map on the d_v-dimensional list of physical variables
is a different object from either G_v or this tensor product. Confusing
these spaces is precisely the failed shortcut in §1.

## 3. Exact invariants for group-preserving integral gauges

Fix any partition of the Boolean indices into groups, and flatten H
across any union of those groups versus its complement. Suppose each
G_v acts wholly inside one side of this cut. The transformed flattening
has the form

    M' = A M B

with A,B invertible over the coefficient ring R. For every k, let I_k(M)
be the ideal generated by its k-by-k minors. Cauchy–Binet expresses every
k-minor of A M B as an R-linear combination of the k-minors of M. This
shows I_k(M') is contained in I_k(M). Applying the same argument using
A^(-1) and B^(-1) gives the reverse containment. Consequently

    I_k(M')=I_k(M) for every k.

This argument holds over every commutative ring, including the finite
precision ring with the source's nilpotent V variable, and over a
truncated jet ring. Over a field it preserves flattening rank. Over
Z/2^p it also preserves the truncated2-adic content of each minors
ideal. A Smith form is not assumed over the general ring with V.

An integral invertible gauge confined to the chosen groups therefore
cannot increase the common divisibility of all k-minors or lower the
intrinsic flattening rank. These invariants do not by themselves settle
the desired mode-density count. They leave room for a better explicit
basis, cancellation, aggregation, a different cut or a transformation
that mixes groups and charges the resulting entry support.

This is a scoped statement about true group-preserving gauges, not a
lower bound on all contraction algorithms or common kernels.

## 4. One-step entry costs fit; recursive preservation needs proof

For a d-port Boolean tensor f and independent one-bit gauges H_e,
a transformed entry is

    f'(a)=sum_(b in {0,1}^d) f(b) product_e H_e[b_e,a_e].

It uses2^d old-entry calls, polynomial extra arithmetic per assignment,
and polynomial workspace. For d<=Delta=O(s^0.9), one such transformation
fits the source's exp(O(s^0.9)) entry allowance, assuming uniform
polynomial-size gauge data and efficient scalar evaluation. It leaves
the original vertex arity unchanged. Two-by-two inverses are cheap
when their determinants are units at the requested precision.

For a general dense2^d-by-2^d vertex gauge, both the entry oracle and the
inverse-entry oracle must be supplied compactly. Its full table cannot
be stored in polynomial space. An inverse existing over a fraction
field is not enough: even denominators require an explicit precision
debit and can invalidate the ring-level invariant above.

One-step costs do not automatically certify a recursively composed
entry evaluator. Repeated transformations and creation-time canonical
lifts must be tracked together. Canonical lifting does not commute
with additions or products. The actual gauge is applied to the same
immutable lifted entries defining the parent F, reduced at the required
node precision; each new complete tensor entry is then tagged and
canonically lifted according to the source protocol. Re-evaluating
older tensor code at a larger creation precision changes the fixed
deformation and cannot silently replace this process.

## 5. Exact remaining constructive obligation

A useful next lemma would explicitly give, for each active i,t, local
finite-index gauges G_(i,t), with integral inverses and charged compact
evaluation, such that the ACTUAL transformed Boolean kernel admits

- O(M/D_i^2) targeted mode families for each pair (b1,b2),
- divisibility at least b1+b2+2 in those coefficients,
- child shrink, degree and immutable-entry invariants compatible with
  the same common deformation and the full recursion.

Ordinary Holant invariance gives the common-value identity but none of
these density or recursive-cost conclusions. The physical-coordinate
functoriality shortcut is false. No construction meeting the displayed
conditions has been found in this review.

## Formal scope and independent check

`/workspace/audit/common-kernel-basis-lean/GaugeCounterexample.lean`
formalizes the exact convention above and proves three named results:

- `composed_kernel_entry`: the first entry is4;
- `kernel_product_entry`: the second is6;
- `kernel_not_multiplicative`: the two whole matrices differ.

The pinned build and fresh replay receipts accompany that project.
The general gauge identity, determinantal-ideal proof and cost bounds
are written-only here. The independent mathematical review is
`/workspace/audit/gauge-countercheck-20261030.md`. No timing or performance
experiment was run, and no new ECDLP algorithm is established.
