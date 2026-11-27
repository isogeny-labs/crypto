# Reproduce the cold-accounting component

This is local software qualification. It does not reproduce an admitted
benchmark. [Protocol](COLD_ACCOUNTING.md) · [result](COLD_ACCOUNTING_RESULT.md).

## Source and build

Frozen algorithm source: `a4cd5ee0b3cf3f83c71c78b96ceb904b69772158`.
The subsequent identity-only commit is `7d4ecdff41c98849a43b9151efb0442ddf120efb`.
A subsequent [source-layout repair](native/SOURCE_LAYOUT.md) preserves the
historical production.rs pin while compiling the unchanged current source from
production_current.rs. The fixed control remains bound to a4cd5ee0b3.
The optional production-source closure and its original validation remain bound
as described in [production reproduction](PRODUCTION_REPRODUCE.md).

From a clean checkout of the frozen source, with Rust 1.99.1 and the recorded
source closure available:

```sh
cargo +1.99.1 test --release --locked --features production-controls \
  --manifest-path research/smooth_p_plus_one_20261123/native/Cargo.toml
cargo +1.99.1 test --release --locked \
  --manifest-path research/smooth_p_plus_one_20261123/native/Cargo.toml
cargo +1.99.1 build --release --locked --features production-controls \
  --manifest-path research/smooth_p_plus_one_20261123/native/Cargo.toml
```

The recorded build uses two jobs and the study's unchanged production-closure
`opt-level=0, codegen-units=256` override; it is not a tuned benchmark build.
[Source identity](evidence/cold_accounting_v1/source_identity.json) pins the
executable, msolve 0.7.5, protocol, command and consumer. Binary hashes may differ
on another toolchain/host; preserve the actual new identity rather than asserting
it is the original executable.

## Fixed control

The [exact executed command](evidence/cold_accounting_v1/command.sh) includes this
workspace's library paths, frozen executable, capture hook, memory bound and
outer deadline. Its destinations are write-once; do not overwrite the retained
control. For a reproduction, copy that script, select a new evidence directory
and native executable, and retain stdout, stderr and exit status separately.
The native interface, without workspace-specific capture, is:

```sh
RAYON_NUM_THREADS=1 timeout --kill-after=5s 400 bash -c \
  'ulimit -v 2097152; exec "$@"' cold-control \
  /absolute/dickson-four-summand 127 4 /absolute/msolve \
  account-suite=781815 draws=30 direct refined cofactor-msolve
```

Supply msolve's runtime library path when required by its installation.
`account-suite` executes each arm in a new process, records every draw, then
emits its summary and parent receipt. `account=781815 draws=30 refined` is the
single-child mode; it cannot supply the parent's full process interval itself.
All accounting modes require `production-controls`. The old `recover=` path is
unchanged. Maximum draws are 256 and maximum suite arms eight.

The fixed control recovered scalar 69 in all three arms, while five cofactor
queries remained unsupported. It emitted 37 JSONL records and exited zero.
These recorded outcomes do not authorize retries to obtain a preferred answer
or a change in the frozen bounds. [pcc:unverified]

## Inspect recorded evidence

```sh
jq 'select(.kind=="account_summary" or .kind=="account_process_receipt")' \
  research/smooth_p_plus_one_20261123/evidence/cold_accounting_v1/control.jsonl
bash research/smooth_p_plus_one_20261123/evidence/cold_accounting_v1/history_check.sh
sha256sum -c research/smooth_p_plus_one_20261123/evidence/COLD_ACCOUNTING_SHA256SUMS
```

The history script compares existing JSON fields only; it runs no solver.
All timers and partial operation counters are diagnostics. For admitted costs,
first register/build the actual workload for the supported EKS route, obtain its
resource/isolation admission, and retain independent replay plus the declared
reference, A/A, warmup and repeated trial schedule. Canonical experiment export
validation is still unavailable here; the local flow receipt does not impersonate
that format.
