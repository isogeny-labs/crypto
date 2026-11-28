# Fixed published Lucas index 9551

The owner's original target remains an evaluated ordinary elliptic-curve
isogeny crossing a certified prime endomorphism-conductor ratio of at least
80 bits, with repeated paired scalar/ECDLP measurements and IC comparison.
This is a separate fixed published arithmetic fixture, not a completed map.

## Frozen scope and acceptance

OEIS A101087 lists n=9551 only among probable-prime entries. Reconstruct
exactly that fixed input; do not accept a probable-prime test as certification.
Use E: y²+xy=x³+1 over GF(2^9551), descending from the trace -1 curve over F₂.
For U₀=0, U₁=1 and Uₙ=-Uₙ₋₁-2Uₙ₋₂, the Frobenius conductor is |U₉₅₅₁|,
by tₙ²-4·2ⁿ=-7Uₙ². The source geometric endomorphism conductor remains 1.
No second endpoint, kernel or evaluated map is inferred from this equality.

Before any call, pin the sources, GP binary and C++/GMP oracle by SHA-256.
The native oracle must agree with the GP recurrence and matrix power on
the exact trace and conductor; retain small fixed controls, K-571 and 7351.
Require a complete PARI ECPP certificate with the exact subject, a fresh GP
process replay, and rejection after changing the subject. Require exit 0,
empty stderr and explicit complete outputs; GP exit 0 alone is insufficient.
Any timeout/error/missing certificate leaves primality unestablished here.

One fixed certificate attempt: 300-second cap, 512 MiB PARI stack,
single-thread deterministic seed 20261118, no paid resource or auto retry.
Oracle/replay caps are 30/60 seconds. This is local development validation
of a published arithmetic fixture; no parameter sweep, benchmark or ECDLP
session. Store every raw outcome in fresh attempt-a. Freeze before calls;
retain earlier fixture and every historical proof subject unchanged.

Primary review is in sources.json. The author's +1-leading recurrence
and the -1-leading Frobenius recurrence differ by U⁻ₙ=(-1)^(n-1) U⁺ₙ;
absolute values coincide. KB unavailable, wrapper 127. Larger source entries
remain PRPs unless individually certified; no world-record claim is made.

Results are pending. All original endpoint, transport and measurement
obligations remain open.
