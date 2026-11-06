# Independent partial-snapshot consumer — proposed contract

Status: design for parent review, 2026-10-29. This document does not authorize implementation, producer execution, cloud access, retries, publication, spending, or an extension of the existing deadline. The original USD200 total ceiling and worker-role restrictions remain unchanged. No partial-consumer implementation or validation result is claimed.

The implementation base is the standalone consumer at `0ceec70eb18d0b0c8642e09b68879f527cd23bdf`, including its pending independent checked-phase adapter. The producer contract is `27679c4e59bc38e8c18e34dbc9687f9a2d5eb317`; the later `b7c11` retention change is not a new runtime contract. Read-only source snapshots are in `source/`. The producer inventory is `/home/ec2-user/ml-dataset-resume-20261029/partial-consumer-contract-inventory/INTERFACE.md`.

## Outcome and scope

Add one independent partial-lineage verifier with local-object, restored-tree, and read-only S3 transport adapters. A successful result requires all predecessor metadata, every admitted artifact, committed-event byte replay and payload membership, reconstruction of the actual authoritative context, and the applicable checked-phase prefix. It restores artifacts, not a solver. The adapter must never synthesize a sealed-cell manifest, call sealed-cell audit with fabricated membership, emit a training row, infer a missing capture stream, or substitute metadata-only success when captured events exist.

The smallest complete software increment includes capture deltas from its first release. Separate deployment stages may postpone fresh S3 access; they may not silently postpone captured-event validation while calling local partial restore complete. The new adapter uses independent consumer implementations, not producer validation helpers or imports. It does not invoke the producer executable, a native arithmetic verifier, Python, a solver, a counter collector, or a generator.

Proposed CLI, with duplicate/unknown options rejected:

```text
dataset-consumer partial-restore --anchor FILE --expected-sha256 HEX --objects DIR --out NEW_DIR
dataset-consumer partial-audit --restore DIR --expected-anchor-sha256 HEX
dataset-consumer partial-fetch --anchor FILE --expected-sha256 HEX --out NEW_DIR
```

`partial-restore` reads a caller-selected local mirror whose paths are object keys. It never follows the anchor's `file://` location or opens a network connection; the declared remote remains an exact lineage identity. This makes archived fixtures portable without rewriting their anchor bytes. `partial-audit` rechecks the stored original anchor and all retained metadata/artifacts, ignoring prior success reports. `partial-fetch` reads only the existing canonical S3 bucket/prefix using bounded HEAD/GET, with no LIST, PUT, DELETE, instance, EKS, IAM, or publication action. This design permits its implementation and mock testing only after approval; a real read still requires the parent's existing authorization/accounting decision.

Both restore commands require a new destination. There is no overwrite, resume, merge, in-place repair, discovery of a latest catalog, or fallback to a different anchor. A failure retains a visibly incomplete isolated directory and produces a nonzero exit. `partial-audit` is read-only. Existing `audit`, `export`, `fetch` and their v1 sealed outputs remain byte-compatible.

## Trust, profiles and exact parsing

The supplied expected SHA256 pins the raw anchor bytes. It supplies integrity relative to the caller's chosen anchor, not authorship or cryptographic authenticity. A self-consistent fabricated chain can pass structural checks; the report must not imply trusted generation.

All typed metadata rejects unknown fields, duplicate decoded keys, trailing JSON, invalid UTF-8, wrong types, negative unsigned values, nonintegral counters, nonfinite numbers and overflow. Require every non-optional producer field. Preserve producer `Option<T>` compatibility: omitted or JSON-null `previous`, `delta`, observation exit/signal/reason, coverage stop and head stop normalize to `None`; reject every other type. Required nullable fields inside checked-phase schemas retain the existing independent phase parser's exact rules. This is an explicit distinction, not a global relaxed-null mode.

Anchor, catalog and receipt remain their `/v1` schemas for both profiles. Dispatch only on each manifest's schema:

| Profile | Manifest schema | Context and phase behavior |
| --- | --- | --- |
| Legacy partial | `ml-dataset.partial-snapshot/v1` | Five required context files plus the five optional legacy context names; no measurement policy or telemetry context; capture envelopes still use live-event/v2. Reconstruct actual binding independently. Report phase validation `not_applicable_legacy_profile`. |
| Checked partial | `ml-dataset.partial-snapshot/v2` | The same context plus exact fixed policy and finite phase prefix, at most 19 identities and 16 MiB context bytes. Require policy in ordinal zero and validate every admitted prefix. |

Reject an unknown manifest schema or profile switch anywhere in a lineage. Reject telemetry or policy smuggled into v1, including misleading roles. In v2 enforce exact path-role-encoding tuples. Legacy producer validation accepts arbitrary nonblank roles on its finite context paths; preserve those roles as opaque restricted provenance instead of inventing a new v1 role requirement. In both profiles all context encoding is `json`, and no role alone admits a public feature. Optional legacy context objects are integrity-checked and preserved; their domain semantics are not newly certified by this adapter.

Hash syntax is lowercase hexadecimal: 64 digits for SHA256, 40 for source commit. All integers use the producer's declared signedness and width. Observation status is exactly running/exited/interrupted, reason at most 4096 UTF-8 bytes, signal 1..127 when present; running forbids exit/signal. Times cannot regress; terminal status cannot return to running. Do not invent a requirement that terminal status has an exit code, or reject exited-to-interrupted solely for being a different terminal label.

## Wire fields and digest domains

Use independent closed structs matching the source snapshots. Their field inventory is frozen in `WIRE_AND_MODULES.md`. Object identity is `{key, sha256, bytes}`. Artifact identity is `{path, role, encoding, sha256, bytes}`. Normal object keys must be exactly `sha256/<first-two-hex>/<sha256>`. Catalog keys must be exactly `ml-dataset/interrupted-attempts/<run>/<attempt>/snapshots/<ordinal-as-20-digits>.json`.

Artifact paths are nonempty, at most 1024 bytes, relative UTF-8 normal components, with no absolute prefix, backslash, ASCII control, empty segment, `.` or `..`. Role/encoding are nonblank with producer bounds 256/64 bytes. Detect a file/directory prefix conflict before creating anything. Payload names are a single basename of 1..255 bytes excluding slash/backslash/dot/dotdot; event chunk names use the exact 12-digit counter.

Three different hashes must never be conflated:

1. Artifact and transport references hash their exact original bytes, including whitespace/newlines. Metadata parsing must use the same bounded byte buffer whose hash was checked.
2. AttemptBinding digest hashes compact JSON in the exact declared order: `schema, run_sha256, plan_sha256, protocol_sha256, config_sha256, source_commit, binary_sha256, cell_index, attempt_sha256`, no newline. All strings in this binding have fixed ASCII grammar, so independent fixed-order emission is sufficient; do not sort its fields. The binding's `plan_sha256` hashes raw plan bytes, whereas `run.plan_sha256` and `attempt.plan_sha256` refer to canonical typed-plan bytes.
3. Capture genesis hashes `crypto.ml-dataset.capture-genesis/v1\n<BINDING_DIGEST>\n`. Each event hash covers its complete original JSONL record INCLUDING its terminating LF. Event hashing never reserializes an object or drops whitespace.

Use the existing consumer protocol verifier for Python-compatible frozen protocol canonicalization, including preserved integer tokens and finite binary64 float behavior. This is a separate digest domain from binding/event hashing. Reuse its pinned vectors and admission model; no global serde-number change.

## Verification sequence

### 1. Anchor and bounded metadata acquisition

Reserve fixed bookkeeping, input buffers and output envelope before opening the anchor. Read at most the anchor cap plus one byte, hash/parse the same bytes, check the caller pin, fields and reference grammar. Local source roots and new output parent are opened through the descriptor-based path boundary below. Remote mode additionally validates the canonical bucket and prefix before any command.

Walk backwards from the pinned anchor. Enforce ordinal less than the chain cap, receipt-digest cycle rejection and exact remote/binding/run/attempt identity. Before every metadata read, admit its declared bytes and one logical reference. Fetch and hash catalog, receipt and manifest independently. Catalog must exactly bind anchor identity and receipt; receipt must bind anchor identity, ordinal and manifest; manifest binding digest/run/attempt must match anchor. Receipt and manifest previous anchors must be equal. Ordinal zero requires no previous; every other ordinal requires previous ordinal exactly one smaller. All three completion/restoration/science flags must be false in every envelope containing them. Receipt artifact count and bytes must equal the manifest objects.

Retain exact metadata bytes on disk under immutable per-ordinal paths. A bounded typed parser may retain compact descriptors and ancestry; it must not retain an unaccounted second forest of JSON Values. Reject cycles, gaps, missing objects, unequal predecessor anchors, substituted remote and stale catalog content. A terminal anchor necessarily resolves to a complete chain down to zero; a missing prefix is an error, not a verified suffix.

### 2. Forward structural admission before artifact transfer

Reverse the bounded chain. Keep one cumulative path map, context inventory, binding/profile, previous observation, capture cursor and stop state. Objects must equal the exact ordered concatenation of context followed by delta artifacts, with each CAS reference derived from that artifact. Reject duplicates in a snapshot, any earlier path redefinition even with identical bytes, and namespace conflicts. Charge logical reference counts and bytes before artifact reads; no deduplication discount in the producer-compatible logical accounting.

Ordinal zero includes the five authoritative context artifacts, whose raw SHA values equal the binding. Later manifests cannot repeat them. Other admitted context is only the profile's finite names. V2 has exact role/size checks, initial policy and a dependency-closed structural prefix at each ordinal. Check capture/head/cursor structural consistency, sticky stop, exact recomputed coverage, observation progression and all claims before downloading artifact bodies.

Metadata admission failure must make zero artifact GETs. Metadata GETs already used remain counted and are reported in failure diagnostics. Do not claim that the whole chain was preflighted before any network traffic: discovering it requires bounded metadata reads.

### 3. Immutable local bytes and actual authoritative context

Stream every artifact to an exclusive private staging file, enforcing declared bytes plus one sentinel byte, computing SHA and length, then finalize only an exact match. Do not hardlink an untrusted source. Context and event parsing opens the finalized descriptor, rehashes the bytes actually consumed, and compares their admitted identity; prior download success does not authorize an unbound second read.

Reconstruct run, plan, protocol, config and attempt from actual restored bytes in BOTH profiles. Independently check schemas, raw hashes, full source/binary identifiers, canonical plan commitment, source/run/attempt agreement, cell index and shard membership, mixed-radix selected config and frozen protocol digest. Compare the reconstructed binding with every manifest's binding. A claimed matching digest alone is insufficient. Existing independent plan/config supported-version limits apply: unsupported context is a named unsupported result with failure exit, never structural success or a fabricated config.

Optional public/witness/factor-base/host/process context is retained and hash-checked, without new arithmetic or hardware conclusions. All inputs stay restricted. Recheck the authoritative context and admitted identity set during subsequent `partial-audit`; do not trust a saved producer restore report or this consumer's previous report.

### 4. Committed capture replay

Start with no capture state. The first actual delta starts from the exact genesis cursor; an absent delta leaves prior state unchanged. Distinguish `capture_present=false` from a supplied zero-event delta whose cursor is genesis. The latter has capture present and zero events.

For each delta, check its binding, head schema and binding digest, monotonic counters, cursor constraints and segment/artifact caps. A prior non-null stop cannot change or clear, and a later supplied delta cannot advance its cursor. Absence of a later delta preserves the stopped state. Validate every stop enum value, but do not certify that the reported resource/storage cause actually occurred.

Build a bounded exact map of this delta's artifact descriptors. Replay ordered segments from the previous cursor:

- Segment path is `events/<next_chunk-as-12-digits>.jsonl`, role `committed-solver-event-prefix`, encoding `jsonl`. Its descriptor must exactly appear in the map and be referenced once. Byte start, first sequence and preceding digest equal the previous cursor.
- Stream lines with a 1 MiB inclusive-LF bound and 16 MiB segment bound. Reject empty segments, empty/invalid records, missing final LF, oversized lines, wrong event schema, binding, sequence or predecessor hash. Sequence/end/chunk arithmetic is checked before increment. Hash the original complete line bytes.
- Event envelope is closed `{schema,binding_sha256,sequence,previous_event_sha256,event}`. The `event` value may be any bounded valid JSON value; unknown event kinds remain opaque. Duplicate keys and nonfinite/unsupported numeric JSON are rejected independently. No arbitrary event-kind handler runs.
- When `event.kind` is exactly `capture_payload_committed`, require a valid artifact at `event.artifact`, exact membership in the same delta, unique reference, basename-safe payload path, actual payload length/hash and bounded totals. Other event-body fields remain opaque; do not invent a closed schema for arbitrary producer event data. Do not treat `kind` fields elsewhere or text containing that string as payload declarations.
- Recompute each segment end/last sequence/last digest, then the final cursor and head. The set of referenced segment/payload descriptors must equal all delta artifacts: no omitted, duplicate, extra or unreferenced payload/chunk can pass. Payload bytes are streamed, not loaded wholesale.

At each ordinal recompute coverage from verified state, including committed event count/end, capture presence and stop. Require artifact_only=true and every memory/tail/backend-internals/resume coverage claim false. A torn or uncommitted source tail is not admitted, reconstructed or appended. The result certifies the supplied committed prefix's byte and structural chain; it does not certify event truth, solver completeness, private witness equations, UNSAT or fall degrees.

### 5. Checked-phase prefix, using the consumer core

Extract a consumer-owned context reader/semantic core from the existing sealed adapter. Feed it a typed partial-context inventory, actual context descriptors and reconstructed binding, not a fake manifest. Validate fixed policy bytes first, then each newly admitted start/end pair and its exact raw start digest. Check the prefix at each ordinal; an earlier invalid failure/continuation cannot become valid because later context exists. Cache only already hash-bound immutable context records and validated prefix state; charge retained cache and output once and charge actual revalidation scratch before reuse.

Preserve the existing consumer's fixed policy, native lifecycle/projection, start-only, error, unavailable, fallback and unsupported platform rules. In particular: native acquisition support is little-endian 64-bit Linux x86_64/AArch64; unsupported requests may declare another byte order but have no native calls/TIDs. Closed descriptors may be reused by a later attempt; uniqueness applies while open. Native IDs may be zero wherever the pinned lifecycle permits. A producer-reported elapsed value or projected counter remains reported diagnostic evidence. Do not add a positivity rule or infer a whole-worker count.

Start-only is valid incomplete evidence and blocks later phases. A recorded error end blocks later phases; a successful recorded end permits the next ordered phase. All four ends still do not prove complete cell execution. Retain private raw measurements or their exact artifact references, but make public export unchanged. The sealed adapter retains its original summary schema/validator and `partial_snapshot_chain_verified=false`; a new partial wrapper owns its different chain claim. Do not globally change the sealed summary's claim flag.

### 6. Final output and fresh audit

Use a new directory with `PARTIAL_RESTORE_INCOMPLETE` present throughout work. Output layout:

```text
NEW_DIR/
  PARTIAL_RESTORE_INCOMPLETE                 # removed only after complete success
  partial-consumer/anchor.json              # exact caller-pinned bytes
  partial-consumer/report.json              # new consumer schema, restricted
  interrupted-attempts/<run>/<attempt>/
    snapshots/<ordinal20>/{catalog,receipt,manifest}.json
    artifacts/context/...
    artifacts/events/...
    artifacts/payloads/...
```

Do not write a producer `restores/*.json`, sealed `manifest.json`, cell registry, export row or solver checkpoint. Staging has a reserved private name not usable by artifact paths. Final audit enumerates this finite exact namespace, rejects symlinks/nonregular files, unexpected artifact/metadata members and any missing expected member. It recomputes all metadata and artifact bindings from opened descriptors and runs the same forward/context/capture/phase checks. Write a bounded new consumer report atomically, flush required files/directories, then remove the incomplete marker last. No success is emitted if the final audit or durable completion step fails. A fresh standalone audit must ignore the saved success flag.

## Admission, memory and filesystem contract

Initial fixed policy is conservative and immutable from input metadata. No CLI flag increases these limits in this increment.

| Resource | Limit / rule |
| --- | --- |
| Anchor | 64 KiB raw bytes |
| Chain | 256 snapshots, ordinal 0..255 |
| Each metadata object | 4 MiB; duplicate-rejecting typed parser, depth at most 64 |
| Artifact | 64 MiB overall; event chunk at most 16 MiB; policy/start/end have existing smaller phase caps |
| Logical objects | 65,536, including each catalog/receipt/manifest reference and each artifact occurrence |
| Logical object bytes | 8 GiB cumulative, checked before reads; includes metadata as producer accounting does |
| Snapshot artifacts | 8192; capture event count/chunks/payload count additionally bounded by their pinned 100,000 limits |
| Capture stream/payload totals | At most 4 GiB each, plus total operation/object limits |
| Checked context | 19 unique identities, 16 MiB raw total; legacy context 10 finite names |
| Metadata/control modeled allocation ledger | 256 MiB cumulative admitted allocation; explicitly described below |
| Checked-phase child ledger | Existing 32 MiB cumulative model, its reservations also charged to the parent; never reset at each ordinal |
| Summary | 1 MiB maximum serialized output; reserve before building/serializing it |
| Remote child | Existing bounded process-group time/output controls; cumulative read attempts fixed and no automatic retries |

These are consumer admission limits, not a promise to accept every producer-valid configured maximum. Earlier `consumer_resource_limit` refusal is explicit; no downgrade to weaker verification. The 256 MiB ledger is a model of admitted Rust-owned allocations, not measured RSS or an OS memory guarantee.

Before allocating, charge raw metadata, decoder scratch, every retained typed record, owned key/string, collection capacity, path map/set, canonical binding bytes, event parser nodes, canonical plan/config copies, protocol token/canonical scratch, phase records/clones, summary and serialization. Use checked arithmetic and fallible allocation. An allocator-container growth path must reserve its next actual capacity before growth. Do not parse with unbounded `serde_json::Value` and account afterwards. Reserve the fixed envelope before creating root paths or error structures. The protocol/plan reservations from the sealed adapter are not exempt: carry them into the shared ledger before any new partial allocations.

A single fixed event-line buffer and bounded decoder arena may be reused across events; the parent reserves their worst-case capacity once and the decoder resets only after all references are discarded. Retained artifact identities/cursors live outside that arena and have cumulative charges. Metadata parsing similarly uses one explicitly owned bounded transient arena; retained descriptors are separately charged before copying. If implementation instead allocates fresh owned parse trees, it must charge every such allocation cumulatively and may reject earlier; it cannot call that reuse. Do not reset a fresh budget per object, ordinal or protocol preflight. Store large original metadata on disk rather than retaining raw+Value+typed clones for all snapshots.

Track separate logical-reference bytes, actual read bytes, written/restored bytes and scratch reservation. No CAS-dedup discount to logical admission. The implementation may avoid a duplicate physical transfer only after rechecking an already retained immutable object's exact hash/length; report both counts honestly. Reserve destination-space upper bounds before artifact transfers, based on metadata plus declared artifacts, retained anchor/report and fixed scratch. Use rename/move rather than a second whole-corpus copy. Disk free-space observation and a polling guard are advisory, not a filesystem quota; bounded writers enforce their own exact sizes. S3 CLI temp-file polling has that same limitation and must be stated.

Use Linux directory-descriptor-relative traversal (`openat`/equivalent) with no-follow at every component, regular-file checks and exclusive output creation, under an opened trusted parent. Do not rely solely on `canonicalize`, a prior symlink scan or `common::contained` to prevent ancestor replacement. Pin descriptors while hashing/parsing; never reopen a path and assume the prior bytes remain authoritative. Fixture hooks must exercise same-length replacement and ancestor swaps between lookup, open, hash and parse. Newly created destination is private; an adversary with the same UID can still edit open files, so consumed bytes are rehashed and final audit detects persistent mutation. No claim of an immutable snapshot against a continuously malicious privileged process.

S3 mode retains existing canonical-bucket validation, HEAD exact length, ETag-conditioned GET and fresh SHA readback. ETag is a version guard, not a digest. GET failure/412/timeout or changed size is terminal for that invocation. Command arguments are structured, never a shell-built request. Use one admitted session: no new budget per object and no credential fallback. Reads do not prove the worker had write permissions when it generated data.

## Restricted report and leakage boundary

New schema: `ml-dataset.independent-partial-audit/v1`; validator ID: `independent-partial-lineage/v1`. Required fields include pinned anchor SHA, terminal profile/ordinal and declared remote; binding; snapshot/context/artifact counts and accounted/read bytes; context and capture states; final cursor/stop/observation; hash-bound input metadata identities; checked-phase summary or explicit legacy not-applicable state; modeled allocation counters and immutable limits. No arbitrary captured event body, private scalar, witness, seed, error text or remote credentials enter an exported feature row.

On success, the narrowly named booleans may be true: `metadata_chain_verified`, `artifact_bytes_verified`, `authoritative_context_verified`, `committed_event_chain_verified` (only if a capture delta was supplied), `partial_snapshot_chain_verified`. Use `capture_validation_state` = `not_present`, `verified_empty_prefix` or `verified_committed_prefix`; do not represent absent capture as a verified empty recording. `checked_phase_contract_verified` is true only for checked v2 after its core succeeds; legacy uses false plus the explicit not-applicable state. `restored_locally` is true only for restore/fetch, not merely a read-only audit of a directory. `fresh_remote_readback_verified` is true only for an actual S3-backed invocation that completed all required reads; local/mock runs use false.

Always false: `cell_complete`, `solver_state_restorable`, `solver_resumed`, `scientific_result_verified`, `complete_execution_verified`, `hardware_independently_verified`, `platform_admitted`, `training_cost_eligible`, `clock_valid_for_comparison`, `whole_worker_coverage_verified`, `worker_publication_verified`. `whole_worker_instruction_count` is null. Preserve recorded phase values as restricted reported diagnostics; do not rename structural native lifecycle consistency to hardware truth. Return no generic ambiguous `independently_verified=true` flag.

Producer observation/stop flags are labeled reported. A valid chain ending in running is still an artifact-only incomplete attempt. Neither exited nor four successful phase ends upgrades it to a sealed cell. Local restore proves local artifact recoverability at that anchor. Controller archival of a packet to S3 is a distinct receipt outside this command. Worker S3 conditional publication/readback remains a separate authorization, IAM, budget and execution gate. A later controller upload of these reports cannot set worker publication or generation flags.

## Compatibility limits and staged work

This increment supports the two frozen partial profiles, complete predecessor chains, finite context, and full committed capture prefixes within admission caps. No captured-event mode is silently omitted. Unsupported plan/protocol versions, future profiles, larger producer-configured limits, non-Linux filesystem implementation, signatures/authentication, solver-state continuation, importing partials into training exports, helper retry/prepared-record certification and cloud writing are explicit non-goals or named unsupported cases. Optional helper request/prepared/ack/confirmation records may be retained by an external archive; they are not anchor dependencies and their presence does not substitute for required catalog/receipt/manifest verification.

The retained initial producer fixture and new independent static fixtures can establish local parser/restore acceptance. Mock S3 can establish command/admission behavior only. Fresh S3 readback remains an unfulfilled external gate until separately performed and receipted by the parent within the existing scope. The design does not infer fresh S3 success from earlier controller archival or a local `file://` fixture.

Implementation starts only after the parent freezes this design, wire inventory, vectors and acceptance matrix on an isolated consumer branch stacked on the current consumer. Existing T780 ownership covers the sealed adapter, and T766 covers producer work; this design creates no lease or ownership claim. Parent chooses the implementation owner and source pin. Review the final diff, standalone tests/strict Clippy and retained CLI fixtures before any PR delivery or cloud consideration.
