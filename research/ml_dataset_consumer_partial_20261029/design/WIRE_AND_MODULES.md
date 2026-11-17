# Frozen wire inventory and implementation boundaries — proposed

This is a proposed independent consumer contract. Names below describe new consumer modules, not code already implemented. No producer module is imported. `source/` pins the inspected implementations; line references refer to those snapshots.

## Wire inventory

All non-optional fields are required; object field order is not significant for parsing. Decoded duplicate keys and unknown typed-envelope fields reject. Optional fields follow the explicit omission/null rule in DESIGN. Exact binding output order is significant only when computing its digest.

| Type | Fields |
| --- | --- |
| Artifact | path:string, role:string, encoding:string, sha256:hex64, bytes:u64 |
| ObjectRef | key:string, sha256:hex64, bytes:u64 |
| AttemptBinding | schema, run_sha256, plan_sha256, protocol_sha256, config_sha256, source_commit:hex40, binary_sha256, cell_index:u64, attempt_sha256 |
| Anchor | schema, remote, binding_sha256, run_sha256, attempt_sha256, ordinal:u64, receipt:ObjectRef, catalog:ObjectRef |
| Catalog | schema, remote, binding_sha256, run_sha256, attempt_sha256, ordinal, receipt:ObjectRef, cell_complete:false, solver_state_restorable:false, scientific_result_verified:false |
| Receipt | schema, remote, binding_sha256, ordinal, previous:Option<Anchor>, manifest:ObjectRef, artifact_count:u64, artifact_bytes:u64, three false claim fields |
| SnapshotObject | artifact:Artifact, object:ObjectRef |
| Manifest | schema, binding:AttemptBinding, ordinal, previous:Option<Anchor>, context:Artifact[], delta:Option<Delta>, observation:Observation, coverage:Coverage, objects:SnapshotObject[], three false claim fields |
| Observation | status:running/exited/interrupted, observed_unix_seconds:u64, exit_code:Option<i32>, signal:Option<i32>, reason:Option<string> |
| Coverage | capture_present:bool, committed_events:u64, captured_event_end:u64, capture_stopped:bool, stop:Option<Stop>, artifact_only:true, in_flight_solver_memory_captured:false, uncommitted_event_tail_captured:false, unemitted_backend_internals_captured:false, solver_resume_supported:false |
| Delta | binding:AttemptBinding, head:Head, segments:Segment[], artifacts:Artifact[], cursor:Cursor |
| Head | schema, binding_sha256, event_count:u64, byte_end:u64, last_event_sha256, payload_count:u64, payload_bytes:u64, stop:Option<Stop> |
| Cursor | byte_end:u64, event_count:u64, last_event_sha256, next_chunk:u64, payload_count:u64, payload_bytes:u64 |
| Segment | artifact:Artifact, byte_start:u64, byte_end:u64, first_sequence:u64, last_sequence:u64, preceding_event_sha256, last_event_sha256 |
| EventRecord | schema, binding_sha256, sequence:u64, previous_event_sha256, event:any bounded JSON value |

Schema literals:

```text
ml-dataset.partial-anchor/v1
ml-dataset.partial-catalog/v1
ml-dataset.partial-receipt/v1
ml-dataset.partial-snapshot/v1
ml-dataset.partial-snapshot/v2
crypto.ml-dataset.attempt-binding/v1
crypto.ml-dataset.capture-head/v1
crypto.ml-dataset.live-event/v2
```

Stop is exactly `event_limit`, `stream_byte_limit`, `payload_byte_limit`, `payload_count_limit`, `local_byte_limit`, `chunk_limit`, `storage_failure`. Its enum identity is validated; actual exhaustion/failure is not independently established.

The only event-body specialization is `event.kind == "capture_payload_committed"`, then `event.artifact` has the Artifact grammar. Extra event-body properties remain opaque. The closed outer EventRecord and the closed embedded Artifact still reject unknown keys. Event body numbers must be valid finite values representable by the frozen producer Value grammar; raw line hashing is independent of that parse representation. Protocol numbers use their separate token-preserving canonical verifier.

## Context map

Transport paths prepend `context/`. All context encoding is `json`. V2 enforces these exact roles; v1 accepts its finite non-telemetry names with producer-compatible opaque roles.

| Local path | V2 role | Presence |
| --- | --- | --- |
| run.json | restricted-run-binding | ordinal zero |
| plan.json | restricted-frozen-plan | ordinal zero |
| protocol.json | restricted-frozen-protocol | ordinal zero |
| config.json | restricted-provenance | ordinal zero |
| attempt.json | attempt-provenance | ordinal zero |
| public.json | public-instance | optional |
| witness.json | restricted-known-answer | optional |
| factor-base.json | factor-base | optional |
| host.json | host-measurement-context | optional |
| worker-process.json | process-provenance | optional |
| measurement-policy.json | restricted-measurement-policy | v2 ordinal zero |
| telemetry/<phase>-start.json | phase-start | v2 optional ordered prefix |
| telemetry/<phase>.json | hardware-and-resource-telemetry | v2 exact start required |

Phases are exactly `curve-construction`, `known-answer-replay`, `factor-base-replay`, `solver`, in that order. Policy bytes are the consumer's frozen 785-byte LF-terminated file, SHA256 `45c9f43fa4df83c6c478094a84478690dd791e27fccaa19af5b87db33f6b4a50`. Policy/start/end limits are 16384/65536/524288 bytes. The existing checked-phase schema and lifecycle implementation are reused without loosening required nullable fields, enum values or claim flags.

## New module boundaries

Suggested layout under the existing standalone native crate:

```text
src/partial.rs
src/partial/schema.rs
src/partial/budget.rs
src/partial/io.rs
src/partial/chain.rs
src/partial/capture.rs
src/partial/restore.rs
src/partial/report.rs
src/partial/tests.rs
tests/partial_cli.rs
fixtures/partial/<finite-fixture>/...
```

| Module | Responsibility | Explicit exclusion |
| --- | --- | --- |
| partial.rs | Public library entrypoints, fixed admission policy, profile dispatch, whole-operation ownership | No CLI side effects or producer process |
| schema.rs | Independent closed envelope types, bounded visitors, enums, exact path/role/reference validation, fixed binding digest emitter | No transport, no accepted unbounded Value conversion |
| budget.rs | One parent ledger, logical and actual byte counters, preallocation reservation, stream/disk/scratch limits | No per-object reset or dynamic cap supplied by untrusted metadata |
| io.rs | Descriptor-rooted local opens, exact-size hash-bound readers/writers, typed parse from same bytes, local/mock/remote object readers | No publication; no trust in HEAD alone |
| chain.rs | Backward ancestry, metadata cross-bindings, forward exact membership, finite namespace/context/coverage/observation state | No direct filesystem writes or event truth claim |
| capture.rs | Independent genesis, event-line replay, payload membership, segment/cursor/head checks, sticky stop | No solver-specific payload execution, arithmetic or event-kind plugins |
| restore.rs | New output tree, incomplete marker, immutable staging, final fresh audit, exact original metadata retention | No sealed-cell fabrication or resume of a failed output |
| report.rs | Bounded restricted result with precise states and false admission claims | No public training row |
| tests/fixtures | Hand-derived wire vectors, retained producer fixtures, local/mock backend, mutation/rebinding helper | No generator or hardware calls |

Production interfaces should carry admitted identities and opened descriptors, not arbitrary paths or `serde_json::Value` blobs. Suggested conceptual interfaces (names may change before freeze, semantics may not):

```rust
pub fn restore_local(anchor: &Path, expected: &str, objects: &Path, out: &Path)
    -> Result<PartialAudit>;
pub fn audit_restored(root: &Path, expected_anchor: &str) -> Result<PartialAudit>;
pub fn fetch_remote(anchor: &Path, expected: &str, out: &Path)
    -> Result<PartialAudit>;

trait ObjectReader {
    // Caller admits ref before I/O. Returned bytes/descriptor are still hash checked.
    fn read_object(&mut self, admitted: &AdmittedRef, sink: &mut ExactSink)
        -> Result<ReadReceipt>;
}

fn load_chain(reader: &mut dyn ObjectReader, anchor: &PinnedAnchor,
              budget: &mut PartialBudget) -> Result<AdmittedChain>;
fn replay_delta(reader: &mut ArtifactReader, delta: &Delta, prior: &CaptureState,
                budget: &mut PartialBudget) -> Result<CaptureState>;
```

These are design signatures, not a buildable implementation. The mutable `PartialBudget` follows all parsing, path construction, replay and report stages. `AdmittedRef` must be constructible only after grammar/cap/accounting checks. A `ReadReceipt` labels local versus mock versus actual remote; backend trait use alone cannot set fresh-S3 success. No reader can change the declared anchor remote or bypass the exact pinned bytes.

## Checked-phase core extraction

Current `phase::audit_sealed` at `consumer-phase.rs:82` owns both sealed membership discovery and semantic loading; `binding::reconstruct` is private and expects manifest-derived Values. `validate_context` at `consumer-phase.rs:218` emits a sealed-specific validator label and explicitly false partial-chain flag. They cannot be called unchanged with a fabricated manifest to get an honest partial result.

Extract a narrow typed `ContextView`/`AuthoritativeContext` boundary within the consumer. It supplies the five actual context files and admitted finite phase identities through opened hash-bound readers. The common binding constructor computes canonical plan commitment internally from admitted raw plan, validates run/attempt commitments, config/index/shard and frozen protocol, then returns the reconstructed binding. Do not require a partial caller to obtain `plan_sha` through unaccounted legacy `plan_info` parsing first.

Separate a pure phase semantic result from presentation. It contains validated binding, immutable input identities, prefix state, per-phase lifecycle/projection consistency and private recorded observations. It does not set `partial_snapshot_chain_verified`. Existing sealed projection produces exactly the old output; partial projection adds chain evidence only after the independent partial verifier completes. Public exported feature bytes remain unchanged.

The partial caller reserves the existing 32 MiB phase allowance within the whole-operation model and carries all actual phase charges, including large-protocol preflight and fixed envelopes. A verified cache may retain immutable starts/ends and check newly extended prefix state once per ordinal; if it rereads/reallocates, those operations are admitted and byte-bound. Reuse native/schema/protocol logic within this consumer; never import producer `AttemptBinding`, `verify_captured_delta` or `checked::validate_context`.

This extraction may need small changes to `phase.rs`, `phase/binding.rs`, `phase/io.rs` and visibility, plus `lib.rs` and CLI dispatch. New partial types should not force unrelated changes to arithmetic, generic-filter certificates, existing transport fetch, or public feature whitelists. The owner must freeze exact edit scopes before implementation.

## Transport and test seams

Reuse bounded process execution and canonical S3 prefix validation from current `transport.rs` through minimal consumer-owned internal helpers. Do not reuse `fetch_cell`'s completed-receipt parser, root-provenance promotion or sealed-cell audit. Preserve its existing behavior with differential tests. New object reader uses only HEAD and ETag-conditioned GET, exact key and cap, no retries. Keep the backend substitution private/test-only so a release CLI cannot report mock evidence as S3 readback.

Use deterministic injection at four boundaries: object reader after HEAD/before GET; descriptor acquisition between component lookups; bounded input Reader before a sentinel byte; phase context reader before its second read. Tests assert actual attempted I/O counts, exact output files, marker retention and field values. They should not merely compare implementation helper outputs with themselves.

Fixtures have a manifest listing every byte file and checksum plus a separate expected-result JSON authored from the contract. A rehashing helper may rewrite intentionally changed inner bytes and every dependent reference up to a new anchor. That helper is test infrastructure, not the verification oracle. Expected semantic failures remain independently enumerated in ACCEPTANCE.

## Source map and relevant limits

| Snapshot | Lines | Contract used |
| --- | --- | --- |
| producer-snapshot.rs | 23–57; 65–124; 129–261 | Profiles, observations, caps and envelope fields |
| producer-snapshot.rs | 275–320; 327–367 | Path/reference/anchor grammar and cumulative accounting |
| producer-snapshot.rs | 503–531; 586–800 | Hash-bound metadata, coverage, exact membership and forward state |
| producer-snapshot.rs | 803–919; 1171–1272 | Actual context reconstruction, ancestry and restore layout |
| producer-capture.rs | 35–133; 188–288 | Binding, stop/head/cursor/segment/delta fields and digest |
| producer-capture.rs | 638–733; 948–1045 | Cursor/head/event checks and complete delta replay |
| producer-live.rs | 141–208; 504–621 | Finite context role map, immutable acknowledgment, policy-first prefix |
| consumer-common.rs | 15–33; 35–104; 151–239 | Existing no-follow final opens, duplicate parser, byte-binding/path helpers; insufficient alone for ancestor-race guarantee |
| consumer-transport.rs | 24–211; 224–428 | Existing canonical bucket, bounded process and sealed-only transport |
| consumer-phase.rs | 29–41; 82–194; 218–326 | Carried protocol reservations, sealed adapter, pure semantic/projection extraction point |
| consumer-main.rs | 1–114 | Current CLI options and sealed command separation |

T780 and T766 owner snapshots are included only as read-only coordination evidence. They are not a lease for this proposed increment.
