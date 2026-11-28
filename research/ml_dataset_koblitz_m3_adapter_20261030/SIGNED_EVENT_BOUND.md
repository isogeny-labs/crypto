# Closed signed-addition event cap — external draft

Patch signed-event-cap.patch SHA2567071eda53e43d76c17e4329f72a046c367cb1aa3977e1eac86586066356dcbd0; baseline koblitz_m3.rs SHA25677496f631e34d647c8e3aa3a7ffd4599df43bc2eff01439c141bb59b36a420c1. Only BoundSignedEvents constants/buffer choice/emission and appended tests change. BoundSignedPublisher witness path and parent32/byte tariff remain untouched. No worktree writes/builds/tests/cloud actions. Rustfmt with skip_children=true parsed/formatted the external module; ordinary rustfmt initially refused the external missing sibling koblitz_m3_causal.rs, so no compilation claim follows.

## Exact closed wire bound

SignedPrefix has sequenceu16 (max5digits), tupleu8 (3), indices[u8;3] (3digits each), summandu8 (3), prefix Option<(u64,u64)> (null4bytes or pair43bytes), matches_targetbool (false5bytes maximum). SignedEventView adds fixed phase literal signed-domain and progress. Maximum payload181bytes.

The FULL capture TypedEventRecord adds schema crypto.ml-dataset.live-event/v2, binding_sha256 and previous_event_sha256 each64hex, outer sequenceu64 (20digits), fixed eventkind koblitz_m3, payload nesting/punctuation. Exact complete serialized JSON plus newline is474bytes. All strings are fixed JSON-safe ASCII or hexadecimal; the maximum integer representations/false/Some(max,max) dominate every other value. No actual tuple/index/sequence validity restriction is needed for this byte bound, so even representable but semantically invalid prefixes fit. Production event type contains no arbitrary user strings/vectors. Field additions or renamed literals must update these controls.

signed-event-maximal-wire.jsonl retains the474byte maximum and signed-event-cap-proof.json records deterministic byte arithmetic, source/patch hashes and tariff. Draft Rust test independently uses capture::maximum_sequence_typed_wire, which constructs the actual private TypedEventRecord serializer at outeru64max and digest64, to assert474. It also crosses all inner numeric digit-width transitions, both bool variants and None/Some point variants, and roundtrips full SignedPrefix without dropping fields. This Rust control has not run.

## Enforcement and accounting

Logical event cap512 provides38bytes slack. Original2048buffer capacity and its requested_bytes reservation are kept under SIGNED_EVENT_BUFFER_CAP. Both serde passes go through existing event_typed_precommit_with_limit(...,512,...), which min-bounds caller/buffer/capture limits, includes newline, refuses before precommit/head mutation, then commits the same encoded bytes. Bridge goes directly to the already identity-checked Bound sink and rejects missing/legacy sink; no generic allocating path. This replaces only BoundSignedEvents' existing live_typed_event invocation. Every encoded field and byte remains unchanged for admitted records.

Entry charges1024+8*512=5120modeled work/event and512trace bytes before publication, retaining4096per-event memory reservation. Oldwork17408 -> saving12288/event,2359296over192events; trace saving1536/event,294912over192events. Setup64KiB work and buffer/memory unchanged. No coefficient change, refund, degree/domain change or limit renewal. Encoded-byte counters still record actual acknowledged bytes; global/signed counts increment only after acknowledgement and preserve post-ack deadline semantics.

Additional draft controls exercise actual first-record exact455byte cap versus454 and0, checking callback/head/file unchanged on rejection; cap512 success; and adapter admission delta/buffer2048/actual acknowledged byte count. Existing actual signed-domain+first-witness193event controls and capture/payload mutation tests remain unchanged. All three new controls require parent compilation/testing; current runtime failure results are not reclassified by this draft.

# Independent signed-event cap review

Reviewed patch SHA256 `7071eda53e43d76c17e4329f72a046c367cb1aa3977e1eac86586066356dcbd0`, actual SignedPrefix/SignedEventView/capture serialization and admission code, and the external proof artifacts. `git apply --check` passed against the current worktree. No source mutation, compilation, Rust tests, dataset or cloud execution performed.

**No concrete integration blocker found.** This is a tighter enforced bound on the same closed signed event, not a coefficient reduction or changed search domain.

SignedPrefix has only fixed keys, u16 sequence, five u8 values (tuple, three indices, summand), Option of two u64 words and a boolean. SignedEventView adds a fixed phase string. The outer TypedEventRecord contributes fixed schema/kind, two64hex digests, u64 sequence, nesting/punctuation and newline. The maximal representable values, Some(max,max) and false dominate every valid representation without using semantic assumptions about actual tuple ranges. I independently parsed/re-encoded the saved maximal wire: payload181bytes; full record including newline474bytes. Actual Rust serializer equality is asserted in drafted controls but has not been executed by this reviewer.

The512 logical cap has38bytes slack over that full envelope. Existing event_typed_precommit_with_limit enforces it during both count and write passes, includes newline in admission and refuses before callback/commit when the cap is too small. The original2048 buffer/requested-bytes setup remains. Per-event checking admission remains1024+8*cap and memory4096; trace admission iscap. Both admissions occur before the cached bound-sink identity/count check and serialization. Original limits/deadline are not changed. The acknowledgement/global/actual-encoded-byte updates and post-ack clock handling remain unchanged.

Replacing live_typed_event with the direct capped Bound call preserves its applicable behavior: the old wrapper only selected the Bound sink, called event_typed and converted error to String; the caller discarded error text. Missing/legacy sinks still refuse, and the prior identity check remains. Dedicated witness publisher and parent storage tariffs are untouched.

Draft controls are meaningful: actual private TypedEventRecord serialization at maximal outer/inner widths; numeric digit transitions and both bool/point variants; first-record exact455-byte admission versus454/zero refusal with no callback/event-count/file changes;512 success; actual adapter checking/trace/memory deltas and acknowledged byte count. The existing signed-event/witness shared-sequence control remains important after compilation.

Future reserved work decreases17408->5120 per acknowledged/admitted event, a12288-unit reduction, or2359296 across192 admissions. Trace allowance decreases1536 per admission. These are modeled reservation differences, not measured CPU/instruction savings or a completion prediction. Failed admitted attempts still retain their charges. Root should require the actual signed/capture/native tests on the combined immutable source before updating readiness.

Integration: applied after current-source review and Conductor admission. The next combined dataset suite covers causal group projection, capability metadata, signed-event bounds and frozen wide worker controls. Tests pending; no producer limit change or new cloud job.
