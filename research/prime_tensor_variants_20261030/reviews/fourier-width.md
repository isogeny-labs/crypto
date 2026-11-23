# Fourier multiplication: small local degree does not give small dense contraction width

2026-10-30 UTC. Read-only mathematical/source review by `/root/fft_width_math`.
No solver, benchmark, scientific computation or new ECDLP algorithm was
implemented. The conclusion is a width bound for ordinary dense binary
contraction of the stated representation, not a lower bound for all
contraction algorithms or for discrete logarithms.

## Source actually read

Read Theorem3, Lemmas4-7, their proofs, the parameter choices and the
forward-transform wiring in sections2.3-2.4 of the pinned Bochum v0.2
preprint, printedpp7-10:

<https://raw.githubusercontent.com/bochum-crypto/subexponential-ecdl/1ee44e562b1b9a50ea339f3b8f81112b5a24b96f/subexponential-ecdl.pdf>.

The local PDF SHA256 is

    82e273629b437660607117d5e99f400141ed7aa78b25c6749b03651f42deaa11

The corresponding extracted source is
`/workspace/audit/tensor-primary/bochum-v02.txt`. This is a standard
balanced-cut argument specialized to the actual wiring, not a claimed new
lower-bound technique.

## 1. The actual complete bipartite word block

Use the source's notation for multiplying two d-bit integers:

    J = the least power of two with J^2 >=64(d+1),
    k=J/4, n=ceil(d/k), L=2^mu >=2n,
    L1=2^floor(mu/2), L2=2^ceil(mu/2), L1 L2=L.

Its first forward layer has one F_(j2) vertex for every j2<L2. Each such
vertex outputs every intermediate word y_(j2,t1), t1<L1. The next layer
has one G_(t1) vertex for every t1<L1, and it consumes all the words
y_(j2,t1), j2<L2. Thus the intermediate word interfaces form exactly

    K_(p,q), p=L2, q=L1,

with a canonical (J+1)-bit ring word on each pair. In the literal
uniform-word presentation this is J+1 distinct Boolean edges per pair.
The output word is used once in each of those two predicates; it is not a
single index of constant dimension merely because it is called a word.

For sufficiently large d, every first-layer column contains a genuine
nonconstant input limb. For example n>=9 implies L2<n, since L<4n and
L2<=sqrt(2L)<sqrt(8n)<n. This avoids relying on a padded all-zero column
in a small-parameter corner case. Asymptotically,

    J=Theta(sqrt(d)), L=Theta(sqrt(d)),
    L1,L2=Theta(d^(1/4)).

## 2. A rectangular balanced-cut proof with the same 4/9 constant

The parent argument for equal layer sizes is correct. Rectangular sizes
can retain the same constant by balancing appropriate vertex weights.

Give every vertex on the p-side weight q, every vertex on the q-side
weight p, and every other network vertex weight0. Total weight is 2pq.
Each leaf has at most half this total. In any rooted binary contraction
tree, there is a proper subtree with weight between one third and two
thirds of the total: take a deepest subtree whose weight exceeds two
thirds. It is not a leaf; both its children have weight at most two thirds,
and at least one has weight at least one third. For the asymptotic source
parameters, no single FFT leaf has one third of the weight, so the chosen
node is an actual intermediate contraction rather than just an input
tensor.

Let its leaf set contain u of the p-side vertices and v of the q-side
vertices. Put

    x=u/p, y=v/q, z=x+y.

Weighted balance says 2/3<=z<=4/3. The number of separated word pairs is

    B=u(q-v)+v(p-u)=pq(x+y-2xy).

Since 4xy<=z^2,

    B/(pq)>=z-z^2/2
            =1/2-(z-1)^2/2
            >=4/9.

This works for all positive rectangular sizes p,q; the source's ratio of
at most2 is not needed. An entirely integer certificate of the same
inequality uses W=qu+pv and A=qu-pv:

    18pq B-8(pq)^2
      =9A^2+[pq-3(W-pq)]*[pq+3(W-pq)] >=0.

Both bracketed factors are nonnegative under weighted balance. Dividing
by 2pq>0 gives 9B>=4pq.

The qualification about weighting matters. Reusing the 4/9 constant with
unweighted vertex balance for arbitrary rectangles is false. For
p=4,q=8,u=1,v=3, the subset has4 of12 vertices, but B=14 and
14/32=7/16<4/9. Weighted balancing selects a different cut and avoids this
incorrect inference.

## 3. Boolean width is Omega(d)

Every separated pair contributes J+1 distinct Boolean edge indices in the
literal graph. Consequently some intermediate has at least

    (4/9)(J+1)L =Omega(d)

Boolean boundary indices. Here L>=2n and kn>=d, with J=4k. Subdividing
each bit edge through a new degree-two equality vertex does not remove
the conclusion: whenever the original FFT endpoints are on opposite
sides, that distinct bit path has at least one crossing edge. The weights
of the added equality vertices are0. Other parts of the multiplication or
ECDLP network only add possible boundary indices.

The same asymptotic conclusion survives deleting only output bits that
are constant for every unconstrained gadget input, without further symbolic rewiring. For
n>=9, every input a_(j2), j2<L2, can range over all 2^k values of a full
limb while the other limbs are0. In this restriction,

    y_(j2,t1)=omega^(j2*t1) a_(j2).

The multiplier is a unit, so this word has at least2^k distinct canonical
values. At least k of its bit positions are therefore nonconstant.
Every pair retains at least k Boolean edges under constant-bit deletion.
The bound becomes

    (4/9)kL >=(8/9)d.

This argument does not cover identifying correlated bits across distinct
words, replacing predicates, or introducing a new symbolic factorization.
Those change the representation and need a separate uniform construction.

## 4. Exact scope of the consequence

In an ordinary binary contraction tree, an intermediate obtained from a
leaf subset S, summing all its internal indices, has one remaining index
for each edge of the cut delta(S). A dense table for it has
2^|delta(S)| entries. The proved cut therefore forces an intermediate
table of size 2^Omega(d) for this graph if those tables are materialized
densely. Local vertex degree O(d^(3/4)) alone cannot justify a
2^O(d^(3/4)) dense-contraction algorithm.

This fixed-graph model does not include slicing/conditioning indices,
input enumeration, support pruning or graph rewriting. Those are distinct
algorithms or representations. This argument does not lower-bound the
optimized total cost of every such scheme.

This is emphatically not a lower bound on computing the contraction by
all means. The proof does not lower-bound tensor rank, nonzero support,
an optimized symbolic representation or any implicit deformation algorithm.
It also does not count the number of independent unknown input bits as
equal to the number of exposed word-interface bits. The source's tensor
theorem deliberately uses a different implicit/deformation procedure;
the width argument neither refutes that theorem nor its stated
sub-exponential result.

For a concrete sanity check of this distinction, terminate all interface
bits of the isolated multiplication network with tautology predicates.
Lemma6 gives exactly one extension for each pair of d-bit inputs, so its
contraction is simply2^(2d), computable symbolically in polynomial bit
work. Its literal graph still contains the wide block above. Wide dense
intermediates therefore do not imply hardness of even that same isolated
contraction instance.

## 5. Constructive alternatives examined

1. **Compose the two transform layers symbolically.** This can avoid
   materializing their dense intermediate tensor. A single predicate for
   the complete transform, however, exposes Theta(d) bits. In an O(d)-edge
   network it no longer meets the sublinear-degree hypothesis used by D.1.
   Merely assigning an efficient entry evaluator does not restore that
   hypothesis.
2. **Use all stages of an ordinary FFT.** The source already notes the
   O(d log d) edge cost. This is a different graph, but the extra edge
   factor destroys the saving needed in the stated D.1-to-ECDLP balance.
   It is not a width or asymptotic improvement proved here.
3. **Evaluate the functional network after fixing its inputs.** This is
   polynomial for the multiplication gadget and is a valid symbolic
   shortcut for that task. In the ECDLP network the input assignments are
   unknown and constrained elsewhere; finding them is the unsolved search.
4. **Exploit a stronger uniform compressed representation.** Not ruled
   out by the cut argument, but no new such representation or improved
   contraction theorem was constructed in this review.

## Verification and formalization boundary

The source wiring and elementary inequalities above were checked on paper.
No empirical timings or finite-census result is asserted. No Lean or Lake
toolchain is configured in the inspected environment (`command -v` found
neither), and no checked Lean proof is supplied. A faithful formalization
would need the weighted binary-tree cut lemma and graph/path boundary
bridge as well as the algebraic inequality; an arithmetic identity alone
would not certify the full width statement.

The selected shortcut is closed in its stated dense fixed-representation
form. No global impossibility or new sub-exponential algorithm follows.
