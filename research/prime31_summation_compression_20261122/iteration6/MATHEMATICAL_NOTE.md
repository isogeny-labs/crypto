# Prime31 continuation 6: rational S3 chains and an oriented quadratic lift

Date: 2026-11-23 UTC. Mathematical working note for the existing
`isogeny-labs/crypto#4928` lane; parent agent owns integration and publication.
The temporary filename retains the worker's original `v5` assignment, but the
live lane acquired a separate fifth iteration while this work was in progress.

**Standing:** written derivations, **[pcc:unverified]**. This worker performed
read-only source review and mathematical analysis. No cryptographic experiment,
timing run, paid compute, new point count, Lean proof, or signed certificate was
produced here. No global novelty or solving-speed claim is made.

## 1. Concrete result and remaining gate

There is an exact all-arity replacement for the *liftable* Dickson membership
polynomial in a connected **affine S3 tree over Fp** with a nonzero rational
target on an odd-order curve: use the full split Dickson fibre instead. Every
leaf is forced to lift rationally by oriented recovery from the root.

A parent-oriented augmentation turns the S3 tree and Dickson ladders into a
system whose equations all have total degree at most two. For m>=2 leaves and
N = 2^k, k >= 1, the lean system has

    variables = mk + 3m - 5,
    equations = mk + 3m - 4.

These counts assume that solutions are sought **over Fp**. An explicit finite
field guard below makes the algebraic-closure zero set equally precise.

For a prime-order group of order n, a left-associated chain with freely
permutable leaf slots represents every unordered signed multiset of m <= n
nonzero points whose total is the nonzero target. Repeated leaves and
cancelling pairs are allowed. A fixed sorted leaf order does not have this
completeness guarantee.

This establishes a compact, exact representation and deterministic witness
extraction. It does not establish a bound on its root-solving degree, number of
elimination states, or cost of certifying an empty target. That is the next
solver obligation.

## 2. Parameters, domains, and existing inputs

Use a nonsingular short Weierstrass curve

    E/Fp: y^2 = f(x) = x^3 + ax + b,  p > 3,

and assume E(Fp) has odd order. The benchmark parameters recorded in the lane
are

    p = 2147483647 = 2^31 - 1,
    a = 847514846,
    b = 2087444724,
    n = 2147446831,
    G = (625037715, 2100627251).

The recorded n is prime. This note inherits those curve/order inputs from the
existing lane rather than independently certifying them. Odd order implies
that every finite rational point has nonzero y-coordinate.

Let F(X) = D_N(X) - c have N distinct roots in Fp, with N = 2^k and

    D_1(X) = X,
    D_(2j)(X) = D_j(X)^2 - 2.

Write Xset for the full fibre, W for its liftable coordinates, B = |W|, and
H(X) = product_(w in W) (X-w). The signed rational point base is

    A = { (x,y) in E(Fp) : F(x)=0 },       |A| = L = 2B.

The identity is excluded, and A is closed under negation. The target is a
specified rational point R=(r,s) != O. Keeping the *signed target point*, not
only r, is necessary for canonical oriented extraction.

Existing S3:

    S3(x,z,r)
      = (x-r)^2 z^2
        - 2[(x+r)(xr+a)+2b] z
        + (xr-a)^2 - 4b(x+r).

The prior pair note already supplies the discriminant identity and oriented
pair formula. The new proofs below apply that local identity to an entire
tree and make its domain/completeness requirements explicit.

Immutable inputs read for this continuation include:

* `NEXT_ROOT_INTERSECTION.md` in
  `research/prime31_summation_compression_20261122/`.
* `NEXT_IMPLICIT_INDEX.md`, blob
  `2ece7d19b0600dc08bb2fa8a7e24b2e243a63043`, from PR #4928 head
  `65f413e2ca1014a28e5ba6bb456d599f020b8f29`.
* The companion audit `rust_frontier_v5.md`, including its exact distinctions
  among native finite-root extraction, free Fp-coefficient auxiliary ideals,
  and mixed S3/S4 systems.

The parent subsequently observed live PR #4928 head
`e1d1519aa7f2003658df56b83af75ab4eb93e824`; this note does not overwrite or
reinterpret the separate fifth-iteration bucket/exponent work at that head.

## 3. Local oriented lift, including repeated coordinates

**Lemma.** Suppose x,z are in Fp, R=(r,s) is a finite rational point with s!=0,
and S3(x,z,r)=0. Define

    u = x-r,             v = z-r,
    lambda = [uv - (3r^2+a)] / (2s),
    y_x = lambda u - s,  y_z = lambda v - s.

Then P=(x,y_x) and Q=(z,y_z) are rational points and P+Q=R. This includes
x=z, x=r, z=r, and all permitted combinations of those equalities.

**Proof.** Direct rearrangement of S3 gives

    S3(x,z,r) = [uv-(3r^2+a)]^2 - 4s^2(u+v+3r).

Consequently lambda^2=u+v+3r=x+z+r. Since f(r)=s^2,

    f(r+T) - (lambda T-s)^2
      = T^3 + (3r-lambda^2)T^2 + (3r^2+a+2s lambda)T
      = T(T-u)(T-v).

The line y=lambda(x-r)-s intersects E at -R, P and Q, with the displayed
intersection multiplicities. The elliptic chord-and-tangent rule gives
(-R)+P+Q=O. Repeated coordinates are roots with multiplicity, so they require
no division by x-r or x-z. The nonvertical line is defined because 2s!=0.
All displayed coordinates lie in Fp. QED.

For a fixed ordered pair of child abscissae and oriented parent R, this lift
is unique. In particular, there is no independent sign choice left at the
node. The formula is a deterministic gluing rule, not merely a test that both
coordinates have some square root.

## 4. Rational-tree purity and full-fibre replacement

Take a full binary tree with m leaves. Each nonroot vertex has an affine
abscissa; the root has abscissa r. Impose S3(x_left,x_right,x_parent)=0 at
every internal vertex, and constrain **every abscissa to Fp**.

**Theorem.** If the root is oriented by R in E(Fp) minus {O}, every solution
has a unique compatible orientation of its vertices as rational points,
with each pair of children summing to its parent. In particular every leaf
abscissa is liftable.

**Proof.** Apply the local lemma to the root. Its children become finite
rational points. Their ordinates are nonzero because E(Fp) has odd order.
Apply the same lemma at each internal child, using its already determined
oriented point as parent. Induction reaches all leaves and preserves each
edge's orientation. The local uniqueness proves global uniqueness. QED.

It follows, as an equality of Fp-valued assignments to the abscissa variables,
that

    V_Fp(S3 tree, F(x_1),...,F(x_m))
      = V_Fp(S3 tree, H(x_1),...,H(x_m)).

The right-to-left inclusion uses W subset Xset. The reverse inclusion uses
the theorem. Thus the full Dickson membership circuit may be retained at
every leaf without enumerating W or constructing H for this representation.

This is a statement about a connected finite-Fp S3 tree. It is not a statement
about an arbitrary system with coefficients in Fp, an auxiliary in Fp^2, a
projected S_(m+1) equation, or a tree containing an unexpanded terminal S4.

## 5. Exact-m completeness and the ordering issue

Use the left-associated chain, with q_j the abscissa of the sum of its first
j leaves. For m>=3 the equations are:

    S3(x_1,x_2,q_2) = 0,
    S3(q_(j-1),x_j,q_j) = 0,       3 <= j <= m-1,
    S3(q_(m-1),x_m,r) = 0.

There are m-2 auxiliary abscissae and m-1 S3 equations. A particular ordered
tuple is represented exactly when all its internal prefix sums are nonzero.
For m=2 use only S3(x_1,x_2,r)=0 with no auxiliary abscissa. For m=1 test
the target's own factor-base membership directly; the subsequent system-size
formulas are stated for m>=2.

**Ordering lemma.** Let a prime-order group have order n. If 1 <= m <= n,
and P_1,...,P_m are nonzero group elements with nonzero total R, some ordering
has every nonempty prefix sum nonzero.

**Proof.** Work backward with a remaining multiset of t>=2 points whose sum
S is nonzero. Choose a remaining point a different from S, remove it, and
make it the last point of this stage. The remaining sum is S-a!=O. Such a
choice can fail only if all t points equal S. In that event S=tS and hence
(t-1)S=O, impossible because S has order n and 1 <= t-1 <= m-1 < n. Continue
to one nonzero point. Read the removals in reverse, after that last point.
Every prefix sum was a nonzero remaining sum in this construction. QED.

The proof covers m=n as well as the narrower m<n scope initially proposed.
It also works when every nonzero element has order greater than m-1. That
condition cannot simply be omitted: four equal elements of order three have
nonzero total, but every length-three prefix is zero.

Therefore the affine left chain with free leaf slots is complete for
existence and enumeration of **unordered signed multisets** of exactly m
base points, for m<=n and R!=O. Cancelling pairs and repeated points remain
valid exact-m decompositions; they need not be removed or reclassified as
shorter relations.

The qualifications matter:

* A sorted/canonical leaf-order constraint can remove every admissible chain
  ordering of a multiset. For example P,-P,Q has a forbidden zero prefix in
  that order although an admissible reordering exists when m<=n and Q!=O.
* The affine chain does not enumerate every *ordered* tuple. To return all
  orderings, recover a signed multiset and expand its distinct permutations,
  charging their output cost, or implement identity-intermediate branches.
* Different chain solutions can encode the same signed multiset. Exact
  deduplication is a separate output operation, with its memory/time charged.
* This completeness proof selects a left chain. It does not establish
  permutation completeness for every preselected balanced tree topology.
  In fact, for prime n>=5, the multiset P,P,P,-P has nonzero total 2P,
  but every balanced two-pair partition has a cancelling P,-P pair.
  Every such balanced affine tree therefore omits that multiset, while
  a suitable left chain includes it.
* R=O and identity leaves are outside the theorem. They need explicit
  separate handling if a solver's output contract includes them.

Given an already known tuple, the backward proof can be implemented using
O(m^2) equality checks and O(m) group subtractions with a straightforward
scan. Its existence does not supply an algorithm that discovers the tuple.

## 6. Concrete candidate: all-node quadratic orientation plus Dickson ladders

For each internal vertex v introduce a slope lambda_v. For each nonroot
internal vertex also introduce an ordinate y_v. The root coordinates are
the constants (r,s). All nonroot x variables are retained.

At each internal node impose two equations:

    lambda_v^2 = x_left + x_right + x_v,                  (Q1)
    2 y_v lambda_v = (x_left-x_v)(x_right-x_v)
                       - (3 x_v^2+a).                  (Q2)

For every nonroot internal child w of v impose

    y_w = lambda_v (x_w-x_v) - y_v.                     (Q3)

Leaf ordinates need not be solver variables. Recover them afterward by the
same formula as Q3. Every equation Q1--Q3 has total degree at most two.

The cubic-factorization identity in Section 3 proves inductively that Q1--Q3
define curve points and the required sums, starting from the actual root R.
Separate cubic curve equations for each internal point are unnecessary.
Conversely, the unique oriented lift of a rational S3-tree solution satisfies
Q1--Q3. The augmentation is therefore a bijection on Fp-valued solutions.

For each leaf i add its compact Dickson membership ladder. Put z_(i,0)=x_i;
for k>=1 introduce z_(i,1),...,z_(i,k-1), impose the first k-1 equations
z_(i,j+1)=z_(i,j)^2-2, and terminate with

    z_(i,k-1)^2 - 2 = c.

There are k equations and k-1 new variables per leaf. For k=1 the terminating
equation is simply x_i^2-2=c. The N=1/k=0 case is handled separately by x_i=c.

The exact lean counts are:

| Component | Variables | Equations |
| --- | ---: | ---: |
| Nonroot x coordinates | 2m-2 | 0 |
| Nonroot internal y coordinates | m-2 | m-2 propagation equations |
| Slopes | m-1 | 2(m-1) node equations |
| Dickson intermediates | m(k-1) | mk |
| **Total** | **mk+3m-5** | **mk+3m-4** |

The construction has O(mk) field-sized coefficient data and avoids an explicit
H of degree B. Relative to expanded monomial membership this reduces input
construction and assigned-point evaluation. A polynomial system's small
arithmetic circuit and bounded equation degrees do not bound its elimination
degree or its solution cost.
This construction count takes the already specified split fibre (N,c) and
curve/target parameters as inputs. Any cost of selecting or validating those
inputs belongs to setup; it has not been eliminated by the representation.

### 6.1 Explicit Fp guard for closure-based elimination

Coefficients in Fp do not constrain roots to Fp. If an algebraic-closure
solver is used, add a finite-field constraint on each of the m-2 internal
abscissae. Leaf abscissae are already in Fp because F splits there; their
ladder values are then in Fp. Once all abscissae are rational, oriented
induction forces every y and slope into Fp, so they need no additional
field guards.

For p=2^ell-1, impose x^(2^ell)=x^2 on each internal x. Its zero set is
exactly Fp because

    x^(2^ell) - x^2 = x(x^p-x).

For ell=31, introduce h_1,...,h_30 and impose

    h_1=x^2,
    h_j=h_(j-1)^2             for 2<=j<=30,
    h_30^2=h_1.

This adds **30 variables and 31 quadratic equations per internal x**. The
closure-safe zero-set representation therefore has

    variables = mk + 33m - 65,
    equations = mk + 34m - 66.

At x=0 the guard polynomial has multiplicity two. Its zero set and canonical
guard-variable assignments are exact, but raw algebraic multiplicities must
not be used as witness counts. If a squarefree eliminated field polynomial
is required, use a binary exponentiation circuit for x^p-x instead, with
O(log p) new quadratic equations. For this Mersenne p, iterating
a_(j+1)=a_j^2 x from a_1=x and separating each square/product gives a guard
with 59 new variables and 60 quadratic equations after substituting the final
a_31=x. Its eliminated polynomial is exactly x^p-x.

Over a native Fp solver the extra guards are unnecessary. Under general
primes the same binary-exponentiation construction is available; no infinite
family of Mersenne primes is assumed for an asymptotic argument.

## 7. Exact extraction and complete-work accounting

For an assigned abscissa-tree solution, top-down recovery requires one
inversion of 2y_v at each of m-1 internal nodes. A direct implementation of
the local formula uses, per node, one inversion, four general
multiplications, one squaring, six additions/subtractions, and two constant
scalings (by 2 and 3). Extra consistency checks are counted separately.

Let I_p, M_p, S_p, A_p and C_p denote those operation costs. The direct lift
has the explicit upper bound

    W_lift(m)
      <= (m-1)(I_p+4M_p+S_p+6A_p+2C_p).

The membership ladder costs mk squarings and mk subtractions to evaluate on
an assigned leaf tuple. Final independent point/on-curve and signed group
replay adds O(m) field/group operations. If the quadratic solver already
returns all slopes/internal y values, extraction may reuse them, but their
construction and verification remain charged somewhere in the solver.

Writing an accepted row with m point labels costs at least Omega(m). Returning
z rows therefore costs Omega(mz), before multiset deduplication or downstream
linear-algebra rank filtering. A field trace gives counts modulo p and must
not be interpreted as an integer count without an appropriate bound or
separate integer reconstruction.

The full cold relation cost must retain the unresolved term:

    C_relation = C_parameter_and_fibre_setup
               + C_input(m,k,p)
               + C_solve(m,k,p,R)
               + C_field_root_isolation
               + C_oriented_extraction
               + C_output_and_deduplication.

Here C_input is O(mk) in the Fp-domain model, or O(m(k+log p)) with field
guards. The representation does not currently prove any sublinear empty-query
bound for C_solve+C_field_root_isolation. Reporting only input size, one
candidate evaluation, or extraction would omit the hard part.

## 8. Target-independent sharing: an exact recurrence and its charged boundary

One exact way to share left-chain work is a partial-sum state graph. For
S!=O, let f_j(S) be the integer number of ordered j-tuples from A with total
S and no zero prefix; set f_j(O)=0. Then

    f_1(S) = 1_A(S),
    f_j(S) = 1_(S!=O) sum_(P in A) f_(j-1)(S-P).         (DP)

This is an exact recurrence for the affine chain's signed semantics. It uses
group points directly and does not assume an efficiently known discrete-log
coordinate or integer-additive bucket map.

Let M_j=|supp f_j| and mu_j=sum_S f_j(S). A sparse state/transition build by
explicit extension has, after base setup, the recurrence

    C_j = C_(j-1) + Theta(L M_(j-1)) group/hash operations,

with count bit lengths O(j log L). Deduplicating equal group sums may reduce
M_j, while storing every witness explicitly requires mu_j entries. Since a
nonzero prefix sum has at most one forbidden next point (its negative),

    mu_1 = L,
    mu_j = L mu_(j-1) - sum_(S in A) f_(j-1)(S),
    L(L-1)^(j-1) <= mu_j <= L^j.

For a nonempty signed base L>=2. Thus for fixed j and growing L, a complete
explicit prefix-witness list has
Theta(L^j) entries even when many sums collide. This does **not** lower-bound
an aggregate-count graph or arbitrary implicit representation: its support
can saturate at n-1 or be much smaller on a structured base.

The forward DP needs O(M_(j-1)+M_j) point/count slots for two adjacent levels.
Retaining all count levels for later witness backtracking needs
O(sum_(h=1)^j M_h) such slots. Storing every explicit transition instead can
use O(L sum_(h=1)^(j-1) M_h) edges; omitting those edges requires regenerating
candidate predecessors, whose group operations must then be charged during
queries. A complete witness list uses mu_j entries and at least j labels
per explicitly written tuple. None of these memory choices is a free
constant-size subtree oracle.

A standard meet-in-the-middle split m=r+s similarly builds all signed
s-point tuples, including those with zero total, and queries their sums for
the complements of all r-point choices. Building complete s-witness lists
costs Theta(L^s) for fixed s; querying enumerated r choices costs O(L^r+z)
expected hash operations. If the requested output is the affine-chain subset,
its global prefix condition is checked on recovered tuples; a suffix's own
prefixes need not satisfy the same condition. A pair-invariant elimination polynomial or
unbalanced product tree must improve on these actually charged operations,
not merely rename the same list.

The quadratic system avoids constructing this graph or list as input. The
remaining possibility is that its algebraic structure permits a smaller
exact contraction. Neither a bound on that contraction nor a counterexample
excluding every nonlinear contraction is established here.

## 9. Why projected higher summation polynomials need a separate ghost audit

Suppose w in Xset is nonliftable and r=x(R) is liftable and in Xset. Over
Fp^2 choose P=(w,sqrt(f(w))). The geometric tuple P,-P,R has total R, so its
x-coordinates satisfy S4(w,w,r,r)=0, equivalently S4(w,r,w,r)=0 by symmetry.
The first two points are not rational factor-base points.

The nondegenerate ordering (w,r,w,r) has an especially clear certificate.
Put q(U)=S3(w,r,U). Its discriminant is 16 f(w) f(r), a nonsquare in Fp,
and w!=r makes its leading coefficient nonzero. Thus q has two Fp^2 roots
and no Fp root, while the projected resultant Res_U(q,q) is zero. The
fully rational finite chain rejects the tuple. In another ordering the
missing auxiliary can instead be an identity/infinity branch.

This is precisely why eliminating auxiliaries before enforcing their
rationality can change the relation problem.

For completeness, Frobenius also gives an exact count classification. Every
point with rational x is rational or anti-rational (sigma(P)=-P). A sum of
such points equal to rational R has anti-rational subtotal T that is also
rational. Hence 2T=O; odd E(Fp) forces T=O. No separate assumption about
the order of E(Fp^2) is needed for this conclusion.

Let A_j(R) count ordered signed rational j-tuples with total R; let b_h count
ordered signed anti-rational h-tuples with total O; and let C_m(R) count full
signed geometric fibre m-tuples with total R. As integer counts,

    C_m(R) = sum_(h=0)^m binom(m,h) A_(m-h)(R) b_h.

Here b_0=1, b_1=0 and b_2=L_minus=2(N-B). For nonzero R,

    C_3(R) = A_3(R) + 3 L_minus A_1(R),
    C_4(R) = A_4(R) + 6 L_minus A_2(R) + 4 b_3 A_1(R).

These are signed point-tuple counts. They are not unweighted x-solution
counts, resultant multiplicities, or arbitrary chain-auxiliary counts.
Computing b_h and C_m and extracting witnesses is not free; this correction
does not provide a fast counting oracle. The rational-chain theorem avoids
these ghosts directly rather than relying on aggregate count subtraction.

## 10. Prior art and verification boundary

Primary source reviewed: Igor Semaev, *New algorithm for the discrete logarithm
problem on elliptic curves* (2015), https://arxiv.org/pdf/1504.01175,
Section 3 equation (4), Section 4.1 Lemma 2, and Section 4.2. The paper already
uses S3 chains with auxiliaries in the ground field. Under a minimality
assumption excluding shorter decompositions, Lemma 2 obtains rational
auxiliaries and identifies the possible nonrational subtotal as 2-torsion.
Section 4.2 leaves the large-prime-field solving method unresolved. This
continuation credits that chain construction; its scoped contribution is the
full-Dickson membership application, explicit root-oriented quadratic system,
field-domain accounting, and exact-m free-permutation completeness proof.
These statements are not a claim of literature-wide novelty.

The sibling's bounded source audit also found prior native orientation in
`research/prime31_ic_20261115/native/examples/prime31_m8_oriented_f4.rs`,
blob `e65aff0b1ee1166aefdfd79775f1c4bc2f695a16`. It has original Dickson
ladders and a chord/tangent orientation of its final leaf pair, together
with remaining S4 constraints. Thus neither Dickson lifting nor group-law
orientation is new by itself. The present exact theorem concerns an
all-node oriented rational S3 tree, not that mixed projected system.

Other relevant native scope distinctions are recorded in `rust_frontier_v5.md`:
`f4_fp_tower.rs` does not automatically impose x^p-x on free auxiliaries;
the existing m8 S3-chain mode retains a terminal S4; the pair adapter does
extract actual Fp roots and check original signed points. Those different
objects must not receive the same completeness claim.

Sources checked (KB): no callable `kb_status`, `search_knowledge`, or
`research-kb` tool was exposed by the available-tool inventory, and no local
wrapper appeared in the bounded materialized files. The fallback used the
lane notes, sibling source audit, and primary paper directly. No absence of
prior art is inferred from that limited access.

### Focused correctness controls for the parent-owned next action

1. Compare the local explicit lift with original group replay, including
   repeated x coordinates and x equal to the parent's x.
2. On a tiny frozen signed rational base, compare all affine m=3/m=4 chain
   witnesses with direct ordered enumeration under the same prefix convention.
3. Check exact unordered multiset recovery after allowing leaf permutations;
   preserve a sorted P,-P,Q failure and a valid reordered witness.
4. Retain the explicit nonsquare-discriminant projected S4 ghost, its Fp^2
   auxiliary roots, and its rejection by the finite-Fp chain.
5. For the quadratic lift, verify the cubic factorization and every Q1--Q3
   equation against independently replayed original curve points.

These are correctness controls. A subsequent solver evaluation must bind the
actual field restrictions, root-extraction contract, setup, empty targets,
all signed outputs, and memory to a complete cost recurrence.
