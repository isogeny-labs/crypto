# Aggregate sign projectors and coefficient moments

T-1792, 2026-11-07. Producer derivation and native software checks; unsigned and not independently verified.

## Exact update without a subset expansion

Let r>=5 be a caller-certified odd prime, h=(r-1)/2 and A=F_r[X]/(X^r-X). This is a split algebra of functions on the scalar prime field, not an extension field. Write C_a=(X+a)^h. Its values are chi_r(x+a), including zero. The exact indicators for a sign s are

    E_(a,+1) = (C_a^2+C_a)/2,
    E_(a,-1) = (C_a^2-C_a)/2,
    E_(a, 0) = 1-C_a^2.

These are 0/1 at every x, including the exceptional endpoint x=-a. Thus the sequential product I <- I*E_(a,s) in A is exactly the indicator of the transcript's candidate set. Repeated constraints are idempotent; contradictory constraints give zero. This update does not expand the 2^q subsets of q observations and requires no endpoint correction or characteristic-power lifting.

The native code forms the binomial coefficients of C_a from the leading coefficient downward. If c_h=1, then c_(j-1)=c_j*j*a/(h-j+1). Denominators lie in1..h and are invertible. A public inverse-coefficient array is built in O(r) scalar-field operations; it is not a scalar-answer lookup table. Every product is reduced using X^0 -> X^0 and X^e -> X^(1+(e-1) mod(r-1)) for positive e, preserving the value at zero.

## Two coefficients give count and first moment

Write the unique degree-below-r representative I=sum_(j=0)^(r-1) b_j X^j. Finite-field power sums give

    N = -b_(r-1) mod r,
    M = -b_(r-2) mod r.

Indeed sum_x1=r=0 in F_r and sum_x x^j is -1 exactly for positive j divisible by r-1, otherwise0. Applying this once to I and once to X*I proves the formulas. After any sign constraint, the integer count is at most h for a nonzero sign and at most1 for a zero sign; intersections cannot increase it. Hence 0<=N<r and its canonical residue is the exact integer count, without lifting. Before the first constraint, count r would alias zero, so moments() explicitly rejects NoConstraints. If N=1, M is the unique candidate. If N=0 there is no candidate; M alone is never treated as a scalar.

## A representation constraint and a next opportunity

For every nonempty proper candidate set, N is nonzero modulo r; therefore b_(r-1)=-N is nonzero and the exact indicator has degree r-1. Degree trimming cannot track shrinking candidate sets in this representation. This is not a circuit, sparsity or arbitrary-compression lower bound.

For j=0..r-2 the same expansion of delta_t=1-(X-t)^(r-1) gives b_(r-1-j)=-sum_t t^j. The coefficient vector therefore contains candidate power sums. A next construction can use these moments to reconstruct a low-degree candidate root polynomial once the count is small, retaining explicit conversion cost. Later sign updates would then use the smaller quotient. No such conversion is implemented here; the earlier algebraic gcd extractor already supplies the relevant root-set interpretation.

## Public-point recovery

RecoverySource receives public curve/base/order and resource caps. It obtains consecutive signs from the standalone source at Q+iP, updates the aggregate projector, and stops only at a singleton. Before returning M it checks [M]P=Q using binary scalar multiplication. A zero sign is processed by the same indicator formula. No unknown scalar label, enumerated coefficient-field evaluations, fiber list, compiled quartic decoder or source-point table enters runtime. Curve arithmetic validates the initial target before its first addition. A cap exhaustion returns Unresolved, never a guessed answer. A logarithmic query bound is not assumed.

The source retains its existing restricted prime-field, word-size and projection-order domain. The destination projector works for caller-certified primes r>=5. All cryptographic computation is native Rust. This implementation uses single-point source queries; shared-window integration is a separate composition task.

## Costs and checks

Per constraint, C_a construction is O(r); squaring C_a, multiplying the indicator and reducing require O(M(r)+r) coefficient-field work. Setup stores O(r) inverse coefficients. State dimension is at most r coefficients, with polynomial multiplication temporaries in addition. The imported engine uses Karatsuba above its cutoff. This replaces explicit subset expansion with dense coefficient arithmetic; it is not a polylogarithmic representation or a measured speed comparison. max_coefficients bounds state dimension, not process RSS. No scalar-field value enumeration occurs in production; loops over coefficient indices still have length proportional to r.

validation-01 passes three selected methods with157 imported methods filtered:324 transcript-prefix checks over r5,7,11,31 compare exact counts, first moments, all function values and idempotence against a test-only enumerated oracle. Repeated/contradictory signs, zero endpoint, empty-transcript and limits are checked. All20 frozen public-point cases at p509,r479 recover with singleton certification and forward equality; a one-query control remains unresolved and noncanonical coordinates reject. Test/normal builds retain16/28 warnings.26 source hashes bound. Normal dep-info excludes the old full-value/transport paths and the quartic/bitplane compiler modules. The original4563-case cohort is unchanged; no cloud job or timing panel ran.

## Sources checked

KB wrapper unavailable. Fallback: current CONTINUATION.md sections on algebraic-shift-extractor, Chebyshev pair constraints and lifted-sign-moments; feature-native-polynomial-constructor/DERIVATION.md; actual frozen compiler and current standalone source. The older lifted-sign-moments report itself is absent from this checkout and git's tracked files, so its summary is context, not an independently rechecked result. No novelty claim.

Sage primary quotient-ring documentation: https://doc.sagemath.org/html/en/reference/polynomial_rings/sage/rings/polynomial/polynomial_quotient_ring.html . Van Dam, Hallgren and Ip, Quantum Algorithms for Some Hidden Shift Problems, introduction: https://www.cse.psu.edu/~sjh26/shift.pdf . The latter identifies the shifted-character problem and distinguishes quantum superposition access from the classical oracle interface here; no quantum algorithm is imported.
