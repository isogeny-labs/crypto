# Reuse the encoded native capture payload

The first parent-storage integration retained a real work-limit failure: in the
F4 positive fixture, input/events/witness storage succeeded but only 17,307 units
remained before a group-replay storage quote of 755,232. The original 16,777,216
work cap and the parent-storage quotes are preserved. This is a diagnostic of the
declared work model, not a retired-instruction measurement.

Previously each native boundary's canonical view was independently serialized
into a pending buffer, and then the same view was encoded inside capture's typed
wire record. Capture already performs bounded counting and encoding passes.

`CaptureEmitter::event_typed_precommit` exposes the complete internal wire bytes
to a callback before durable publication. Existing `event_typed` uses a no-op
callback and keeps its behavior. Callback refusal leaves the capture head
unchanged and does not poison the sink; serialization and capture-reservation
failures retain their existing behavior. Encoded bytes are not acknowledgement.

The native adapter extracts its final embedded payload from the trusted typed
encoding and copies it into its preallocated canonical buffer, adding the
canonical newline. The extractor is not an external JSON validator. Independent
saved-artifact verification still uses the strict parsers and authenticated
capture verifier. Copy admission preserves any trace-limited unacknowledged
prefix before refusing publication.

The old native-event reservation charged 8 units per capped byte for typed wire
encoding and another 8 for separate canonical serialization. The latter operation
is removed and replaced with a 1-unit-per-byte copy reservation. Identity/fixed
metadata work, byte/event/trace caps, original deadlines and parent storage
quotes are unchanged. No refund is introduced. The tariff change follows the
implementation change; it is not a claim of measured machine-work improvement.

Controls retain the original partial-prefix test and all parent-storage success
requirements, compare reused canonical bytes with the original typed view
serialization, and verify precommit refusal leaves the head unchanged and sink
usable. The full focused Koblitz suite and shared capture suite are required.
Both pre-optimization failure receipts remain part of the evidence trail.
