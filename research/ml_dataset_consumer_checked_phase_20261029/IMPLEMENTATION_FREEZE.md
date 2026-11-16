# Sealed checked-phase consumer implementation freeze

Parent authorized this increment on 2026-10-29 after reviewing DESIGN SHA256
70ff59d2394b245516293489144534e904335f36408ea6fb4d994990d5506713 and ACCEPTANCE
b346160e550800f93f4bc6efa678a66aa736973dd70069d390ef99b482d25a46. Their original
proposal wording is retained verbatim; this document records implementation
admission, concrete interfaces and output choices before runtime edits.

Base is PR2816 commit67629e36fc45775ad2166f80f0e7b9c0eab1d144. Conductor T-780 owns
only the standalone native consumer crate, this research directory and its consumer
README. Producer runtime9f1d54818cd791a25d9beeb74840fd0b94bf2de9 supplies the wire
inventory; its validation is pending an owner compile correction. Any final
interoperability claim will name the corrected owner pin and retained fixture
hashes. No producer execution, PMU open or cloud worker is part of this increment.

## Exact restricted output

Only checked cells gain `checked_phase_contract` on the audit row and
`labels.checked_phase_contract` on export. Its schema is
`ml-dataset.checked-phase-contract/v1`. Fields are: schema, validator
(`independent-sealed-phase-v2/v1`), policy_sha256, binding (the independently
reconstructed AttemptBinding), input_artifacts (exact policy/start/end descriptors
in policy then ordinal order), supplied_starts, supplied_ends, prefix_state
(`policy_only`, `completed_recorded_prefix`, `interrupted_or_unfinished`, or
`operation_error`), phases, authoritative_binding_checked, modeled_storage_limit,
modeled_storage_reserved, and the explicit claims below.

Each phases element has phase, ordinal, start_sha256, end_sha256:null|string,
state (`recorded_end_checked` or `interrupted_or_unfinished`),
start_binding_checked:true, lifecycle_checked:bool, projection_checked:bool,
error_message_check (`not_supplied`, `full_preview_checked`, or
`truncated_suffix_unverified`). Policy-only has no phases and zero counts;
start-only has no fabricated lifecycle or projection check. Recorded end success
does not assert execution completeness. Original supplied start/end objects remain
in phase_measurements, separate from this summary, with producer bits unchanged.

Summary claims are always independently_verified:false,
hardware_independently_verified:false, platform_admitted:false,
training_cost_eligible:false, complete_execution_verified:false,
clock_valid_for_comparison:false, whole_worker_instruction_count:null,
partial_snapshot_chain_verified:false. No sum or cost proxy is emitted.
Legacy rows and exports gain no new field. Public features and splits are unchanged.

## Concrete allocation admission

The per-cell new component limit is33,554,432 modeled bytes. This is a conservative
cumulative reservation ledger: reservations are not recycled within the call, so
it bounds the modeled live storage even when earlier scratch has already dropped.
It can refuse a producer-valid record as `consumer_resource_limit`; this is neither
producer censoring nor successful validation. It is not an RSS/allocator/time bound.

- Before new component allocations, reserve a fixed131,072-byte envelope for the
  finite binding/descriptor/summary bookkeeping, native replay cursors, stack and
  parser-control scratch. This is separate from variable input charges.
- Each raw input reserves four times (declared size plus one), before allocating a
  file buffer. Parent review clarified this before any new consumer execution:
  logical decoded length alone does not bound geometric string/scratch capacity.
  This covers the fixed raw Vec capacity plus bounded parser decoding scratch and
  one temporary decoded key, including geometric capacity slack. Retained strings
  and output copies remain separately charged. Ordinary contained file,
  descriptor length/hash, size-plus-one EOF and second-read binding remain required.
- Before typed or general JSON construction, a nonmaterializing structural pass
  charges1,024 bytes for each scalar/container node and8 times decoded string
  bytes. These conservative units cover typed storage, container capacities,
  strict parser bookkeeping and up to four retained/temporary output-tree copies.
  No container is grown during this admission pass. Input/scratch bounds apply
  even to rejected unknown fields. Depth is64; excessive depth refuses before
  descent. The raw reservation covers temporary string decoding before its
  length-dependent retained charge.
- Typed arrays use bounded deserialization: attempts2, events2, calls12,
  context checks2, observations4, attr bytes112, with the first excess element
  rejected before vector growth. Actual field/status validation follows strict
  typed parsing; missing nullable keys remain errors.
- Binding rereads of five authoritative files are charged as new raw inputs and
  their new JSON/protocol structures are charged. Existing plan_info/manifest
  allocations remain outside this component, but canonical-plan copies and all
  newly introduced copies are inside its reservations.
- The protocol module reserves its nodes, strings, integer tokens, key-sort and
  float/string canonical scratch before allocation using the same ledger. It
  streams canonical output into SHA256; tests that retain canonical bytes charge
  that buffer separately. It does not change global serde number semantics.
- Policy/start/end caps are16KiB/64KiB/512KiB before allocation, at most nine
  checked members. The unchanged general raw-file cap is64MiB; the32MiB component
  can refuse it earlier. Reservations persist through result handoff; cross-cell
  pre-existing audit/export retention is outside this component's bound.

## Module and collaboration interfaces

`phase::audit_sealed(root:&Path, members:&BTreeMap<String,Value>, index:u64,
canonical_plan_sha:&str) -> anyhow::Result<Option<CheckedPhaseAudit>>` dispatches
reserved path/role/policy evidence. CheckedPhaseAudit owns measurements:Value and
summary:Value. None is allowed only for legacy/no-checked evidence. V2 binding
validation is independent of the algebraic arithmetic flag.

`phase/budget.rs` defines `Budget::new(limit:usize)`, `charge(bytes:usize,
what:&str)->anyhow::Result<()>`, `used()->usize` and `LIMIT`. charge checks overflow
and returns an error beginning `consumer_resource_limit` before allocation.

`phase/protocol.rs` owns only independent token-preserving protocol parsing and
canonicalization. Its sibling-visible entrypoint is
`pub(super) fn verify(bytes:&[u8], budget:&mut super::budget::Budget)
-> anyhow::Result<String>`: input raw storage is already charged by the caller;
return the verified canonical body SHA256 after exact schema/freeze checks.
The module must charge returned digest and all its own allocations, reject
duplicates/trailing/nonfinite input, remove only top-level freeze, and reproduce
all14positive/11negative vectors in protocol-canonical-vectors.json. It may use
standard parsing/hash libraries but no producer imports or Python implementation.
Its unit tests live in that file; its owner must not edit parent module, Cargo,
budget, integration or other files. Consumer owner performs final integration,
standalone build/test coordination and delivery.

`phase/schema.rs` independently owns the inventory in WIRE_INVENTORY.json;
`phase/native.rs` checks the documented FSM and exact derived projection without
calling or importing producer validators. `phase/io.rs` handles bounded identity
reads and structural preflight; `phase/binding.rs` reconstructs raw-byte binding
and reuses only the existing consumer canonical-plan/index implementation.

## Source and evidence boundaries

The source inventory pins every nested wire field. Enums remain closed: the
native status/operation grammar in the approved design, two task policy choices,
four phase names, known observation/context states and little/big byte order.
Lifecycle Ok/Err is one externally tagged key; required nullable fields are
present even when null. Byte-order/FD/ID details follow the final owner's retained
contract, including unsupported big-endian evidence, unattempted cycles placeholders
and legal closed-FD reuse by fallback. No raw PMU record establishes hardware truth.

Primary references reviewed before implementation: Serde container attributes
(https://serde.rs/container-attrs.html), RFC8259 sections4/9
(https://www.rfc-editor.org/rfc/rfc8259.html), and Linux perf_event_open
(https://man7.org/linux/man-pages/man2/perf_event_open.2.html). They inform strict
structure, explicit parser admission and calling-thread/diagnostic scope. The
repository's pinned protocol encoding remains normative rather than generic JCS.

The approved acceptance matrix, retained producer fixtures, parent source review,
fresh S3 restore/hash verification and regular PR stacked on2816 remain required.
No runtime validation or completed implementation is asserted by this freeze.

## Implementation clarifications after the initial freeze

The absent-policy adapter preserves legacy opaque diagnostics. It hashes the full
same file while inspecting only a bounded512KiB-plus-one prefix, recognizing a
decoded top-level checked-v2 schema regardless of role or encoding. Recognition is
sticky through later duplicate/trailing parse errors. Malformed prefixes and
oversized opaque diagnostics are not checked-v2 admissions and remain legacy
diagnostics. The per-file probe releases its bounded scratch and has no retained
output; the cumulative32MiB ledger applies to the checked-cell component.

Raw checked input uses exact-size allocation plus read_exact and a one-byte EOF
probe. Native array cardinality is enforced in the structural admission pass as
well as bounded typed deserialization. The protocol shortest-number primitive is
the pinned serde_json binary64 writer into a64-byte fixed stack buffer, followed
by independent Python-compatible layout normalization. This resolves the root
review's decimal tie counterexample; additive tie vectors supplement, without
altering, the original frozen14positive/11negative vectors. Protocol parser and
all canonical scratch still reserve against the same checked-cell ledger.

The producer native checker explicitly caps syscall-error message bytes at4096;
this implementation preserves that final wire constraint. The original proposal's
statement about not inventing a smaller diagnostic-string cap does not override
that inspected producer check. Producer27679c4e59bc38e8c18e34dbc9687f9a2d5eb317 has
byte-identical reviewed native/schema files to the9f1d inventory. Final retained
interop remains a separately reported gate.

The integration review further required an explicit reservation before canonical
plan cloning/serialization and repeated config validation:32times(raw plan bytes
plus one). It covers geometric encoded buffers, canonical-tree/config copies and
validation deduplication state, in addition to structural node/string charges.
The ledger may therefore refuse a large arbitrary v1 solver string earlier.
The legacy hash-bound reread also caps the reader at declared size plus one, so
an appended stream cannot extend the identity check indefinitely.

The original upstream protocol Value/schema preflight cannot represent integers
beyond binary64 range. Its previously accepted path is preserved exactly. Only a
serde_json number-range error can enter the strict frozen-protocol fallback;
IO, hash, duplicate, syntax and other failures cannot. The fallback uses the
independent token-preserving validator and returns its reservation in PlanInfo.
The checked-cell entrypoint receives this additional reservation argument and
charges it before its own allocations. A fresh cell hash must equal the already
verified root protocol digest, and the full binding verifier runs again under
the same cumulative ledger. This accepts bounded1000-digit integers without
weakening nonfinite-float rejection or changing global JSON numeric semantics.
