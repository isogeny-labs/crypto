# Native M31 projective preclosure

Continuation of [crypto PR #5087](https://github.com/isogeny-labs/crypto/pull/5087).
All arithmetic is over p = 2147483647, with the unchanged curve
`y^2 = x^3 - 3x + 21`. This dependency-free native Rust crate replaces the
relevant execution path of the historical Python/C++ diagnostics.

## What is implemented

| Component | Contract |
|---|---|
| `src/quotient.rs` | Exact normal forms for every chart relation; complete bounded quotient-multiplier row construction |
| `src/f4.rs` | Bounded quotient-aware F4 reference prototype, with required chart critical pairs; separate ordinary Buchberger ideal-equality checker |
| `src/f4/projection.rs` | Checked exact geometric projection on the prescribed four-point parameter section, with elimination-ideal generators |
| `src/systems.rs` | Fixed-degree monolithic Semaev construction, preclosed sequential/balanced tensors, guarded multiplicity branch; exact geometric-to-quotient adapter |
| `src/multiplicity.rs` | Independently formulated M31/Fp2 arithmetic, complete fiber-root/multiplicity oracle, exceptional infinity and negative controls |
| `src/system_certificate_tests.rs` | 50 generator-level witness fixtures per projective configuration, exact monolithic controls and corruption tests |
| `src/main.rs`, `src/solve_command.rs` | Immutable matched-input preparation, checking of supplied msolve output, and an explicit bounded single-instance native solve/project command |

The reference F4 prototype computes an ambient Gröbner basis after adding
the fixed chart generators. It is not yet wired into the production F4
visitor or complete relation pipeline. The new projection module extracts
the exact parameter support on the fixed section from its checked basis. The
independent checker recomputes the original ideal in the ordinary polynomial
ring and verifies both containments. Its `Verified` enum denotes this local
algorithmic check, not signed PCC standing. [pcc:unverified]

See [REPORT.md](REPORT.md), [multiplicity.md](multiplicity.md), and the full
[comparison protocol](PROTOCOL.md). A native vector report is in [REPORT.pdf](REPORT.pdf).
Those reports retain their historical frozen bytes. The subsequent projection
component has its own [prospective protocol](projection_20261124/PROTOCOL.md)
and [algebraic argument](projection_20261124/PROOF.md).
Its [qualification report](projection_20261124/REPORT.md) records 40 library
and five CLI tests, and the existing coupled fixture's complete projected
set `{(0,0)}`. The data statement `PCC-20261124-3de82e` has unverified standing.
[pcc:unverified]

## Development qualification

```sh
cargo test --release --manifest-path summation_split/m31_native/Cargo.toml -- --nocapture --test-threads=1
```

The frozen-source replay receipt and manifests live in `evidence/`.
The earlier frozen replay passed all 27 library tests and both CLI tests. See
[the structured qualification receipt](evidence/QUALIFICATION.json) and
[evidence scopes and retained attempts](evidence/README.md). The broader
repository library build reached two compilation caps before tests ran;
its incomplete gate is retained separately. These historical counts do not
include the later projection supplement.
Specialized fibers, generator witnesses, and completed Gröbner computations
are counted separately. None is silently substituted for the requested
50 complete projected Gröbner output sets per configuration.

The one m=3 coupled F4 development fixture completes with 561 pairs and ten
matrices, reaching raw product degree six and post-normal-form matrix-column
degree six. Separate ordinary Buchberger checking completes both ideal
containments. Earlier verifier resource failures are retained. This is one
unique correctness instance, not a comparative runtime result. [pcc:unverified]

## Preparing the identical input stream

The CLI does not execute external solvers or submit an experiment:

```sh
cargo build --release --manifest-path summation_split/m31_native/Cargo.toml
summation_split/m31_native/target/release/m31-projective-preclosure prepare --out /tmp/m31-projective-fp2 --cases 50 --max-summands 4 --mode fp2
summation_split/m31_native/target/release/m31-projective-preclosure prepare --out /tmp/m31-projective-geometric --cases 50 --max-summands 4 --mode geometric
```

Each command creates 400 systems: 50 cases for each of four arms at m=3,4.
It refuses an existing output directory or fewer than 50 cases. `PREPARED`
is written only after all inputs, fixture parameters, expected projections,
and SHA-256 hashes have been written. `PREPARING` remains as a provenance
marker; its presence alone is not completion. The section has four M31
points, so these are deliberately small algebraic correctness fixtures.

Geometric decomposed exports use the same chart-first variable order as the
quotient engine. The manifest records each actual variable order. The two
witness-mode directories include identical monolithic inputs; their 800 files
must not be described as 800 distinct mathematical systems.

The default explicit Fp2 presentation preserves extension witnesses.
The geometric presentation preserves them through algebraic projection.
Forcing every geometric auxiliary variable to lie in Fp would be a different
problem and is rejected as a comparison shortcut.

After an admitted external solve with the auxiliary block eliminated, check
its retained reduced-basis file with:

```sh
summation_split/m31_native/target/release/m31-projective-preclosure check-projection --summands 3 --case 0 --basis /path/to/retained-projection.ms
```

This checker requires the M31 field header and complete msolve `[...]:`
format, rejects leftover auxiliary variables, evaluates every projected
polynomial at all four prescribed section points, and compares the entire
set with independent signed-point arithmetic. A parser/checker pass is not
an execution attestation. Retain the solver command, exit, source/input
identities and elimination-order contract alongside it.

## Explicit native solve and projection

For an admitted single-instance run with a separately frozen executable:

```sh
/path/to/frozen/m31-projective-preclosure solve-one --out /path/to/NEW_OUTPUT --summands 3 --case 0 --arm preclosed_sequential --mode geometric
```

The command supports the existing four arm names (`monolithic`,
`preclosed_sequential`, `preclosed_balanced`, `multiplicity_balanced`),
m=3 or m=4, and `geometric` or `fp2`. It uses the fixed `m3-development-v1`
producer/verifier budgets from the projection protocol. These caps are not
an assertion that every supported configuration finishes. Geometric chart
models use the quotient adapter; explicit Fp2 models retain all their
equations in the ordinary polynomial ring. The output directory must be new.

`input.json` records exact polynomials, variable order, fixture and limits;
`input.ms` is the existing canonical solver export; `basis.json` records the
native basis and construction counters; `result.json` records the ordinary
check, six projection-query counters, exact support and elimination
generators. The signed-point oracle is consulted after extraction and is
never supplied to the projection algorithm. `STARTED` alone is incomplete;
an exceptional error is retained as `error.json`.

Exit codes distinguish matched complete output (0), command/arithmetic error
(1), resource-limited output (2), rejected basis/section (3), and a complete
projection differing from the signed-point oracle (4). Incomplete or rejected
outputs have `solution: null`. An empty point list appears only for a completed
empty support. The command does not measure runtime or RSS or perform campaign
admission; retain those external receipts for a scientific comparison.

## Remaining comparison

Run the full four-arm projected-output gate under the frozen protocol,
then compare native quotient F4, the applicable production Rust F4 and
msolve on identical inputs and term orders. Record actual matrix degrees,
complete construction/guard/solve/extraction costs and peak process RSS.
The current authenticated shared queue, checkpoint and EKS admission gap
is tracked in the [existing access record](../../problems/shared-s3-prime31-execution-gates.md).
Workspace PR #109 already owns the shared connection work.

The full 50-case-per-configuration solver comparison and signed PCC standing
remain open. Keep the research PR draft and unmerged while that work remains.
