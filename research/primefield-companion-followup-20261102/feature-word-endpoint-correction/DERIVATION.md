# Endpoint-corrected trace factors

Derived construction under the same certified prime-r, compatible public-source,
m|h=(r-1)/2, primitive zeta, and nonzero-sign hypotheses as T1940. Let L=h/m,
epsilon=chi(k), and t be the simultaneous word stabilizer. Observed zero signs
are independently verified public endpoints before the transcript is constructed.
Prime-field curve/source compatibility is inherited; this component does not
certify primality or create a new character source.

## Linear factors and finite correction

For recorded offsets a_i=b*zeta^j and signs s_i in {+1,-1}, define

    K(x)=product_i (1+s_i*chi(x-a_i))/2.

Let A be the set of DISTINCT offsets and J the exact word indicator. For x not
in A, all character values are +/-1, so K(x)=J(x). At an endpoint a in A, the
observed word requires a nonzero sign whereas the candidate has a zero; thus
J(a)=0, but K(a) can be nonzero. Hence J differs from K only at the known A.
We sum over nonzero x with chi(x)=epsilon. This excludes x=0 and all endpoints
in the other initial-sign phase.

Build K in F_r[Y][X]/(X^m-Y) from the existing shared sections of (X-b)^h.
There is no per-radius square of that section vector. Put R(Y)=(m/t)*K_0(Y).
For a phase root y, orthogonality implies R(y)=(1/t)*sum_(x^m=y)K(x).
The exact indicator is therefore

    I(y)=R(y)-C(y),
    C(y)=(1/t)*sum_(a in A,chi(a)=epsilon,a^m=y) K(a).

The implementation stores C only as a map from known endpoint images to field
weights. It never expands their delta polynomials in the phase quotient. A group
of weights that sums to zero is removed. Multiple occurrences of an offset are
removed from A only: every occurrence remains in the product defining K. This is
necessary because the endpoint's half-factor multiplicity changes its weight.

## Two corrected moments

For the canonical representative R=sum c_eY^e in F_r[Y]/(Y^L-epsilon),

    raw_count=L*c_0,
    raw_sum=L*epsilon*c_(L-1).

These also hold for L=1. Define correction moments d0=sum_y C(y), d1=sum_y y*C(y).
Then exact count=raw_count-d0 and exact first moment=raw_sum-d1, all modulo r.
Only AFTER correction may count be interpreted as the integer in [0,L]. Raw
R is not necessarily a binary indicator, so applying that bound before correcting
would be invalid. The corrected singleton moment gives the unknown image.
The supplied-image phase chart similarly requires R(y)-C(y)=1 before any Bezout
combination. All-mode denominators and the stabilizer/gcd proof are unchanged
from T1940; final scalar multiplication verifies the public target.

## Work, bounds and failure handling

R is still evaluated through explicit sparse circuit-node representatives. The
component removes character-section squaring and avoids constructing endpoint
delta polynomials; it does not eliminate the large phase-quotient representation.
An O(B*m)-size correction map is derived from public offsets, not a search over
unknown scalars. The simple correction builder performs O((B*m)^2) character
powers in the worst case, plus inversions and image powers. It does not stop the
factor loop at a zero weight, making that accounting straightforward. The fixed
bounds B<=4, m<=64 also bound the map size and rotation comparisons.

A separate cap of100000 correction field multiplications applies. Binary-power
multiplications, inversions implemented by exponentiation, offset construction,
sign/half factors, products and weighted first moments are included in that count.
Exponentiation count is a descriptive sub-operation count: DO NOT add it again to
the counted multiplications. Field additions, map/set operations, allocations,
source setup/calibration/calls, curve shifts, circuit construction, DFT/stabilizer,
phase inversions/gcd and final verification remain additional work.
Circuit evaluation retains the predecessor limits on nodes, terms, cumulative
stored coefficient slots and monomial pairs. Stored slots are not total RAM.
Builder cap failure returns the construction-cap route; the current public wrapper
does not export a partial correction-counter object for that failure. The cap is
still enforced before the next multiplication. Nominal successful builds and
algebra-cap failures retain their completed correction counters.

## Exact controls and scope

The paired checks cover all220 retained transcript systems and1140 phase-image
memberships, including L1, both epsilon signs, and repeated radii. Thirty-five
image evaluations differ before the correction is applied, so these checks
exercise its necessity. Four additional synthetic endpoint cases force nonzero
endpoint weights, duplicated factors and opposite phases. A zero correction cap
is refused. Public tests compare both methods on the original29 component cases,
including their one-call guards, with no scalar/image supplied to either method.
Scalar enumeration is confined to deterministic correctness oracles.

Sources checked: T1940 DERIVATION.md, NEXT.md and word.rs, and the inherited
phase trace proof in feature-selected-moments. The unnormalized transform convention
was checked in https://www.fftw.org/fftw3_doc/The-1d-Discrete-Fourier-Transform-_0028DFT_0029.html;
that manual concerns complex DFTs, not this finite-field correction construction.
The finite-field proof is supplied above. KB unavailable: no loaded tools and
scripts/research-kb reports no executable. No novelty, Lean, timing or total-work
superiority claim; independent signed review pending.

## Next

Push selected count/first-moment functionals into the corrected word-product
circuit before explicit phase-quotient expansion. The endpoint correction is
already available in two scalars and a small membership map. Any next contraction
must charge the operands or dual states it constructs; it must not hide a dense
node representation behind a compressed output interface.
