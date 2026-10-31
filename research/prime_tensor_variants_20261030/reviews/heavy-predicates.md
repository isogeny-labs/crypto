# Few explicit heavy predicates: what D.4 can and cannot preserve

2026-10-30 UTC. Pure mathematical analysis. No solver, attack, contraction,
benchmark or scientific computation was executed. This note does not establish
a new ECDLP algorithm or claim priority for the tensor contraction theorem.

## Question and sources

The target is a uniform prime-field scalar-verification encoding with
M=O(b*polyloglog(b)) Boolean edges, only polylogarithmically many heavy public
predicates, and a contraction guarantee exp(o(b)). The heavy predicates must
have polynomial-size descriptions and polynomial-time entry evaluation. The
specific question is whether their public structure can replace D.1's fixed
maximum-degree restriction without expanding a scalar-multiplication trace
back to quadratic size.

Literature was read before proposing changes: the pinned Oct25 manuscript
`/workspace/audit/tensor-primary/openai-oct25.txt`, TheoremD.1 and
SectionsD.4.2-D.4.7, and the pinned Bochum v0.2 manuscript
`/workspace/audit/tensor-primary/bochum-v02.txt`, Theorems2-4 and the compact
arithmetic/point-addition encodings. Source identities and hashes are in the
existing audit. An independent read of the relevant D.4 proof is recorded in
`/workspace/audit/heavy_variance_child.md`.

The result here is a collection of exact refinements and scoped obstructions.
They show why changing only the degree statistic, dropping common padding or
counting occupied tiles does not supply the claimed source bound. They do not
preclude a genuinely code-aware contraction algorithm, a new aggregation of
mode terms or a different compact scalar encoding.

## 1. Maximum degree has three distinct roles in the source

At an active D.4 call, let M be its real edge count, r=64, a its accumulated
label length, L=2^a its packet size, and q=r^a its number of vertex labels on
each side. The source's D_arr equals q^2. A vertex receives one label, shared
by all its ports.

First, LemmaD.3 bounds the label-pair collision variance by 2M*Delta, then pads
every one of the q^2 pair classes to a common load. Its conditions imply at
most3M total padded slots and at most3M/L packets.

Second, D.3(3) bounds the real ports of every group by 3M/r on each side.
D.4.6 keeps all ports of an old vertex in one child and obtains at most6M/r
edges per child. An old vertex's degree is unchanged. It is not distributed
among children merely because its entry has a short public program.

Third, the maximum degree and new-packet arity remain within the same bound
at every recursive call. Any replacement of the root collision bound must
therefore be proved again for retained original vertices and newly created
packet tensors. A favorable root statistic alone is not a recursive invariant.

## 2. An exact degree-moment collision formula

Let m_uv be the multiplicity of the pair of endpoints (u,v), and put

    P=sum_(u,v) m_uv^2,
    S2=sum_vertices deg(v)^2.

With independent uniform labels in [q] on both sides, let X_gamma count real
edges in pair class gamma. Then the source's variance expression is exactly

    V=E sum_gamma (X_gamma-M/q^2)^2
     =P*(1-1/q^2)+(S2-2P)*(1/q-1/q^2).

Proof: expand the square as a sum over ordered edge pairs. Disjoint endpoint
pairs contribute zero after subtracting 1/q^2. Pairs sharing exactly one
endpoint have agreement probability1/q; parallel or equal edges have
probability1. There are P pairs of the latter type and S2-2P of the former.
All counts include multiplicity and equal ordered pairs.

The conditional-expectation implementation remains polynomial: evaluate the
same exact pair probabilities while fixing labels. In particular the proof's
common padding estimate remains valid if, for suitable fixed constants,

    V <= M^2/q^4,                 L <= M/q^2.

Indeed each class load is at most M/q^2+sqrt(V), and rounding to a multiple of
L then gives H_pad<=3M/q^2. This is a genuine refinement of the proof's
sufficient hypothesis, not a sufficient condition for an ECDLP breakthrough.
It also does not prove that these same inequalities survive the recursion.

## 3. Few heavy vertices cannot fit the same padding or children

There are deterministic restrictions stronger than a variance calculation.
One row vertex of degree delta has a fixed row label, so its delta edges can
occupy at most q pair-label classes. Consequently

    max_gamma X_gamma >= delta/q.

The required common-load bound H_pad<=3M/q^2 therefore demands

    delta <= 3M/q.                                      (1)

No entry-evaluation property changes this incidence statement. Independently,
the source's group-port conclusion requires delta<=3M/r, since all delta row
ports of that vertex lie in one group at every coordinate.

A useful aggregate form is the following. If h row vertices carry eta*M real
ports, their edges occupy at most h*q pair classes. Thus

    H_pad >= eta*M/(h*q),
    H_pad*q^2 >= eta*M*q/h.

At most3M common padded slots is impossible when q>3h/eta. For an explicit
example take h disjoint stars with equally sized centers and public center
predicate identically1; all leaves are unary tautologies. The predicates are
trivial. This is an obstruction to that incidence/padding proof, not a claim
that these networks are difficult to contract. Their explicit code permits
immediate simplification.

The same warning applies to replacing Delta by S2/M. Even a single star has
S2=Theta(M^2), and the exact variance has scale M^2/q rather than the
M^2/q^4 needed by that padding estimate. Small heavy-vertex count does not
automatically make this weighted statistic small.

## 4. Consequence for the explicit scalar windows

For the previously constructed window verifier let m be the number of scalar
windows, w=Theta(b/m), and split each full canonical point output into
Theta(b/w)=Theta(m) chunks. There are Theta(m^2) chunk predicates, each of
declared degree Theta(w), and the network has M=Theta(mb) real edges. No
exponential table or unknown logarithm is supplied.

Applying (1) gives

    q=O(M/w)=O(m^2),
    a=log_r q=O(log m),
    u<=a=O(log m).                                    (2)

This also follows from the heavy-set aggregate bound: the Theta(m^2) chunk
predicates carry a constant fraction of the relevant ports.

Here and below O(log m) means O(log(m+1)) when bounded m is allowed. The
original source takes u=Theta(log s/2^d) and proves its precision and
recursion estimates only in that large-u regime. Thus (2) is incompatible
with simply retaining its parameters when m is polylogarithmic. Even if one
optimistically granted the same formal mode-count envelope with the smaller
u, the root exponent would have scale

    M/sqrt(u) >= c*b*m/sqrt(log m),       m>=2.

For every diverging m the multiplier m/sqrt(log m) diverges. For bounded m,
including m=1 separately, u remains bounded and this envelope supplies no
o(b) exponent. The supplied bound therefore cannot certify exp(o(b))
through this substitution. This is a limitation of that guarantee, not a
lower bound on actual running time.

In particular m=(loglog b)^C, for fixed C>0, would meet the desired
M=O(b*polyloglog b) size, but yields only u=O(logloglog b) under the unchanged
padding. Its formal envelope is superlinear in b. Enlarging a without
changing how heavy tensors are assigned to groups violates the deterministic
padding restriction before variance estimates are relevant.

## 5. Counting only real occupied tiles does not give D.13 for free

D.13 bounds the number of modes of each weight by at most6M/D, with
D=L/2^i and i=v2(t). It uses at most3M/L packets and at most2^(i+1) tiles per
packet. If sparsely filled packets are allowed instead of common padding,
the elementary occupancy bound becomes

    N_active_tiles <= min(number_of_real_columns,
                          number_of_packets*2^(i+1)).

The first term is only M, not O(M/D). A tile must contain both a real source
column and a real crossing target row to matter, but even that refinement
does not imply the desired bound.

Here is an exact example for the unchanged kernel and expansion. Choose two
labels A,B differing in every coordinate, and the orientation convention so
the permitted slot for this ordered pair is0. In each of M packets put one
real edge in slot x=y=0; the row label is A and the column label is B. All
other slots are unused. This is a legitimate
sparse orientation embedding, with M real edges and M packets. At every
active coordinate i the two real ports cross. There is one nonempty source
tile and one target row per packet, hence M distinct active source tiles.
For D tending to infinity, this violates N_active_tiles=O(M/D).

This is not merely an artifact of assigning the conservative weight1 to the
first mode. At T=t+alpha*zeta, alpha=2^(a+1), choose zeta a two-adic unit.
The sole real kernel entry is

    omega(T)=W_xx(T)=C(T)/(C'(0)*T).

The source's equal derivative valuations imply exactly

    v2(omega(t+alpha*zeta))=a+1-i.

Every packet's crossing form has only its b=0 source mode X_e: its real
column's tile coordinate is0. Give that mode the stronger actual weight
v=a+1-i. For a real row value Y_e=0 its affine factor is1-omega*X_e.
Choose the heavy center predicate to enforce Y_1=...=Y_M=0. This is public
and polynomial-time evaluable.
The expanded product has a nonzero monomial for every subset of the M
sources. D.5's evaluation precision is

    p_t=p+8*h*D+8*M*(a+1),

so even the largest subset valuation M*(a+1-i) is strictly below p_t. All
2^M subset mode choices survive valuation-only truncation. They are not
eliminated by omitting empty tiles or charging their exact larger valuation.

The example itself is easy: the center predicate factors into unary tests,
and its deformed contraction is (2-omega)^M. It demonstrates that the
unchanged mode-enumeration rule can be exponentially wasteful even with
explicit simple code. A code-aware method may collapse the terms; it must
actually supply and prove that collapse for scalar predicates.

An alternative labeling can improve the occupied-tile count in this example.
For instance, labels differing in only one coordinate allow many real edges
per packet. That does not repair the other problem: an unsplit heavy center
keeps all M of its row ports in one child, which does not shrink. The example
is therefore not a lower bound for every possible sparse embedding. It
refutes the proposed universal occupied-tile estimate and separates that
estimate from the necessary child-size invariant.

## 6. What an exact split would need to preserve

Portwise relabeling a high-degree vertex changes the proof's separation
identity. Its value must then be reconstructed from information held in
several groups. One exact split across a chosen cut has form

    T(x,z)=sum_(a in {0,1}^t) U(x,a)*V(a,z).

Over a field this implies rank(T_(x|z))<=2^t. For the public window map

    T(u,Z)=1 iff Z is the canonical point word for [u]P_j,
    0<=u<2^w<r,

the matrix between all scalar bits u and the entire point word Z has rank
exactly2^w, over F2 or any field. Its rows are distinct standard basis
vectors, since P_j has order r. Thus that particular split requires t>=w.
Public entry evaluation does not reduce this rank.

The corresponding summary statement is elementary: if all outputs Z are
available locally and the scalar u is visible only through a summary S(u),
an exact verifier for every Z requires S to be injective. It has at least2^w
values. For t bounded integer linear measurements with each range at most
2D+1, this requires t*log2(2D+1)>=w. One large weighted measurement can encode
the input, but enumerating its range then pays the same w bits.

Invertible changes of basis on the two sides leave the rank unchanged.
Likewise, if T is recovered as a linear combination of J coded tensors,
each of rank at mostR across this same cut, then J*R>=2^w. Interpolation
cannot erase this cost just by renaming the indices. This rank statement
does not cover nonlinear recovery or a different interleaved partition.

This is explicitly not a lower bound for all low-degree layouts. The identity
map also has full rank across the analogous input/output cut, yet factors
into bitwise equalities. A new interleaved scalar circuit could avoid this
cut. For a fixed final target, the scalar truth tensor has singleton support
and formally rank1; constructing its rank1 factors still requires finding
the unknown scalar bits. Neither observation supplies the desired public
factorization.

Conditioning likewise charges information, not merely vertices: conditioning
h heavy entries may expose Theta(b) independent scalar bits and require
2^Theta(b) choices. A proof must show how it avoids joint enumeration and
preserves a polynomial-size exact boundary description.

## 7. Explicit Boolean code is not a deformed entry oracle

The D.4 deformation does not evaluate the original Boolean program directly
at W(T)X. It forms the Boolean multilinear extension

    H_tilde(z)=sum_(Y Boolean) H(Y)*product_j l_(Y_j)(z_j).

This distinction matters for scalar verification. Arithmetizing the gates of
a public Boolean program and substituting z into that circuit need not give
H_tilde(z), because repetitions of Boolean variables must be reduced modulo
Y_j^2-Y_j before substitution. For example the Boolean program Y AND Y is
the function Y. Its multilinear extension is z; naive gate evaluation gives
z^2. Compact original code does not automatically provide a compact exact
multilinear extension or its mode coefficients.

The missing operation can be stated precisely for the full-point verifier
f_Q(k)=1[[k]P=Q, 0<=k<r], which has a unique accepting bit vector k_Q.
Its weighted extension is

    G_Q(A,B)=sum_k f_Q(k)*product_j A_j^(1-k_j)*B_j^k_j
            =product_j A_j^(1-(k_Q)_j)*B_j^((k_Q)_j).

All coefficients and evaluation points can be integral. Set every A_j=B_j=1
except B_i=0. The value is1 if bit i is0 and0 if it is1. Thus an efficient
uniform evaluator of this weighted extension already recovers the discrete
logarithm through O(b) evaluations. This is not an impossibility argument:
such an evaluator would be a breakthrough. It identifies exactly why the
original polynomial-time Boolean predicate is insufficient as the claimed
code-aware deformation subroutine.

Read-once public factorization, symmetry or another verified structure can
make some extensions easy. The sparse-packet example in Section5 has that
property. A scalar-map proof must exhibit such structure uniformly from
E,P,Q and show that coefficient extraction and new packet tensors preserve
it throughout recursion. Calling the predicate polynomial-time evaluable
does not prove any of those stronger properties.

## Result and the precise remaining opportunity

The exact degree-moment formula is a valid improvement of LemmaD.3's
variance estimate. It does not bypass heavy-vertex indivisibility. Common
padding, occupied-tile accounting and retaining whole old tensors together
cannot yield the desired direct scalar algorithm by changing only the
maximum-degree parameter. For the explicit windows the unchanged packing
forces a,u=O(log m), and even the optimistic source rate has exponent scale
b*m/sqrt(log m), which does not certify exp(o(b)).

A viable code-aware extension must supply an exact public splitting or
weighted-extension procedure for the scalar-map predicates, with both
coefficient extraction and retained boundary descriptions controlled after
deformation. If its total Boolean edges are O(b*(loglog b)^C), degree is at
most s^0.9, description length and entry evaluation obey D.1, then D.1 would
give exponent O(b*(loglog b)^C/sqrt(log b))=o(b), including polynomially many
prefix queries. No such scalar splitting has been proved here. This is the
specific missing constructible lemma, rather than an uncharged assumption
about public code, few vertices or few originally free scalar bits.
