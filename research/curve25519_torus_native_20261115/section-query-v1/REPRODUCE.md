# Reproduce with new paths

Use the exact30-input source snapshot and Cargo.lock; registry dependencies
remain the Cargo checksum/toolchain trust boundary. Extract source-v1.tar.xz
into a fresh directory, then run cargo build --locked --release with
--manifest-path tools/torus-quotient-query/Cargo.toml. The original qualified
binary was built in the November17 worktree. Current main has matching18
context inputs; do not silently substitute later dependency edits.

Copy the resulting binary and QUALIFIER.sh read-only to fresh paths. Record
source/binary hashes in distinct BUILD/PRECALL manifests, then execute:

```sh
QUALIFIER ROOT SOURCES.sha256 IMMUTABLE_BINARY EXPECTED_SHA256
IMMUTABLE_BINARY --qualify > NEW_QUALIFIER_JSON
bash tools/torus-quotient-query/run-control.sh IMMUTABLE_BINARY --transport NEW_RAW_JSONL NEW_TRANSPORT_LOG_PREFIX
# Repeat only the transport mode with the same raw path and a fresh log prefix:
# expect nonzero File-exists and unchanged raw hash.
bash tools/torus-quotient-query/run-control.sh IMMUTABLE_BINARY --controls NEW_CONTROL_JSONL NEW_CONTROL_LOG_PREFIX
```

The shell wrapper enforces180seconds and refuses existing stdout/stderr/exit
paths. Rust owns raw output with create_new and flushes every case. A final
completion record and exit0 are required for complete fixed controls. Missing
completion/resource refusal is incomplete, not absence or solver hardness.
Keep stdout/stderr separate from the native raw file. No Python launcher.
Unchanged metadata publication does not require another scientific call.
