# Independent review: proposed ternary packet and 3-adic contraction variant

2026-10-30 UTC. Pure mathematical scrutiny of the supplied candidate. No implementation, attack, experiment or performance claim. This checks a proposed adaptation of Appendix D.4; it does not establish novelty or an improved exponent.

## Verdict

The orbit packing and locality mechanism work as stated. The proposed mode step needs one substantive structural correction: a tile can cross to two target groups, so it must be split into target-specific modes. This increases the stated bound from9M/D to at most18M/D modes per weight. With that correction, the remaining displayed counts and precision inequalities support the same asymptotic recurrence as the source proof. I found no fatal obstruction in this mathematical check, but the proposed variant is not yet a complete independently formalized theorem or a new ECDLP mechanism.

## 1. Ternary orbits really pack uniformly

Over K=F64, characteristic2 is different from3. The map

    S(a,b)=(-a-b,a)

satisfies S³=I. Its fixed vectors satisfy a=b and3a=0, so only (0,0) is fixed. Thus each nonzero pair has orbit size3.

For coordinate i, U_i acts as S on (R_(i,1),C_(i,1)) and S^-1 on (R_(i,2),C_(i,2)). It has order3 and is fixed exactly when all four entries are zero. Different U_i act on disjoint coordinates and commute. Therefore each orbit of pair classes has size3^a', with a' the number of nonfixed coordinates. The slot map from {0,1,2}^a onto that orbit has exactly L/3^a' preimages per class, where L=3^a.

Uniform vertex labels in K^(2a) give Darr=r^(4a) ordered pair classes. The original edge-pair variance proof still gives total squared load deviation at most2M Delta. Padding each class to the same H, a multiple of L, is possible within H<=3M/Darr under M>=2Darr² Delta and M>=LDarr. An orbit of size q is covered by Hq/L packets, an integer, each using L/q copies of every class. Thus the total packet count is HDarr/L<=3M/L.

Every padded class has the same load. For each component and symbol, exactly1/r of all pair classes have that row component, and exactly1/r have that column component. Hence each selected-symbol group has at most3M/r real ports on each side. This argument does not require S to preserve individual row labels along a packet; the assigned pair classes already provide the required labels for its ports.

## 2. Locality at a valuation node

Let t have v3(t)=i and leading ternary digit c=1 or2. If y-x=t, the i-th slot digits satisfy y_i=x_i+c modulo3. Higher carries do not change this fact.

For c=1, U_i acts as S on component1, so the new column component equals the old row component. For c=2, U_i² acts as S on component2 because that component uses S^-1. Thus in either case

    C_(i,c)(y)=R_(i,c)(x).

Consequently every nonzero W_xy(t) joins the same selected-symbol group. Coordinates fixed by U_i have zero symbols and satisfy the same equality. This verifies the exact locality implication, rather than only an average or a probabilistic coincidence.

## 3. Necessary target-specific modes

The binary proof's statement that a crossing tile has a unique other target group no longer holds. At a selected ternary coordinate, the row and column symbols can run through the three distinct values

    a, b, -a-b.

For example take a=1 and b any element of F64 outside {0,1}; then these values are distinct. A fixed column tile has one source symbol, but rows crossing to that tile can belong to either of the other two groups.

A single formal mode for the tile and expansion index would then appear in row factors of two target groups. Fixing its total exponent does not factor those two coefficient extractions independently, so the original one-source/one-target justification of (D.16) would fail.

The correction is to give each tile/expansion index a distinct formal mode for each crossing target group. There are at most two. Both modes may have the same numerical source linear form; this is harmless when substituted after the separate coefficient extractions. Each has exactly one source and one target. The count becomes

    2*(3M/L)*3^(i+1)=18M/D,

where D=L/3^i. Coefficient valuations remain at least the mode weight. This changes a fixed constant in the partition bound and does not change its exponent class. The correction must be present in any formal proof.

## 4. Three-adic kernel and Hermite arithmetic

The grid {-L,...,2L-1} has3L=3^(a+1) points and is a complete residue system modulo3^(a+1). It contains every possible slot difference. Thus the scalar kernel C(T)/(C'(0)(T-delta)) is polynomial and W(0)=I. The derivative valuations are uniform across the grid. With alpha=3^(a+1), the source integrality proof transfers by cancelling known powers of3; the coefficient extension must now have residue characteristic3.

At level i there are2L/3^i=2D nonzero grid nodes. For taper q_i=h(u-i)_+, h=ceil(2M/u), total multiplicity is at least2Lhu>=4ML, exceeding deg F<=M(3L-1). Also h<=3M/u and q_i<=3M because M>=u.

The analogue of (D.10) is

    v3(E_t(0))
      =2D sum_(j=1)^(a-i) (j/3^j)(q_(i+j)-q_i)
      >=-3Dh.

The last step uses q_(i+j)-q_i>=-hj and sum_(j>=1) j²/3^j=3/2. Jet evaluation has degree below q_i<=3M and loses at most3M(a+1) powers of3. Therefore

    p_t=p+4hD+8M(a+1)

is conservative for the same termwise-lift recovery protocol. The inverse local Hermite factor remains integral with constant term1. No floating-point precision is involved.

For crossing tiles modulo3^(i+1), locality implies v3(t-y0+x)<=i; otherwise that tile would have the same selected group as the row. The geometric series ratio has valuation at least1. Its b-th term has valuation at least b+1. With the corrected mode count the generating-function bound is still

    exp(O(sqrt(Mp_t/D)))=exp(O(M/sqrt(u))).

Inherited precision is absorbed in the same way because a_j=a_(j-1)+2u_j gives D>=3^(a_(j-1)+u_j). The extra3^u_j dominates fixed powers of log s on the active depth range.

## 5. Recurrence and scope

The selected-symbol port bound still gives at most6M/r=3M/32<M/8 edges per child. New packet degree is at most2L. The explicit epsilon=2^-20 makes Darr and L small fixed powers of s, so the packing inequalities, active-depth bound and entry-cost bound can hold for an effective sufficiently large range exactly as in the source accounting. Target-specific splitting adds no port indices and only polynomial description data.

With u_j=floor(epsilon log2(s)/2^j), the per-depth mode exponent remains proportional to (s/sqrt(log s))*(sqrt(2)/8)^j. Polynomial interpolation overhead, leaf evaluation, immutable creation-precision lifts and depth-first space accounting can therefore retain the original exponent class after the described corrections.

The coefficient ring is a separate scope condition. This is naturally a contraction theorem for entries modulo powers of3, not automatically the original theorem for arbitrary entries modulo powers of2. Boolean 0/1 counting can instead use a sufficiently large power of3 to recover its integer count, but conversion of arbitrary modular input tensors needs its own argument. Nothing here establishes that this adaptation is new in the literature, improves the published asymptotic exponent, or provides a distinct novel ECDLP breakthrough.
