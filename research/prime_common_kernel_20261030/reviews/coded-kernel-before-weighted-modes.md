# A syndrome-coded common kernel: exact identity, unsuccessful mode bound

2026-10-30 UTC. This is a bounded written analysis of one proposed common
kernel. It is not a novel ECDLP algorithm, performance experiment or general
impossibility theorem. The code projection identity below is positive; the
specific Lagrange selector then produces full-rank crossing blocks and does
not supply the requested moment density.

## Sources checked before choosing the construction

- Pinned Oct25 manuscript, AppendixD.4.3–D.4.6,
  `/workspace/audit/tensor-primary/openai-oct25.txt`: Boolean extension kernel,
  integral Cauchy charts, targeted linear modes, and immutable child-entry
  descriptions. In particular its mode saving concerns *linear forms in
  physical Boolean column variables*, not just the number of scalar
  coefficients in a matrix expression.
- Knill and Laflamme, *A Theory of Quantum Error-Correcting Codes*,
  [arXiv:quant-ph/9604034v1](https://arxiv.org/pdf/quant-ph/9604034), Sections2–3:
  syndrome subspaces and projected operator/error conditions. The elementary
  binary syndrome calculation used here is established coding algebra;
  applying projection is not by itself an algorithmic novelty claim.
- Existing common-kernel problem and matched-level reviews in
  `/workspace/crypto/problems/correlated-packet-common-kernel.md` and
  `research/prime_tensor_variants_20261030/reviews/matched-level.md`.
  The lead-atlas algebraic-transform corpus was checked; no direct matching
  entry was located in that bounded check. This is not a complete priority
  search or proof of novelty.

## 1. Exact syndrome projection identity

Let `a>=1`, `r=ceil(log2(a+1))`, and choose a full-row-rank binary
`r by a` matrix H whose columns `s_j` are distinct and nonzero. Such a
choice exists: the first a nonzero r-bit vectors include an r-vector basis,
because `a>=r`. Define

    C={(x,y) in F2^a x F2^a : H(x+y)=0},
    K=|C|=2^(2a-r).

Let P be the coordinate projector onto the span of the basis vectors indexed
by C, over the rational or two-adic coefficient ring. Let `E_j^(1)` flip bitj
of x, `E_j^(2)` flip bitj of y, and let `E_*^(1)=E_*^(2)=I` have syndrome0.
Distinct symbols `*,0,...,a-1` have distinct syndromes. For any basis state in
C the operator `E_j^(1) E_k^(2)` changes its syndrome by `s_j+s_k`.
Consequently

    P E_j^(1) E_k^(2) P=0                 if j!=k,
    P E_j^(1) E_j^(2) P=U_j               if j=k,

where Uj is the paired-flip permutation of C, and U*=I. This is an *exact*
matrix identity, not an approximate error-detection assertion.

For scalar polynomials fj and

    A(T)=f_*(T)I+sum_j f_j(T)X_j,

the same identity gives the genuine common deformation

    W(T)=(P(A(T) tensor A(T))P)|C
        =f_*(T)^2 I +sum_j f_j(T)^2 U_j.                 (1)

In particular, `f_*(0)=1`, `f_j(0)=0` imply `W(0)=I` without a change of
deformation between levels. This removes the simple passive-axis terms
`f_j f_k` for j!=k for this carefully restricted operator family. It does
not assert a similar identity for general Cauchy translation matrices.

For every first-register orientation x, its partners are `y=x+z` with
`z in ker H`. Thus each of the `2^a` orientation vectors occurs exactly
`2^(a-r)` times in C. Paired flip Ui changes exactly orientation biti. A
packet scheme with row/column orientation conventions opposite in each
coordinate therefore has the source's coordinate-locality at any node
where W=Ui. Arbitrary original vertex tensors have not been transformed
or assumed to be fermionic here.

## 2. An explicit common polynomial with integral charts

Put `L=2^a`, `S={-L,...,L-1}`, `alpha=2^(a+1)`, and

    C0(T)=product_(q in S)(T-q),
    ell_q(T)=C0(T)/(C0'(q)(T-q)).

These ordinary Lagrange cardinal polynomials have degree `2L-1`. Their
coefficients are rational; no claim of global coefficientwise 2-integrality
is needed. Every chart `ell_q(t+alpha*z)`, t in S, is coefficientwise
2-integral. Indeed, all `C0'(q)` have the same two-adic valuation, and the
source's D.4 factor-cancellation proof applies with denominator `C0'(q)`
instead of `C0'(0)`; their ratio is a two-adic unit. The products involve
O(L) rational linear factors and O(L log L)-bit denominator descriptions.

Choose

    f_*=ell_0,
    f_j=sum_(q in S, v2(q)=j) ell_q    for0<=j<a,

and add `ell_(-L)` to f0. The exceptional q=-L has valuationa and is not an
active node when the source retains only levels i<u<=a. This assignment
also ensures `f_*+sum f_j=1`. At zero, f*=1 and the others vanish. At every
active nonzero node t of leveli, fi=1 and all other selectors are0. Thus
W(t)=Ui, as required for the coordinate-label locality just described.

The entries of W have degree at most `4L-2`. In the original Boolean
multilinear-extension contraction, the resulting deformation has degree
at most `M(4L-2)`. Its local charts are integral by (1). Individual entries
can be calculated in polynomially many operations in L and the requested
precision, using the same explicit denominator cancellation. These positive
facts do not establish the desired mode expansion. A recursive packet
vertex would have up to2K ports, so K rather than L must be charged against
the degree and entry-evaluation invariants.

## 3. Exact crossing-block failure for this selector

Take `a>=2`, an odd node t in S, and evaluate at `T=t+alpha`. Work over the
two-adic integers after the charged odd-denominator cancellation. Write
`v=v2`. For an even q in S the cardinal polynomial has the expansion

    ell_q(t+alpha*z)
       = [alpha*C0'(t)/(C0'(q)*(t-q))] z
           *product_(s in S, s!=t,q)(1+alpha*z/(t-s)).       (2)

Every factor in the last product is1 modulo2 because `v(t-s)<=a`. The
bracket has valuation exactly `a+1`: t-q is odd and the derivative ratio
is a unit. Therefore

    v(f_*(t+alpha))=a+1.

For j>=1 the set of q with `v(q)=j` has the even cardinality `2^(a-j)`.
Every corresponding bracket in (2), divided by `2^(a+1)`, is an odd
unit. Summing an even number of them gives

    v(f_j(t+alpha))>=a+2       for every j>=1.              (3)

The selector assigned the exceptional -L was f0, which is local at odd
leveli=0 and is excluded from (3).

For a packet whose coordinate0 symbols differ, cross-group entries are
those with equal first-register biti=0. U0 flips that bit and is local;
I and Uj forj>=1 preserve it and are crossing. Restrict to either fixed
orientation0 side, containing `K/2` slots. Its crossing block is

    B=f_*^2 I_(K/2)+sum_(j>=1) f_j^2 (U_j)|side.

By (2)–(3), with `w=2a+2`,

    B=2^w (c I+4D),

where c is a two-adic unit and D is integral. Thus `cI+4D` is invertible
over Z2. All `K/2` Smith elementary-divisor orders of B are exactly w.
In particular this is a full-rank crossing block over Q2, with a nonzero
Smith factor in every direction modulo any precision p>w.

This gives a specific obstruction to a direct linear-form moment ansatz.
Over `Z/2^p`, p>w, expressing B as a sum of rank-one column-form modes
requires at least `K/2` modes in total: otherwise its factorization through
fewer generators has fewer nonzero Smith factors. Equivalently, put the
left factor in Smith normal form; a matrix factoring through fewer than
K/2 intermediate coordinates cannot have an image equal to
`2^w R^(K/2)`. The same conclusion holds if the coefficients are extended
unramifiedly. It does not require claiming rational rank is an invariant
over an arbitrary truncated ring containing nilpotent variables; reducing
those variables to zero already supplies this necessary condition.

For example, choose `a=3`, syndrome columns1,2,3, `L=8`, `t=1`, `T=17`.
Then K=16, each side has8 slots, f* has valuation4, and the other crossing
selectors have valuation at least5. Thus B has eight Smith factors of
order8. These are symbolic specialization consequences; no timing or
scaling result is asserted.

This does not rule out arbitrary nonlinear encodings of column assignments,
aggregate cancellations in the contraction, or an entirely different
polynomial/kernel. It closes only the inference from (1)'s few scalar
coefficients to a few targeted *linear forms*: every Uj is a permutation
of many independently assigned Boolean port variables.

## 4. The idealized density is already insufficient in the inherited schedule

Even before the full-rank issue, the code has capacity

    K=L^2/2^r,            a+1<=2^r<2(a+1).

Consequently a dense-feasible packet count is order `2^r M/L^2`, rather
than M/L². If one optimistically granted two-index mode density
`A_i=O(2^r M/D_i^2)` and precision `p_i=O(MD_i/u)`, plane-partition counting
would provide only the conditional envelope

    exp(O(M*(2^r)^(1/3)/u^(2/3))).

For the inherited root schedule `a=Theta(u)`, this becomes
`exp(O(M/u^(1/3)))`, weaker than the existing square-root-logarithmic
envelope. This is a calculation for that granted mode bound and schedule,
not an unconditional runtime lower bound. The actual selector in Section3
does not even establish that optimistic bound. Another choice of code,
selectors, weights or schedule would require a separate complete analysis.

## 5. Formalization status and exact remaining work

Separate pinned Lean project:
`/workspace/audit/coded-projection-lean/`, Leanv4.35.0, shared CryptoLean
`148f5dc389b341401e06d76abe7783c4ccf42f2e`, Mathlib
`cfc55fc58714a59dc06e6e79edc6d1d95140d3c2`.

Six named declarations prove the actual finite-index matrix projector bridge,
zero/nonzero-syndrome translation laws, matched/mismatched-pair consequences,
and weighted double-sum diagonalization. They are general over additive
groups and a coefficient ring. For paired errors their hypotheses explicitly
require an injective syndrome map on labels and opposite error syndromes;
in the binary construction, opposite syndromes are identical. The finite
binary-code cardinality, Lagrange charts, valuation proof and full-rank/density
arguments above have readable proofs here but are not thereby Lean proved.
No theorem assumes the desired algorithm or mode bound.

Initial builds exposed unresolved scalar-ring implicit arguments and a missing
explicit matrix-multiplication import; their failed source snapshots/logs
are retained. Final `lake build` and the shared axiom auditor pass, with
only `propext`, `Classical.choice` and `Quot.sound`; six named declarations
plus three generated congruence lemmas are audited. Fresh replay is being
tracked separately; do not call the complete review formally verified.

## Result and next discriminating question

The syndrome-code projector does give an exact shared-origin common kernel
and genuinely kills mismatched operator pairs in the specified small
operator algebra. Its explicit integral-chart selector leaves many full-rank
crossing directions, and its code density also loses the intended saving
under the inherited schedule. There is no improved contraction or ECDLP
algorithm from this construction.

A successor would have to specify a kernel/error family with a charged,
bounded-rank crossing expansion at the actual interpolation precisions,
without paying an equivalent density, degree or entry-description cost.
Replacing full-rank permutation channels by the phrase "one operator mode"
does not meet that acceptance condition.
