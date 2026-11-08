# Streaming character-product trace with precision loss

This is a single-prime coefficient computation, not a many-prime average-time
algorithm. Scalar field F_r, r odd prime, h=(r-1)/2, m|h and epsilon in {+1,-1}
are as in the preceding calibrated public source. Let f have degree d, nonzero
constant and leading coefficients, and set C(X)=f(X)^h, degree D=d*h. Integer
lifts of the input field coefficients are used below. Repeated roots are allowed:
the coefficient identity needs no smooth hyperelliptic curve assumption.

## Established recurrence and the characteristic obstruction

The identity f C'=h f' C gives, for k>0,

    k*f_0*c_k = sum_(j=1)^d ((h+1)*j-k)*f_j*c_(k-j).

This is the standard horizontal coefficient recurrence in Harvey-Sutherland,
Computing Hasse-Witt matrices of hyperelliptic curves in average polynomial time II,
section2 equation2, https://math.mit.edu/~drew/conm13352.pdf. Their remarks2.1-2.2
describe matrix-product acceleration and lifting when characteristic divides the
denominator. Their many-prime average bound is not a bound for this implementation.
Here the recurrence is deliberately implemented as bounded sequential stepping.

Simply inverting k modulo r fails at k divisible by r. Worse, the differential
equation alone can leave Frobenius coefficients undetermined even if constant
and leading coefficients and degree are fixed. A concrete control is r19,
f=(1+X)^5, h9. The correct C=(1+X)^45=(1+X)^7(1+X^19)^2 in F19[X]. The different
D=(1+X)^7(1+X^38) has the same degree, constant and leading coefficient and
satisfies fD'=9 f'D, but has a different weighted phase trace for m3. This is a
counterexample to extracting the needed answer from that first-order mod-r
recurrence plus those boundary data. It is not an impossibility result for other
constraints or methods.

## A degree-sized window with finite precision

Let v=v_r(D!)=sum_(j>=1) floor(D/r^j). Initialize modulus M=r^(v+1) and the
constant coefficient c0=f0^h mod M. Maintain the last at most d coefficients,
known modulo the current M. For a step k, compute the recurrence numerator modulo
M, set a=v_r(k), and divide numerator and modulus by r^a. The normalized numerator
is divisible by r^a: both the exact integer numerator and the old modulus are.
Invert (k/r^a)*f0 in the reduced modulus, which is a unit, to obtain the next
coefficient modulo that modulus. Reduce the retained previous coefficients too.
The remaining precision always suffices because the entire v_r(D!) budget was
reserved initially. At the final step the modulus is exactly r. All coefficient
callbacks receive reduction modulo r, so earlier values remain valid.

The implementation refuses before stepping if D exceeds the remaining step
budget or the required modulus exceeds i64MAX. Multiplication uses u128 products
of bounded residues; signed recurrence factors and Euclidean inverses use i128.
The reported window size is retained entries between steps. A new scalar and a
transient pushed entry, VecDeque capacity, input f and arithmetic temporaries are
additional storage. It is an O(d) coefficient window, not a complete RAM bound.
Without the native word-size ceiling, precision lambda=1+v_r(D!) also contributes
to storage: the window requires O(d*lambda*log r) bits, generally quadratic in d
for fixed r scaling, not merely d field elements.
No vector of D+1 coefficients is retained in production; tests alone collect that
vector to compare with ordinary dense polynomial powering.

## Two selected traces from the coefficient stream

For a polynomial C and w in {0,m}, orthogonality on the h roots of X^h=epsilon
in F_r gives

    sum_(x^h=epsilon) x^w C(x)
      = h * sum_(k+w divisible by h) c_k * epsilon^((k+w)/h).

Thus the production callback keeps two running sums. It still visits every
coefficient through D; selected output does not imply sublinear stepping.
A degree-zero polynomial and L=h/m=1 require no special dense representation.

For N observed sign factors, expand the corrected linear product
K=2^(-N)*sum_(S subset {1..N}) (product_(i in S) s_i)*f_S^h,
where f_S=product_(i in S)(X-a_i). For each subset, build its small polynomial and
stream its two character traces. Multiply the accumulated sums by 2^(-N)/t,
where t is the observed stabilizer, then subtract the already-established public
endpoint correction moments. The result is the same exact image count and first
moment as T1943. No unknown scalar is enumerated by this runtime; enumerating
sign-factor subsets is explicit, capped work. Internal helper inputs must be the
same words/zeta used to construct its CorrectedProblem; the public wrapper passes
those objects together by construction.

A singleton uses the existing scalar circuit evaluation for corrected membership
and the all-mode Bezout chart, then independent group verification. The circuit
is built for phase/corrections but never expanded into phase-quotient polynomials
by this path. Circuit-node storage, source arithmetic and correction maps remain.

## Work and representational boundaries

The per-call limits are500000 recurrence steps,4096 subsets and modulus<=i64MAX,
plus inherited source/node/correction limits. A polynomial is refused before
stepping if it cannot fit the remaining step budget. Subsets counts include the
last attempted subset when that refusal follows its polynomial construction.
All refusals retain StreamStats; the public caller also retains completed endpoint
correction counters. The late algebra stage may therefore refuse after source,
construction and endpoint work has already happened.

The stream tracks modular products, unit inversions, precision drops, attempted
subsets and polynomial coefficient products separately. These products can be
modulo powers of r, so they are not interchangeable with earlier F_r monomial
pair counts. Trace scaling, subset signs/offsets, additions, division/reduction,
Euclidean iterations, setup/calibration, public queries/shifts, DFT/stabilizer,
allocation, phase and verification are additional work. No total-work or timing
ranking is asserted.

The representation removes the dense coefficient array for each subset, but
retains O(d*r) coefficient positions and a 2^N subset expansion. Each position
uses up to d recurrence terms: O(d^2*r) modular products plus unit inversions
per subset before accounting for bit cost. For fixed radii
with B<=4, at most B image values are queried endpoints. If every other image
in a phase of size L must be identified uniquely, its nonzero sign words must
distinguish at least L-B images, so necessarily 2^N>=L-B. This elementary fixed-
observation information bound makes universal constant-N identification impossible
as L grows. It does not bound adaptive algorithms, other observations, or all
ways of computing the word-product trace.

## Sources and next construction

Sources checked: T1943 trace/endpoint derivation and implementation; Harvey-
Sutherland II section2 including remarks2.1-2.2. KB unavailable: no loaded MCP and
scripts/research-kb has no executable. The recurrence and precision-lift idea are
established methods, not novelty claims. Independent signed review pending.

Next investigate structured matrix products for the coefficient steps and a
representation of the complete word trace that avoids explicit subset expansion.
These are distinct dependencies: accelerating one subset alone does not remove
2^N subsets. Preserve single-target source costs and both budgets in that work.
