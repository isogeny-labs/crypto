# m3 first-certificate live persistence correction

Separate follow-up to semantic review A–F; original m3 scope unchanged.

Confirmed existing mechanisms: CaptureEmitter::payload (capture.rs523–590)
compares path/role/encoding/hash/bytes on duplicate, verifies existing bytes and
returns without further event/count charge. First write reserves before storage,
then commits the payload event/watermark; sticky CaptureEmitter failure rejects
later submissions. Legacy LiveSink similarly accepts identical existing bytes.
collect_cell_artifacts in mod.rs classifies unregistered nested live/payloads/* as
unregistered-partial-artifact/native. It does not promote them to solver results.

Change: one bounded compact witness_payload helper creates the exact bytes used
both at first verified certificate and later canonical SolverData staging. After
all3 group replay adds and required original-equation replay succeed, freeze the
first cursor and call live_payload immediately BEFORE any subsequent triple.
Do not use the differently formatted ArtifactPayload::json path. A live refusal
keeps the independently verified candidate/cursor, records the first bounded
error, stops all subsequent work and submissions, and leaves canonical staging
available when extraction returns. The result remains partial/solver_completed
false on persistence failure. Later canonical flushing is idempotent after a
successful early write; after failure existing sticky handling prevents retries.

Expose a private extraction-persistence interface for an actual bound-emitter
control, not a public schema. Production delegates witness/event calls to current
live sinks. Test wraps a real CaptureEmitter: first witness must already exist
when a later triple callback deliberately refuses; it remains byte-identical to
canonical staging and duplicate publication incurs no additional capture event.
Also inject first witness refusal and assert no subsequent triple/event submission.

An interrupted worker may have only the nested early payload and incomplete or
absent canonical result/registry. Collector must preserve the sealed nested bytes
as opaque native evidence; outer censored flags remain false. Add central test
using actual collect_cell_artifacts on a valid provenance fixture with nested
payload and truncated input, then reject promoted completion. Registered complete
producer records continue through ordinary semantic replay, unchanged.

Tests will be authored, not run by this agent. Parent controls source review,
compilation and validation. This does not assert durability after storage refusal
or elevate live prefix evidence into a completed mathematical run.
