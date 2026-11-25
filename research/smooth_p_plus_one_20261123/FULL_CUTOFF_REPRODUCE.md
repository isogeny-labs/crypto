# Reproduce the complete cutoff qualification

[Frozen protocol](FULL_CUTOFF.md) · [results](FULL_CUTOFF_RESULT.md).
This checks finite witness coverage and local ranks. It calls no solver and
does not measure relation collection or performance.

Use source `16f5902e2d833915c65b6f4afa8429cc1c671738` in a separate checkout.
The [before-call identity](evidence/full_cutoff_v1/precall_identity.json),
published at `5f69c1ea75`, pins both immutable executables, protocol, command,
lockfile and relevant native sources. Builds use Rust 1.99.1 and two jobs.
The existing production closure remains optional; the source tests exercise
both feature configurations. Build commands from the repository root:

```sh
cargo +1.99.1 test --release --locked --bin dickson-four-summand \
  --manifest-path research/smooth_p_plus_one_20261123/native/Cargo.toml \
  --features production-controls
cargo +1.99.1 test --release --locked --bin dickson-four-summand \
  --manifest-path research/smooth_p_plus_one_20261123/native/Cargo.toml
cargo +1.99.1 build --release --locked --bin dickson-four-summand \
  --example cutoff_identity --features production-controls \
  --manifest-path research/smooth_p_plus_one_20261123/native/Cargo.toml
```

The certificate example includes the original repository files
`src/cryptanalysis/identity_certificate.rs` and `src/hash/sha256.rs`; do not
substitute a rewritten checker. Their hashes are in the before-call record.
The local binary hashes are:

```text
driver   29f5871241c9c50c0c6f669166d831fc75ffb6540ddf729135f89d1a69e7268f
identity f0e21aed23a07c0c8902bfd9dd10a7fa1ed123b1a9e80b52ac29cffb7a243341
```

Copy your built binaries to new immutable paths and record their actual hashes.
Preserve the frozen evidence; use new output paths and the limits in the
[exact command](evidence/full_cutoff_v1/command.sh). It executes these interfaces:

```sh
/absolute/cutoff_identity > /new/evidence/identity_issue.json
/absolute/cutoff_identity /new/evidence/identity_issue.json
/absolute/driver 127 4 /unused partition-audit
/absolute/driver 8191 4 /unused partition-audit
/absolute/driver 8191 5 /unused partition-audit
```

Keep the timeout, virtual-memory, per-command exit/stderr capture, no-overwrite
and attempt-every-independent-command behavior from the frozen wrapper.
Each fixture audit has 120 seconds plus 5 seconds termination grace and 2 GiB
virtual memory; identity issue/replay each has 30 seconds. No retries or larger
domains are part of this protocol. The solver argument is unused.

Recorded-data checks, without rerunning the qualification:

```sh
sha256sum -c research/smooth_p_plus_one_20261123/evidence/FULL_CUTOFF_SHA256SUMS
jq -s -f research/smooth_p_plus_one_20261123/evidence/full_cutoff_v1/summarize.jq \
  research/smooth_p_plus_one_20261123/evidence/full_cutoff_v1/f127_d4.jsonl \
  research/smooth_p_plus_one_20261123/evidence/full_cutoff_v1/f8191_d4.jsonl \
  research/smooth_p_plus_one_20261123/evidence/full_cutoff_v1/f8191_d5.jsonl
```

The catalog records every image multiset, its coefficients, tuple count and
effective-rank histogram. Each valid sign assignment is a binding with its
generated target; it is not an independent fixed-target relation sample.
Raw data and evaluated certificate are local unsigned evidence. Earlier root
library, canonical-export, artifact migration and admitted EKS comparison
dependencies remain in [development checks](DEVELOPMENT_CHECKS.md).
