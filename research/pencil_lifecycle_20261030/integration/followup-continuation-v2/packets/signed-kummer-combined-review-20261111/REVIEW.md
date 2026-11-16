# Signed Kummer combined runner independent source review

## Result

The runner preserves the complete planned panel and has strong bounded-process,
source-membership, binary-rehash, owner-permit, and cgroup-cleanup controls.
It is **not ready for release** because one required Rust test module is absent
from the staged source set and the driver acceptance does not authenticate the
trace's source identity. A third Cargo workspace-selection joint remains
statically unresolved and should be made unambiguous before lock generation.

No compile, test, lock generation, solver, or cloud execution was performed.

## Blocking findings

### F1 — `driver-controls.rs` is bound but never staged

`src/pruned.rs` ends with:

```rust
#[cfg(test)]
#[path = "driver-controls.rs"]
mod driver_controls;
```

The two driver qualification tests invoked by `durable-worker.py` live in the
external `kummer-signed-driver-20261109/driver-controls.rs`. The packet hashes
that file in `input-bindings.json`, but it is absent from all 49 entries in
`driver-inputs.json`. `stage_driver.py` copies only those 49 entries, the
generated lock, and three derived fixture JSONL files. Therefore the staged
driver crate has no `src/driver-controls.rs`, and the selected test target
cannot compile as written.

Smallest correction: add that exact reviewed file to `driver-inputs.json` at
the exact destination
`research/pencil_lifecycle_20261030/integration/followup-continuation-v2/packets/kummer-duplicate-adapter/src/driver-controls.rs`, then refresh all packet
memberships and hashes. Keep both driver tests and all 11 calls; do not replace
the missing test module with a reduced panel.

### F2 — driver result acceptance does not bind trace source identity

The worker compiles the driver with:

- `KUMMER_DRIVER_REVISION = published_source_revision`; and
- `KUMMER_DRIVER_SOURCE_SHA256 = sha256(staged src/pruned.rs)`.

The trace `ObservedHeader` serializes both values. However,
`panel_acceptance.verify_driver` only checks that `complete_original_row` has
`source_verified == true`. In `pruned.rs`, that field is literally
`status == "verified"`; it records successful scalar recovery, not source
provenance. Acceptance only checks that `observed.jsonl` and the eight fault
JSONL files are nonempty. It never parses their first records or compares the
serialized `Identity.source_revision` and `Identity.source_sha256` with the
published revision and staged source hash.

Thus a result row and nonempty trace can be accepted without demonstrating
that the retained trace was emitted by the source identity declared by this
attempt.

Smallest correction: pass the expected revision and staged `pruned.rs` hash to
the acceptance checker, parse each applicable JSONL header, and require exact
identity equality. Keep the semantic `source_verified` result field, but label
and test it as scalar/outcome verification rather than code-source identity.
Add negative tests for a changed revision and changed source hash.

### F3 — adapter package selection is ambiguous across nested workspaces

The adapter compile command at `durable-worker.py:214` uses the driver crate
manifest and then selects `-p pencil-symbolic-graph`. The driver manifest has
an empty `[workspace]` at line 5 and lists `pencil-symbolic-graph` only as a
path dev-dependency at lines 14–15. The selected package's own manifest has a
separate empty `[workspace]` at line 7. It is therefore not a member of the
driver workspace merely because it is a dependency, and `-p` through the
driver manifest is not a demonstrated way to select its lib test target. The
packet itself records that actual offline dependency/package resolution is
unexecuted.

This review was forbidden to invoke Cargo, so it does not convert the static
workspace defect into a compiler observation. The direct standalone selection
is:

```text
cargo test --manifest-path <driver-source>/research/pencil_large_prime_20261029/Cargo.toml \
  --features reporting --release --lib --locked --offline --no-run \
  --message-format=json
```

That command also requires an exact published lock at the standalone workspace
root; the current staging copies the generated lock only into the driver crate.
The correction must therefore either bind and copy a lock validated for this
standalone invocation or create one reviewed workspace containing both
packages. The executable-selection guard should still require the exact target
name, kind, source path, test profile, and one executable.

## Checks that passed statically

- `driver-inputs.json` declares 49 distinct destinations. Every declared
  source exists and matches its byte count and SHA-256; there are no absolute
  or parent-traversal destinations.
- The packet manifest and `admission-draft.packet_files` matched their current
  files during this review.
- The root test executable selection requires the exact `crypto_lib` lib test
  artifact from the private `src/lib.rs`. Driver and adapter selection likewise
  require one exact named target, target kind, source path, test profile, and
  executable under the private target directory.
- The planned acceptance retains all 28 signed rows, all seven named Labels
  rows, original/disabled/observed equality, eight ordered driver faults,
  forced fallback, the zero-vector case, and all four adapter negatives. Missing
  rows are not silently converted to a smaller denominator.
- Private source and driver-source membership are checked before execution and
  again afterwards; selected executable hashes are checked before calls and at
  the end.
- Materialization and all three compiles share the build boundary ending 600
  seconds before the attempt end. Execution then receives at most 600 seconds
  and retains a 40-second cleanup reserve. The containing unit fixes two CPU,
  16 GiB RAM, zero swap, 512 tasks, one attempt, and no retry.
- The launcher binds the systemd InvocationID and cgroup, requires a live owner
  permit, stops only the captured invocation, and requires inactive/failed
  state, MainPID zero, and an empty captured cgroup before sealing terminal
  cleanup.

## Expected release blockers, not defects

The draft is correctly fail-closed: `released` is false and the generated lock,
published revision, attempt, deadline, authorization, and release receipts are
null. A future release must bind the actually generated published lock and an
explicit 43,200-worker-second suballocation under the current larger campaign;
the present 129,600-second campaign receipt is not interchangeable with the
runner's exact 43,200-second equality check.

## Recommendation

Correct F1 and F2, make F3 unambiguous, refresh the source and packet manifests,
and obtain a fresh independent source review. Only then generate and publish
the lock and fill release admission. The complete panels and finite resource
controls should remain unchanged.
