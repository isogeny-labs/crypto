# Division-polynomial and windowed scalar-verification route

2026-10-30 UTC. Pure mathematical analysis, with no code execution, timing,
contraction or discrete-log computation. No original algorithm or novelty
claim is established. The useful constructive result below is a smaller
windowed verification encoding; its bound still does not certify
sub-exponential time in the field bit length.

## Sources and scope

Read the pinned Bochum v0.2 text at
`/workspace/audit/tensor-primary/bochum-v02.txt`, Theorems2-4,
Lemma8/Corollary1 and Corollary2. The underlying D.1 hypothesis was read in
`/workspace/audit/tensor-primary/openai-oct25.txt`: Boolean edges, global
parameter s, maximum degree at most ceil(s^(9/10)), charged implicit
predicate evaluation, and time poly(s+L)*2^O(s/sqrt(log s)).

For the exact net identities, retrieved and read Katherine E. Stange,
*Elliptic Nets and Elliptic Curves*, arXiv0710.1316v4,12September2010:

- Theorem2.2, equations(6)-(7), printedp7;
- Lemma4.2, equation(17), printedp21;
- Corollary5.2 and its appropriate-point conditions, printedpp24-25.

Primary PDF: <https://export.arxiv.org/pdf/0710.1316>, retrieved bytes
SHA256 `de00cacd1b574ba44d13a8e3ac00d9fb4b019972af2f105acce178d30f8a6030`.
Local PDF/text: `/workspace/audit/division-poly-sources/stange-nets.*`.
The initial arxiv.org request returned502; export.arxiv.org succeeded.
The author homepage was also retrieved, but no author-page theorem was used.

Repository context read: `LEAD-EXO-elliptic-net-eds`, `KN-FIND-002`, and
the P1540 translated-pole gate. The gate itself is marked unreviewed and
its pole-count closure is restricted to particular annihilators. I do not
use its historical timings or a generic-model classification to prove a
lower bound for the coordinate-sensitive tensor constructions here.

## 1. What a division-polynomial terminal condition actually supplies

Let E/Fp have known odd prime whole order r, p>3, with P generating it.
Write b=ceil(log2 p) and B=ceil(log2 r)=b+O(1). Treat Q=O directly and
restrict candidate scalars to1<=k<r. Small exceptional groups can be
handled separately; take r>3 below.

Put d_n=Psi_n(P), normalized by d_1=1. Since P has odd order,
d_2=2y(P) is nonzero. Stange's rank-one recurrences give

    d_(2n+1)=d_(n+2)*d_n^3-d_(n-1)*d_(n+1)^3,
    d_(2n)=(d_n/d_2)
            *(d_(n+2)*d_(n-1)^2-d_(n-2)*d_(n+1)^2).

Only the fixed public d_2 is divided by here. Intermediate zero values
therefore do not require a hidden nonvanishing promise.

Specializing Lemma4.2 to v=k,w=1 gives

    x([k]P)=x(P)-d_(k+1)*d_(k-1)/d_k^2.

By Corollary5.2, d_k is nonzero in the specified range. Thus the scalar
test

    (x(P)-x(Q))*d_k^2-d_(k+1)*d_(k-1)=0

is exact for x([k]P)=x(Q). It admits k and -k modulo r; checking [k]P
against Q outside the contraction selects the sign with polynomial work.
Alternatively encode the full point relation. No target logarithm is
needed to evaluate either test on a proposed k.

But k is a sequence index here, not the polynomial's x-indeterminate.
The formulas do not construct a low-degree polynomial in the unknown
integer index k. Finding the unique relation index of W(n,1) is likewise
not supplied by fast evaluation of a rank-two net.

## 2. The ordinary recurrence encoding is quadratic in b wires

An explicit constant-width evaluator exists. Maintain the eight terms

    d_(n-3),d_(n-2),...,d_(n+4).

Both child windows for2n and2n+1 depend only on that same parent window:
the smallest needed parent index is n-3 and the largest is n+4.
The displayed recurrences and d_(-j)=-d_j handle them with a fixed
number of field operations. A constant base window is computable from
the public curve/P and d_2 inverse. A scalar bit selects one of the two
child windows; evaluating both candidate formulas and selecting the
result uses only a constant number of additional field constraints.

There are Theta(B) such binary stages. The literal trace exposes a
constant number of canonical b-bit field words per stage. Using the
Bochum O(b)-edge gadgets per scalar field constraint gives O(Bb)=O(b^2)
edges, with Theta(Bb) explicit state-word ports in this representation.
This is not a lower bound on every arithmetic circuit for the relation.

A conventional addition chain for a scalar near2^B needs at leastB-O(1)
steps, since one addition can at most double its largest constructed
positive index. That observation applies to an addition-chain primitive;
it does not prove a general circuit lower bound. Treating a large-index
division map as one implicit primitive leads instead to the degree issue
in the next section.

Dense expansion is a different, also insufficient strategy. For odd
indices prime to the characteristic, Psi_k has x-degree(k^2-1)/2.
A dense coefficient array for a scalar of sizeTheta(p) has exponentially
many slots in b. This says nothing against a short straight-line
evaluation program: the recurrence above already provides one.

## 3. One implicit predicate violates the useful degree regime

A central predicate can check Q=[k]P from the B scalar bits in polynomial
time. A star with that vertex and tautology leaves has only B edges and
polynomial description size. Its central vertex nevertheless has degreeB.
To apply D.1, one must choose the global parameter at least

    s=Omega(B^(10/9)).

This can be done by inflating the theorem parameter without physically
adding dummy edges, or by padding the graph. Either way the theorem's
stated bound becomes2^O(B^(10/9)/sqrt(log B)), which does not establish
exp(o(B)). This is a failure of the stated upper bound to certify the
goal, not a runtime lower bound for the predicate or a proof that every
algorithm on the star is slow.

The full star contraction is simply1 for a valid target because the
canonical scalar is unique. Recovering its bits needs prefix-conditioned
queries. Those prefix counts are precisely the information missing from
the known total count.

## 4. A constructible windowed encoding: O(b^(20/19)) edges

One can improve the literal O(b^2) size without a precomputed lookup
table, though not enough for sub-exponential time under D.1.

Choose m windows and w=ceil(B/m). For j=0,...,m-1 let
P_j=[2^(jw)]P, computed cold from P with O(B) doublings in total. Write

    k=sum_j k_j*2^(jw),  0<=k_j<2^w.

Fix unused top bits to zero and check the canonical range k<r with a
linear-size comparator. Introduce a canonical point word Z_j for
[k_j]P_j. Its2b+1 bits are split into h=ceil((2b+1)/w) chunks.

For each chunk define an implicit Boolean predicate on all w bits of
k_j and at most w output bits. It computes [k_j]P_j by ordinary scalar
multiplication, then checks the requested output chunk. Its evaluation
time is polynomial in b, its description is polynomial, and its arity
is at most2w. No target logarithm, truth table or solved-instance advice
appears in that description.

The scalar bits are copied to the h chunk predicates with equality
vertices/trees. This costs O(wh)=O(b+w) edges per window. Every output
bit is forced, so the chunk representation has exactly one extension for
each scalar window. The point-sum relation sum_j Z_j=Q uses Corollary2
and O(mb) edges. Joining the canonical words preserves uniqueness.
Overall the construction has

    edges O(mb),
    max degree O(b/m+b^(3/4)+m).

Public setup and every implicit predicate evaluation are polynomial in b.
Point identities are permitted when a window scalar is zero. The final
range check prevents the redundant scalar representation modulo r.

Take m=ceil(b^(1/19)). Then w=Theta(b^(18/19)); the graph has
O(b^(20/19)) edges and maximum degree O(b^(18/19)). By choosing the
constant in D.1's global parameter large enough, the9/10 degree cap is
met because(20/19)*(9/10)=18/19. Bipartite subdivision only changes
constants. Witness self-reduction uses polynomially many calls, giving
the certified bound

    2^O(b^(20/19)/sqrt(log b)).

That is still not exp(o(b)). The construction improves a verification
representation, but does not give the requested sub-exponential ECDLP
algorithm. No priority or novelty claim is made for this chunking trick.

For the same explicit full-point-window template, m windows costTheta(mb)
state ports and have a declared window-input arityTheta(b/m). For a
fixed cap exponent alpha in[3/5,1), the corresponding size balance is

    s=Theta(b^(2/(1+alpha))).

The lower endpoint ensures the cited b^(3/4) point-arithmetic gadgets
also fit the cap; equality-copy degrees fit as well. This is a statement
about this specified construction and its degree/size tradeoff, not a
universal restriction on compressed point representations or arbitrary
Boolean encodings.

## 5. Formal low rank does not supply a uniform factorization

For a fixed valid target, the full scalar verifier f_Q(k) has exactly one
accepting bit string. Its coefficient tensor is therefore formally rank1
across any partition of the scalar bits: it is a product of local delta
functions. Describing those factors by the unknown valid blocks would
already reveal the logarithm.

More precisely, suppose an efficiently constructed factorization into
disjoint blocks and Boolean-valued factors were supplied,

    f_Q(k)=product_j g_j(k restricted to block_j),

where each block has at mostw bits and each g_j is efficiently evaluable
with values in{0,1}.
The support is a Cartesian product. Since f_Q has exactly one accepting
string, every factor has a single accepting block. Enumerating each
block recovers k in O((B/w)2^w) predicate evaluations. For w=o(B),
this is already sub-exponential; for w=O(log B), it is polynomial.
Uniformly producing such factors is consequently the missing algorithmic
operation, not a free consequence of the formal rank.

This observation does not rule out overlapping low-arity constraints or
compact auxiliary certificates. They need their own public construction
and complete edge/description/verification accounting. Ordinary universal
elliptic-net identities do not by themselves provide such a target-specific
factorization.

## Remaining open operation

One additional bookkeeping pitfall is worth making explicit. If an
implicit chunk predicate internally sums over hidden scalar-window bits,
different chunks can use different hidden labels. Matching aggregate
first and second moments across several windows does not restore per-window
consistency. For two windows with disjoint label intervals[0,3] and[4,7],
take reference labels1 and6, each intended to appear twice. The counterfeit
chunk labels0,3 in the first window and4,7 in the second satisfy

    0+3+4+7=2*(1+6)=14,
    0^2+3^2+4^2+7^2=2*(1^2+6^2)=74,

although neither window is constant. A proposal that removes the hidden
labels from the Boolean edges must therefore charge an exact consistency
mechanism. Naively retaining a separate coefficient index of size H for
each of m windows has H^m states; for H=2^(B/m) this is2^B. These examples
close only those bookkeeping shortcuts, not every possible coefficient
encoding or tensor representation.

A surviving division-polynomial/net approach must provide an exact uniform
index locator or a more compact target-dependent verification encoding.
The sequence recurrence, one giant implicit predicate, dense expansion,
and explicit full-point windowing above do not certify the requested
sub-exponential bound under the cited contraction theorem. These are
scoped strategy closures; no general ECDLP impossibility or generic-group
lower bound is claimed.
