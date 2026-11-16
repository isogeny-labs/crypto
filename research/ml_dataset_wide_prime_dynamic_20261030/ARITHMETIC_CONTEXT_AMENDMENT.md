# Arithmetic context amendment before dynamic source freeze

2026-10-30. Supersedes DESIGN.md's borrowed Fp64 signature only; original freeze23727b3a241 remains preserved. Parent directed coordination with the separately owned bounded-field-setup lane. Reviewed external wide-prime-bounded-field-setup-contract-20261030.md. No source freeze or execution preceded this amendment; in-progress source has not been built.

Change `run(&Fp64,...)` to `run(&Fp64Arithmetic,...)`, importing the distinct concrete arithmetic-only context from fp_backend. It has no PrimeField implementation or false square-root contract. Dynamic engine needs p/one/from_u64/to_u64/mul/sqr/sub/neg; zero/equality use valid Montgomery zero/raw words. Dynamic inversion remains charged p-2 binary exponentiation. Canonical report conversion uses to_u64 returning u64. No constructor or context cloning in native run.

Setup is still excluded from native run's accounting: caller obtains the separately bounded prime-verified setup receipt once and subtracts its actual reservation from the same original run budget before native/residual invocations. The new context closes the capability mismatch; this document does not claim its separate implementation or validation has finished. Parent will integrate that lane before compiling this module. No fp_backend edits authorized in this worktree.

Legacy m2/m3 Fp64 callers and tower engine are unchanged. Full original corpus/extraction/group/replay scope remains outstanding as recorded in DESIGN.md. Parent owns validation/publication; this research record is the scoped session handoff because shared session logs are absent and outside delegated ownership.
