# Appendix D.4: heavy vertices, exact variance, and the scope of splitting

2026-10-30 UTC. Read-only mathematical review of the pinned
`/workspace/audit/tensor-primary/openai-oct25.txt`, D.4.2, D.4.5--D.4.7.
No implementation, benchmark, scientific computation or ECDLP evaluation.
This is a refinement and obstruction for the supplied proof, not a new
contraction theorem or a lower bound for algorithms using explicit code.

## 1. The maximum-degree uses

Write `q=r^a` for the number of labels on each side in Lemma D.3, so
`D_arr=q^2`; the paper fixes `r=64`. In D.4.2 the hypotheses are

    M >= 2 D_arr^2 Delta,   M >= L D_arr,   2L <= Delta.

The first controls collisions through

    E sum_gamma (X_gamma-M/D_arr)^2 <= 2M Delta.

The labeling is followed by *common* padding of every pair-label class.
The intended conclusion is

    H_pad <= M/q^2 + sqrt(2M Delta) + L <= 3M/q^2.

This is used both for at most `3M/L` packets and for at most `3M/r`
real ports in any coordinate/symbol on either side. D.4.6 then gives child
edge count at most `6M/r<M/8`. Old vertices and all their ports remain
intact in one child; their degrees do not decrease. New packet vertices
have degree at most `2L`, accounting for the last inequality above.

At a leaf the algorithm enumerates all edge assignments. In particular,
an unchanged old vertex of degree delta cannot occur in a child with fewer
than delta edges. Raising the permissible old degree requires adjusting
these graph/termination invariants, even if its public entry code evaluates
in polynomial time. Entry cost alone is not the obstruction here.

## 2. A valid exact weighted-degree refinement

Let `m_uv` be multiplicity between an original row vertex u and column
vertex v. Define

    P = sum_(u,v) m_uv^2,
    S2 = sum_v deg(v)^2.

Here P counts ordered pairs of edges sharing *both* endpoints, including
an edge paired with itself. The number sharing exactly one endpoint is
`S2-2P`. Independent uniform labels from [q] give the exact identity

    V := E sum_gamma (X_gamma-M/q^2)^2
       = P(1-q^(-2)) + (S2-2P)(q^(-1)-q^(-2)).

Indeed disjoint endpoint pairs have pair-label equality probability
`q^(-2)` and cancel. Exactly-one-endpoint pairs have equality probability
`q^(-1)`. Both-endpoint pairs have equality probability one. This proves
the displayed formula without using entry values or code.

The same conditional-expectation calculation as D.3 constructs a labeling
with squared deviation at most V. Thus a valid replacement hypothesis for
that part of D.3 is

    V <= M^2/q^4,   L <= M/q^2.

It gives the same `H_pad<=3M/q^2` conclusion and hence the same packet and
coordinate-load conclusions. This is a genuine weighted-degree refinement
of that *one lemma*. One must still prove these hypotheses at every child;
they do not automatically follow from their root validity. Retaining one
heavy vertex of degree delta retains its contribution delta^2 to S2 while
child edge count drops.

For a simple graph, P=M. Replacing S2 by `2M Delta` loses useful information,
but the exact formula can still be large when only a few vertices are heavy.
The number of heavy vertices by itself supplies no upper bound small enough.

## 3. An explicit low-heavy-count obstruction to the same packing

Take h disjoint stars, with all centers on the row side. Each center has
degree M/h; every column vertex has degree one. Give every vertex the
identically-one predicate, with its trivial explicit polynomial-time code.
All expressions below concern the graph, irrespective of that easy code.

Under *any* assignment of q labels, at most h row labels are occupied.
Consequently at most hq pair-label classes contain real edges. Therefore

    max_gamma X_gamma >= M/(h q),
    H_pad q^2 >= M q/h.

The desired common-padding conclusion `H_pad q^2<=3M` is impossible for
`q>3h`. For h=1 this is already a one-heavy-vertex example. Independently,
D.3(3) is impossible if any single degree exceeds `3M/r`, since all ports
of a vertex belong to the same coordinate/symbol.

A slightly more general statement is useful: if h vertices on one side
carry eta M ports, then common padding needs at least `eta M q/h` slots,
so the same constant-overhead packing requires `q<=3h/eta`.

In the source q is a fixed positive power of the global size: at the root
`u=floor(epsilon log_2 s)`, `a=2u`, and `q=64^a=2^(12u)`.
Polylogarithmically many unsplit heavy centers carrying a fixed fraction of
ports cannot meet this condition. This is a concrete failure of the proposed
proof extension. It is not a hard contraction instance: its contraction is
trivial, and an algorithm inspecting the code can simplify it immediately.
That distinction is essential.

For the h-star graph the exact formula also reads

    S2 = M^2/h + M,
    V = M(1-q^(-2))
        + (M^2/h-M)(q^(-1)-q^(-2)).

The variance-based sufficient condition can therefore be stricter than the
necessary padding condition. Sharpening variance is not by itself enough
to repair padding.

## 4. Conditioning and fixed group membership do not remove the issue

Assigning a heavy vertex a group only puts all its ports in that group; it
does not reduce its arity. Old vertices are indivisible in D.4.6. A child
containing a degree-delta vertex must keep at least delta edges, so such a
path cannot reach the stated leaf threshold H while delta>H.

Literal conditioning on t distinct edge bits creates up to `2^t` branches.
Conditioning all edges incident to heavy vertices is a correct reduction,
with a cost governed by the size of that union, rather than by the number
of heavy vertices. This can be linear in M even for one heavy vertex. For
functional scalar-window checks, conditioning all free scalar windows also
allows ordinary evaluation and propagation, but enumerates all B scalar
bits. There is no generic small branching bound from functionality alone.
This does not exclude better code-specific conditioning or simplification.

The paper's deformed child packet tensors are not themselves scalar-
multiplication checks. Local row forms involve sums of several column bits
and targeted modes, and coefficient extraction couples choices. A claim
that functional witnesses retain just their original free inputs therefore
needs an invariant for these transformed tensors, rather than an assertion
about the root verifier.

## 5. Exact splitting needs a representation theorem, not just explicit code

Across a chosen bipartition of its ports, an exact factorization of a Boolean
tensor over F2 of the form

    T(x,y)=sum_(z in {0,1}^t) U(x,z)V(z,y)

implies matrix rank at most `2^t`. This remains a valid necessary condition
for a proposed split intended to work for all D.1 coefficient choices, since
`P0=ell=1` is allowed. The public equality predicate

    T(x,y)=1[x=y],   |x|=|y|=d/2,

has identity matrix of rank `2^(d/2)` despite its linear-time explicit code.
Thus this particular cut needs at least d/2 connecting Boolean bits.

There is a similarly exact statement for the full scalar-window map. Let
P_j have prime order r and `2^w<r`, and define

    T(u,z)=1[z is the canonical point encoding of [u]P_j],
    0<=u<2^w.

The matrix between all w input bits and the entire point word has rank
`2^w`: its rows are distinct standard basis vectors, because the map is
injective on that range. A split across *that cut* needs at least w internal
Boolean bits. No heuristic mixing assumption or elliptic coordinate
distribution is used.

These rank facts do not prove that such tensors lack O(d)-size bounded-
degree representations with a different layout. Equality itself has one,
and the identity function demonstrates why a full input/output rank does
not prohibit local wiring. They also do not supply the rank of every partial
output-chunk predicate. The fixed-target full scalar predicate has singleton
support and formal rank one; constructing its unknown factors uniformly
remains a separate algorithmic task.

An explicit polynomial-time entry program can always be expanded into a
Boolean circuit with polynomial overhead. The needed claim here is much
stronger: a uniformly constructible near-linear representation satisfying
the theorem's degree and charged-evaluation hypotheses, or a new recursive
invariant exploiting this particular code. None follows merely from having
the program.

## 6. Consequence for the explicit window template

This paragraph only analyzes the *same common-padding and source-mode-count
template*, not all possible heavy-aware contraction algorithms. The window
construction has `M=Theta(mb)`, `h=Theta(m^2)` heavy chunk predicates,
degree `Theta(b/m)`, and a fixed fraction of ports at those predicates.
Thus the necessary padding bound gives `q=O(m^2)` and hence `a=O(log m)`.
Because the source requires `u<=a`, its displayed mode-count guarantee
has exponent scale `M/sqrt(u)` and cannot by this substitution certify better
than a scale `bm/sqrt(log m)`. For growing m this is not o(b); fixed m
also gives no sub-exponential guarantee. These are limitations of that
displayed bound, not runtime lower bounds or exact mode-support counts.

A successful refinement must change more than the coarse maximum-degree
variance estimate: for example, the orbit/common-padding arrangement, the
handling of indivisible predicates, the actual support of modes, or a
constructible special representation of the public entry programs. No such
replacement lemma is proved here.
