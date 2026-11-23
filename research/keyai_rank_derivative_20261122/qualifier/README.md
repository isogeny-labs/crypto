# KeyAI rank and derivative correctness qualifier

This package compiles the actual `useful_rank.rs` and
`semaev_prime_poly.rs` modules, the actual example, and a declared dependency
closure. It reuses the source-selection approach in
`research/s3_shared_elimination_20261122/integration_qualifier` without changing
that earlier qualifier or its frozen evidence.

Broad dependency modules contribute explicit, complete items or contiguous
implementation spans. The selector checks each whole source against
`source-pins.json`, then copies exact byte ranges and verifies them against the
original. Namespace wrappers supply imports only; no arithmetic or solver is
substituted. Whole production modules and the root example are also hash-pinned.
The generated source manifest records hashes, byte offsets, and scope.

The root library is not compiled by this package. Its broad legacy module unit
tests are disabled. A separate test target imports the complete unchanged
`semaev_prime_poly.rs` with its original tests and executes the six tests whose
names contain `support`; other tests in that module are compiled but filtered
out. The root example tests and independent integration tests also exercise
the actual exposed implementation. A successful result is scoped
correctness qualification, not the full-repository gate, a Lean proof, a new
general theorem, or comparative performance evidence.

## Frozen qualification plan

1. Freeze the two touched production modules, root example, dependency closure,
   wrapper files, and independent tests. Replace every `UNFROZEN` pin only after
   reviewing the final source. Keep prior attempts and their raw logs.
2. Resolve cached packages offline. Build with one Cargo worker, a finite
   deadline, and the correctness profile. Preserve build commands, source
   manifest, toolchain versions, exit status, and compiler output separately.
3. Copy the produced binaries to a new attempt directory. Hash these immutable
   copies. Write an exclusive-create pre-call manifest binding commands,
   binary hashes, source manifest hash, and the declared correctness scope.
4. Run only the root example's test binary, the independent integration test
   binary, and the original support-module test binary with the `support`
   filter, with one test thread. Preserve stdout/stderr and exit codes; write
   separate post-call evidence. Never overwrite or reconstruct a pre-call
   manifest after execution.
5. Check all source and binary hashes after execution. Parent-owned full-root
   gates are reported separately; scoped success cannot replace them.

## Required controls

- The 16-summand cancellation family uses actual signed curve points. Distinct
  ordered witnesses `A,A,+B1,-B1,...,+B7,-B7` collapse to one coefficient row;
  each witness replays on the curve, and the row family has rank one. The target
  stays unpinned until an explicit base-log anchor is supplied.
- Distinct nonproportional rows may still be dependent. Test a sum of two prior
  rows, scalar multiples, and a corrupt group equation. Compare row rank and
  base-projection rank to a separate slow elimination implementation.
- Deleting the target column lowers rank by one exactly when the accepted
  consistent system determines that target. Check this against the actual
  matrix's pinned value and replay `[k]G=Q`.
- Normalize relation coefficients modulo the declared subgroup order `r`, not
  the coordinate field modulus `p`. The Prime31 fixture uses `p=2147483647` and
  `r=2147446831`; test their difference and the negative target.
- Compare derivative-support acceptance with an independent slow divisibility
  oracle `g | F^deg(g)`, including irreducible factors, repeated factors,
  nonzero scaled support, and trailing zeros on support. The API deliberately
  requires a positive-degree literal monic candidate, so constant/nonmonic or
  trailing-zero candidates must fail.
- Check `deg(g)<p`: in characteristic three, `g=X^3` has zero derivative but
  unsupported roots for `F=X-1`; the guarded API must reject it.
- Squarefree support is sufficient for the local Jacobian to be nonsingular.
  Nonsquarefree support remains accepted by the support predicate; include a
  singular repeated-support control and a harmless repeated factor away from
  the candidate. Neither property establishes solving degree or search cost.
- Validate Prime31 field arithmetic with a separate integer modular oracle,
  including boundary residues. Primality is an explicit caller precondition
  for `Fp`; tests use declared primes and do not treat `Fp::new` as a certificate.

## Commands after source freeze

```sh
CARGO_BUILD_JOBS=1 timeout 300 cargo build --offline --release --locked \
  --manifest-path research/keyai_rank_derivative_20261122/qualifier/Cargo.toml \
  --target-dir /workspace/scratch/99b214ce852d/build \
  --bin keyai-rank-derivative-source-manifest
CARGO_BUILD_JOBS=1 timeout 300 cargo test --offline --release --locked \
  --manifest-path research/keyai_rank_derivative_20261122/qualifier/Cargo.toml \
  --target-dir /workspace/scratch/99b214ce852d/build \
  --bin keyai-rank-derivative-controls --test adversarial \
  --test support-module-tests --no-run
```

These commands compile only. Execute the copied test binaries only after their
write-once pre-call manifest exists. Do not run a benchmark or attach timing
interpretations to this correctness profile.
