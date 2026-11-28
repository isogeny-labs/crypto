# Dickson–S3 actual-source development qualifier

This package builds `examples/dickson_s3_quadratic.rs` against a declared closure
of the repository's production Rust sources. It is a development correctness
target, not a solver-performance benchmark or a replacement for the root
library's validation gate.

`build.rs` selects byte-for-byte source ranges from broad modules and records
their complete source hashes, selection anchors, range hashes and generated
hashes. A pinned source mismatch fails the build. Whole modules are imported
directly or copied unchanged where Rust's child-module resolution requires it.
No replacement field arithmetic, root builder, S3 evaluator, point operation,
signed lift or F4 solver is supplied by this package. The local namespace file
provides only imports and module wiring.

The closure follows the published source-integration pattern in
`research/s3_shared_elimination_20261122/integration_qualifier/`, adapted for the
existing Dickson root builder, the production identity-certificate module and
the new coupling example. That earlier study and its frozen outputs are not
modified. Source selection was refreshed against this PR's base; the generated
manifest is the authoritative inventory of the exact build.

## Targets

- `dickson-s3-quadratic`: the same example source used by the main crate;
  `--plan` prints scope, `--export --m 4 --depth 3` exports polynomial systems,
  and `--check` performs bounded development correctness checks.
- `dickson-s3-identity-records`: native evaluated identity records and negative
  controls using the unchanged production issuer/checker. Its records have
  `proof: null`; they are evaluated certificates, not formal proofs.
- `dickson-s3-source-manifest`: prints the build's source inventory without
  invoking field operations.

Build separately from execution:

```sh
cargo build --release --locked --manifest-path research/dickson_s3_quadratic_20261122/qualifier/Cargo.toml --bins
cargo test --release --locked --manifest-path research/dickson_s3_quadratic_20261122/qualifier/Cargo.toml --no-run
```

Before a correctness call, copy the built executable to a new immutable path,
hash it, and create a write-once pre-call manifest identifying source, command,
scope and expected output paths. Preserve stdout, stderr and exit status in a
distinct post-call receipt. Do not overwrite a prior attempt. The checked-in
evidence records the commands actually used, including failures.

The example's tiny solve uses all-field domains, a degree-eight cap and a
ten-second guard per arm. It compares the complete projected solution set on
one F31 fixture and retains every `VisitReport` counter. Counters and duration
from this unoptimized correctness profile do not constitute comparative
performance evidence. No M31 solver sweep is admitted by this package.

The library target is deliberately built without its legacy modules' test
suites. The example and identity-record binary have their own focused tests.
Root `cargo test --release --lib` and its actual outcome are reported separately.
