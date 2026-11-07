#set page(paper: "a4", margin: 18mm)
#set text(size: 10.5pt)
= Closing the inert consecutive seed
T-1797 • 2026-11-07 • PCC-20261107-01b30a • unsigned producer evidence

The missing weighted seed for consecutive signs at scalar primes r=3 modulo4 now reduces to a central binomial coefficient. A native scalar polynomial-block evaluator supplies it, and a new adapter connects triple moments to public-point recovery under explicit root-degree and query caps.

#image("flow.svg", width: 100%)

== Derivation and implementation
Write r=4m+3 and h=2m+1. Substitution z=X+1 gives the cubic z³−z. Its unweighted character sum vanishes by odd symmetry. In the weighted sum, finite-field power sums select the coefficient with index m, giving B1=(−1)^(m+1) binom(2m+1,m), with B0=0.

Evaluate the binomial as the product of m+2 through2m+1 divided by m factorial. One polynomial P(Y)=product of (Y+i), i=1..w, w=ceil(sqrt(m)), serves both block progressions. Existing native product/remainder trees evaluate them together. Only the shorter tails are multiplied individually; one checked inversion completes the ratio. Primality is caller-certified. Width and evaluation-count caps precede tree allocation.

The adapter retains the earlier CM branch for r=1 modulo4. It handles zero signs at each of the first three offsets through forward-verified endpoints, initializes roots from exact triple moments, and applies subsequent modular-power/gcd constraints. Every returned scalar passes its point equality check.

== Native checks
Three selected methods pass; 160 imported methods are filtered. Nineteen primes through100003 match separate linear-product seed references. Eighty-eight sign classes across eleven primes match846 moments and exact root polynomials against enumerated references. At least one seed fixture exercises Karatsuba and Newton code paths.

All20 saved public-point targets on the displayed curve recover, with maximum initial root degree60 under cap64. A separate target −2P recovers477 through the third zero sign. Wrong candidates, two-query and degree16 limits, invalid coordinates and seed allocation caps have controls. Test/normal builds retain17/24 warning lines, including summaries. There were no compilation or test failures. The original4,563-case cohort is unchanged.

== Cost and remaining construction
Seed setup is target-independent and reusable, but remains charged. With multiplication cost M, the conservative seed bound is O(M(sqrt(r)) log r + log r) field work and O(sqrt(r) log r) words. Installed Karatsuba implies exponent log₂(3)/2; no faster FFT bound or measured speedup is attributed to this code.

Root initialization still requests all candidate moments and uses O(m_candidates²) Newton work, with candidate count generally of order r/8. Source evaluation and later gcd costs remain. Next extend the moment representation through additional signs without materializing that root set, and investigate more direct seed computation.

== Sources and custody
Bostan–Gaudry–Schost (2007), recurrence and Cartier–Manin setup, informs the established block arithmetic. Existing blocked-character, pair/triple and polynomial modules were inspected; KB unavailable. DERIVATION.md contains proof and source links. The28 source/input pins, commands, logs, extraction hashes, manifest and this inspected report are included. No independent signed verdict, timing panel or cloud job.
