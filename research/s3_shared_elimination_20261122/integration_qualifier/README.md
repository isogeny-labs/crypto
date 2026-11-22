# Shared-S3 production-source integration qualifier

This package gives the unchanged `examples/shared_s3_matrix.rs` a small,
declared production dependency closure. It exists because compilation of the
entire root library was terminated by SIGKILL; memory exhaustion was not
independently established. A result from this package is scoped integration
evidence; it is not a successful build or test of the full root crate.

The binary and its five tests are compiled directly from the existing example
path. No fixture, expected result, equation, solver option, or test body is
rewritten here. The library is named `crypto_lib` so the example's imports stay
unchanged. Cargo builds this library in production mode for the binary test
target; legacy module unit tests are not part of this qualifier. The adjacent
`../qualifier/` package covers the new modules' focused component tests.

## Included production code

The following modules are included in full with `#[path]`: `f4_fp`, including
its real reuse, pair-certificate, and certified-relation children;
`exact_modular_matrix`; `shared_macaulay`; `fp_backend`; `fx_hash`; `jv_bitops`,
including its uniform-sampling child; `bitops`; and `ic_measurement`. The two
certificate crates are actual repository path dependencies.

For F4, the build script copies the complete parent source to
`OUT_DIR/f4_fp/mod.rs` and its three complete children beside it, then emits an
absolute `#[path]` binding. This preserves the parent's implicit child-module
resolution. Each copy is checked byte for byte and hashed; none of the original
module declarations or function bodies changes. Other complete modules are
loaded directly from their repository paths.

The broader modules contribute exact source byte ranges selected by the native
Rust `build.rs`:

| Source | Included items |
| --- | --- |
| `challenge_ladder` | Its own modular product, power, and primality test |
| `glv_invariant_base` | Arithmetic and the complete polynomial root-extraction dependency closure |
| `groebner_f4` | `Ordering` and `cmp_monomial` |
| `ic_boundary` | Field/group ledgers, phase counters, map/arithmetic helpers, complete `IncrementalGauss` and `CountedGroup`, the cubic automorphism constructor required by `PrimeCurve`, complete `PrimePoint`/`PrimeCurve`/counted field operations, the full `CountedGroup for PrimeCurve` implementation, complete `FactorBase` construction, and `lift_abscissae` |
| `prime_fast` | Actual Montgomery arithmetic and curve methods needed by the full group implementation; only the unused `from_params`, `to_textbook`, and `from_textbook` adapters are omitted from that inherent implementation |
| `residual_walk` | Its own modular arithmetic, root and primality helpers, preserving actual `jv_bitops` hooks |
| `symmetrized_semaev` | Actual numeric scalar helpers, `s3_quad_coeffs_u64`, and `eval_s3_u64` |
| `pq_groebner_f2` | Actual mask aliases/constants required by `fx_hash` |
| `prime_hyperelliptic::ops` | Actual counter types and operations, excluding the unused divisor-hash helper and legacy tests |

Each extracted source file has a pinned whole-file SHA-256. Every range uses
unique start and end anchors, with the end excluded. The build rejects missing
or ambiguous anchors, source drift, overlapping selections, and a changed
write-back. Selected bytes are concatenated in source order without inserted
function bodies, attributes, braces, or other bytes. Imports needed by those
items live visibly in `src/cryptanalysis.rs`. There are no alternate field
operations, root finders, group operations, solvers, or no-op counter functions.

## Correctness commands

From the repository root, after the development build slot is available:

```sh
cargo test --release --offline --manifest-path research/s3_shared_elimination_20261122/integration_qualifier/Cargo.toml --bin shared-s3-matrix -- --test-threads=1
cargo run --release --offline --manifest-path research/s3_shared_elimination_20261122/integration_qualifier/Cargo.toml --bin shared-s3-matrix -- --check
cargo run --release --offline --manifest-path research/s3_shared_elimination_20261122/integration_qualifier/Cargo.toml --bin shared-s3-integration-source-manifest
```

The first command executes only the example's existing five correctness tests.
The second prints its correctness report. Without `--check`, that example
prints its plan. There is no benchmark flag. Existing solver deadlines remain
cancellation guards, and incomplete outcomes are errors, not proofs of no
solutions. The package's release override compiles the source closure without
optimization to reduce compiler memory; these commands provide no performance
comparison or complete-cost result.

The first integration execution exposed reversed visitor predicates in the
example: the actual `solve_visit` API stops when a callback returns `true`.
Exhaustive enumeration now returns `true` only on a domain-cap breach; relation
search returns `true` after a verified lift. The existing complete-outcome guard
and solution-set equality checks remain in force. The toy recovery test also
checks that found relations stop traversal. The failed receipt remains separate
from the subsequent corrected-source qualification.

The source-manifest binary prints whole-file hashes, exact extracted ranges and
their hashes, resolved dependency provenance, and the declared exclusions. The
manifest records a build's inputs; a passing test receipt is separate evidence.
It is not a complete hermetic build manifest: registry package bodies, toolchain,
linker, system libraries, CPU state, and environment flags must be captured
separately for an experiment.

## Limits of the evidence

The finite auxiliary chart is exhausted only in the bounded F17 fixture. Its
exceptional-prefix control demonstrates a valid signed cancellation relation
outside that chart. It does not establish complete point-decomposition coverage.
The example's F4 integration and toy public-target recovery use the integer
backend; all three arithmetic backends are compared for bounded preconditioning,
consequences, and witnesses. The committed p31 curve check is a known-point
witness check, not a p31 ECDLP solve. The qualifier does not run the production
chain-chart adapter, rho reference, paid experiment, full cost ledger, or legacy
test suites. Neither a faster native matrix product nor an improved ECDLP
complexity follows from passing these tests.

Until a separately recorded execution succeeds, this directory establishes
only an authored qualification path. Evidence remains PCC-unverified.
