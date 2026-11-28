# Selected normalization of a denominator-cleared trace recurrence

Assume the same certified prime r, h=(r-1)/2, unit f0 and selected phase moments
as T1944. The coefficient recurrence and common-denominator matrix formulation
are established: Harvey-Sutherland II, section2 equations2-4 and remark2.2,
https://math.mit.edu/~drew/conm13352.pdf. This implementation changes normalization
frequency; it does not implement accelerated block products or import the paper's
many-prime average bound.

## Scaled-window invariant

Let c_k be the integer-lift coefficient of f^h. Before step k, retain a window
V_j=U*c_(k-j) modulo the current M, with U a unit modulo r and hence modulo M.
Initially U=1 and the window contains f0^h. The required numerator is

    A = sum_j ((h+1)*j-k)*f_j*V_j mod M.

By the invariant and the ordinary recurrence, A is congruent to U*k*f0*c_k.
Write k=r^a*u with r not dividing u. Set M'=M/r^a. The canonical representative
A is divisible by r^a, since its exact integer numerator and M both are.
Define q=u*f0 mod M' and update

    U' = U*q mod M',
    V_new = A/r^a mod M',
    V_old_j' = q*V_old_j mod M' for retained old entries.

Then V_new=U'*c_k and every retained old entry has the same denominator U'.
Both q and U' remain units. No inverse is needed for this transition. Dropping
one old entry before pushing the new one keeps at most d=degree(f) entries in
the deque. Container capacity, scalar temporaries, U, the input polynomial and
the requested-index set are additional storage.

Precision is exactly T1944's rule: start with M=r^(1+v_r((d*h)!)) and remove only
v_r(k) powers at step k. This supplies at least one residue digit at every stage
and leaves M=r at the end. The native modulus ceiling remains i64MAX, with u128
products and i128 signed factors/Euclidean calculations. General bit storage still
includes precision; a degree-sized window is not d ordinary field elements.

## Normalize only requested coefficients

At a requested k>0, recover c_k mod r as (V_new mod r)*(U' mod r)^(-1).
These are inversions in F_r, not in the larger current lifted ring. Constant c0
is emitted directly, with no inverse. Arbitrary requested indices are checked
against degree(f^h); an empty set returns without stepping.

For the count and first-moment traces, requested indices are the union of

    k = q*h,                   0<=k<=d*h,
    k = q*h-m,                 0<=k<=d*h.

There are at most 2d distinct positive indices. If m=h (L1), the two sets coincide
and there are only d. Thus at most2d inversions are needed per subset polynomial,
versus d*h in the predecessor, while all d*h coefficient steps remain. The set
is constructed in O(d) entries; membership lookups are additional work at each
step. Both trace sums use the same field orthogonality identity as before.

For word moments, the subset expansion, signs, endpoint correction, count bound,
corrected membership and all-mode scalar chart are unchanged. The public wrapper
uses exactly the same observations and budgets. It passes no hidden scalar/image.
The original recurrence and recovery functions remain byte-identical references.

## Tradeoff and controls

Each transition now scales the surviving window entries and updates U. These
extra modular products are counted; selected-output normalization products are
also counted. No count-only reduction in inversions establishes a timing or total-
work improvement. Inversion moduli differ between arms. Subset construction,
source setup/queries, point shifts, correction, additions/reductions, set lookups,
Euclidean iterations, allocation, phase and verification remain additional.

Tests compare all7965 coefficients in45 polynomial controls by requesting every
index (a test-only dense request), then separately compare90 selected character
traces. The selected trace controls use648 field inversions versus15840 prior
unit inversions, with284086 modular products versus210078. The distinction between
full-index testing and sparse production requests is explicit.

Across all36 word systems, including all9 refusals, inversions are184722 versus
2764200 and modular products48651090 versus35982184. Moments, precision drops,
step counts and refusal reasons match. The public29 fixtures likewise match
scalars, calls, steps and cap reasons:12 recovered,11 identity/endpoints,6 caps.
Their selected inversions589824 replace2654208 prior inversions; modular products
rise to53059596 from38436864. Each arm makes332 nominal and27 guard source queries,
718 combined excluding setup. Endpoint correction cost19844 multiplications per
arm is unchanged. There is no timing or total-work ranking.

Step/subset/precision caps are unchanged at500000/4096/i64MAX. Large step and
precision preflight refusals, empty request and malformed-index controls pass.
All failed-case rows remain; no hidden fallback or cap expansion. The inherited
Frobenius ambiguity control still checks why precision loss cannot be ignored.

## Matrix-product opportunity and remaining dependency

Away from characteristic-divisible indices, the update is a linear companion-
style matrix action on V, paired with scalar multiplication of U. At singular
indices the exact r-power division and modulus change are explicit boundaries.
This gives a representation suitable for subsequent products over nonsingular
blocks, with normalization at requested indices. Merely batching these updates
without a faster product evaluation does not reduce the number of steps.

A later block method must retain endpoint checkpoints and precision boundaries,
account for matrix-polynomial construction/evaluation, and compare complete
outputs. Separately, explicit2^N subset expansion remains; this normalization
change does not remove it or alter T1944's fixed-observation information bound.

Sources checked: T1944 implementation/precision proof, Harvey-Sutherland II
section2 and the predecessor's exact receipts. KB unavailable: no loaded MCP
and wrapper reports no executable. No novelty or signed independent verdict.
