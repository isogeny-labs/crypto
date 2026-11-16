# Controller-private coefficients and bounded base restarts

Status: proposed clarification of this campaign's conditional reduction,
using standard randomized index-calculus and black-box linear algebra.
Independent review and full formalization remain open. This removes an
unnecessary *statistical row-rank hypothesis* from the proposed protocol;
it supplies no sublinear decomposition algorithm or new attack exponent.
[pcc:unverified]

## Exact protocol and cancellation

Let G=<P> have prime order ell and Q=[x]P be the public target. Fix a factor
base F_1,...,F_B in G. For each query the controller privately samples a,b
independently uniformly in F_ell and computes T=[a]P+[b]Q. The decomposition
oracle receives only the base and T, with independent internal randomness;
it receives neither a,b nor their generating seeds. Every returned signed
decomposition is checked as an actual curve relation before retention.

Retain the first R>B successful rows, including duplicates and dependent
rows. They have the form

    sum_j C_ij F_j = [a_i]P+[b_i]Q.

Choose a nonzero left-kernel vector v of C using C and independent randomness
only. The kernel computation also receives neither a nor b. Such a vector
exists for every R-by-B matrix with R>B, of any rank. Then

    [sum_i v_i a_i]P+[sum_i v_i b_i]Q=O.

If beta=sum_i v_i b_i is nonzero, the candidate is

    x=-(sum_i v_i a_i)/beta mod ell.

Always check [x]P=Q. A zero beta, failed kernel certificate or invalid group
relation is inconclusive, never a successful logarithm. [pcc:unverified]

## Why full row/column rank is not a probability assumption

For fixed x, the map (a,b)->(a+b*x,b) is a bijection of F_ell^2. Thus T is
uniform in G and independent of the controller's b. Acceptance, row C_i,
and stopping after R accepted rows depend only on T and independent oracle
coins. Conditional on the retained targets and rows, the b_i are still
independent uniform field elements. A kernel vector chosen from C alone is
independent of those tags. [pcc:unverified]

Fix an index h with v_h!=0. Conditional on every other b_i, exactly one b_h
makes beta=0. Therefore the cancellation fails through this denominator with
probability exactly 1/ell. No full-rank or random-row-distribution claim is
needed. This argument requires the declared independence and retention rule;
feeding tags to the oracle/kernel, filtering by tags or dropping duplicates
changes its hypotheses. [pcc:unverified]

The factor-base column universe must be common to these rows. A target-
dependent base that keeps introducing new columns invalidates R>B: for
example, nonzero rows with disjoint fresh supports have full row rank and no nonzero
left kernel. This is an obstruction to that particular linear-elimination
argument, not a lower bound for all adaptive-base or index-calculus algorithms.
[pcc:unverified]

A concrete omitted-hypothesis counterexample uses three identical rows
C_i=(1,0). After seeing b, choose

    v=(b_1-b_2,b_2-b_0,b_0-b_1),

with v=(1,-1,0) when all b_i are equal. It is always a nonzero kernel vector,
but beta=0 always. Thus "any kernel vector, even tag-dependent" is false.
Finite F5 controls exhaust all 125 tag vectors. [pcc:unverified]

## Keep the sparse-linear-algebra cost explicit

Padding C^T with zero rows gives a square sparse matrix A with exactly the
same right kernel. Its dimension R=O(B), since the torus bound L<=8B+8
permits the safe fixed choice R=L+1. Four-summand rows have O(R) nonzeros.
Standard black-box methods can sample the kernel of this singular matrix
without assuming its rank, using soft-O(R^2) field work when the field is
sufficiently large. Kernel sampling must use independent coins and its
returned vector must be checked against C. [pcc:unverified]

Primary source read: Kaltofen–Saunders,
[On Wiedemann's Method of Solving Sparse Linear Systems](https://users.cs.duke.edu/~elk27/bibliography/91/KaSa91.pdf),
introduction and section 4 (singular systems, rank and random solution/kernel
sampling). The source gives O(n log n) black-box calls plus O(n^2 log^2 n)
field work, with sufficiently large field, e.g. 50 n^2 log n elements.
This is a cited algorithmic bound, not a timing of our code. The native
example uses dimension-capped dense Gaussian elimination, which is cubic;
it does not implement or measure that sparse bound. [pcc:unverified]

The use of aP+bQ relations and cancellation to isolate a logarithm is standard;
see Petit–Kosters–Messeng,
[Algebraic approaches for the Elliptic Curve Discrete Logarithm Problem](https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf),
section 2.1 and Algorithm 1. No novelty is asserted for this reduction.

## Bounded restarts replace mandatory per-base energy testing

Assume the earlier proposed energy average E[C]<25W0, and assume the fixed
budgeted, public-input oracle has mean planted hit rate E[s]>=3/4 over
uniform admissible base randomizers. The earlier Cauchy–Schwarz transfer
gives uniform yield at least W0/(400ell) whenever s>=1/2 and C<=100W0.
The two Markov estimates put at least one quarter of bases in that intersection.
This is still conditional on the unproved geometric and solver hypotheses.
[pcc:unverified]

Pick one base uniformly, give it a fixed complete budget, then restart
independently a preregistered bounded number of times. No diagnostic has to
identify a good base before the attempt. For a good base and R=L+1 rows,

    queries = ceil(800*ell*R/W0)

give at least 2R expected successes under independent stateless oracle calls.
A standard Chernoff bound puts insufficient rows below exp(-R/4). The kernel
step then fails only through its own declared algorithmic error or the
1/ell coefficient event. All failed bases consume the same bounded budget;
every successful candidate is checked on the public curve instance.
Adaptive/stateful oracles need their own conditional-success argument.
[pcc:unverified]

Consequently the conditional relation-work term remains soft-O(ell*D/B^3),
with a sparse kernel phase soft-O(B^2), linear sparse storage and no statistical
row-rank assumption. Setup, base generation, scalar randomization, verification,
indexing, all eight raw cofactor sections and restart work must still be priced.
The decomposition budget D=soft-O(B^gamma), gamma<1, the actual geometric
bound, admissible growing family and complete phase bounds remain open.
The optional birthday diagnostic uses quadratic RAM and is not placed on this
linear-memory attack path. [pcc:unverified]

## Corrections and formalization limits

The frozen earlier draft required independent-row/rank assumptions. That was
stronger than necessary for this controller-private protocol; the old artifact
is preserved, and this is an explicit proposed correction, not a rewrite.

Native controls separately cover exact conditional F5 tags, the tag-dependent
counterexample, and full-column-rank and rank-one actual-Curve25519 fixtures
with known secret 1234567. Their rows and final candidate are checked by curve
arithmetic. They do not break an unknown DLP or test full-base decomposition.
[pcc:unverified]

The generic recovery and mixed-label laws already exist in the pinned
CryptoLean.SingleRelation module. The study's ../lean-kernel certificate
reuses those laws and Mathlib's matrix/group-cardinality laws for three
scalar-field endpoints: extraction, mixed-label fiber size and the exact
nonzero-functional annihilator count. They are conditional on the declared
prime scalar field and checked equations; no new generic law was rederived.
The retained-row sampling/selection bridge and the actual point-group instance
are separate formalization obligations. No independence theorem is assumed
as a Lean axiom. PCC-20261116-0bbe9a. [pcc:unverified]
