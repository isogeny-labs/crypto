# Character-signature exclusion and sparse witness selection

2026-11-03. Derived theorem and source construction. No numerical trials.

## Why this is the next compiler mechanism

The existing feature-span-dual study certifies that X is outside the width-15
cubic feature span over F479; the certified-exclusion compiler then skips
further Gram-weight attempts at an excluded width. Its successful wider
representation is separate. The present component investigates a structural
precheck and a way to construct sparse exclusion witnesses without a Gram
factorization. It does not change the existing compiler or its frozen trials.

Use a caller-certified prime r>3 and 1<=q<r. Put
s(x)=(chi(x),chi(x+1),...,chi(x+q-1)). The original cubic feature row phi(x)
contains all distinct-offset products of at most three coordinates, including
one. Its length is m=1+q+binom(q,2)+binom(q,3). All arithmetic is in F_r.
The elliptic source remains over its original prime field F_p; F_r is the
public scalar coefficient field. No unknown target scalar enters this work.

## Signature cardinality and exact agreement bound

There are exactly q exceptional points x=-d, 0<=d<q. Each has one zero
signature coordinate, since offsets are distinct modulo r. Every other point
has a binary sign vector in {1,-1}^q. Thus the image of s has at most

    K = min(r, 2^q + q)

values. This is an upper bound; some binary patterns can be absent.
For ANY function f on signatures, f(s(x))=x can hold at most once in each
signature fiber, because f(s(x)) is fixed and x is injective. Therefore it
holds at at most K points. In particular, if 2^q+q<r then X is not any
function of these q characters, regardless of polynomial degree. Every
cubic linear combination is excluded, independently of Gram weights.
A necessary condition for any successful representation is 2^q+q>=r.

This is a direct pigeonhole/agreement argument using the Legendre-symbol
alphabet, not a claim of a new general information bound. Source:
[NIST DLMF 27.9](https://dlmf.nist.gov/27.9), definition of the quadratic
character, including zero. No Weil estimate, independence of signs, uniform
pattern distribution, extension field, elliptic collision table, index calculus
or GHS is assumed. The preceding character-sum literature search was unnecessary
for this bound; no unverified character-sum estimate is used.

`signature_bound` returns an analytic_excluded theorem record only when
K<r. Otherwise it returns inconclusive, never represented. It avoids computing
a huge 2^q when q>=bit_length(r). `check_bound.verify` independently checks
the record's arithmetic premises and fixed schema, not primality or a proof
assistant derivation. The mathematical argument above is the proof. This
analytic record is distinct from an explicit normalized sparse dual.

The earlier F479 width-15 exclusion is outside the strict inequality's regime:
2^15+15>479. This new bound does not explain that saved rank defect or replace
its certificate. The frozen recovery panel starts above the present guaranteed
regime, so this component claims no saved-panel speed improvement.

## Streaming sparse certificate without Gram pivots

`Stream.offer(x)` evaluates phi(x), then eliminates it against retained
independent feature rows. Each normalized row retains its transformed target
value and its combination of original accepted rows. Echelon pivots follow
insertion order, not necessarily numerical index order; each new row is zero
at every earlier pivot, which makes a single elimination pass valid.

The invariant for each normalized row v with target beta and coefficients a is

    v = sum_j a_j phi(x_j),  beta = sum_j a_j x_j.

A new row starts with its own coefficient one. The implementation subtracts
exactly the same multiples from its features, target and combination vector.
There are three possible outcomes:

1. A nonzero residual feature row increases retained rank by one; normalize
   it by its leading nonzero entry and retain the new public point.
2. A zero feature row with zero target is a consistent dependency; discard
   the row, charge all work, and make no exclusion or representation claim.
3. A zero feature row with target d!=0 yields a certificate after scaling its
   original-row coefficients by d^-1: sum w phi(x)=0 and sum w x=1.

There are at most m retained rows. A returned certificate has at most m+1
support points, and they are distinct: an offered point already retained can
only produce a consistent dependency. Zero weights may remain. No signature
hash table is built. The algorithm uses feature-specific evaluation and ordinary
exact row elimination; the latter is not claimed as new algebra.

The final independent checker is copied byte-for-byte from the preceding
coefficient construction and recomputes all features from support. It does not
trust producer rows, target residuals, counters or combinations. Production
status says verification is still required. Direct primitive validation and
this final checker have not been numerically run in this round.

## Point-selection guarantees and their exact hypotheses

Suppose K<r. At any nonterminal state, the retained t feature rows are
independent, so some coefficient vector a interpolates all their target values.
We need not construct a for the proof. Let g(x)=phi(x)^T a. The agreement
bound says at least r-K points have g(x)!=x. Every such offered point either
increases retained rank or produces a witness: if its feature row were a
consistent dependency, interpolation would force g(x)=x.

For independently uniform public x in F_r, including repeats, the conditional
probability of progress at every nonterminal state is at least

    delta = (r-K)/r > 0.

After at most m rank increases, the next progress event must be a witness.
Each waiting period has conditional expected length at most 1/delta. Therefore

    E[number of offers until witness] <= (m+1)*r/(r-K).

This is a derived expectation under independent uniform sampling, not a
measured sample count and not a guarantee for arbitrary caller streams. It
uses no sign-independence assumption. The implemented collector takes an
explicit finite limit and supplied iterable, generates no randomness and
returns inconclusive on budget/stream exhaustion. A production randomized
run needs an admitted sampler, seed/custody policy and fixed stopping rule.

There is also a deterministic cardinality guarantee: any K+1 distinct supplied
points must produce a witness when K<r. Otherwise the retained independent
rows interpolate every processed point, contradicting the agreement bound.
This does not make K+1 inexpensive; K can be exponential in q. The code neither
enumerates the field nor silently escalates to this bound. When K=r, neither
guarantee applies, though the stream can still find a valid certificate.

## Derived costs and storage

Let J=q+2*binom(q,2)+3*binom(q,3), h=(r-1)/2, and
L=popcount(h)+bit_length(h)-1. Each offered point uses qL+J products for its
feature evaluation using the preserved binary-power routine.
At retained rank t, elimination against every row (including zero factors)
uses exactly

    A_t = t*(m+2) + t*(t-1)/2

products and the same number of additions/subtractions. This includes all m
feature entries, one target entry and each row's j+1 combination coefficients.
An independent-row outcome adds m+t+2 products and one inversion. A witness
outcome adds t+1 products and one inversion. A consistent dependency adds no
field operations beyond elimination. Native pivot scans, random generation if
used, allocation, modulus, integer arithmetic and final verification are extra.

Stored field entries at rank t are t*m + t + t*(t+1)/2, plus t public point
indices, t pivot indices, immutable labels and meter metadata. A current offer
has m+t+2 transient field values before normalization/copies; Python peak heap
is not measured. O(m^2) storage and O(m^2) field work per offered point imply
an O((m+1)/delta * (q log r + J + m^2)) expected product/addition upper bound
in the strict regime, plus at most m+1 inversions and final checking. The
integer theorem precheck alone needs no feature rows or field operations.

Weights are untouched; an annihilator of all features excludes every weight
choice. None of these costs is a complete scalar-recovery cost. The component
serves compiler admission/exclusion, with the existing separate represented
program path responsible for positive target extraction.

## Verification status and next work

Static audit covers dependency hashes/bytes, Python syntax, independent-checker
imports, and absence of field sweeps in the selector or checker. It does not
establish runtime invariants or measured counts. Numerical, corruption, heap
and performance trials: zero. No existing job was restarted.

Required admitted controls: strict/boundary/vacuous signature bounds; prime
premise misuse; increasing/decreasing pivot indices; independent and both
dependent outcomes; repeated points; capped streams; normalized support;
mutated target/feature/combination/certificate; direct full-matrix comparison;
operation counts; and independence between production and final checking.

Next focus on the unresolved K=r regime used by the retained compiler panels:
use degree-three structure to improve agreement bounds or build a public
interpolant/exclusion separator with verified coverage. A small sampled fit is
not proof of global representation. Preserve the existing width-15 dual and
width-16 represented program as separate controls.
