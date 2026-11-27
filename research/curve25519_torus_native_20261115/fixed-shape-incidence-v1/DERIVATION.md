# Fixed-shape incidence: exact model and cost gates

This is a written conditional counting argument for the existing original-map
adapted-base mechanism. Classical finite union bounds and linear algebra are
used. The original native group/map refinement and full sampling bridge are
not formalized by this note. No novelty or attack exponent is asserted.
[pcc:unverified]

## 1. Domain and finite encoding multiplicity

Let H be the rational point group with order8ell and exactly eight rational
points killed by8. Let G0=[8]H be cyclic of odd prime orderell. Fix the
original nonzero Montgomery A, odd characteristic, norm-one parameter
z=(u,v), u²-2v²=1. The admitted map selects a unique rational x branch BEFORE
discarding zero ordinate or torsion images, and then allows both ordinate
signs. Chart poles, identity images and ambiguous branches are excluded.
Each admitted z therefore determines at most two signed raw points.
These are hypotheses/readable consequences of the original map definition,
not a new native arithmetic proof. [pcc:unverified]

Fix a signed physical column {P,-P}, P!=0 in G0. There are eight raw points
above P. If x is a raw point's abscissa, every encoding has

    x=A*(epsilon-u)/(2u), u=epsilon*A/(2x+A), v²=(u²-1)/2.

The denominator is nonzero whenever this admitted equation holds, because
A and u are nonzero. There are at most2 choices of epsilon and2 roots for v,
hence at most4 encodings/raw point and32 encodings/P. Negating a raw point
preserves x, branch admission and the available signed encodings. Thus the
same set of at most32 encodings covers the entire physical column; it is
not doubled to64. This bound already includes all cofactor sectors.
[pcc:unverified]

## 2. Target incidence for a fixed relative template

Fix m>=1 units alpha_1,...,alpha_m in the parameter group, independent of
future queried targets. A witness uses one common multiplier sigma, parameters
z_i=sigma*alpha_i, admitted signed raw points, and image sum T. Define S(T)
as the set of physical columns appearing in ANY valid witness for T, before
any oracle selection. Raw distinctness/base-range guards can only remove
witnesses; the upper bound deliberately relaxes them. [pcc:unverified]

For a fixed column P, choose a role i and one of its at most32 encodings z.
Then sigma=z/alpha_i is fixed. For that sigma there are at most2^m signed raw
tuples, each giving a single image target. Therefore

    d(P):=#{T!=0 : P in S(T)} <= C_m :=32*m*2^m.

Multiplicity, swapped roles, duplicate targets and inadmissible tuples reduce
the count. For m2, C_2=256. This bound does not require solving a resultant,
a complete root oracle, an assumed smooth distribution or a timing fit.
An adaptive choice from a PREDECLARED finite family of D fixed templates
instead has d(P)<=C_m*D, by taking the union of their potential witnesses.
An unrestricted target-dependent template family has no such fixed D.
[pcc:unverified]

## 3. All-rows finite collision bound

Let N=ell-1. There are exactly N/2 nonzero physical columns modulo sign.
For independent uniform nonidentity targets U,V, including equal outcomes,

    Pr(S(U) intersects S(V))
      <= sum_P d(P)^2/N² <= C_m²/(2N).

Proof: for each column, the ordered target pairs containing it form the
Cartesian square of its incidence set. Take a union bound over columns.
This is a finite count, not a claim that S(T) is random or evenly sized.
For a fixed anchor set A of a physical columns, Pr(S(U) hits A)<=a*C_m/N.
For t iid nonidentity query targets, a union bound gives

    Pr(any cross-query sharing or anchor hit)
      <= C_m²*t*(t-1)/(4N) + a*C_m*t/N.                 (ALL)

The bound can exceed1; then it is merely uninformative. All potential roots
are allowed, so adaptive selection or returning every decomposition cannot
invalidate (ALL). No bound on the number of roots per individual target is
needed. Correlated targets such as T,-T do not satisfy iid sampling.
[pcc:unverified]

## 4. Why separated row blocks have no useful target tag

Fix one DLP Q=xG, with G a generator of G0. Query i has public point
T_i=a_iG+b_iQ; private labels are withheld from the oracle. Each retained row
is normalized to the same T_i, even if first obtained with sign -T_i. Only
physical equality/negation and known anchor substitution are allowed in
recovery; any extra logarithm transport is outside this model. Recovering a
scalar by unrelated computations or an already known answer is also outside
this row-recovery statement. [pcc:unverified]

On the complement of event(ALL), no row contains an anchor and every physical
column belongs to a unique query block. Let M be the matrix of ALL retained
rows. For any v with v^T M=0, restrict the column equations to block i and
evaluate those columns at their hidden logs to baseG. Valid group replay gives

    (sum_{r in block i} v_r)*T_i=0.

Since T_i has prime orderell, each block coefficient sum is0. The a,b tags
are constant within a block, hence v^T a=v^T b=0. Thus even a large internal
kernel cannot recover x through -v^Ta/v^Tb. Hidden logs are used only in
this proof, never as oracle inputs. This argument allows any number of
decompositions per query, unlike a private-column rank argument.
[pcc:unverified]

Consequently (ALL) bounds the probability of useful equality-only tagged
kernel recovery by t iid nonzero queries. For fixed m, fixed a and fixed
templates, constant success needs Omega(sqrt(ell)) queries along a growing
family satisfying these hypotheses, even granting free complete roots.
This is not a lower bound for all adaptive bases, nonuniform target policies,
additional algebraic/logarithm transport, or Curve25519 ECDLP generally.
Curve25519 itself is one fixed finite instance; (ALL), rather than an
asymptotic exponent, is its direct statement. [pcc:unverified]

## 5. Private-query generation and zero targets

For fixed Q=xG, fresh iid uniform a,b in F_ell give uniform T; conditional on
T, b stays uniform because a=log_G(T)-x*b is unique. Conditioning T!=0
preserves this label fibre. This is the classical Diem selection condition;
existing CryptoLean.SingleRelation.mixedLabelFiberEquiv gives the semantic
label equivalence. It does not itself prove the native sampler.
[pcc:unverified]

A zero target with b!=0 already recovers x. Rejection sampling must not hide
that event. Among at most n original iid label draws, probability of any
zero target is <=n/ell. Conditional on a fixed zero/nonzero pattern the
nonzero targets are iid uniform; use at most n of them in (ALL), giving

    success <= n/ell + C_m²*n*(n-1)/(4*(ell-1))
                        + a*C_m*n/(ell-1).              (RAW)

This loose bound charges failed/no-root attempts too. Conditioning the record
on successful decompositions and applying the original iid distribution is
invalid without accounting for that selection and all discarded draws.
Known-target preprocessing and arbitrary external solvers are not covered.
[pcc:unverified]

## 6. One retained row: sharper adaptive bound

For the original online one-row contract, let each retained row have at most
m physical columns. Before query i there are at most m*(i-1)+a old/anchor
columns. Fresh T_i is independent of that history. Regardless of how its
oracle chooses one row from S(T_i), an overlap is possible only when T_i
belongs to one of those columns' incidence sets. Summing the conditional
bounds yields

    success <= C_m*(m*t*(t-1)/2 + a*t)/(ell-1).          (ONE)

For m2,a2: 256*t*(t+1)/(ell-1). With D predeclared templates replace C_m by
C_m*D. For raw label draws also add n/ell as in(RAW). Online commitment is
essential: retaining every possible row and selecting after future queries
is governed by(ALL), not(ONE). Neither inequality says that the first shared
column is sufficient for an informative kernel. [pcc:unverified]

## 7. Growing-family research gate

Under(ONE), if D=ell^delta, a constant-probability useful collision needs
at least order sqrt(ell/D) fresh queries, up to fixed constants. If a future
method has an ACTUAL tight query work Theta(D^alpha), and the lower query
scale is attained with useful kernels, its query-phase exponent would be

    1/2 + delta*(alpha-1/2).

Thus alpha<1/2 is a necessary cost target within this tight-cost/attained-
collision model. It is not sufficient. A mere upper bound O(D^alpha) with
alpha>=1/2 is not a lower bound and cannot disprove a different algorithm.
With a lower cost bound Omega(D^alpha), the same algebra is a conditional
phase obstruction when alpha>=1/2. [pcc:unverified]

Returning all D witnesses per query changes the bound to(ALL), whose crude
collision scale is sqrt(ell)/D, while explicit output alone may cost Omega(D).
That combination reaches square-root work at this optimistic scale; it is
not a free exponent improvement. Do not transplant the ONE alpha threshold
into an all-rows model, or treat a collision lower bound as an achieved rate.
All cold family construction, point queries, lifting/indexing, row/kernel
work, useful yield, memory and checked scalar recovery still require pricing.
The old supplied-section Karatsuba upper bound is a different primitive.
[pcc:unverified]

The next constructive question is an implicit one-witness selector over a
growing ratio family, with sub-square-root-in-D TOTAL query work and useful
tagged cycles. Building D resultants/roots or a D-sized table per target fails
that prospective gate. No such selector is implemented or scheduled here.
