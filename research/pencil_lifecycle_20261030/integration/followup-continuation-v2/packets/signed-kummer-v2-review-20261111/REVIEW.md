# Signed Kummer v2 independent source review

Date: 2026-11-11 UTC  
Reviewer: `/root/artifact_final_audit`

## Result

F1 and F3 from the prior review are structurally corrected in the sealed v2
sources. F2 remains release-blocking: the trace identity checker parses a JSON
shape that the Rust trace writer does not emit. Consequently the planned
combined runner would reject every nonempty real driver trace before comparing
its identity. The producer packets remain prospective and unadmitted.

No compiler, resolver, test, payload, source application, Git index mutation,
or canonical-journal write was performed.

## Blocking finding

### F2-R1 — acceptance omits the serialized `Record` envelope

The exact staged Rust source defines each JSONL row as a `Record` with
`sequence` and `event` fields (`relation_stream.rs.candidate:137-140`) and
serializes that `Record` (`relation_stream.rs.candidate:242-248`). The first
driver row therefore has the structural shape:

```json
{"sequence":0,"event":{"kind":"ObservedHeader","data":{"identity":{},"native":{}}}}
```

The exact v2 acceptance source instead reads the first JSON object and requires
`header["kind"] == "ObservedHeader"`, followed by `header["data"]`
(`panel_acceptance.py:8-12`). It never descends through `event`, nor does it
require sequence zero. The two Python identity-negative controls construct the
same non-native top-level `kind`/`data` shape (`acceptance_negative_controls.py:
7-13`), so they cannot expose this producer/consumer mismatch.

`durable-worker.py:238-248` invokes `verify_driver` before the four adapter
negative tests. A real nonempty observed trace will therefore fail in
`trace_identity` (a missing top-level `kind`) and the attempt cannot reach a
passing terminal receipt. The intentional header-write failure remains a valid
special case: `panel_acceptance.py:32-35` correctly requires fault 0 to be an
empty prefix and only sends observed plus faults 1 through 7 to identity
parsing.

Required correction: parse and validate the exact serialized `Record` envelope,
require `sequence == 0`, then compare `record.event.kind`, identity, and complete
native header. Change the revision/hash negative fixtures to use that same
envelope. Preserve the empty fault-0 rule and all other panels. Refresh all
affected hashes and repeat independent source review before release.

The source-only report currently says F2 is addressed and that all other traces
require exact identity. Those statements remain proposed behavior, not a valid
static correction, until this shape mismatch is fixed.

## Prior findings

### F1 — corrected statically

- Both current staged-input inventories are byte-identical, contain exactly 50
  distinct source paths and 50 distinct destinations, and all declared source
  hashes and byte counts match.
- The inventory contains exactly one `src/driver-controls.rs` entry, bound to
  SHA-256 `854c8219f8aaef926b026d3ca101bca6992414b3a666f7a17f505105611b2f9b`.
- `pruned.rs.candidate:939-941` imports that exact sibling path.
- `stage_driver.py:17-25` copies every declared input and copies the separately
  generated driver and adapter locks to their two declared workspace roots.

### F3 — corrected structurally; actual resolution remains unperformed

- The input contract names independent workspace roots for `driver` and
  `adapter`.
- The standalone pencil manifest declares its own `[workspace]` and library.
- `generate_lock.py:46-65` resolves the driver and adapter manifests separately
  and retains two separately named lock outputs.
- `durable-worker.py:212-215` compiles the driver through the driver manifest and
  the adapter through the standalone pencil manifest, with exact artifact-name,
  kind, source-path, test-profile, and executable selection at lines 150-160.
- The actual locks are still null and no resolver was invoked, so this is source
  structure only, not a successful lock/build observation.

## Complete-panel checks

- The sealed signed panel has 28 rows. The producer test requires exactly 28,
  rejects constructor/target skips, requires each recovered scalar to replay,
  and emits a final `completed == 28` summary
  (`native_producer_tests.rs:48-74`). Acceptance requires exact row equality to
  that frozen panel and the final summary (`durable-worker.py:220-232`).
- The labels test contains exactly the seven declared cases and acceptance
  requires their exact ordered names (`native_producer_tests.rs:77-102`;
  `durable-worker.py:233-237`).
- The full driver qualification retains 11 calls: original, disabled, observed,
  and eight ordered fault reruns. Acceptance requires `native_runs == 11`, exact
  original/disabled/observed output equality, and all eight fault joints
  (`driver-controls.rs:36-91`; `panel_acceptance.py:19-37`).
- The second driver test retains both forced native probes: fallback successor
  reconstruction and zero-vector lift disposition (`driver-controls.rs:94-139`).
- All four declared adapter negatives are selected by exact name and a one-test
  passing summary is required (`durable-worker.py:243-249`). No success-only
  denominator reduction is present.

## Caps, custody, and campaign state

- The lock worker establishes one monotonic 45-second deadline before either
  resolution and passes only the shared remaining time to each subprocess
  (`generate_lock.py:5-9,46-64`). The enclosing unit has a 50-second runtime,
  cleanup is bounded before 58 seconds, and the caller is bounded at 59 seconds
  inside the 60-second reservation (`commands.json:2-34`; `launch.py:21-45`).
- The combined admission requires 1,800 build plus 600 execution seconds,
  totaling 2,400 seconds, one attempt, no automatic retry, 2 CPUs, 16 GiB and no
  swap (`durable_io.py:38-59`). Materialization and all three compiles share the
  build boundary; execution is shortened to at most 600 seconds and retains a
  40-second guard (`durable-worker.py:9,58-63,188-219`).
- Both packet manifests and every listed member match. The current manifest
  SHA-256 values are `6467e061c332c1f8ebeb251d4a3c8a643b94bfe2577353c3345a49ff996e7a51`
  and `74dd88670a3635736684af62c6470f3cfa67c8ea8d9d6c3c25e41bc56469dbfb`.
- Both campaign corrections bind 129,600 worker-seconds and deadline
  `2026-11-11T12:00:00Z`, while `actual_prior_seconds` and
  `actual_remaining_seconds` remain null. Their `prior_manifest_sha256` values
  exactly match the preserved `initial-campaign-binding/manifest.json` files
  (`564ecaad...f9369` and `8ee72003...fa2c`).
- Admission remains fail-closed: combined `released` is false, published source
  revision, lease, campaign accounting, release receipts, and both generated
  locks are null. Lock-gate `released` and `serial_owner_confirmed` are false;
  publication, review, and ledger bindings are null. No actual runtime result or
  cost ratio is claimed.

## Disposition

Do not release or invoke these v2 packets as currently sealed. Preserve them as
reviewed evidence, create a successor that fixes F2-R1 without reducing any
panel or changing the campaign/attempt caps, refresh its manifests, and request
a new independent source review.
