# Independent source review: diagnostic resource custody

Reviewed 2026-10-30 by the launch/child integration agent, not the resource module author. Scope is source inspection only: no edits to repository source, builds, tests, Git operations, cloud calls, or scientific execution.

Reviewed source: `src/cryptanalysis/ecbench/diagnostic_resources.rs`, SHA-256 `e25bdaa342ed7351eef92299ac576fdba5aeea946821be7bd164b6e5274e6c41`. Checked against the existing private all-arm lifecycle/cost patch specification, especially the resource publication-tail and external owner closure requirements. This is a software review, not a new protocol or admission gate.

## Result

No concrete defect found in the stated pure metadata-validation scope. The module validates structural ordering and exact supplied-byte bindings. It does not establish that supplied timestamps, durable-write observations, or external receipts are authentic. That boundary is explicit in the source and must remain explicit in integration and reports.

## Source observations

- `ExecutionEnvelope::mark` (132–150) enforces the exact five-stage prefix with no repeated/skipped stage; timestamps are nondecreasing. Equal timestamps remain valid for finite clock resolution. Addition to the execution start is checked for overflow before mutation.
- `seal_prefix` (152–208) rejects backwards coverage, invalid digests, durable-stage/binding mismatches, absent context binding after start, incomplete Returned status, and empty failure reasons. Every generated prefix has pending publication tail and unknown active time. Partial failure prefixes preserve absent marks and hashes instead of substituting zero.
- `PublicationTail::observed` (220–260) bounds exact input bytes, rejects duplicate keys/unknown schema fields, compares decoded bytes with the supplied prefix, then reconstructs the envelope and reseals it. A deserialized prefix cannot bypass stage-order/monotonicity checks, change its schema, claim active time, or clear its pending-tail flag. The exact published byte sequence is hashed; whitespace-equivalent prefixes have distinct publication hashes, as expected.
- `append_tail` (319–359) validates identities/hashes, no overlap with the preceding execution's published receipt, ordered internal boundaries, strict sequence increase, and stable scientific run/attempt/protocol/source/binary bindings across a journal. It does not require contiguous sequences; no cohort-completeness claim is made by this module.
- `close_on_external_exit` (360–411) decodes bounded exact journal bytes and reconstructs all append operations. Deserialized journal history therefore cannot bypass monotonicity, sequence, identity, or schema checks. Without an exit observation, closed wall time stays null, the tail stays pending, and active time stays null. With matching supplied exit data, final wall interval is `runner_exit_ns - payload_start_ns`, never a sum of overlapping child/phase intervals. Lower-bound checks make this subtraction safe even for an empty journal. Active worker time remains null in both branches.

## Exact trust boundary

`PublicationTail` and `ExternalExit` are public deserializable data. A caller can manufacture a structurally consistent tail or exit receipt. Journal validation rechecks their structure and association, but cannot fetch/re-hash the prefix object behind every stored `prefix_sha256` because those bytes are not arguments to `append_tail`/closure. Similarly, `observation_receipt_sha256` is checked for digest syntax; the module does not receive the original observation receipt bytes, verify the observer, or authenticate the source of its monotonic timestamps.

Consequently, `final_publication_tail_pending=false` means closure according to an externally supplied observation that the caller has trusted and bound. It does not independently attest runner exit, fsync completion, elapsed resource ownership, or authenticity of the observation. The existing specification assigns those observations to the external owner, so this is an integration responsibility, not a new acceptance condition. Root must continue using `None` when there is no actual owner receipt; a synthetic fixture or controller deadline must not be converted to an ExternalExit.

The resource journal alone does not prove coverage of all executions, completion of mathematics, complete active-worker accounting, or a finite-model cost ratio. Those claims remain outside this module.

## Fixture coverage inspected, not executed

Six tests currently cover byte caps/duplicate keys, partial stage failures, stage ordering and clock reversal, missing bindings, offset overflow, exact prefix mismatch, publication reversal, non-overlap, null unclosed totals, external closure identity mismatch, and external durability lower bounds. There are no explicit mutation fixtures for every forged-deserialized prefix/journal field; source reconstruction checks cover those paths by inspection. Adding such targeted fixtures would strengthen regression coverage but is not a new scientific gate and does not substitute for the root build/tests already planned.

## Handoff

Resource owner and root notified. No requested source repair arises from this review. Preserve the external-observation trust boundary and unknown active-time field when wiring the actual runner.
