# Independent consumer support for checked phase v2 — proposed increment

2026-10-29. Read-only design under the existing native corpus and independent replay task. This packet is a proposal for the next software increment, not implementation, test evidence, campaign admission, or launch approval.

Consumer baseline: PR #2816 source `67629e36fc45775ad2166f80f0e7b9c0eab1d144`, standalone crate `research/ml_dataset_independent_validation_20261028/native/`, in `/home/ec2-user/crypto-ml-consumer-generic-filter-20261028`.

Normative producer contract: `research/ml_dataset_checked_phase_20261028/DESIGN.md` and its exact policy bytes at frozen commit `e9de83e5c6cc1ac2eb9633227b53d3ce3a568413`. The corrected producer phase runtime SHA is still pending owner delivery. The phase/native wire contract stays frozen unless its owner explicitly changes it; this consumer proposal does not silently redefine it.

## Proposed scope

Add an independent native validator for the finite supplied phase-v2 evidence in a sealed cell, integrate its results into the current consumer audit and restricted labels, and keep public feature bytes unchanged. A sealed cell can contain a valid start-only interrupted prefix; sealing is artifact packaging, not proof of a successful solver or complete execution.

The validator checks the recorded contract, binding and derived arithmetic. It cannot establish that a kernel actually performed the reported operations, that the binary ran from its claimed source, or that a PMU count measures total cost. No counter is opened by this consumer. Every hardware, platform, complete-execution and training-cost admission claim remains false; whole-worker count remains null.

Keep the core independent of its input adapter: `phase::validate_context` consumes an exact finite descriptor set and independently bound authoritative inputs. Initially call it only from the existing sealed-cell audit. A later restored-partial adapter can reuse it, but this increment does not add partial-snapshot transport, verify a snapshot receipt chain, audit unsealed directories, or certify S3 persistence. The current consumer `audit` lists unsealed attempts without checking their phase contents, and `fetch_cell` supports sealed `ml-dataset.publication/v1` only. Those limits must remain visible.

## Existing owners and non-overlap

Read-only Conductor checks identify T-766 as the claimed producer phase/snapshot task, with producer telemetry/report/live/snapshot/mod and dedicated research scopes. T-763 is the independent generic-filter consumer task; T-700 is the earlier consumer phase-label task. At the observed check, both were `ready` with empty scopes. T-754, the native collector task, was also `ready` with empty scopes; T-771 was the released consumer journal task. Their recorded owner is `ec2-user`. Exact timestamps and scope lists are in `owners.json`.

Released scopes are not an implementation claim by this reviewer. Parent and the existing consumer owner choose the implementation task and scope before any source change. This proposal does not claim or modify T-766, native collector code, producer validators, other consumers, journals or PRs. The existing USD200 total maximum, original deadlines and other owners persist.

## Why a new validator is necessary

At the pinned consumer, `audit.rs:436` recognizes phase evidence by role and then requires v1 schemas. Its `validate_metrics` checks legacy tuples and legacy scaling, including saturation behavior. V2 uses a strict native lifecycle, checked overflow, fixed policy and actual attempt binding. It must have its own dispatch and validator; treating the legacy parser as a v2 parser would conflate materially different contracts.

`common.rs` already provides unique-key JSON parsing, contained ordinary files, hash/length checks and protection against bytes changing between hashing and parsing. The generic-v2 reader adds hash-bound streaming classification and pre-allocation resource admission. Reuse those principles, not the generic filter's 32MiB wire limit or its arithmetic implementation. The existing standalone crate has no dependency on producer code and already has Serde derives.

One extra binding obligation is easy to miss: current `plan_info` checks protocol file hash and schema, but not the protocol freeze-body digest or all fields of `AttemptBinding`. The new v2-only path must independently perform those checks before reporting contract success. Do not treat the old audit result alone as a reconstructed phase binding.

## Dispatch and artifact admission

Recognize the union of reserved paths and relevant roles. Inspect the exact `measurement-policy.json` path, its `restricted-measurement-policy` role, the four finite phase start/end paths, and `phase-start` / `hardware-and-resource-telemetry` roles. A reserved path with the wrong role or encoding is an error; a relevant role at an unexpected path is also an error. A role-only scan must not allow misclassified v2 files to evade verification. Unknown telemetry JSON within a checked attempt must fail rather than become an ignored measurement.

The checked set is at most nine JSON artifacts: policy plus a start and optional end for each phase. Require unique descriptor paths, ordinary contained files, exact byte length/hash and encoding `json`. A present reserved policy file not admitted by the sealed manifest must not cause a legacy fallback. Missing policy with v2 evidence, malformed policy, mixed v1/v2 records and unknown versions reject. Unrelated manifest artifacts retain their existing handling; this is not a request to parse every unrelated JSON file as phase evidence.

No policy and only valid legacy evidence uses the unchanged v1 path. No telemetry remains a valid legacy/no-measurement case. Exact policy with no phase records is a valid checked policy-only context, with zero checked phases and no vacuous claim of measured work.

The policy must be both strict-typed and byte-exact: 785 UTF-8 bytes including LF, SHA-256 `45c9f43fa4df83c6c478094a84478690dd791e27fccaa19af5b87db33f6b4a50`. Rehashed replacements, alternate whitespace, missing LF and unknown fields are invalid. The policy is evidence metadata, not a configurable consumer option or public feature.

## Independently reconstruct authoritative binding

Use hash-bound bytes of `run.json`, `plan.json`, `protocol.json`, `config.json`, and `attempt.json`, admitted by the manifest. Validate the existing plan/schema/shard and exact mixed-radix cell configuration. Require original run and attempt schemas, matching cell index, full lowercase source commit and binary SHA-256.

There are distinct hash domains:

1. The run's and attempt's `plan_sha256` bind compact serialization in the producer plan's declared field order, including v1/v2 policy-axis differences. Reuse the consumer's independently implemented canonical plan/index logic, with vectors for both forms.
2. The binding's run/plan/protocol/config/attempt hashes are hashes of the actual local file bytes, including whitespace and LF. The binding's `plan_sha256` is therefore not generally the canonical plan digest from item 1.
3. The protocol file hash must match the plan. Its freeze block must have frozen status, `filed_before_affected_data=true`, nonempty frozen timestamp and valid body digest. Remove only the top-level freeze block, then independently compute the pinned protocol canonical body digest.
4. A phase end's `start_sha256` binds the exact immutable start bytes, not a deserialized/reformatted equivalent.

The protocol canonical encoding is the pinned `experiment_protocol.py` contract referenced by producer `mod.rs`, including sorted object keys, array order, UTF-8 strings, arbitrary integer values and Python-compatible finite-float formatting. It is not generic sorted Serde JSON and must not be replaced with RFC8785/JCS. Preserve number tokens through a dedicated bounded reader; do not globally change the consumer's legacy `Unique(Value)` representation. The authored `protocol-canonical-vectors.json` specifies exact bytes/digests for 14 positive and 11 negative cases, including integers outside u64, negative zero/floats/exponent boundaries, Unicode keys/strings, nested arrays and removing only the top-level freeze. These are design fixtures, not executed consumer validation. Supporting the full official experiment-protocol schema remains outside this digest/binding check.

The independently constructed `crypto.ml-dataset.attempt-binding/v1` must equal every recorded phase binding. Rehashing a transplanted attempt, source/binary identity, plan or config cannot substitute for this reconstruction. These checks establish internal binding to supplied bytes; authenticity of source, timestamp, execution or kernel identity is not established.

## Strict phase and lifecycle contract

Use independent owned wire types with exact fields, closed enums and required nullable fields. Reject duplicates, missing required keys, unknown keys, wrong JSON number tokens, overflows and trailing input. No permissive `Value` subtree or untagged version fallback may bypass strict nested-native validation.

Phases are exactly construction 0, known-answer replay 1, factor-base replay 2, solver 3, with the frozen path spellings. A supplied prefix is dependency-closed: no end without start; no later start without all earlier successful ends; no continuation after failure; no repetition. A valid start-only prefix is `interrupted_or_unfinished` regardless of the overall packaging. Do not invent an end, zero count, operation result or successful solver label.

Validate start schema, phase/ordinal, reconstructed binding, policy digest, positive recording PID, exact durable-start boundary string and strict optional process usage. Validate end schema, same identity, actual start-byte digest, complementary returned-ok/error state, strict usage, canonical elapsed diagnostic and every required false/null claim. Resource snapshots are process-cumulative diagnostics; do not subtract them or interpret RSS high-water values as phase allocations.

For an error summary, check the UTF-8 preview bound, byte count, digest syntax and consistent truncation. When untruncated, independently verify preview byte length and hash. When truncated, the missing suffix cannot be verified: preserve its claimed length/hash explicitly as producer evidence, not as an independently checked error message.

Independently validate the nested `checked-native-pmu/v1` evidence with a finite-state checker. Sharing the documented wire definition or a standard hash/parser library is allowed; importing or calling producer lifecycle/projection validators is not. Before implementation, inventory and pin all nested field sets against the final corrected source, without changing the frozen schema.

- Policy is calling-thread with optional cycles and a single pre-work standalone-instruction fallback. Preserve failed attempts. At most two attempts and two events per attempt are possible. No retry after work/read failure and no fallback after cleanup failure.
- Check platform/ABI declaration, owner PID against the phase start and all required PID/TID context evidence. Following the owner's native `4a57` correction, the supported syscall backend is little-endian, 64-bit Linux x86_64/AArch64 only. An unsupported big-endian record may still declare request byte order but must have no native calls/TIDs. Unsupported platform/event and permission outcomes remain valid explicit unavailable evidence. A valid unsupported outcome must not be rejected merely for lacking a successful syscall record. The final corrected native/phase source pins must confirm these states before interoperability acceptance.
- Decode all 112 requested attr bytes using the recorded byte order. Check event type/config, size, read format 7, leader/member flags 97/96, pid 0, cpu -1, CLOEXEC 8, no inheritance and all other frozen bytes. Check process-wide byte-order consistency and successful descriptor/group relationships. An unattempted cycles placeholder has group_fd -1 and leader-style attrs; it is not evidence of an opened member.
- Reconstruct legal open/ID/reset/enable/disable/read/close order, exact externally tagged `Ok`/`Err` results and bounded context checks. Descriptor uniqueness concerns simultaneously open descriptors within an attempt; a later fallback may legally reuse a descriptor number after it was closed. Validate event-ID equality/uniqueness and zero-ID treatment exactly as allowed by the final pinned native contract; do not invent a positive-ID requirement. Check selection/fallback sequence and all required fixed claim fields. The wrapper supports one start; a repeated-start lifecycle is invalid for this phase contract even though native grammar can name it.
- Verify each recorded status and assessment against its lifecycle and tuple. Distinguish identity preparation failure from a read-ID mismatch. Do not equate `prepared`, cancelled, unsupported, unstarted, context/control failure, short read, invalid time, not running or overflow with zero work. Preserve diagnostic tuples even when they cannot support a projected observation.
- Recompute the ordered per-event observation projection and compare it exactly. Clean usable tuples require the recorded successful interval/ID/context predicates, exactly 32 read bytes, positive running time and `running <= enabled`. Measured zero is possible when running is positive. For equal enabled/running the scoped raw count is retained; multiplexing uses floor(raw × enabled / running) in u128 with checked u64 conversion. Overflow is unavailable, never the v1 saturated maximum. Cycles cannot validate an incomplete instruction tuple. Cleanup errors stay visible and prevent `lifecycle_clean=true`; an observation is still a producer diagnostic, never admitted cost.

The complete frozen native status/operation grammar is in the retained producer DESIGN excerpt. Do not shorten it to success/failure or discard unavailable records to make fixtures pass. Any mismatch between that design and the final producer source is an owner reconciliation item before interoperability acceptance.

## Bounded parsing and resource admission

Proposed independent software envelope, to freeze with the implementation owner:

| Input/state | Limit and enforcement |
|---|---|
| Policy/start/metrics files | 16KiB / 64KiB / 512KiB, checked before allocation/read; exact policy still only 785 bytes. |
| Checked phase set | At most 9 artifacts / 4 phase pairs; summed raw phase caps at most 2,375,680 bytes. Checked sums, no duplicate descriptors. |
| Native nested arrays | Attempts ≤2, events per attempt ≤2, calls per event ≤12, context checks ≤2; reject at the next element before growing a vector. |
| Error preview | At most 4096 UTF-8 bytes. Other native diagnostic strings remain bounded by their frame and resource admission; do not invent a smaller wire cap silently. |
| Parser depth | Proposed independent cap 64, checked before descent; unknown nested extensions reject. |
| New module modeled live storage | Proposed 32MiB across retained phase output, typed structures, raw buffers, binding/canonicalization scratch and temporary clones; reserve before allocation. This can refuse producer-valid large provenance objects as `consumer_resource_limit`; it is not producer censoring. |
| Existing provenance/general files | Preserve existing 64MiB raw JSON limit and artifact integrity rules. New binding work also consumes the new module's model. Do not increase generic-filter or existing whole-task caps. |

Use a hash-bound bounded read or stream with size-plus-one detection, strict EOF, bounded keys/strings and structural admission before materializing a `Value` forest or vectors. A post-deserialization length check is insufficient. Protocol integer/string scratch and canonical output must be charged; streaming its digest avoids constructing a second unbounded body. Existing generic-reader tests provide an early-refusal pattern but its domain constants cannot be copied blindly.

The 32MiB model is an explicit proposed per-cell component envelope, not a whole-process RSS, allocator, CPU-time or total-cost proof. The existing `plan_info`/manifest `Value` allocations and audit retention across many cells remain outside this component and are not bounded by the new claim. Every new raw/typed buffer and clone introduced by this component, including copies of existing values, protocol numeric tokens/canonicalization scratch and restricted phase output, must be reserved before allocation. A valid input outside admission must return a distinct consumer-resource-limit error, with no successful check, partial count label or fabricated producer result. Invalid wire evidence remains a validation error. Keep both separate from a valid unavailable counter or interrupted phase.

## Restricted output and leakage controls

Preserve `static_features`, split selection and public feature schema. For the same admitted public/base/config inputs, v1 versus v2 telemetry and all private counter/outcome/provenance changes must yield byte-identical `features`. IDs, labels and split metadata are not model inputs. Keep existing whole-field-size grouping and its documented cross-size limitations; this increment does not introduce a stronger component split.

Keep phase evidence in `labels.phase_measurements`. Add a v2-only, independently versioned restricted summary such as `labels.checked_phase_contract` (name proposed), with validator/policy identity, supplied starts/ends, validated prefix, input artifact identities and separate binding/lifecycle/projection-check status. A policy-only context has zero checked phases. A start-only phase can have checked start/binding but no lifecycle/projection result. Do not use a single ambiguous global `verified=true`.

Original producer records stay immutable, including their `consumer_v2_admitted=false`. The consumer's separate result can state that this version's finite recorded contract was checked; it must not rewrite the producer admission bit or imply hardware/platform approval. Keep `labels.independently_verified=false`, whole-worker count null, training-cost eligibility false, execution-completeness false, PMU truth unverified, and comparison-clock admission false. Completed phase ends do not prove complete solver execution or validate terminal mathematical labels.

Policy hash, source/binary/attempt identity, PID/TID, platform, attrs, native errors, availability/multiplexing, counters, usage, elapsed time, outcome/status, witness/scalar and parser/admission result all remain restricted. Do not make feature presence, feature hash or split selection conditional on a private success/counter value. Keep valid unavailable/start-only samples; no success-only or measured-only implicit filtering. Invalid evidence rejects the audit/export rather than producing a silently altered row. Consumer resource refusal is explicit and cannot be presented as a producer-censored training label.

Retain the existing export gate: producer replay or explicit unreplayed preview, separately labeled. Producer replay is not the independent check. Audit phase semantics regardless of the optional algebraic `--arithmetic` flag; that flag controls separate mathematical certificate replay, not whether malformed measurement evidence is accepted.

## Compatibility and delivery limits

V1 phase parsing/labels and legacy static exports remain byte-compatible; do not replace their historical semantics with the new checked overflow model. Consumer static-routing-view v1/v2 are feature-policy versions and are independent of phase-start/metrics v2. The root, factor-base and generic-filter certificate checks remain unchanged. Manifest-driven sealed fetch already retains declared phase artifacts and requires no new cloud transport for this increment; use the existing mock transport to verify that property.

Before declaring checked-phase consumer support: reconcile the final corrected producer SHA and schema/policy hash; pin independent consumer source and executable; complete the acceptance matrix in ACCEPTANCE.md; pass standalone native regression/strict lint and retained fixtures; then obtain source review and existing durability/delivery gates through their owners. MODULE_BOUNDARIES.md specifies the isolated branch stacked on exact PR #2816 source and design-freeze-before-implementation sequence. Do not reuse a historical binary or a same-implementation replay as independent proof. No producer execution, hardware experiment, cloud generation, IAM change, extra budget or rollout is authorized by this design packet.

Remaining after this increment: independent partial-snapshot chain/restore adapter; platform/PMU calibration or trusted execution; whole-worker/whole-solve accounting; admitted training-cost targets; broader corpus completion. All remain distinct open deliverables.

## Sources and design basis

Repository sources are pinned in `source-pin.json` and mapped in SOURCE_MAP.md. The consumer README's feature/measurement contract is the directly referenced learning-policy note; no broader training contract is claimed. Memory guidance was used only to preserve design-first planning and restricted-label separation; current code and ownership were read afresh.

The Linux manual specifies calling-thread targeting, grouped descriptors and read fields, and describes enabled/running scaling as an estimate. This supports preserving scope and diagnostics without promoting records into hardware truth. [perf_event_open(2)](https://man7.org/linux/man-pages/man2/perf_event_open.2.html).

Serde's strict-structure option rejects unknown fields; its default-filling and untagged behavior do not supply the frozen required-nullable semantics automatically. Use explicit required fields and closed dispatch. [Serde container attributes](https://serde.rs/container-attrs.html).

JSON permits implementation size, nesting, numeric and string limits, while object member uniqueness is important for interoperability. The new admission limits are explicit consumer constraints; they do not change the producer wire contract. [RFC8259, sections 4 and 9](https://www.rfc-editor.org/rfc/rfc8259.html).
