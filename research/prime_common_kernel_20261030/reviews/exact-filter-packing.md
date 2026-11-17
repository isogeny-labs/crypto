# Exact mismatch filtering pays a syndrome-packing dimension factor

2026-10-30 UTC. A standalone versioned addendum; no frozen package is changed.
This elementary error-packing lemma is not a new algorithm or runtime bound.

## Prior art

Knill and Laflamme, *A Theory of Quantum Error-Correcting Codes*,
[arXiv:quant-ph/9604034v1](https://arxiv.org/pdf/quant-ph/9604034),
Theorem3.5, separates logical information from an error-syndrome space.
Section5.2 discusses the Hamming dimension count and explains why counting
every error label as an independent syndrome fails for general degenerate
codes. Here independence is proved from stronger exact cross-annihilation
and injective-diagonal hypotheses. This is standard linear algebra applied
to the proposed template; no priority claim is made.

## Exact lemma and proof

Let k be a field, C,V vector spaces, and I a finite label set. Suppose
linear maps Ei:C->V and Rj:V->C satisfy

    Rj Ei=0 for i!=j; every Uj:=Rj Ej is injective.       (1)

Then T:direct_sum_i C->V, T((ci))=sum_i Ei ci, is injective. Applying Rj
to the sum gives Rj T((ci))=Uj cj. If the sum is zero, injectivity of Uj
gives cj=0 for everyj. Equivalently the subspaces Ei(C) form an internal
direct sum in V, with each Ei injective. When C,V are finite-dimensional,

    |I| dim_k C <= dim_k V.                             (2)

This includes emptyI and zero-dimensionalC. There is no inner-product,
positivity, orthogonality, unitarity or syndrome-linearity assumption.
Invertibility of each Uj is a sufficient stronger hypothesis.

## Application to the exact projected-pair template

Let C be one fixed code subspace of V and P:V->C a linear projection.
Suppose ambient operators Fi,Ei simultaneously satisfy

    P Fj Ei|C=0 if i!=j;
    P Fi Ei|C:C->C is injective for everyi.              (3)

Set Rj=P Fj and restrict Ei to C. These are exactly (1), so (2) holds.
With identity plusu retained level labels, `(u+1)dimC<=dimV`.

For the two-register construction V is the span overk of L² position
basis vectors; C is the span of K admissible position basis vectors.
Thus dimV=L² and dimC=K, distinct from the binary dimension of the index
code itself. Consequently

    K <= L²/(u+1).                                     (4)

The earlier full-rankH construction hasalabels plusidentity and
K=L²/2^r, witha+1<=2^r<2(a+1). Foru=a it is within2 of this bound.
The density factor is intrinsic to the stated exact mismatch-filter and
injective-matching template, rather than merely a feature of linear parity
checks. It is not intrinsic to every possible common kernel.

If M real edge positions are uniquely assigned to such packets, (4)
requires at least(u+1)M/L² packets. This alone does not force a mode count
or a contraction-time lower bound. In the optimistic packet-by-packet
template, saturating this density and granting

    Ai=O((u+1)M/Di²), pi=O(M Di/u)

would yield the conditional plane-partition envelope

    exp(O(Ai^(1/3) pi^(2/3)))=exp(O(M/u^(1/3))).

This is weaker than the source's existing square-root saving. It is a
cost substitution under a granted bound, not a measured runtime or an
unconditional lower bound. Cross-packet aggregation, different weights,
another interpolation schedule or a genuinely different kernel can change
the counting problem. The earlier explicit selector independently fails
its hoped-for low-rank moment expansion as well.

## Exact scope limits

- Approximate or truncated cross-annihilation cannot silently replace (1)
  or(3). The theorem is over a field with exact linear identities. A
  ring/module analogue requires its own torsion and precision proof.
- Singular matchedUi, discarded code directions, or different code subspaces
  at different levels are outside the stated assumptions.
- General degenerate quantum codes need not have independent error-label
  images; this is the qualification in the primary source.
- A common deformation with another factorization need not arise from(3).

This closes only an exact mismatch-filter proposal for avoiding its density
cost, and leaves the common-kernel research problem and original ECDLP
goal open.

## Formalization

Separate new project `/workspace/audit/coded-packing-lean/`, with existing
pins Lean4.35.0, CryptoLean148f5dc389b341401e06d76abe7783c4ccf42f2e,
Mathlibcfc55fc58714a59dc06e6e79edc6d1d95140d3c2.

| Written statement | Lean declaration | Exact scope |
| --- | --- | --- |
| T is the sum of error images | `CodedPacking.assemble_apply` | Finite labels, k-vector spaces |
| Component recovery | `CodedPacking.recover_component` | Exact cross-annihilation |
| T is injective | `CodedPacking.assemble_injective` | Additionally all diagonal compositions injective |
| Full dimension bound(2) | `CodedPacking.packing_dimension_bound` | Finite-dimensional C,V, exact hypotheses(1) |

The full abstract dimension inequality is formalized, rather than a finite
arithmetic instance. Its projected-operator interpretation and asymptotic
substitution are readable corollaries, not silently included in the Lean
statement. Initial failed sources/build logs are retained. Verification
receipts are recorded separately from this readable proof.
