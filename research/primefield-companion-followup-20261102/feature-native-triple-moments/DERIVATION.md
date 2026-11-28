# Third-sign moments from two seeds

2026-11-07. T-1795. Derived construction with unsigned native software evidence;
no novelty claim, timing panel, cloud launch, or independent review.

## Domain and representation

Let r >= 5 be a caller-certified odd prime, h=(r-1)/2, tau in F_r excluding
0 and 1, and chi the quadratic character extended by chi(0)=0. The native
implementation restricts r <= u64::MAX/2 and accepts canonical tau and seeds.
All arithmetic is in the prime scalar field F_r. This module consumes public
sign values; it does not change or extend the domain of the existing source
oracle. No unknown scalar labels or enumerated candidate tables are inputs.

Write H(X)=((X+1)(X+tau))^h=sum c_n X^n and B_j=c_(h-j).
For 0 <= j < h, finite-field power sums give

    T_j = sum_(x in F_r) x^j chi(x(x+1)(x+tau)) = -B_j mod r.

Indeed the summed polynomial is X^(h+j)H(X); its only positive exponent
divisible by r-1 is 2h, because h+j > 0 and 3h+j < 4h.
The differential identity g H'=h g' H, g=(X+1)(X+tau), yields

    tau(n+1)c_(n+1) = (h-n)(1+tau)c_n + (2h-n+1)c_(n-1),
    (h+j+1) B_(j+1) = tau(h-j+1) B_(j-1) - j(1+tau) B_j.

Thus B0 and B1 supply every bounded prefix through d < h. All recurrence
denominators are nonzero. The API `Triple::with_seeds` assumes these two
coefficients are correct; it does not authenticate arbitrary supplied seeds.
B0 is the trace of y^2=X(X+1)(X+tau) modulo r. A point count alone supplies
B0, not the second seed required by this construction.

## Pair terms and endpoint correction

For offsets a,b, write q_j=[Z^j]((1+aZ)(1+bZ))^h. Then for j<h,

    sum_x x^j chi(x+a)chi(x+b) = -q_j,
    q_0=1,
    j q_j = (h-j+1)(a+b)q_(j-1) + (2h-j+2)ab q_(j-2).

Set q_(-1)=0. This follows from the same differential identity or reversal
of the degree-2h product. At j=1 the second coefficient vanishes modulo r.
For offsets a_i=(0,1,tau), signs s_i in {+1,-1}, let

    E_i = product_(l != i) (1+s_l chi(a_l-a_i)).

The desired candidate moments are exactly

    mu_j = ( -sum_(i<l) s_i s_l q_(i,l),j
             -s_0 s_1 s_2 B_j -sum_i (-a_i)^j E_i ) / 8 mod r.

Expand the product of the three (1+s_i chi(x+a_i)) factors and subtract
its values at the three distinct zero endpoints. Away from those endpoints
it is eight times the desired indicator. Constant and single-character
weighted sums vanish for j<h; for a single character, substitute y=x+a
and all resulting positive exponents lie between h and h+j < r-1.
Use 0^0=1 at j=0. The canonical residue mu_0 is the exact count, since a
nonzero sign constraint gives count <= h < r. Other moments are residues.
Zero signs are rejected by this module; the existing recovery endpoint path
is separate.

## Effective consecutive seed, r = 1 modulo 4

For tau=2, z=X+1 transforms the cubic to z^3-z. Reflection X -> -2-X
negates the cubic. If chi(-1)=1, substitution in the first weighted sum gives
T1=-2T0-T1, hence T1=-T0 and B1=-B0.

For r=a^2+b^2 with a odd, b even and a+b=1 modulo4, the classical CM
formula for y^2=z^3-z is trace=2a. This exact normalization is stated in
Kowalski, Example1.10 (pages8-9):
https://people.math.ethz.ch/~kowalski/elliptic-curves-families.pdf

`consecutive_cm` searches at most max_attempts successive public field
values for a quadratic nonresidue z, sets sqrt(-1)=z^((r-1)/4), and runs
Cornacchia's Euclidean reduction. It verifies the sum-of-squares representation,
parities and congruence, and returns seeds (2a,-2a) modulo r. It returns Limit
if the bounded search exhausts; no fixed small nonresidue bound is assumed.
A certificate is conditional on caller-certified primality. It is not a prime
certificate. Native operations use u128 intermediates with checked word bounds.

For r=3 modulo4, trace=0, but reflection does not determine T1. Consequently
this automatic constructor rejects that branch, including r=479. The general
supplied-two-seed stream remains available. This round does not synthesize
that missing weighted seed or a fourth-sign stream.

## Costs and next construction

For requested d<h, constant-many batch inversions plus recurrences and endpoint
powers cost O(d+log r) field operations and O(d) words. Denominators are batched
rather than inverted individually. Bounds are checked before allocation;
these are element bounds, not RSS caps. CM seed work adds at most A modular
exponentiations, Euclidean reduction and integer square root: O(A log r)
field multiplications plus logarithmic word-arithmetic iterations. Memory for
seed synthesis is constant in the word model. The search can return Limit.

A bounded moment prefix does not by itself identify one candidate. Supplying
all m candidate moments to Newton conversion still takes O(m) words and the
existing O(m^2) field work, where the triple count is generally of order r/8.
No full root initialization or public-point recovery integration is claimed
in this component. Original4563-case cohort and prior frozen20 recovery cases
are unchanged and were not rerun here. Next pursue the r=3 modulo4 weighted
seed and higher-constraint moment updates before growing root states.

## Sources checked and evidence

Repository pair-moment implementation/clarification, moment-root derivation,
and CONTINUATION triple/Chebyshev summaries informed scope. The prior triple
log-derivative computation streams a field-sized coefficient range; this
construction instead exposes the two central seeds and bounded recurrence.
KB MCP unavailable; prior wrapper unavailable. No absence or novelty inference.
Also read Zagier seminar notes by Voight, page45, but use Kowalski for sign
normalization because the notes' trace typography is ambiguous:
https://jvoight.github.io/notes/274-Zagier.pdf

Validation-02 receipt binds the single native source, rustc version, exact
commands and logs. General checks enumerate every tau=2..r-1 and all eight
nonzero sign classes for r=5,7,11,13,17,31,43,101, comparing all moments through
min(8,h-1) with direct finite sums (14184 equalities). The CM checks compare
trace to direct point counting, seeds to independent dense multiplication,
and1352 moment equalities across20 listed small primes. Scalar prime998244353
checks eight33-moment prefixes and their partition identity; this is an
internal consistency check, not independent large-field enumeration. Mutated
certificates, altered supplied seed, cap/domain/sign controls are retained.
All three selected methods pass. These are unsigned producer assertions.
Validation-01's passing source is snapshotted; validation-02 adds a malformed
large-b certificate rejection before squaring. No test or compile failures.
