# Native normalized division-polynomial core

## Place in the source

The preserved public sign oracle follows this chain:

    mixed-projective-strides-v2/run.py : setup / Oracle
    shared-projective-grid/evaluator.py : raw
    normalized-progression-v2/kernel.py : phase / division

The complete source also constructs elliptic point progressions and evaluates
polynomial resultants. This component ports phase and division only. It is not
an alternative oracle returning precomputed signs. It computes from a public
curve and affine point, with no scalar label, lookup search or orbit table.
The original kernel is retained byte-for-byte as original-kernel.py.txt for
review; it is evidence, not a Python runtime dependency.

## Derivation and preserved contract

Write W_n(Q)=psi_n(Q), with W_0=0,W_1=1,W_2=2y and the standard short-Weierstrass
W_3,W_4. The original halving recurrences are

    W_(2t+1) = W_(t+2) W_t^3 - W_(t-1) W_(t+1)^3
    W_(2t)   = W_t / W_2 * (W_(t+2) W_(t-1)^2 - W_(t-2) W_(t+1)^2).

The invariant derivation D has D(x)=2y and D(y)=3x^2+a. It preserves the curve
relation because D(y^2-x^3-ax-b)=0. Implement jets (v,d) with the product rule
(v,d)(w,e)=(vw,dw+ve); inversion is (v^-1,-d v^-2). Thus the same recurrences
compute W_n and D W_n without symbolic polynomials of degree proportional to n².
The native engine uses binary jet powers; the historical kernel's jet powers
used a closed derivative formula. Both implement the same algebra, but their
operation schedules differ and no count equivalence is claimed.

For odd r != p with gcd(r,p-1)=1, W_r(Q)=0 and d=D W_r(Q) != 0, the preserved
normalization is c=(-r/d)^e, where e=(r²)^-1 mod(p-1). Consequently c^(r²)d=-r.
Exponentiation is a bijection on F_p^* under the gcd premise. This verifies the
normalization equation; it does not on its own prove the whole source's
quadratic-character formula, which also depends on the progression/resultant
construction. No new theorem about source signs is asserted here.

## Native representation and reuse

`Division::new` binds p,a,b,x,y in a private cache, checks canonical coordinates,
nonsingularity, curve membership and nonzero y, and computes the inverse jet of
2y once. Primality of p is a caller precondition. The arithmetic backend admits
5<=p<=floor(u64::MAX/2); additions/subtractions fit u64 and products reduce u128.
The inverse routine uses signed i128 Euclid. This is a fixed-width backend,
not yet an implementation for cryptographic-size prime fields.

Every new index is evaluated once and retained in a BTreeMap; its dependencies
are recursively halved. Repeated requests hit the cache. Point-bound ownership
prevents reusing an old point's values for a new target. Each entry contains both
value and derivative; map metadata and allocation costs remain. A plain value
request currently computes derivative data too. This matters when a caller
already supplies a seed from the outer recurrence. Such paths need a separate
value-only mode before any claim about total arithmetic savings.

The original `division` creates a new memoized recurrence for each request.
The native object allows reuse between phase and later requested indices. A
repeated phase call still repeats scalar inversion and final exponentiation;
only its W_r jet is cached. No timing or total-cost reduction is measured.

## Executed checks

One standalone rustc1.99.1 test build and one test invocation at source
21dce60cfc189875673f005184f22c6c3ba7f452; both exit0. All3 methods pass:

1. Public generators on E/F353,E/F509,E/F701 reproduce saved seeds184,67,647
   and all9 saved division values. Repeated requests leave retained-index count
   unchanged. Fixtures are projected from the original immutable setup records.
2. Ward's addition relation is checked separately from the halving recurrence:

       W_(m+n) W_(m-n) = W_(m+1)W_(m-1)W_n² - W_(n+1)W_(n-1)W_m².

   Value and derivative agree for4<=m<15 and2<=n<m on all3 fixed generators.
   These are finite algebra/software checks, not a proof for every point.
3. Explicit D(2y)=2(3x²+a), invalid points, singular curve, zero y, small field,
   noncanonical coordinates, invalid r, noncoprime r and nontorsion input
   are checked. Upper-bound arithmetic is not separately stress-tested here.

Source locks, original-input hashes, projection script, receipt and raw logs
are preserved. No complete source call, fresh recovery cohort, benchmark,
cloud job, or workspace-wide Cargo suite ran. Test timing text is not evidence
of performance. Earlier cohorts and Python receipts remain unchanged.

## Next work

Port the native point-grid and normalized recurrence/resultant stages using
this public-point engine. Retain all original curves, arms and4563 target cases.
Before an execution comparison, separate value-only and jet requests, count
cache setup/storage and complete source costs, and publish/admit the frozen
native runtime and full-cohort protocol. No existing EKS job was duplicated.

## Sources

- The three original source modules above and original-kernel.py.txt.
- [Mathlib EDS documentation](https://leanprover-community.github.io/mathlib4_docs/Mathlib/NumberTheory/EllipticDivisibilitySequence.html),
  mathematical background and even/odd relations: established recurrence
  identities used here. This native implementation has no Lean proof artifact.
- IACR2008/444 fulltext fetch returned403; no claim is based on unseen text.
  A direct Mathlib division-polynomial URL was inaccessible; the EDS page was
  successfully read. No novelty is claimed for division recurrences or jets.
