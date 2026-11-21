# Inert consecutive seed and triple-root recovery

2026-11-07, T-1797. Derived construction and unsigned producer software evidence.
Prime fields only. No timing panel, cloud submission, generic group collision
search, extension-field arithmetic, index calculus or GHS construction.

## Exact weighted seed

Let r=4m+3 be a caller-certified prime, h=(r-1)/2=2m+1, and chi(0)=0.
For the consecutive cubic f(X)=X(X+1)(X+2), put z=X+1, so f=z^3-z.
The previous triple module uses B_j=[X^(h-j)]((X+1)(X+2))^h and
T_j=sum_X X^j chi(f(X))=-B_j for j<h.

The involution z -> -z negates chi(z^3-z), since chi(-1)=-1. Therefore
T0=0 and B0=0. Since X=z-1 and the unweighted sum is zero,

    T1 = sum_z z chi(z^3-z)
       = sum_z z^(h+1)(z^2-1)^h
       = -(-1)^(m+1) binom(2m+1,m) modulo r.

For the last equality, only exponent r-1=2h contributes: h+1+2k=2h
requires k=m; every other exponent is positive and below 2(r-1).
Consequently the previously missing seed is

    B1 = (-1)^(m+1) binom(2m+1,m)
       = (-1)^(m+1) product_(i=1..m)(m+1+i) / product_(i=1..m)i.

Both product intervals lie strictly between 0 and r, so the denominator is
nonzero. The proof supplies the sign, not a fitted small-prime convention.

## Shared scalar polynomial blocks

Set w=ceil(sqrt(m)), q=floor(m/w), t=m-qw, and

    P(Y)=product_(i=1..w)(Y+i).

The denominator blocks are P(jw), and numerator blocks P(m+1+jw), for
j=0..q-1. Multiply these q values per side and the t remaining terms,
then perform one checked inversion and apply (-1)^(m+1).

The native implementation builds P through the existing product tree on
roots -1,..,-w, and evaluates it on both progressions through one evaluation
tree and the existing Newton/reversed-division remainder algorithm. It reuses
scalar polynomial arithmetic from feature-native-half-gcd/polynomial.rs;
this seed evaluator does not invoke the resultant or half-gcd routine.
This is established polynomial-block product acceleration, specialized to a
shared numerator/denominator polynomial. It is not a new factorial algorithm.
There is no discrete-log table, collision test, or unknown-scalar iteration.

With polynomial multiplication cost M, conservative time is
O(M(sqrt(r)) log r + log r) field operations and storage O(sqrt(r) log r)
words for retained trees and temporaries. The installed Karatsuba backend
has M(n)=O(n^log_2(3)); this gives a derived exponent log_2(3)/2 for seed
arithmetic, not a measured runtime or a square-root bound for this backend.
The scalar setup is independent of the public target and sign class, and can
be reused. It must still be charged in one-target total cost; no amortized
speed inference is made from the twenty-case software check.

The native domain additionally requires r<=u64::MAX/2. Width and 2q evaluation
point limits are checked before allocating either tree. They are element
caps, not an RSS or dollar budget. Huge input outside these caps returns Limit.
The caller remains responsible for proving primality. No hidden linear
factorial loop occurs in production: only the tail, of length below w, is
multiplied term by term. Linear reference products exist under cfg(test).

## Recovery composition

`Consecutive::new` chooses this seed for r=3 modulo4 and the previous bounded
CM seed for r=1 modulo4. It supports both branches under their explicit caps.
`Consecutive::roots` obtains the exact count from the zeroth moment, rejects
counts beyond max_degree before allocating full root state, and requests the
moments through that count. Newton identities create the monic product of
candidate linear factors. Existing RootSet declaration and implementation are
extracted unchanged; generate.py records source and output hashes.

`RecoverySource` prepares the existing public-point sign source and reusable
seed. Online it observes offsets0,1,2. Any zero sign identifies the endpoint
and still passes the mandatory [candidate]P=Q check. Otherwise it initializes
the triple root polynomial, applies later sign constraints by modular powers
and gcd, and accepts a singleton only after that same forward check. Invalid
target coordinates are rejected by the source before affine point addition.
The source's word, subgroup and projection restrictions remain unchanged.

Full root initialization still needs m_candidates moments and O(m_candidates^2)
Newton field work, with m_candidates generally of order r/8. The code has not
made later-sign moments available directly without root materialization.
Source evaluation, seed setup, root initialization, later gcds and final group
verification all remain distinct costs. No total-speed improvement is asserted.

## Native evidence and scope

validation-01 reports three selected methods pass,160 imported methods filtered.
Nineteen listed primes through100003 compare blocked seeds against separate
linear products. At least one fixture exercises Karatsuba and Newton paths.
Across eleven primes (both modulo4 branches),88 sign classes compare846 moments
and all root coefficients against direct candidate enumeration/product.
All20 existing frozen public-point cases at p509, r479 recover and forward-check,
with maximum initial root degree60 under cap64. An additional public target
-2P checks the third zero-sign endpoint, scalar477. Two-query, degree16,
invalid-coordinate, width and evaluation-count limits and wrong-candidate
forward-verification controls pass. Normal dep-info excludes six legacy paths.

These are unsigned producer assertions. There is no independent signed verdict,
no statistical experiment, no benchmark clock, and no substitution for the
original4563-case cohort. The original cohort is unchanged. Test and normal
builds retain17/24 warning lines, including summaries; raw warnings are saved.
No test, compile or numerical failures occurred in this validation.

## Literature and next construction

Sources checked (KB): MCP unavailable; scripts/research-kb status reported no
executable. Fallback was CONTINUATION's triple-log-derivative and blocked-character-
recurrence sections, existing native polynomial and pair/triple moment modules.
The prior blocked correction uses polynomial matrices and computes a different
quantity; this work specializes the consecutive weighted seed to scalar products.

Read Bostan, Gaudry and Schost, Linear recurrences with polynomial coefficients
and application to integer factorization and Cartier-Manin operator (2007),
introduction and recurrence/Cartier-Manin setup:
https://mathexp.eu/bostan/publications/BoGaSc07.pdf
Its polynomial-block methods motivate reuse; its stronger arithmetic-progression
optimization and FFT complexity are not implemented or attributed to this code.
A 2026 ECCC factorial abstract was found, but full-text download failed twice.
No algorithm or complexity assertion from that inaccessible paper is adopted.

Next: extend the moment representation through additional sign constraints
without constructing all approximately r/8 candidate roots; also investigate
more direct prime-field computation of the central binomial seed. The current
implementation closes the explicit missing consecutive seed for the frozen
r479 family and composes it with real source-to-scalar recovery.
