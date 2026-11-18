# Balanced reciprocal lift qualification

The optional BalancedCharacterImage replaces the certified lift's iterative
reciprocal substitution with balanced homogeneous recursion and shared powers.
It retains domain provenance, endpoint multiplicity correction, phase-free
initialization, capacity bounds and final point verification.
PCC-20261108-1228b4 is unsigned producer evidence. [pcc:unverified]

Validation01 passes1728 coefficient equalities against an independent
iterative reference and6912 nonzero-point identities. Inputs cover four
fields r=7,19,281,998244353, lengths1..65 and127,128,129,255,256,257,1025,
and six patterns including zero and vanishing high coefficients. These are
polynomial fixtures; the large field is not a large-field recovery run.
Invalid empty/out-of-range inputs are rejected. [pcc:unverified]

The integrated suite passes641 phase-certified lift equalities,1282 endpoint
controls,641 initial phase no-op comparisons and641 exact-cap controls.
There are1936 character-domain factor checks,23552 direct-sign support
comparisons,20352 seeded-predecessor support comparisons and3600 public
recovery cases including228 odd-L cases. All3372 order-281 public cases
retain certified-predecessor outcomes, routes and recorded source statistics.
298 single-call guards and2660 subsequent updates are covered. [pcc:unverified]

The fresh optimized Rust build, focused qualification and inherited regression
passed under90/60/60-second limits. No failed attempt occurred in this
component.140 copied sources and157 compiler dependencies are hash-bound;
inherited stdout is byte-identical. Prior failed attempts remain in their
original components. [pcc:unverified]

With regular polynomial multiplication cost M(n), the balanced recurrence
has the standard O(M(n) log n) arithmetic bound. This is not a timing
comparison or crossover policy. Power-map operations, copies, shifts,
allocations, polynomial scratch memory and output remain charged. Initial
phase-domain expansion still grows with L, and the lift output is explicit.
Independent signed review remains pending.
