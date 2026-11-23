# Shared-S3 production-source integration qualifier

This package gives the unchanged `examples/shared_s3_matrix.rs` a small,
declared production dependency closure. It exists because compilation of the
entire root library was terminated by SIGKILL; memory exhaustion was not
independently established. A result from this package is scoped integration
evidence; it is not a successful build or test of the full root crate.

The binary and its tests are compiled directly from the existing example
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

The first command executes only the example's correctness tests.
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

The earlier five-case qualification and its frozen evidence cover bounded F17
F4 integration and recovery, plus a known Prime31 point witness. The current
Prime31 follow-up is specified in [../prime31/PLAN.md](../prime31/PLAN.md):
use actual F4 with the unrestricted auxiliary coordinate, compare the finite
chart solutions with a separate signed-prefix reference, and recover a public
structured known-answer target using relation arithmetic modulo 2147446831.
The coordinate modulus is 2147483647. A new separately captured execution is
required to establish observations for this extension; the old receipt is
not evidence for the changed example.

Finite-chart enumeration does not establish complete point-decomposition
coverage. The exceptional-prefix control retains a valid signed cancellation
relation outside that chart. The structured target/base does not constitute a
uniform-target benchmark. The qualifier does not run the production chain-chart
adapter, rho reference, paid experiment, full cost ledger, or legacy test suites.
Neither a faster native matrix product nor an improved ECDLP complexity follows
from passing these tests.

Until a separately recorded execution succeeds, this directory establishes
only an authored qualification path. Evidence remains PCC-unverified.

## Matched recovery continuation

The next source revision adds Original, independently prepared Fresh, and
Shared recovery arms over the same public parameters. The new
`--check-prime31-recovery-arms` mode retains complete Prime31 arm records for
both target signs and the query-cap/interruption controls, including a
nonzero exit for failed checks. Its JSON is compactly serialized without
omitting any Prime31 arm outcome. Each arm constructs its own family and
factor base; their unpriced work is explicitly recorded.

The separate `matched_arms_recover_every_nonidentity_f17_public_target` unit
test runs all 18 nonidentity F17 targets with all seven arms. With `--nocapture`
it prints every target/arm outcome and query-prefix comparison; an assertion
failure additionally retains the full failed panels. This test is not run by
the Prime31-only CLI mode. Run the example's tests sequentially because
production F4 diagnostics use process-global counters.

The acceptance and evidence contract is
[../matched_recovery/PLAN.md](../matched_recovery/PLAN.md). This continuation
still collects no elapsed-time samples and does not replace the full root
library or experiment-admission gates.
