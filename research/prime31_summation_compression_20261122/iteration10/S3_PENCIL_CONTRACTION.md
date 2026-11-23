# Rank-three S3 pencil: exact contraction and the missing zero-component index

Date: 22 November 2026 America/Los_Angeles / 23 November 2026 UTC.
Standing: written derivation and independent mathematical review only,
**[pcc:unverified]**. No numerical or cryptographic experiment, native test,
performance run, remote edit or signed proof was performed.

## Result and next mathematical obligation

The pair-spectrum S3 matrix has ordinary rank at most three. This gives an
explicit target-independent feature map and a three-by-three target pencil.
The exact relation query requires detecting a zero in the **columnwise
products** of that matrix, rather than an ordinary matrix product. The
product has formal target degree exactly 2D-1 in the D-dimensional split
quotient algebra, with a nonzero leading value at every source root.
Dense reusable coefficient tables consequently use quadratic space in D
under that specified representation. [pcc:unverified]

The discriminant does not prune this source domain: every nondegenerate S3
partner quadratic already has two rational roots. The next precise task is
an exact index for zero components of the structured product, using near-
linear setup in D and, in the D=Theta(B^2) regime, query exponent below 1/2
in D for the conventional m4 budget. The norm and degree identities below
specify that primitive; they do not implement it or establish its bound.
This is a scoped contraction analysis, with no priority or novelty claim.
[pcc:unverified]

## 1. Inputs, relation contract and immutable sources

Let p>3, let E/Fp be nonsingular with equation

    y^2=f(x)=x^3+a x+b,

and use an odd-order rational subgroup G. Let A={+/-P_i : 1<=i<=B}, where
the P_i are nonzero and their abscissae are distinct. Let

    U = (A+A) minus {O},     X = {x(T) : T in U},
    D = |X| >= 1,           H(V) = product_(u in X)(V-u).

Then U is sign-closed, |U|=2D, and every f(u) is a nonzero square. H is
monic, split and squarefree. Iteration9 supplies the exact B^2 unoriented
pair-expression chart and collision/backpointer buckets, so D<=B^2.
Every underlying label and group-sign recovery remains charged.

For R in G minus {O}, put r=x(R). Work in

    A_H = Fp[V]/(H(V)),
    K(T,V) = product_(u in X) S3(u,V,T) mod H(V),
    I_r = {v in X : K(r,v)=0}.

Iteration9's exact emptiness criterion is

    EMPTY(R) iff r not in X AND I_r is empty.             (1)

The r in X branch retains a zero pair subtotal. For all-output reporting,
the other nonzero-pair branches must still be enumerated as well. R=O has
its separate group-semiring contraction; it has no affine target x.

Reviewed input identities:

| Input | Commit | Git blob | SHA-256 |
| --- | --- | --- | --- |
| iteration9/PAIR_SPECTRUM_RESULTANT.md | f043ca05a338637f830e09fa19ad14d9b18a34ce | e2348e91dc08141a663c5adb9798b6ca8f09424e | 224f8223c52c141a0681cdb78ec37c140060922bdd3a72be32789506c9ef622e |
| iteration8/EXACT_CONTRACTION_AND_YIELD.md | a6d7b529ed5cd0586fbf04e916978f302cb65ecc | 20f9428434bf228c75de404bdb8c4fde4f7b3675 | 35a97211f5c0ef84878847203ce850343a465eb369c2900a534b5769697f9f20 |

Exact local copies are retained beside this note. The reviewed iteration9
branch is PR #4951; this audit does not edit that active branch. Its source
and group-order claims are inherited inputs, not new curve certifications.

## 2. A three-dimensional feature map, with the exact target matrix

Use the standard short-Weierstrass Semaev polynomial [S1]:

    S3(u,v,r) = (u-v)^2 r^2
             - 2[(u+v)(uv+a)+2b]r
             + (uv-a)^2 - 4b(u+v).

Set w(u)=(1,u,u^2)^t and

                 [ a^2-4br     -2ar-4b       r^2       ]
    C_r =        [ -2ar-4b     -2(r^2+a)    -2r        ].
                 [ r^2         -2r          1         ]

Expansion gives the exact identity

    S3(u,v,r) = w(u)^t C_r w(v).                         (2)

If W_X has rows w(u)^t, the D-by-D matrix of all S3 values is

    M_r = W_X C_r W_X^t,      rank(M_r) <= min(3,D).     (3)

W_X uses O(D) source words; C_r costs O(1) field operations once r is
known. No discrete-log coordinates or scalar-residue hash is used.
Equations (2)-(3) compress **values of the S3 polynomial**, not the zero
indicator or the operation of asking whether any matrix entry is zero.
[pcc:unverified]

The required component K(r,v) is product_u M_r(u,v). Ordinary low rank
does not replace this product by a linear contraction. The next section
records exactly what that product becomes.

## 3. The homogeneous product/norm that must be evaluated

Define the target-independent degree-D homogeneous polynomial

    Phi_X(Z0,Z1,Z2) = product_(u in X)(Z0+u Z1+u^2 Z2).

It is also the univariate resultant

    Phi_X(Z) = Res_U(H(U), Z0+Z1 U+Z2 U^2).

Consequently the full, unreduced pair-spectrum product is

    F_r(V) = Phi_X(C_r w(V))
           = product_(u in X) S3(u,V,r),               (4)
    K(r,V) = F_r(V) mod H(V).

This is a concrete reusable representation. Its factored form uses D
linear factors and O(D) source words. Evaluating Phi_X at one arbitrary
triple by multiplying those factors takes O(D) field operations. Doing
that independently for all D source v values takes O(D^2); the existing
univariate product-tree construction avoids that redundant calculation
and returns to O(M(D)log D) work for the whole K_r. Thus the rank-three
identity does not itself improve the current empty-query exponent.
[pcc:unverified]

Norms, resultants and power-projection approaches are established methods.
In particular, [S2] explicitly prices the quotient-algebra/output dimension
of special resultants; its near-optimal cases are near-linear in the
*output degree*, not automatically in the size of an implicit source.
Its composed-sum/product results are not a theorem for this elliptic
zero-component query. [S3] supplies the standard multipoint/product-tree
cost context. No current-matrix-exponent or general resultant algorithm
improvement is asserted here.

## 4. Exact formal target degree and its leading unit

**Proposition.** In the polynomial ring A_H[T],

    deg_T K(T,V) = 2D-1,
    [T^(2D-1)] K(T,V) = -4 f(V) H'(V)^2 mod H(V).      (5)

The leading coefficient is a unit of A_H. [pcc:unverified]

**Proof.** Since H is split and squarefree, evaluation at its D roots is
an injective algebra isomorphism A_H -> product_(v in X) Fp. At V=v,
exactly one factor is diagonal:

    S3(v,v,T) = -4 f(v) T + (v^2-a)^2 - 8bv.

Every u!=v factor has degree two in T and leading coefficient (u-v)^2.
Therefore K(T,v) has degree 1+2(D-1)=2D-1, with leading coefficient

    -4 f(v) product_(u!=v)(u-v)^2 = -4 f(v) H'(v)^2.

This is nonzero for every v because p>3, f(v)!=0 and H is squarefree.
The CRT description gives (5), and nonzero values in every component are
exactly the units of the split quotient algebra. The same formal proof
needs f(v)!=0 but does not itself require a rational point lift.

The unreduced leading T^(2D) term is H(V)^2 and vanishes modulo H; the
surviving T^(2D-1) term therefore matters. It is not valid to describe the
remainder as target-independent or of bounded target degree after this
cancellation. [pcc:unverified]

### What a dense reusable table actually allocates

Writing

    K(T,V) = sum_(j=0)^(2D-1) k_j(V) T^j,  deg_V k_j<D

in a full rectangular monomial coefficient table allocates **2D^2 field
slots**. Constructing/writing that explicit table costs at least its output
size in this format. It does not follow that 2D^2 coefficients are nonzero
or independent, or that an X-dependent circuit requires this much space.
Indeed, (4) already supplies a short nonlinear product representation and
(5) has a compact leading coefficient. [pcc:unverified]

For a scalar alternative, set

    Delta(T) = Res_V(H(V),K(T,V)) = product_(v in X)K(T,v).

Its formal degree is exactly D(2D-1); its leading coefficient is the
nonzero product of the D leading values in (5). Then I_r is empty iff
Delta(r)!=0. A full dense Delta table has 2D^2-D+1 coefficient slots,
so this particular materialization again moves quadratic-in-D work to
setup. Multiplicity, squarefree reduction, special source structure and
finite-field function representations can lower other descriptions; this
degree statement is not a lower bound on their size or evaluation time.
[pcc:unverified]

All degree claims above are **formal polynomial degrees**. As functions on
Fp, reductions modulo T^p-T are permitted; the claim does not assert that
2D-1 is a minimal function degree or an arithmetic-circuit lower bound.

## 5. A precise obstruction to a universal fixed linear lift

**Proposition.** If p>=D+2, the family Phi_X, as X ranges over arbitrary
D-element subsets of Fp, spans the entire vector space of homogeneous
degree-D polynomials in three formal variables. Its dimension is

    binomial(D+2,2).                                    (6)

This is an elementary interpolation statement, independently checked
here; no novelty claim is made. [pcc:unverified]

**Proof.** Choose a set T0 of D+2 distinct field elements. For each omitted
unordered pair {a,b} subset T0, use X_ab=T0 minus {a,b} and the evaluation
point z_ab=(ab,-a-b,1). For any u,

    Z0+u Z1+u^2 Z2 evaluated at z_ab = (u-a)(u-b).

Thus Phi_(X_ab)(z_ab) is nonzero. Every other D-subset of T0 contains a or
b and evaluates to zero there. The resulting evaluation matrix is diagonal
with nonzero diagonal, proving linear independence of binomial(D+2,2)
forms. This equals the dimension of the homogeneous polynomial space.
No factorial division or characteristic-zero assumption is used.

**Exact scope.** A fixed basis, independent of X, that must linearly
reconstruct every formal Phi_X needs at least the dimension (6). This
rules out replacing the entire degree-D product family by a universal
constant-size linear feature basis merely because the original factors
have three coordinates. [pcc:unverified]

It does **not** rule out nonlinear representations, the obvious D-factor
representation, X-dependent bases, a much smaller special Dickson-pair
subclass, or an algorithm computing only the needed zero predicate.
It also does not lower-bound the specialized map
Phi_X -> Phi_X(C_r w(V)) mod H_X: both its restriction and quotient vary
with X. These distinctions are essential to the next lemma.

## 6. The discriminant is exhausted; the remaining problem is intersection

For fixed v and r, the coefficient of U^2 in S3(U,v,r) is (v-r)^2 and
its discriminant is

    disc_U S3(U,v,r) = 16 f(v) f(r).                    (7)

This standard identity follows by expansion and is consistent with the
earlier native discriminant record; no new native execution is claimed.
On the present rational odd-order source, f(v) and f(r) are nonzero squares.
If v!=r, every one of these partner quadratics therefore splits into two
distinct rational roots. If v=r, it is a linear polynomial with nonzero
coefficient -4f(r). There is no nonsquare-discriminant rejection to prune
the D source v values. [pcc:unverified]

Using the supplied rational lifts T_v=(v,y_v), the two nondegenerate roots
are available directly from the group law:

    x(R+T_v) = ((y_R-y_v)/(r-v))^2-r-v,
    x(R-T_v) = ((y_R+y_v)/(r-v))^2-r-v.

The unresolved work is deciding which of these roots lie in X. Sharing a
target y_R or batching denominator inversions is legitimate constant-factor
engineering but still visits every v in this explicit algorithm. An exact
dictionary can test membership in O(log D) ordered comparisons, or expected
constant-time hashing with its implementation costs charged. [pcc:unverified]

There is also an exact geometric reading of (4). If r not in X, no
translated point R-T with T in U is O, and

    F_r(V) = H(r)^2 product_(T in U)(V-x(R-T)).         (8)

For each u, the two translated x-values are the two roots of its quadratic,
whose leading coefficient in V is (u-r)^2; multiplying gives (8).
Repeated translated x-values retain their multiplicities. At r in X a
factor drops from degree two to degree one and an O subtotal occurs, so
(8) must not be used by substituting H(r)=0. Formula (4) and criterion (1)
remain the correct algebraic definitions for those exceptional queries.
[pcc:unverified]

The cheap scalar branch r not in X need not evaluate dense H at r. With a
prevalidated sorted root array, binary-search nonmembership and adjacent
canonical field representatives certify that branch. This changes a
specific certificate format if an explicit nonzero field value H(r) was
previously required; it does not resolve the K_r zero-component query.

## 7. The next candidate lemma, with every cost exposed

The proposed research primitive is now sharply defined:

**Structured S3 product index.** Given the actual rational pair-spectrum X,
its point lifts and all pair backpointers, construct target-independent
data I_X. On any valid nonzero target R, return precisely

    I_r = {v in X : Phi_X(C_r w(v))=0},

and a deterministic, zero-error explanation of EMPTY when (1) holds. The
index must price exact finite-field membership, exceptional v=r, failed
queries, stored advice and any proof construction/checking. It may use a
different succinct proof format; it cannot treat dense polynomial output
as free. [pcc:unverified]

Let h=|I_r| and let Z_R count the requested signed tuple outputs. After I_r
is known, each v yields at most two partner roots, followed by X lookup,
pair labels, actual sign recovery and exact group replay. Zero-pair branches
and R=O remain explicit. An output-sensitive completion has additional
O(h polylog p + Z_R C_verify) work, including at least Omega(Z_R) emitted
labels. A one-witness protocol and an all-output protocol must be labelled
separately.

For the negative-query frontier, the desired index bounds would be

    setup after the source chart:  O~(D^s),
    source chart and labels:       O(B^2) work and storage,
    query:                        O~(D^theta + h),
    plus rational recovery, proof work and all outputs above.

These are **unproved requested bounds**, not properties of (2)-(8).
The existing factored polynomial algorithm has theta=1 up to logarithms;
the direct rational complement scan also has theta=1. A dense reusable
Phi/K/Delta coefficient table has s=2 in its stated materialized format,
which is B^4 setup when D=Theta(B^2). A useful new primitive must exploit
the special source and query family beyond those two implementations.
[pcc:unverified]

An interface that requires a full dense representative of K_r or a
length-D coefficient array for its modular inverse/Bézout witness already
has Omega(D) output traffic. That is an obstruction **only to that output
contract**. It is not a lower bound for an exact yes/no algorithm or a
different succinct certificate. The next proof must specify which contract
it uses and charge verification as well as production. [pcc:unverified]

### Relation to the permitted factor-base exponent

Only in the regime D=Theta(B^2), the above bounds give

    sigma = max(2,2s),      tau = 2 theta,
    alpha = max(sigma,2).

For an actually supported B=p^(beta+o(1)), n=p^(1+o(1)), the recorded
conventional uniform-target/full-independent-row model has exponent budget

    alpha beta < 1/2,
    1-beta(3-2theta) < 1/2,                             (9)

with all yield, rank and source-family assumptions still separate. Any
feasible beta>0 forces 3-2theta>0, equivalently theta<3/2; only on that
domain may the second inequality be divided by 3-2theta. Thus

    1/[2(3-2theta)] < beta < 1/(2alpha)

must intersect the **permitted** beta range. For s<=1, alpha=2, a nonempty
unconstrained interval requires theta<1/2, equivalently tau<1. Source-
specific family bounds may restrict that interval further; no numerical
least-prime exponent is assumed in this note. A family correction is being
reviewed separately. When D is smaller, retain the actual D-versus-B law
instead of substituting the generic quadratic regime. [pcc:unverified]

These inequalities are conditional accounting requirements, not a claim
that the needed rank/yield distribution or index exists. In the beta<1/4
range, iteration8's exact first moment already makes most uniform targets
empty; a supplied-positive-only algorithm does not address this gap.

## 8. Bounded conclusion and discriminating next step

The reusable representation is explicit: W_X, C_r and the factored Phi_X.
The exact zero predicate is explicit, including the zero-pair branch.
The formal target-degree calculation and universal interpolation basis
show why two tempting linear/dense closures do not furnish the needed
query bound under their specified formats. The nonsquare-discriminant
filter contributes no pruning on this rational pair-spectrum.
[pcc:unverified]

The next concrete lemma to seek is a target-independent index for the
restricted incidence set in Section 7 with complete misses and a proved
cost recurrence, without expanding its D^2 coefficient/target table or
returning a length-D intermediate vector. A proof for the actual Dickson-
pair subclass would be sufficient; a universal index is not required.
Root extraction and signed-output verification must be included when the
decision primitive is extended to reporting. No experiment was run for
this derivation, and no existing obstruction is broadened to all possible
representations.

## Primary sources and provenance

- **S1:** Igor Semaev, *New algorithm for the discrete logarithm problem on
  elliptic curves*, arXiv:1504.01175 (2015), Section 2 for the explicit S3
  formula, Section 3 equation (4) for rational auxiliary chains, and
  Section 4.2 for the prime-field limitation discussed there:
  https://arxiv.org/pdf/1504.01175 . The original 2004 ePrint PDF returned
  HTTP403 in this runtime; the formula was read directly in this author's
  2015 primary paper instead.
- **S2:** Alin Bostan, Philippe Flajolet, Bruno Salvy and Eric Schost,
  *Fast Computation of Special Resultants* (2005 author preprint),
  introduction/Theorem 1 and Section 4.1 on quotient-algebra dimensions,
  characteristic polynomials and power projection:
  https://cs.uwaterloo.ca/~eschost/publications/BoFlSaSc05.pdf .
- **S3:** Alin Bostan and Eric Schost, *Polynomial evaluation and
  interpolation on special sets of points*, author text, introductory
  complexity statements and subproduct-tree discussion:
  https://cs.uwaterloo.ca/~eschost/publications/BostanSchost.pdf .
- Existing repository sources: the two immutable iteration8/9 notes in
  Section 1; iteration6/MATHEMATICAL_NOTE.md for rationality/sign scope;
  identity-certificates/s3-discriminant.json for the earlier native
  evaluated identity; indexing_literature_v5.md for already audited
  integer-indexing, thin-matrix and generic-reporting limitations.

The research-KB tools and wrapper were unavailable in this bounded runtime.
The review used the exact repository notes and directly read primary
sources above. No literature-wide novelty search, claim promotion or
source-bound execution is asserted. The independent indexer supplied the
diagonal interpolation proof in Section 5 and checked (2), (5), (7)-(8)
and their domain restrictions; its separate review binds the final file.
