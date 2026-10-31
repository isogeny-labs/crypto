# Independent reviewer plan

Reviewer is separate from producer. The first read displayed the full diagnostic and report draft alongside the header; therefore this is NOT a source-blinded review. This plan freezes the remaining source and native checks before examining Basis/GraphSolver or running tests.

Claims: for consistent Ax+bk+c=0 over a prime field, uniqueness iff rank([A b])=rank(A)+1; consistency iff rank([A b c])=rank([A b]). Diagnostic only, not new collector stopping policy or cohort result.

Check mathematical equivalence including empty, inconsistent and zero-nuisance systems. Inspect Basis for prime rejection, modular canonicalization, overflow-safe operations, actual rank accounting, limits, and column bounds. Compare GraphSolver target-pivot conditions with diagnostic criterion, including affine constant convention. Run bounded cargo tests in isolated target directory, pin all source hashes before/after. Report real bugs or exact limits; no claim from hypothetical performance. Independent review does not change research status.
