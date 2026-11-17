#set page(paper: "a4", margin: 17mm)
#set text(size: 10pt)
= Division values without a field inverse
T-1825 · 2026-11-07 · parity-normalized native cache

The value-only division cache now has an inversion-free route. It stores f(n)=W(n)/(2y) for even n and W(n) for odd n, with polynomial-only initialization. The public API restores exact W(n) values. The general dyadic quotient source integrates this cache with unchanged seed, resultant, window and sign logic.

#image("flow.svg", width: 100%)

*Derived recurrence.* Set z=2y. Even: f(2m)=f(m)[f(m+2)f(m−1)²−f(m−2)f(m+1)²]. For odd indices, let L=f(m+2)f(m)³ and R=f(m−1)f(m+1)³. Use z⁴L−R when m is even, L−z⁴R when m is odd. These follow by substituting parity factors into the standard division-polynomial identities. No field inverse or exponentiation is called in the new cache.

#table(columns: (2fr, 1fr, 2fr), inset: 5pt,
 [Exact check], [Count], [Coverage],
 [Full division-value equalities], [144,040], [1,108 points × 130 indices],
 [Antipodal identities], [144,040], [Same points and indices],
 [Repeated reads / cache sizes], [144,040], [No retained-index growth],
 [Integrated source points], [1,108], [Reference, half-products, signs],
 [Identity-crossing windows], [24], [Prepared public windows],
 [Additional recovery targets], [617], [5,929 calls; zero unresolved],
 [Inherited public comparisons], [63], [511 source calls])

*Boundary controls.* Eight prime fields: 5,7,17,41,97,193,257,509. All nonzero-y curve points are checked at indices0–127 and the two largest u64 indices. Domain, noncanonical-input, singular-curve, off-curve and zero-y rejections pass. Maximum indices test recursion bounds, not cryptographic-size fields. Normal metadata build has five dependencies, excluding the old value cache and derivative source.

*Exact cost tradeoff.* Let E and O count new nonbase even and odd recurrence nodes, A all even public reads. The old recurrence uses 6E+6O multiplications; this one uses 5E+7O+A. Constructor changes add one multiplication and remove one inverse. Thus delta is O−E+A+1 multiplications and −1 inversion. These are expression counts, not timings. Under an I/M cost model, this route is preferred only when the saved inversion exceeds the extra multiplication cost, with other costs assessed separately.

*Scope and evidence.* Prime/nonzero-y and word-size conditions remain. Source coprimality, r≠p and lane-cap conditions remain. Geometry and polynomial operations retain their own inversions. Original4,563-case cohort unchanged; no cloud or timing run. All focused native builds/assertions pass; full repository Cargo suite not rerun. PCC-20261107-5b8ec1 is unsigned producer evidence. See DERIVATION.md and receipts for replay/hashes.

*Sources and next step.* Mathlib EDS documentation gives standard even/odd identities; Lauter–Stange2008 provides inherited normalization context. KB unavailable. No novelty or Lean-verified claim. Next compute the public request-index DAG cost and combine it with remaining source arithmetic before backend selection.
