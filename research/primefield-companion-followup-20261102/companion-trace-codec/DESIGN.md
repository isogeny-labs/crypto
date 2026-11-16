# Exact streaming trace fingerprints

2026-11-01. Implemented codec with deterministic software tests. No new kernel
trial, timing benchmark, cohort substitution or continuation of the capped run.

## Inspected path and concrete change

The frozen companion-fused-validation/check.py calls
sha256(json.dumps(trace,sort_keys=True).encode()) for each product. Its trace
contains packed integer intermediates P,T,Q,Y,acc and the final packed value.
The complete JSON byte string is used only for the fingerprint and discarded.
This is a confirmed code path, not a measured explanation of the timeout.
Independent arithmetic reconstruction and coefficient-reference comparison
happen before fingerprinting and must remain unchanged.

trace_codec.py encodes integer magnitudes as binary and feeds canonical chunks
to SHA-256. It never converts the packed integers to decimal strings and does
not assemble a complete JSON trace string. The original trace object remains
in memory; this change does not remove its arrays or independent comparisons.
No elapsed-time improvement or total-memory bound has been measured.

## Canonical encoding and exact scope

The byte stream starts with ASCII `companion-trace/v1` and a zero byte.
Each value has a distinct one-byte tag: n for None, t/f for booleans, i for an
integer, s for a string, l for a list, d for a dictionary. Integers carry a
sign byte, a base-128 length and a minimal big-endian unsigned magnitude.
Zero has positive sign and zero magnitude bytes. Strings are strict UTF-8
with their byte length. Containers carry their element count. Dictionary keys
must be strings, emitted in strictly ascending Python Unicode order. Tuples,
floats, byte strings, non-string keys and unsupported types are rejected.
Nesting is bounded to 128. The decoder bounds length encodings to ten bytes,
rejects impossible remaining lengths and requires complete input consumption.

The independent decoder parses bytes without calling the serializer. It
rejects negative zero, leading-zero magnitudes, invalid signs, redundant
length encodings, duplicate/unsorted keys, invalid UTF-8, unknown tags,
truncation, extra bytes and excessive depth.

Tags plus self-delimiting canonical lengths make the encoding injective over
its supported finite typed values: scalar payloads have unique encodings;
container boundaries recover the unique ordered children; sorted distinct
keys recover the unique map. This is a statement about encoding, not a proof
of SHA-256 collision freedom. The fingerprint retains the same cryptographic
hash assumption as the previous JSON digest.

`fingerprint` returns the schema `companion-trace-fingerprint/v1`, SHA-256 and
encoded byte count. The type/version marker is mandatory. It is not compatible
with the old JSON trace hash and must never be substituted into old records.

## Checks completed

Eight deterministic unittest cases passed. Fixtures cover 100,001-bit integer
magnitudes, negative/zero values, Unicode, empty and nested containers,
handwritten golden encodings, order independence for dictionaries, type and
boundary distinctions, every truncation of a fixed structured fixture,
malformed canonical forms and nesting limits. Streamed fingerprints agree
with SHA-256 of materialized encoded bytes. Test elapsed time is not used as
a benchmark. No production numerical kernel was imported or executed.

The proposed integration patch changes only import wiring and the per-product
trace fingerprint field. The full runner's main schedule and stages function,
and every verify statement before its final returned record, retain identical
ASTs. The existing full 1,698-row/34,248-product cohort, all references,
controls, seeds and both arms remain required. The patch has not been applied
to any frozen or runnable validation directory. Its old/new source hashes are
recorded in integration.json; the proposed whole source passed syntax checks.

## Next gate

Use this codec only in a separately versioned instrument with its own output
directory, complete frozen protocol, source/input locks, full seed expansion
and finite authorized budget. The original capped invocation and its full
candidate/partial A/A record remain immutable. A new record must identify the
codec version and retain the encoded-byte field; comparisons must distinguish
the changed instrument and must not claim a kernel arithmetic improvement.
Before launch, add full-run analyzer checks for the fingerprint schema, hash
format and byte count. An effective faster validation run remains unmeasured.
