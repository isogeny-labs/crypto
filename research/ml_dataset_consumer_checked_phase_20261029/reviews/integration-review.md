# Independent checked-phase integration source review

Status: final bounded source endorsement on 2026-10-29, after the owner supplied
and this reviewer inspected the corrections below. No remaining actionable issue
was found in the exact final integration snapshots. Original and intermediate
snapshots are retained; this result is not an executed test result, CI approval,
whole-process memory proof, or hardware/execution-admission decision.

This reviewer did not author phase.rs, io.rs, budget.rs, binding.rs or their
audit/lib integration. The authored protocol.rs is excluded from this independent
review. Native/schema have a separate source review. No repository edits, build,
Cargo, producer/generator/PMU execution, cloud actions, PR changes or task-owner
mutations are part of this review. Only this external packet is written.

## Original snapshot and review findings

The owner supplied these stable source hashes; copied bytes were checked before
inspection. Original paths are under
`/home/ec2-user/crypto-ml-consumer-checked-phase-20261029/research/ml_dataset_independent_validation_20261028/native/src`.

| Original path | Packet copy | SHA256 |
| --- | --- | --- |
| phase.rs | source/phase.rs | 9de471fb4c08ecb8965beb0355001c100868bdf1b43fdba029ae639f1acce729 |
| phase/io.rs | source/io.rs | 4972c4ffb8c53c0e65aeda2e513625575e793153679784af4dc9f0d7a764f9e7 |
| phase/budget.rs | source/budget.rs | 86b70ac1b64bd60d88dee2d5b855f3466cba923cb2301ed3736c2869b7c3a150 |
| phase/binding.rs | source/binding.rs | 87651a91e7fde733ffac8d7e985df34f28f9da3abbad8a85861b1b9296970e9f |
| audit.rs | source/audit.rs | 786367fbe3aa1c25f3f7490a47916a23f4efb195c9d26b63629740b6019d0aff |
| lib.rs | source/lib.rs | 695458f99d9c97e1429113c21a7a91b1e1346ece5fd5537a46401120e6976e7a |

The unchanged common/policy helper snapshots come from parent consumer pin
`67629e36fc45775ad2166f80f0e7b9c0eab1d144`. Their relevant filesystem, canonical
plan/index and policy-recipe contracts were inspected to assess the new callers;
this is not a new broad review of those modules.

The final source endorsement applies to this exact set:

| Final path | Packet copy | SHA256 |
| --- | --- | --- |
| phase.rs | source/phase-final2.rs | ab55a13d3e32cdb75b5917bcbbc6c83ec499f5211760c3545c4a8f778bf31e5e |
| phase/io.rs | source/io-corrected.rs | d74e8203acb8e37355445a04a020f18df7c66d217d2d72cdb0037f9b569e67a0 |
| phase/budget.rs | source/budget.rs | 86b70ac1b64bd60d88dee2d5b855f3466cba923cb2301ed3736c2869b7c3a150 |
| phase/binding.rs | source/binding-corrected.rs | 7f37c84a1239ba12ad34fa354524c705c39f43eefd2d2bfad0786af25d2c35bc |
| audit.rs | source/audit-final.rs | d7da9adbf9536243aaa2adc80345f06cfa3277c328436c90e4b3580d9edb4af8 |
| lib.rs | source/lib.rs | 695458f99d9c97e1429113c21a7a91b1e1346ece5fd5537a46401120e6976e7a |

These are owner-supplied working-file identities, not an inferred commit/build
identity. Later formatting or code changes require an explicit equivalence or
delta review. SHA256SUMS binds every retained source, diff and review document.

### F1 — legacy probe second read could continue beyond declared byte identity

Original `source/io.rs:103-118` hashes the bounded marker prefix, then drains the
same handle until EOF. It checks actual read count against the manifest only after
that drain. A regular file growing after the first artifact check can therefore
force arbitrary extra second-read I/O or prevent EOF. The existing artifact
checker rejects count overrun during its loop; the new second read lacked an
equivalent bound. A finite mutation still fails the final hash/count checks, so
this finding concerns bounded reading, not acceptance of forged bytes.

The owner supplied `source/io-corrected.rs`, SHA256
`d74e8203acb8e37355445a04a020f18df7c66d217d2d72cdb0037f9b569e67a0`.
The shared Hashed reader now permits at most declared size+1 bytes, covering both
the marker parser and drain; exact final count rejects the extra byte. Its added
infinite-reader test checks that no later suffix is consumed. Source inspection
finds this correction addresses F1. The reviewer did not execute that test.

### F2 — canonical-plan allocation prepayment needed an explicit bound

Original `source/binding.rs:32-33,47` invokes canonical_plan, serializes its cloned
tree, and repeats plan validation in config_at. Those helpers create deduplication
encodings/sets, canonical copies, serialization buffers and temporary config.
The broad node/string admission credits did not explicitly justify this group of
allocations, especially escaped or long v1 solver strings and serialization
capacity growth. This was raised as a resource-accounting review gap, not an
observed RSS or allocation failure.

The owner supplied `source/binding-corrected.rs`, SHA256
`7f37c84a1239ba12ad34fa354524c705c39f43eefd2d2bfad0786af25d2c35bc`.
It reserves an additional checked32*(raw plan bytes+1) before canonical_plan and
before the later config_at call, against the existing cumulative cell budget.
The source includes additive boundary/overflow refusal tests. This addresses the
missing explicit prepayment in the stated modeled-storage contract and can
refuse large producer-valid inputs earlier as consumer_resource_limit. It does
not establish an RSS, actual allocator-volume or CPU-time bound. The reviewer did
not execute the new tests.

### F3 — reserve fixed envelopes before parser/path work

The owner subsequently added a narrow large-integer compatibility fallback.
Intermediate `source/phase-final.rs:33-36` invoked the dedicated verifier before
reserving the frozen fixed parser/bookkeeping envelope. Its audit_sealed also
performed root.join/metadata work before charging that envelope. This ordering
did not match the contract's before-allocation wording.

Final `source/phase-final2.rs:33-37,85-92` reserves131072 before invoking the
protocol parser, carries its complete usage, then charges carried usage plus the
checked-cell's own131072 before root.join. Counting both envelopes is deliberately
conservative. The source correction addresses F3; no extra preflight allowance is
silently dropped or reset when a checked cell begins.

## Integration properties traced

- Original phase.rs74-115 refuses an unmanifested physical policy, recognizes
  checked policy by path, checks the union of relevant roles and telemetry JSON
  paths, requires exact role/path/JSON descriptors and at most nine members. With
  no policy, the marker probe ignores role/encoding for telemetry JSON and keeps
  decoded v2 recognition sticky through later parse errors. Pre-marker malformed
  or oversized opaque diagnostics remain legacy under the frozen clarification.
- phase.rs117-148 checks exact policy bytes/hash and its strict field inventory.
  All checked raw inputs are opened as contained ordinary files, read at exact
  declared size with one-byte EOF probing, and rebound to descriptor hashes before
  typed construction. The prior artifact integrity check is not trusted alone.
- io.rs122-272 performs structural admission before typed/general construction,
  charging node/string credits and rejecting the first excessive native-list
  item before parsing it. Required nullable and unknown-field behavior remains
  the responsibility of the strict typed schema, already reviewed separately.
- binding.rs reconstructs five authoritative raw identities and checks canonical
  plan identity separately from raw plan-byte hash. It checks run/attempt schemas,
  index/config mixed-radix equality, shard relation and full source/binary syntax,
  then verifies protocol file hash and calls the dedicated protocol verifier.
- phase.rs223-278 checks dependency-closed phase order, start/end identity,
  actual start-byte digest, result/error consistency, diagnostic usage/time and
  every required false/null claim. Both native start-return and observations are
  compared against the independent replay outputs. Full error previews receive
  hash/length checks; truncated suffixes remain explicitly unverified.
- phase.rs279-305 separates policy-only, completed recorded prefix, operation
  error and interrupted/unfinished states. It preserves raw producer records and
  does not promote recorded completion into verified complete execution.
- audit.rs693 invokes the checked path regardless of the optional algebraic
  arithmetic flag. The absent checked path retains the existing v1 handler.
  New summary fields are added only to the audit row and restricted export labels
  (audit.rs702-703,1115-1116). static_features and split_for are unchanged in the
  supplied diff; neither reads the new summary. No source-based feature leakage
  path was found. Actual byte-equality regressions remain an owner test gate.

## Final large-integer compatibility integration

The owner identified an integration issue separately: the existing plan_info/cell
protocol Value reader could reject an integer beyond binary64 range before the
dedicated number-preserving verifier. Final `source/audit-final.rs:232-256` keeps
the original read_json_bytes path and schema check for every previously accepted
protocol. Only a downcast serde_json number-out-of-range error enters the strict
full-protocol verifier; other parse failures still propagate. The fallback is not
a permissive schema-only parser and does not admit nonfinite float values merely
because the old reader could not represent them.

PlanInfo carries the fallback's modeled reservation into each audit_sealed call.
The existing canonical-plan/run/source/shard checks remain in place. The cell
protocol descriptor must match the plan's exact protocol-file digest, and a fresh
hash-bound artifact read rechecks the same cell bytes before the checked phase
path rereads/reverifies them. Both raw-file identity and canonical freeze-body
identity therefore keep their separate domains. This review inspected the
wrapper, error dispatch, reservation handoff and fresh-hash call sites only;
protocol.rs itself remains outside this independent endorsement.

The owner reported a1000-digit integer CLI positive and nonfinite1e400 rejection,
but this reviewer did not run or independently inspect those execution receipts.
They are not counted as validation evidence in this source review.

## Allocation and output claim boundary

The model consists of the early finite envelope,4*(raw size+1),1024 per parsed
node,8 times decoded key/string bytes, bounded typed-list capacity, separate
32*(raw plan size+1) canonical/config scratch, the protocol module's own charged
allocations, and carried preflight reservations. The pre-admission credits cover
typed/raw-Value/retained output copies; finite descriptors and summary fields use
the fixed envelope. Existing plan_info/manifest trees and cross-cell audit/export
retention remain outside the component's new bound. These are conservative
modeled-storage credits, not a measurement or proof of actual allocator traffic,
RSS, CPU time or total job memory. Large otherwise valid inputs can be refused.

The final audit diff adds the checked summary only under restricted labels and
does not change static_features, its input selection, or split_for. Policy-only,
start-only, unavailable, operation-error and completed recorded prefixes retain
explicit states. No whole-worker sum, admitted cost, hardware truth, complete
execution or partial-snapshot-chain claim is introduced. No remaining actionable
integration issue was found in the final reviewed snapshot set.

All execution, strict lint, retained-fixture interoperability, software durability,
CI and delivery remain owner gates. Private counter/outcome/provenance evidence
stays outside public features; PMU truth, admitted training cost, full execution
and independent partial-snapshot lineage remain unverified.
