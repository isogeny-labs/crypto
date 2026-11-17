#set page(paper: "a4", margin: 16mm)
#set text(size: 10pt)
= Native division values and normalization seeds
2026-11-03 · Public-point source component · Exact software checks

A dependency-free Rust core now computes division-polynomial values and their
invariant-differential derivatives from a public curve and affine point. It
preserves the normalization formula of the original sign source and retains
one recurrence cache for subsequent requests at the same point.

#image("pipeline.svg", width: 100%)

*Construction.* The derivation has D(x)=2y and D(y)=3x²+a. Value/derivative
pairs obey the product rule, so the original even/odd division recurrences
propagate both. Under the original torsion and coprimality premises, the seed
solves c^(r²) D(psi_r)=-r. No scalar labels or precomputed signs enter this core.

*Native evidence.* At source21dce60cfc18, rustc1.99.1 built the standalone unit
target and one invocation passed3 methods. The original generators over
F353,F509,F701 reproduce seeds184,67,647 and all9 saved division values.
Repeated requests leave the cache size unchanged. A separate Ward addition
identity passes in both value and derivative on the fixed index ranges;
explicit tangent, curve-membership and domain rejection controls also pass.

*Reuse and cost.* Each new division index is retained with its derivative;
repeated requests use it. Original calls rebuilt independent caches. The
native value-only interface still computes derivative data, and the map
retains metadata and allocation costs. Repeated phase calls repeat the final
inversion and exponentiation. No total-cost or elapsed-time saving is measured.

*Scope.* Prime p is a caller precondition; the backend supports p from5 through
half the u64 maximum. Constructor checks nonsingularity, membership, canonical
coordinates and nonzero y. This is a component port, not a completed elliptic
sign source. No fresh recovery panel, cloud job or workspace Cargo suite ran.

*Next.* Port the point-grid and normalized recurrence/resultant stages; price
value-only versus jet paths. Preserve the original full4,563-case cohort and
admit its frozen native runtime before execution.

*Evidence.* lib.rs, fixtures.tsv, source/input hashes, projection script,
check-receipt.txt and build/test logs. DERIVATION.md states contracts and limits.
The byte-preserved original kernel is included for review, not execution.

*Reference.* #link("https://leanprover-community.github.io/mathlib4_docs/Mathlib/NumberTheory/EllipticDivisibilitySequence.html")[Mathlib elliptic divisibility sequences],
plus the preserved source kernel. Standard division recurrences and derivative
rules are reused; no novelty or Lean certification is claimed. IACR fulltext
fetch returned403 and is not used as evidence.
