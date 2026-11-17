# Required deterministic acceptance and semantic tamper matrix

This is a proposed test specification. No tests, builds, generators or hardware were executed by this design task. Preserve all existing native consumer v1/policy/root/generic tests. Use hand-derived evidence and retained software fixtures from the final producer owner; future interoperability is gated on that final source pin.

## Positive fixtures before negative tests

1. Legacy v1 complete and start-only records with exact old labels/export bytes; a legacy no-telemetry cell.
2. Checked exact-policy-only context: zero supplied phases, no invented operation/count and no vacuous phase verification.
3. One checked successful phase with positive equal enabled/running and raw zero; instructions/cycles separately checked. Then a four-phase complete prefix with actual descriptor/hash/binding relationships.
4. Multiplexed estimate with exact hand-calculated floor; raw counts retained separately. Overflow is unavailable, not u64 saturation.
5. Every valid unavailable family admitted as unavailable evidence: unsupported platform/event, permission/open/ID preparation failure, cancelled/unstarted, context failure, reset/enable/disable failure, failed/short read, read-ID mismatch, invalid tuple, zero-running and overflow. Include valid diagnostic tuples in control-failed states and cleanup errors with non-clean lifecycle.
6. Optional cycles failure followed by permitted pre-work standalone instructions, including valid identity-failed preparation and an unattempted cycles placeholder. Ensure no false requirement for a placeholder descriptor/member attrs.
7. An earlier operation error ends the prefix; separate panic/kill-like start-only prefixes stop after each of the four possible starts. Use synthetic artifacts, not process/hardware faults.
8. Native successful/failure syscall vectors only for little-endian 64-bit Linux x86_64/AArch64. Big-endian declaration is tested as an unsupported-platform record with no native calls/TIDs, plus a rejection case that falsely claims a supported native collection. Never interpret recorded attrs using the consumer host's byte order. Include a legitimate fallback reusing a previously closed descriptor number, and retain the final native contract's allowed zero-ID behavior.
9. Error summaries for empty/full Unicode messages and a truncated UTF-8 boundary. Full preview hash verified only when the full message is available; truncated suffix remains unverified.

Hand-derived fixtures establish the independent checker. Retained producer fixtures establish interoperability with a specific source. Producer acceptance or agreement between two calls to the same implementation is not an independent oracle.

## Tamper families — rehash outer artifacts before rejection

| Family | Required counterexamples |
|---|---|
| Dispatch and roles | Policy path with wrong role/encoding, policy role at wrong path, omitted manifest policy, v2 without policy, extra telemetry JSON, duplicate path/phase, unknown schema, mixed v1/v2; no malformed-policy legacy fallback. |
| Byte identity | Policy newline/whitespace mutation, self-consistent replacement policy/hash, alternate serialized start with updated manifest but stale end start hash, size mismatch, changed second-read bytes. |
| Binding | Transplant each raw authoritative file; wrong config/index/shard; full source/binary/attempt substitutions; raw plan hash versus ordered canonical plan hash confusion; mismatched attempt/run canonical digest. Rehash the surrounding artifacts so checksum-only validation cannot pass. |
| Protocol freeze | Missing/false freeze fields; missing timestamp; body mutation; nested freeze mishandling; key/array reorder; UTF-8; large integer precision loss; finite-float/exponent/negative-zero golden mismatch. No false claim of full experiment schema validation. |
| Strict JSON | Missing required nullable fields, duplicate/escaped duplicate keys at every layer, unknown fields, both Ok and Err, empty lifecycle result, wrong integer/float/bool/null types, numeric overflow, invalid enums, trailing JSON, excessive depth/strings/arrays. |
| Phase order | End without start; incorrect ordinal/path/name; gap; repeated phase; later phase after failed end; end after a different start; binding/policy disagreement within a pair. |
| Attr/context/FD/ID | Changed raw attr byte or decoded attr field, type/config/read-format/inherit/pid/cpu/CLOEXEC, member group FD mismatch, simultaneously reused open descriptor, duplicate/wrong event ID as defined by the pinned native contract, mixed endian across requests, unsupported big-endian record claiming native calls/TIDs, inconsistent owner PID/TID or context. Closed descriptor reuse by fallback is a positive control, not a rejection case. |
| Lifecycle | Missing/reordered/extra operation; fabricated Ok; repeated-start; read length 31/33; improper fallback or third attempt; fallback after cleanup failure; fallback after work; incorrect selected attempt; unattempted placeholder falsely promoted to a syscall. |
| Projection | Instructions tuple missing one field despite valid cycles, zero running promoted to raw zero, running greater than enabled, wrong floor, saturated overflow, wrong attempt/event ordering/index, wrong status/raw/estimate/lifecycle_clean, native assessment not matching tuple. |
| Claims/error | Any false/null admission bit promoted, whole-worker total invented, usage mislabeled as per-phase, elapsed malformed, returned_ok/error inconsistency, impossible preview count/hash/truncation. |

For field/state mutations, check the expected rejection reason family rather than any incidental parse failure. Validate the unmutated control first. Count the actual fixtures/checks in the receipt; do not equate a planned table with executed coverage.

## Admission tests that prove early refusal

- Boundary and boundary-plus-one for each file cap and every bounded collection. Confirm refusal before native validation/array expansion, with an invalid unread suffix analogous to existing generic-v2 preflight tests.
- Huge declared file/size refuses before read/allocation. Sparse file, short read, growth between checks, symlink/FIFO/path traversal and duplicate descriptor cases preserve existing integrity behavior.
- Component storage reservation includes raw buffers, decoded strings/maps/vectors, binding/canonicalization scratch (including arbitrary integer tokens), restricted output and temporary clones, with reservation before allocation. Existing `plan_info` and manifest Value allocations are outside the component model; any new copy is charged. Exhaust the component budget at each allocation boundary; a resource refusal must not be reported as successful contract validation, unavailable PMU, producer censoring or partial rank/work.
- Multiple maximum-sized frames consume a cumulative allowance; no fresh allowance for each phase or reread. Protocol canonicalization cannot evade the model through arbitrary integer/string/array tokens.
- Stream and nonstream test vectors must agree on exact digests and EOF handling. No global MAX_JSON increase or weakening of the existing generic-filter admission tests.

## End-to-end consumer tests, without hardware or generation

Use the actual standalone `audit` and `export` commands on synthetic authoritative sealed fixtures. Preserve the export requirement for producer replay or explicit `--allow-unreplayed-preview`. The new measurement validation must run with and without `--arithmetic`; algebraic certificate claims remain separate. A fixture missing mathematical public outputs can test packaging/phase evidence without pretending to be a solved instance.

Reuse one retained final-owner checked worker packet for native interoperability, and one missing-registry valid prefix after the producer has sealed it. Retain source commit, executable digest, all input bytes/hashes, actual expected outcomes, command output and consumer result. No fresh producer execution is necessary for this test increment. Use mock/local sealed transport to demonstrate policy/start/end members survive restore; this is not S3 persistence or partial-snapshot chain evidence.

An unsealed or `artifacts/context` restored directory must not be accepted as a sealed cell or produce a completed-cell export. The existing unsealed-attempt summary remains explicit. A future partial adapter needs its own chain/identity/namespace admission tests before any partial restore claim.

## Leakage and output invariants

Hold public/base/config fixed and vary only valid private evidence: counter values including zero/unavailable/multiplexed, outcomes/errors, PID/TID/platform, source/binary/attempt identity with consistent rebinding, phase completion versus valid start-only, raw usage/time and policy provenance. Recompute legitimate artifact bindings for each control. Require byte-identical `features`, identical split assignment and no private string/key in nested public output. IDs may change with manifests but remain outside model input.

Keep valid unavailable/start-only examples in the restricted view. Do not filter success-only or measured-only implicitly. Assertions must include global independently_verified false; whole-worker count null; producer consumer_v2_admitted unchanged false; training-cost/platform/hardware/complete-execution/clock claims false. Verify no per-phase sum or proxy is created.

Legacy v1 label/export golden bytes stay unchanged. V2 uses a restricted versioned result without changing the feature-policy schema. Verify that producer raw records are preserved and consumer conclusions are separate; full versus truncated error-message checks cannot share an overstated verified flag.

## Delivery acceptance

The frozen producer contract/policy digest and corrected runtime source must be reconciled before interoperability can pass. Any discovered contract/source mismatch is recorded and referred to the producer owner. The consumer owner freezes the new field sets, component model and fixtures before implementation.

Run the standalone release suite, all-target strict lint and relevant retained software fixture tests under the existing build coordination. Preserve old failed evidence. Produce source/executable/test/fixture hashes and actual counts, then follow existing parent review and software durability gates. This document provides neither those results nor permission for cloud, hardware, privilege, budget or deadline changes.
