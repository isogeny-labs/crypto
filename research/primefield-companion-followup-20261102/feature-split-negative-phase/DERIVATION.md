# Bounded negative-phase splitting

Retain caller-certified prime r and the preceding initializer's restrictions:
m>=3,4m|(r-1), n=(r-1)/(4m), h=(r-1)/2. The curve base field and
existing public scalar-character source keep their separate assumptions.
The new route is optional and receives points, not target scalars.

## Checked public trace

For even n the field has 8|(r-1). For at most trace_trials candidates
z=2,3,..., where 0<=trace_trials<=64, form eta=z^((r-1)/8) and test
eta^4=-1. A passing eta has order8. Since eta^-1=-eta^3, compute
s=eta-eta^3 and explicitly check s²=2. No scalar inverse is needed for this
trace identity. The helper returns None when no candidate passes its fixed
budget, or when 8 does not divide r-1. Invalid budgets return Domain.
It is not assumed that 64 candidates always suffice. Exhaustion retains the
unsplit initializer; no additional source query, randomness or target-dependent
search is introduced. Candidate exponentiation/check costs remain charged.

## Polynomial split and orientation

For n=2t, the Dickson identity gives

    D_n(W)=D_t(W)^2-2=(D_t(W)-s)*(D_t(W)+s).

The two monic degree-t factors are coprime: their difference is 2s, nonzero
in the odd prime field. D_n is squarefree for 0<n<r, so both factors are
squarefree. The factory constructs D_t once and produces both factors for
each original orientation u=+i,-i. Each u therefore labels two canonical
branches. A branch is still filtered by V^n=u; its later transport uses the
same u² as before. This is a bounded polynomial split, not scalar enumeration.

Within one u the factor supports are disjoint, so their lifted polynomial
product equals the prior lifted support. Across the two u values the V phases
are also disjoint. Their union therefore has the same materialized image
polynomial. Later quotient intersections preserve the disjoint partition.
Comparing one new canonical signature against the old signature would be
wrong: qualification groups and multiplies all lifted supports with the same u.

## Capacity, fallback and accounting

Positive and odd-n negative routes retain direct_phase_domains. For even-n
negative routes, if 2*m*(n/2) exceeds slot_cap, reject powered initialization
before searching for eta. Otherwise search within trace_trials. If successful,
preflight each new phase quotient using degree n/2. If exhausted, preflight
and build the old degree-n domain. Capacity exhaustion invokes the original
seeded streaming fallback. Unsupported adapter eligibility retains its existing
public recovery fallback. The trace budget is validated even for identity input.

The bound is on represented section-output coefficients per branch, not total
RAM. Four negative branches replace two; total W support degree counted with
orientation multiplicity remains2n. D_t construction, trace search, copied
factor vectors, doubled canonical initialization/validation, phase powers,
quotient sections, gcds, scalar inverses, source extraction, replay and final
point checks are all part of the work. The inherited Dickson helper can form
an adjacent degree-t+1 polynomial. No elapsed-time or total-work superiority
is asserted. The trace is recomputed per eligible initialization in this
version; there is no uncharged cross-target cache.

## Evidence design and sources

Checks cover bounded/exhausted search, s²=2, exact factor products, coprimality,
squarefreeness and invalid inputs; compare materialized candidates and grouped
phase lifts over repeated updates; and recover public fixture points with
unchanged source and verification. Capacity96 is included between the old140
and new70 logical-slot bounds of the order281,m5 negative fixture. A zero
search budget explicitly exercises the unsplit public fallback.

Sources: ../feature-direct-phase-domain/{DERIVATION.md,phase-domain.rs,
direct-initial.rs}, the copied canonical/lift code, and NIST DLMF18.5.1,
https://dlmf.nist.gov/18.5.E1 (read2026-11-08). The reciprocal polynomial
identity is standard. KB unavailable: MCP absent and scripts/research-kb
status reports no executable; exact predecessor fallback used. Written proof
and finite checks do not constitute a new Lean runtime refinement or signed
independent review.

## Next construction: share the two orientations

The initial sign fixes V^L=epsilon with L=2n and epsilon=±1. Negative phases
u=±i share the same W domain, and the canonical representation can retain
both V orientations until observations distinguish them. This suggests storing
one character-phase branch per split W factor, instead of duplicating it for
each quartic u. It would require changing the lift exponent from n to L and
the transported character from u² to epsilon. Reusing branch.u unchanged
would be incorrect. Positive W domains can remain partitioned while sharing
the same character label. This proposal is not implemented here; its grouped
support, capacity behavior and streamed/powered updates need explicit checks.

The character-phase formulation also suggests removing the bounded fourth-root
search from admission. With only m|h, set L=h/m and use the squarefree domain
D_L(W)-2*epsilon, plus V^L=epsilon. The direct positive/negative factor formulas
apply to L even or odd. In principle this covers the previously excluded odd-L
cases where4m does not divide r-1. The current adapter does not implement that
extension; its eligibility, lift invariant and streamed transport formula must
be changed together and qualified on the newly admitted cases.
