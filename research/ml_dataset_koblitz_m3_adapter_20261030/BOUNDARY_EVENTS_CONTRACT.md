# Native boundary event contract

Keep the frozen canonical artifact identity koblitz-m3-events.jsonl,
restricted-koblitz-m3-boundary-events/v1, encoding jsonl. Each strict JSON record
has schema crypto.ml-dataset.koblitz-m3-events/v1, original binding_sha256,
sequence (one-based across construction and matrix boundaries), capture_sequence
(the anticipated one-based global capture sequence), and event. The event is a
strict tagged union: Construction with native stage and reserved work/bytes, or
Matrix with native stage, reserved work/bytes and all native counters. Native
stage variant fields reject unknown keys. Construction stages are Begin, Table,
Selector, Add, Square, Product, Coordinates, Complete. Matrix stages preserve
the native Begin, CriterionLevel, CriterionCandidate, MainCandidate, Pack,
Reduce, Readback and Finish fields exactly.

The dedicated typed capture kind is koblitz_m3_native. Its payload is the same
canonical boundary record. Existing koblitz_m3 signed-domain events remain
separate. On replay, the record's capture_sequence must equal the authenticated
outer capture sequence; canonical sequence and content must match the native
boundary stream in order. Bind the original attempt digest independently.

The canonical JSONL contains acknowledged capture records only. A pending last
boundary that failed capture remains explicitly retained in the result's optional
unacknowledged_boundary field; it is not counted as a capture acknowledgement.
An incomplete encoding remains raw failure evidence and cannot enter this
canonical parser. Pre-admit bounded retention and encoding before the native
operation and retain complete snapshots before callbacks that can fail.

Capture acknowledgements and native observer acknowledgements differ. A callback
can publish a capture record and return, then the adapter's deadline check can
fail before the native engine increments its own acknowledged_events. Preserve
both facts: capture acknowledgement can exceed native acknowledgement by the last
attempted event. Do not force equality or relabel such a phase as complete. Native
attempted/acknowledged counters, capture sequences and adapter stop are separate
result fields. The actual native event snapshot contains attempted_events including
the current callback and acknowledged_events before the callback returns.

Bounded decode uses 4096 records and at most 2048 bytes per canonical line; the
combined stream remains subject to the original trace allowance. These are caps,
not a blanket resource quote. Freeze the exact producer retention layout and
bounded serializer's work/byte reservations before enabling runtime dispatch.

This contract defines boundary evidence, not an instruction trace, row derivation
certificate or full solver verdict. Result semantics, aggregate resources,
mathematical files and supervisor policy remain separate checks. Validation must
include actual native F4/F5 boundaries, strict nested stage mutations, global vs
native sequence mutations, missing/extra/reordered records, and a real capture
acknowledgement followed by adapter deadline failure. No fixture has been generated
under this contract yet and no new cloud execution is authorized by it.

## Emitter storage and failure retention

The emitter reserves its object, the typed capture buffer's requested storage,
4096 times 2048 bytes for canonical retention, and 2048 bytes for pending encoding
before allocating those buffers. Per event it reserves checking work for both
serializations, typed wire trace capacity, and actual canonical serialization
bytes. These are modeled reservations, not retired instructions or allocator RSS.

The original binding digest and limits are checked against the active bound sink,
and the shared global counter must equal its committed event count before any
publication. A snapshot is retained before fallible work. A refused attempt at the
4096-event cap may retain anticipated capture sequence 4097; that is pending
failure evidence, never a canonical acknowledged record. The first stop remains
sticky, and a second call cannot overwrite pending evidence. A successful capture
appends to pre-admitted canonical storage and advances counters before checking
for a subsequent deadline. Runtime dispatch remains disabled pending integration.
