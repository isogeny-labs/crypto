#set page(paper: "a4", margin: 16mm)
#set text(size: 10pt)
= Value-only native division-polynomial evaluation
2026-11-03 · Exact software checks · Derived recurrence cost

The new ValueDivision core computes scalar division values without derivative
arithmetic or storage. It is intended for source calls whose normalization
seed is already supplied by the outer recurrence. The prior checked jet core
remains available for actual normalization and derivative requests.

#image("pipeline.svg", width: 100%)

*Construction.* The standard even/odd recurrences are unchanged. Explicit
squares and cubes require exactly six field multiplications and one subtraction
per newly generated nonbase node, after dependencies are available. Inverse W₂
is prepared once. Cache hits perform no recurrence arithmetic.

*Accounting.* The six-product count excludes curve validation, base polynomial
construction, inversion, map lookup and storage overhead. Scalar wrappers also
retain the modulus. No elapsed-time improvement or complete-source cost is
measured. The two engines have separate caches; constructing both duplicates
setup and must be charged if a caller chooses that path.

*Native checks.* At sourceb3b7dd54a375, one rustc build and one selected-test
invocation succeeded. Three new methods passed; three historical jet methods
were filtered out. All9 saved division values match. On the three original
generators and their negatives, scalar and jet results agree at indices0..63,
r, (r+1)/2 and u64::MAX; independent antipodal parity also passes. Domain,
point-membership and torsion-zero controls pass.

*Scope.* The public API takes no target scalar or precomputed sign. The
production path has no jet dependency; the old module is a test reference.
Prime p is caller-supplied and bounded by half the u64 maximum. Maximum-index
checks do not establish arbitrary-size field arithmetic. No fresh sign source,
recovery cohort, cloud job or benchmark ran.

*Next.* Route supplied-seed calls to ValueDivision and normalization to jets;
port geometric progression/resultant stages and connect the sign-mask decoder.
Retain the full original4,563-case cohort and admit its native runtime before
execution. These unit checks do not substitute for the cohort.

*Evidence.* lib.rs, check-lock.sha256, check-receipt.txt and build/test logs.
DERIVATION.md details cost exclusions, exact test scope and provenance.

*Reference.* #link("https://leanprover-community.github.io/mathlib4_docs/Mathlib/NumberTheory/EllipticDivisibilitySequence.html")[Mathlib elliptic divisibility sequences],
and the preserved native jet core and public setup fixtures. Standard
recurrences are specialized; no novelty or statistical claim is made.
