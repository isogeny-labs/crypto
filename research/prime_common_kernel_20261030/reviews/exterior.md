# Exterior-power deformation and the passive-axis obstruction

2026-10-30 UTC. Written mathematical review, plus finite exact C++/GMP
correctness checks and a narrowly scoped Lean project. This is not an ECDLP
algorithm, timing experiment, or performance result. No essential novelty is
claimed for second quantization, exterior algebra, or determinant identities.

## Sources checked before construction

- Pinned primary tensor manuscript, Oct25, AppendixD, especially D.4.1 and
  D.4.3-D.4.6: `/workspace/audit/tensor-primary/openai-oct25.txt`, PDF
  SHA256 `5b1612dc2eebb2cad5b4cbe62d1fed2982f01ab1908c26f70010a651487e8123`,
  [pinned source](https://raw.githubusercontent.com/openai/math/7ef5c0cc5ca4b0e072b7a050fca09ab24bc6fa3a/preprints/The-Exponential-Time-Hypothesis-is-False-October-25-2026/paper.pdf).
- The common-kernel problem and matched-level reviews in the consumer repo:
  `/workspace/crypto/problems/correlated-packet-common-kernel.md` and
  `/workspace/crypto/research/prime_tensor_variants_20261030/reviews/matched-level.md`.
- Epple, Fries, Hinrichsen, *Implementing the sine transform of fermionic modes
  as a tensor network*, arXiv:1705.10186, SectionV, equations22,23,26 and the
  subsequent discussion of swap costs. [Primary PDF](https://arxiv.org/pdf/1705.10186).
  This explicitly gives determinant occupation-basis entries, functorial
  composition, direct sums, and the signed fermion swap. These are existing
  constructions, not an original algorithmic contribution here.
- Bravyi, *Contraction of matchgate tensor networks on non-planar graphs*,
  arXiv:0801.2989, Section1 and Theorem1.
  [Primary PDF](https://arxiv.org/pdf/0801.2989). Its efficient contraction
  theorem requires matchgate identities at every vertex; arbitrary encoded
  Boolean tensors do not acquire those identities by adjoining a Gaussian
  kernel. Genus is also charged in that theorem.
- Lead-atlas index and literal searches for fermion/exterior/determinant/
  matchgate in the ECDLP lead files; a metadata-only atlas match is preserved
  in `exterior-kernel-atlas-20261030.log`. No specific match establishes no
  novelty priority. The SciPost 18,012 PDF was access-denied and is not used
  as evidence for a theorem here.

## 1. A real common deformation, distinct from the original Boolean lift

The source uses

    K(W)[Y,X] = product_x l_{Y_x}((WX)_x),
    l_0(z)=1-z, l_1(z)=z.

This is a Boolean multilinear-extension kernel. It has K(I)=I, but cannot
in general be treated as a representation of matrix multiplication. In
particular it is not the exterior-power construction below. Replacing K
changes off-origin values and must be analyzed from its own definition.

For an M-by-M matrix W over a commutative coefficient ring define

    Gamma(W)[Y,X] = det W[Y,X]   if |Y|=|X|,
                   0            otherwise,

with occupied-index lists in a fixed increasing order and empty determinant1.
The Cauchy-Binet formula gives Gamma(UV)=Gamma(U)Gamma(V). Also Gamma(I)=I.
Consequently, for the SAME frozen entrywise lifts A(Y),B(X),

    F_ext(T) = sum_{X,Y} A(Y) B(X) Gamma(W(T))[Y,X]

has the correct original contraction at T=0 whenever W(0)=I. This is a valid
alternative common polynomial. It is not equal to the source's F(T) in
general, and does not inherit the source's mode or child-network proof by
notation alone.

If deg W_xy<=d0, every minor has degree at most M*d0. For the original
Cauchy W this gives deg F_ext<=M*(2L-1), as before. For a two-axis product
matrix W1(T1) tensor W2(T2), each individual degree is bounded by the same
M*(2L-1). Restricting the physical slot set preserves W(0)=I. Integral local
charts for W give integral charts for all minors and F_ext because the
coefficient ring is closed under addition and multiplication. Thus the
existing Hermite recovery lemma applies under its same stated coefficient,
local-integrality and precision hypotheses.

An individual Gamma(W) entry can be evaluated in polynomially many ring
operations by a division-free determinant algorithm; arbitrary even pivots
must not be inverted in a 2-power coefficient ring. Source evaluation of
W entries is already polynomial. Exact rational coefficients and
normalizers have polynomial bit descriptions in M,L and the source's
precision/extension parameters. There is no extra 2-adic division introduced
by defining the determinant polynomial. This positive statement concerns
entry evaluation and recovery; the sum over arbitrary A,B remains a
contraction problem.

## 2. What functoriality does and does not solve

A factorization of W genuinely gives a factorization of Gamma(W). This is
an advantage over the source Boolean lift. It does not establish that the
intermediate factors stay within each recursively selected vertex group,
that the number of Boolean intermediate wires contracts, or that their
maximum degree meets Delta. A dense factor normally introduces M
intermediate bits; a two-axis composition does not shrink those by itself.

The one-particle sector of Gamma(W) is exactly W. Therefore an asserted
uniform absence of a passive one-axis channel must already be true for the
single-particle matrix. Determinant antisymmetry cannot make a nonzero
1-by-1 minor vanish. Larger-sector cancellations remain possible, but
arbitrary Boolean entry tensors may isolate a one-particle sector, so they
cannot be assumed uniformly.

Likewise, appending Gaussian/matchgate kernels does not make arbitrary
original vertices Gaussian. Existing matchgate contraction theorems cannot
be applied to the full encoded ECDLP network without a separate proof that
their vertex conditions hold.

There is a real exterior-power simplification for additive rank-one
updates: every minor of A+sum_j z_j*u_j*v_j^T is multiaffine in each z_j,
since repeating the same update in two selected columns makes those columns
proportional. Thus update indices become squarefree. With A0 possible
indices of each positive weight and budget p, however, the generating bound
is exp(theta*p)*product_w(1+exp(-theta*w))^A0. Its logarithm is at most
theta*p+A0/(exp(theta)-1), giving exp(O(sqrt(A0*p))) after optimization.
The full formal weighted subset family also has exp(Omega(sqrt(A0*p)))
admissible subsets in its dense regime. Squarefreeness alone therefore does
not improve the existing partition exponent in this counting argument.
This says nothing about nonzero coefficients or all contraction algorithms.
The independent derivation and its exact parameter qualifications are in
`/workspace/audit/exterior-counterreview-20261030.md`, Section2.

## 3. A dense global parity code does not eliminate the passive channel

One potential repair of per-bit repetition is the dense code

    S = {(x1,x2) in [0,L)^2 : parity(popcount(x1))=parity(popcount(x2))},
    L=2^a, a>=2.

This occupies exactly half the L-by-L slots and detects every individual
bit flip among the 2a address bits. That true binary-code statement does
not imply the required Cauchy locality: an integer difference may change
higher bits as well as the selected bit.

Here is an exact witness using the source's actual puncture polynomials:

    L=4, C(T)=product_{q=-4}^3(T-q), alpha=8,
    W_delta(T)=C(T)/(C'(0)*(T-delta)),
    t1=t2=1, x=(0,0), y=(2,1).

Both x,y belong to S. Their first-axis low bits agree, so this is a selected
first-axis crossing. The second-axis difference is exactly the local
translation1. Direct differentiation of the finite products gives

    C'(0)=-144, C'(1)=240, rho=C'(1)/C'(0)=-5/3,
    W_1(1)=-5/3,
    [z1] W_2(1+8*z1)=8*rho/(1-2)=40/3.

The product-kernel coefficient of z1^1 z2^0 is therefore

    (40/3)*(-5/3)=-200/9 !=0.

A bit-distance argument claiming the passive term is absent is false for
this actual Cauchy kernel and this actual dense code. The same coefficient
is a 1-by-1 minor of the exterior lift. This does not prove a universal
lower bound for common kernels, nor forbid a different weighted basis or
an algorithm aggregating such terms.

## 4. General passive residue rank for this specific code/kernel

The finite witness has a useful generalization. At t1=t2=1, write

    rho=C'(1)/C'(0)=-(L+1)/(L-1),  alpha=2^(a+1).

The displayed ratio follows either from factorial products or canceling
all common factors in the two derivatives; it is an odd 2-adic unit.
Consider only the first-z1 derivative entries whose first-axis low bits
agree (delta1=y1-x1 even) and whose second axis is exactly local
(delta2=y2-x2=1). The coefficient divided by alpha is

    rho^2/(1-delta1).

Its numerator and denominator are odd, hence its residue modulo2 is1.
All other entries of this scoped passive matrix are defined as zero.

Order rows and columns by the second coordinate and common first low bit.
For each x2=0,...,L-2 and q in{0,1}, one block has rows

    x1%2=q, parity(popcount(x1))=parity(popcount(x2)),

and columns

    y1%2=q, parity(popcount(y1))=parity(popcount(x2+1)).

Each set has L/4 elements: after fixing its low bit, one of the remaining
a-1 bits is uniquely determined by parity, and a>=2. Different block
indices use disjoint rows and disjoint columns. The residue matrix is
therefore a direct sum of exactly 2(L-1) nonempty all-ones matrices of size
L/4 by L/4, plus zero rows/columns. Its rank overF2 is exactly

    2(L-1).

This is a rank theorem for one normalized coefficient matrix of this
specific candidate. It shows that constant-density global parity has not
removed the passive channel: even its initial normalized crossing
coefficient has Omega(L) independent channels per L^2-slot packet. It is
not a runtime lower bound, not a count of nonzero final contraction terms,
and does not exclude code-aware global aggregation. The general rank
statement is written and independently reviewed, not Lean formalized.

## 5. Fermionic regrouping signs require an explicit charged representation

If a selected grouping is not contiguous in the fixed occupation basis,
passing from the original order to group order changes determinant signs.
For alternating modes a1,b1,...,an,bn, stable regrouping has phase

    chi(a,b)=(-1)^(sum_{j<k} b_j a_k).

The bilinear exponent matrix T_{j,k}=1[k>j] has F2 rank n-1. The matrix of
chi values across a-versus-b assignments has rational rank 2^(n-1), since
its distinct rows are distinct Walsh characters. Thus an unrestricted
separation chi=sum_{z=1}^R f_z(a)g_z(b) requires R>=2^(n-1). In this order,
a constant total-parity state does not suffice.

This issue can persist for an actually block-local one-particle operator,
not merely an artificial isolated shuffle. Let W cyclically permute the
a-modes and fix every b-mode. On its support Ya=P Xa,Yb=Xb, the joint
sorting phase is

    (-1)^(b^T T(P-I)a),

up to an a-only factor from the within-a determinant. Here im(P-I) is the
even-total-parity subspace, while ker T is the span of the first unit
vector, whose parity is odd. Therefore rank T(P-I)=n-1. The same
2^(n-1) separation rank follows for this block-local Gamma operator in the
alternating ordinary Boolean convention.

Scope matters. This is NOT proof that every ordering or pair-class method
fails, nor that this permutation is realized by the exact source Cauchy
node. At an exact source node W(t) is an order-preserving partial shift;
signs may cancel there. Ordering all ports by full pair-label classes can
make one coordinate's regrouping depend on only Darr class parities, which
could be cheap in the source parameter regime. But factoring the resulting
Gamma of the packet-block-diagonal matrix into ordinary packet tensors in
that class-major order involves another shuffle. No construction here
proves that those packet, group and old-vertex localities can all be achieved
with o(M) extra enumeration/wire cost. That is the actual missing lemma;
it is not replaced by the general high-rank example.

## 6. Validation scope and receipts

The independent reviewer `/root/kernel_counterreview` verified the general
passive-rank proof, including the exact rho ratio, parity block cardinalities
and derivative normalization. The same reviewer separately derived the
single-particle and generic shuffle issues. No contradiction to a specially
constructed common kernel was inferred.

C++/GMP source, protocol, frozen-input manifests and raw outputs are:

- `/workspace/audit/exterior-kernel-check-20261030.cpp`
- `/workspace/audit/exterior-kernel-protocol-20261030.md`
- `/workspace/audit/exterior-kernel-frozen-inputs-003-20261030.sha256`
- `/workspace/audit/exterior-kernel-run-003-20261030.log`

Native compiler: g++14.2.0, flags `-std=c++20 -O2 -Wall -Wextra -Werror
-pedantic`, linked GMP/GMPXX. The source computes the actual punctured
polynomials using finite products over the source grid and separately
computes derivative ratios, rather than inserting the expected coefficient.
It checks8 one-axis origins,8 retained code slots,64 product-origin pairs,
the exact rational coefficient, shuffle ranks n-1 for n=2,...,9, and
normalized passive ranks at L=4,8,16,32 respectively6,14,30,62. The four
passive matrices have respectively6,56,480,3968 retained odd-rational
entries. This is finite correctness evidence only. No host-isolation or
speed conclusion is drawn.

The first strict native build failed for misleading indentation; its log
and exact source are retained as build001/source001. After the formatting
fix build002 passed, and the prospectively frozen general-rank extension
build003 passed. No failed run was replaced or treated as evidence of a
mathematical obstruction. The input hash changed only with the described
source/protocol extension.

SHA256 source version003:
`2e59f29984d9eb91b4055198ea96ca5bd9d02f66f6853aab89840fa3d280bb66`.
SHA256 raw successful output:
`2597210a756a1cb7e1b6489556930b63bedaff5dad015d966f4affe3d20294e3`.

The finite rational Cauchy witness was checked in a separate pinned project
`/workspace/audit/exterior-witness-lean`. Seven named declarations passed
the default and explicit source builds, transitive axiom audit (zero
failing/zero trusted), shared pinned CryptoLean proof gate and actual
import-inclusive `leanchecker --fresh` replay, all exit0. Its exact proof
map and remaining gaps are recorded there, separate from this written
general analysis. No general rank, Gamma factorization, recursion, tensor
contraction, or ECDLP theorem is represented as Lean certified.

## Decision and discriminating next step

The exterior lift is a legitimate common deformation and earns a bounded
positive lemma: correct origin, controlled polynomial degree, integral
local charts, polynomial entry evaluation and genuine composition. It does
not currently earn an improved mode density, child network or ECDLP bound.
The dense global parity proposal specifically fails to remove the passive
one-axis channel, with exact witness and a written residue-rank family.

A next theorem attempt would need one explicitly defined ordered/graded
factorization that preserves the common deformation, permits packet and
vertex-group separation, bounds every sign-state/wire/degree cost, and
produces a genuinely smaller weighted mode family. Special Cauchy minor
identities or cancellations could matter, but would need to hold for the
actual full operator, including its one-particle sector. An abstract
functorial identity or binary code distance alone does not supply it.
