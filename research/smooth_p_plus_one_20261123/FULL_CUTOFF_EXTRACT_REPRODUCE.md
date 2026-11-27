# Reproduce full-cutoff fixed-query extraction

[Frozen protocol](FULL_CUTOFF_EXTRACT.md) · [results](FULL_CUTOFF_EXTRACT_RESULT.md).
This is a whole-output qualification on public fixed targets, with no recovery
or admitted timing comparison.

Use source `42728dba83bf4c4df286077d08c62362499886ae` in a separate checkout.
The [before-call identity](evidence/full_cutoff_extract_v1/precall_identity.json),
published at `d181477253eaceb5ed55597915ec262eac0a4310`, pins the driver,
msolve, protocol, exact command and relevant native sources. The existing
cofactor adapter's SHA-256 is unchanged:
`18e6443599d4aa06e158511447968ed1b67528a81501df78a5f12ae2fd9cbf95`.

Build with Rust 1.99.1, the locked dependencies and two build jobs:

```sh
CARGO_BUILD_JOBS=2 cargo +1.99.1 test --release --locked \
  --manifest-path research/smooth_p_plus_one_20261123/native/Cargo.toml \
  --bin dickson-four-summand --features production-controls
CARGO_BUILD_JOBS=2 cargo +1.99.1 test --release --locked \
  --manifest-path research/smooth_p_plus_one_20261123/native/Cargo.toml \
  --bin dickson-four-summand
CARGO_BUILD_JOBS=2 cargo +1.99.1 build --release --locked \
  --manifest-path research/smooth_p_plus_one_20261123/native/Cargo.toml \
  --bin dickson-four-summand --features production-controls
```

Copy the executable to a new immutable path and record its actual hash.
The recorded driver SHA-256 is
`20f11fbf08b9dd48e0b5c8cc5c779c0c36b4ffc9d7f6c8433e8a439038a36e81`;
the msolve SHA-256 is
`74088d6775c82845faedcbdc24a82d3709e39ee7ebf3e3e4aabde080f9ab3c28`.
[Cofactor reproduction](COFACTOR_OUTPUT_REPRODUCE.md) documents the original
solver/runtime dependencies. The frozen command names the actual library paths.

Use a new evidence directory; never overwrite the recorded panels. Preserve
the [exact wrapper](evidence/full_cutoff_extract_v1/command.sh): one worker,
2 GiB address space, 60-second panels plus five seconds termination grace,
each panel's stdout/stderr/exit capture, and all three planned panels attempted
even after a failure. Their interfaces are:

```sh
/absolute/driver 127 4 /absolute/msolve full-cutoff cofactor-msolve
/absolute/driver 8191 4 /absolute/msolve full-cutoff cofactor-msolve
/absolute/driver 8191 5 /absolute/msolve full-cutoff cofactor-msolve
```

Within each target, full-cutoff precedes cofactor-msolve. Both use the same
three-second per-query limit. A mismatch is recorded before the panel stops;
an incomplete query does not stop later targets. Fixed forms and every chart
share the same deadline. The direct full-set reference is outside the oracle
and checks all accepted rows. No scalar recovery interface is enabled for the
new method in this source revision.

Each complete chart records an exact input prefix, its attempted form equations,
raw RURs and rows. Reconstruct an attempted input by concatenating prefix,
`,\n`, the recorded linear equation and `\n`. Only the original two precise
nonseparation conditions permit another form. Preserve capture omissions and
incomplete query status; neither a row prefix nor an omitted result is a
complete answer.

For recorded data only:

```sh
sha256sum -c research/smooth_p_plus_one_20261123/evidence/FULL_CUTOFF_EXTRACT_SHA256SUMS
jq -s -f research/smooth_p_plus_one_20261123/evidence/full_cutoff_extract_v1/summarize.jq \
  research/smooth_p_plus_one_20261123/evidence/full_cutoff_extract_v1/f127_d4.jsonl \
  research/smooth_p_plus_one_20261123/evidence/full_cutoff_extract_v1/f8191_d4.jsonl \
  research/smooth_p_plus_one_20261123/evidence/full_cutoff_extract_v1/f8191_d5.jsonl
```

If the repository artifact migration has completed, hydrate manifest-backed
files with its documented `artifacts.py pull` route before checking their
hashes. [Development checks](DEVELOPMENT_CHECKS.md#artifact-migration-pending)
records actual publication status. Local availability does not establish a
successful store upload. These controls are producer-local and unsigned.
