# Independent review: exact projected-error packing

2026-10-30 UTC. Bounded written mathematical review; no experiments or shared
repository edits. The reviewed addendum is
`/workspace/audit/coded-packing-addendum-20261030.md`, SHA256
`7919a90f43bacf3881c070aef7891bb927c06a88f2401f841240f77ce9111d2d`.
The inspected Lean source is
`/workspace/audit/coded-packing-lean/CodedPacking.lean`, SHA256
`3a4e37130c769ee3cae11d5aef6710275d880066f18bcf7e142d008653059316`.

## Prior-art and scope check

Read Knill and Laflamme, *A Theory of Quantum Error-Correcting Codes*,
[arXiv:quant-ph/9604034v1](https://arxiv.org/pdf/quant-ph/9604034), Theorem3.5,
the Section5.2 discussion of dimension counting, and Theorem5.2. General
error-label syndrome vectors need not be independent; the paper explicitly
distinguishes the independent-error-image count from its general conditions.
The reviewed result imposes exact cross-annihilation and injective diagonal
maps, which supply the missing independence here. It is not a universal
quantum Hamming bound, nor a new coding-theory result.

## Linear-algebra lemma: correct

Let I be finite, and C,V vector spaces over one field k. Let

    E_i:C->V, F_j:V->C

be linear, with F_j E_i=0 whenever i!=j and every F_j E_j injective. Define

    T: C^I -> V,  T(v)=sum_i E_i(v_i).

If T(v)=T(w), applying F_j to both sides and using linearity and the exact
cross identities gives

    F_j E_j(v_j)=F_j E_j(w_j).

Matched-composition injectivity implies v_j=w_j. Since this holds for every
j, v=w and T is injective. Invertibility of a matched composition is not
needed. Each E_i is itself injective because F_i E_i is. The images E_i(C)
therefore form an internal direct sum.

For finite-dimensional C,V, monotonicity of dimension under the injective
linear map T and dim(C^I)=|I|dim(C) give exactly

    |I|dim_k(C) <= dim_k(V).

Empty I and zero-dimensional C are harmless. No inner product, positivity,
unitarity, or representation by a binary linear syndrome is used. The
result is valid over every field, including fields of characteristic two.

## Application and density consequence: correct with exact assumptions

For the projected-pair template, take E_i to be the ambient first-error map
restricted to C, and F_j to be projection to C after the ambient second-error
map. Exact off-diagonal projected products give the cross identities.
Injective matched projected products give the diagonal hypothesis. In the
specific syndrome construction the matched map is the paired-flip
permutation U_j, with identity included, so that hypothesis holds.

With u retained errors and the identity label, |I|=u+1. In a two-register
position space of L^2 basis vectors, a coordinate code supported on K
positions has dim_k(C)=K and dim_k(V)=L^2. Consequently

    (u+1)K <= L^2.

Thus this exact mismatch-filter template cannot retain a fixed positive
fraction of the position basis as u grows. It pays at least the factor
u+1 in ambient dimension per retained code direction, even if the code is
not constructed from linear parity checks. This statement concerns the
coefficient-field dimension of the position-vector spaces, not the binary
dimension of their index code.

If real edge positions are each assigned to one such coordinate packet,
the capacity bound yields at least (u+1)M/L^2 packets. The full-rank syndrome
construction with u=a and 2^r between a+1 and 2(a+1) attains capacity within
a factor two of this elementary upper bound.

The addendum's conditional substitution is also correct: granting the
proposed two-index mode bound A_i=O((u+1)M/D_i^2) and precision
p_i=O(MD_i/u), one gets

    A_i^(1/3)p_i^(2/3)=O(M/u^(1/3))

for growing u. This upper-envelope calculation does not certify a gain
over M/sqrt(u). Packet capacity alone is not a mode-count or runtime lower
bound, and the addendum correctly distinguishes these statements.

## What this result does not cover

It does not cover approximate or modulo-only cross identities, noninjective
matched maps, discarded code directions, or changing C between levels.
General degenerate quantum error-correcting codes need not satisfy these
strong exact hypotheses. No bound on those codes is inferred. Different
common kernels, weights, interpolations, nonlinear contractions or
cross-packet aggregation are not ruled out.

## Lean statement inspection and verdict

The inspected source defines the actual finite assembly linear map and
proves its application formula, recovery of each component, injectivity,
and the finite-dimensional inequality under precisely the hypotheses
above. Its diagonal assumption is injectivity of the composition F_j E_j;
the desired dimension inequality is the conclusion, not an assumption.
It does not assume a syndrome model or an algorithm. The finite-label
product dimension is handled by the standard finrank-of-pi theorem.

I inspected the source but did not independently rerun its build or fresh
checker in this review; those execution receipts are the author's separate
responsibility. The abstract inequality, projected-template corollary and
stated limits are mathematically sound. No correction to the addendum is
required. The original common-kernel and ECDLP objectives remain open.
