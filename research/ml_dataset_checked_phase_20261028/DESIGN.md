# Checked producer phase telemetry: concrete contract draft

2026-10-28. Concrete contract approved by the parent after final source review;
freeze this document before runtime edits. Scope is Conductor T-766. Base is native collector PR2801,
590b7668c964126b006f64ddfcc962094500f8c5; its tested runtime is9ab6260 and its
full native gate passed5208/0/123. The new branch is
codex/ml-checked-phase-20261028 in /home/ec2-user/crypto-ml-checked-phase-20261028.

Owned paths: src/ml_dataset/{telemetry.rs,report.rs,live.rs,snapshot.rs,mod.rs},
the new telemetry/ child directory, and this dedicated research directory.
The native collector and independent consumer are excluded; consumer T763 remains
active. No Cargo until parent admission. No new PMU campaign, platform admission,
cloud experiment, privilege change, quota change or cap increase. Original USD200
maximum and external owners persist. All fixture execution must use deterministic
mock counters/local software stores; a production collector being implemented is
not authorization to collect hardware observations.

## References and source findings

Reused the local checked-pmu-integration-design/{DESIGN,SOURCE_MAP}.md and native
frozen design. Primary references refreshed before selecting this contract:
https://man7.org/linux/man-pages/man2/perf_event_open.2.html (calling-thread
selection, group/FD controls and read format), https://serde.rs/container-attrs.html
(deny_unknown_fields, explicit strict structures; no flatten). Native source
checked.rs defines checked-native-pmu/v1. No independent hardware claim follows
from checking JSON or mock syscalls.

Source consequences incorporated before code:

1. mod.rs publishes its initial partial snapshot before spawning the worker.
   Exact policy must be installed before that flush; profile cannot switch later.
2. live.rs scans optional files one at a time: start may be absent before end
   appears. New membership must be dependency-closed, including phase order.
3. copied_context is only a path set. V2 must retain and recheck acknowledged
   path/role/hash/length identities, including deletion and replacement.
4. worker-artifacts.json is written only at worker completion. Panic/SIGKILL can
   leave telemetry later catalogued as unregistered-partial-artifact/native.
   Producer recovery needs exact finite path/role classification after strict
   content/binding validation; path alone never promotes an artifact.
5. Native IdentityFailed includes ID-ioctl/duplicate-group-ID preparation
   failures and is a valid optional-cycles fallback trigger. An unattempted cycles
   placeholder has group_fd=-1 and leader-style attrs; it is not an opened member.

Final read-only review: checked-pmu-integration-read-review/REVIEW.md,
SHA2568c176efffac94e8510199039ae621ccff889307d17db0bdf35929ed359dd3ec7.
All four review findings are addressed here; item5 elaborates its fourth finding.

## Fixed policy and admission

measurement-policy.json contains exact785 UTF-8 bytes including its final LF,
SHA25645c9f43fa4df83c6c478094a84478690dd791e27fccaa19af5b87db33f6b4a50.
The accompanying file is the normative byte contract. Runtime writes exactly
these bytes via the immutable byte writer. A typed strict parse plus exact-byte
comparison rejects changed content, extra fields, different whitespace, a missing
LF or an otherwise self-consistent replacement policy. The record hash is over
these exact bytes. This is a fixed source-bound instrumentation policy, not a new
plan axis, configurable environment override or a modification to old protocols.

New supervised attempts install it before the first snapshot or worker spawn.
Every new worker entry route validates a present policy before any counter/open
or operation. The existing no-policy legacy route stays v1 for old artifacts and
explicit legacy callers; a present malformed/mismatched policy cannot fall back
to v1. No new supervised attempt may select the no-policy route. Legacy saved
bytes and readers stay unchanged. The actual AttemptBinding reconstructed from
run/plan/protocol/config/attempt files must match, including canonical plan-cell
and protocol validation, full source commit and binary hash.

Policy uses CallingThread and OptionalWithStandaloneFallback only. Raw event
requests must match the native112-byte attrs: type0, instructions config1/cycles0,
read_format7, flags97 for leaders and96 for members, pid0,cpu-1, CLOEXEC8,
no inheritance, other attr bytes zero, exact declared byte order. An attempted
member's group_fd matches its leader's successfully opened descriptor. A native
unattempted cycles placeholder instead has group_fd=-1 and leader flags97; its
empty lifecycle cannot be treated as an attempted syscall. Platform and ABI
checks follow the retained native version; unsupported outcomes remain explicit.
All measurement, total-execution and hardware-admission flags stay false.

## Strict record types

All v2 objects use named-field strict structs with deny_unknown_fields and no
flatten/untagged fallback. Required keys remain required even if their value can
be null; missing-option and extra-field regressions enforce this. Enumerations
have closed sets. Raw JSON Value is not a permissive escape hatch for the checked
collection. A producer-owned strict mirror validates its nested native contract;
it does not import or modify the separately owned independent consumer.

Phase enum/ordinal: curve-construction=0, known-answer-replay=1,
factor-base-replay=2, solver=3. Exactly one start/end path per phase. No arbitrary
phase names, repetition, nesting or later phase after an earlier error.

PhaseStartV2 required fields:
- schema = crypto.ml-dataset.phase-start/v2
- phase: closed phase enum; ordinal:u8 matching the enum
- binding: existing strict AttemptBinding, reconstructed from authoritative files
- policy_sha256: exact fixed policy digest
- recording_pid:u32 greater than zero
- usage_before: null or strict ProcessUsage (existing getrusage numeric fields,
  explicit process-cumulative scope and RSS high-water unit)
- counter_boundary = durable_start_before_open

ProcessUsage required fields: user_cpu_us:u64, system_cpu_us:u64,
peak_rss_native:u64, peak_rss_unit:(bytes|KiB), minor_faults:u64,
major_faults:u64, voluntary_context_switches:u64,
involuntary_context_switches:u64, scope equal to the original explicit cumulative
worker-process/RSS-high-water string. Unsupported usage is null, never zero-filled.

PhaseMetricsV2 required fields:
- schema = crypto.ml-dataset.phase-metrics/v2
- phase, ordinal, binding, policy_sha256: identical to its start
- start_sha256: digest of the exact immutable start artifact bytes
- returned_ok:bool; returned_error:null or ErrorSummary, with complementary states
- counter_start_succeeded:bool, checked against actual selected-attempt lifecycle
- collection: strict complete native checked-native-pmu/v1 record
- observations: ordered, per-event diagnostic projection of the selected attempt
- usage_after:null or strict ProcessUsage
- elapsed_ns_diagnostic: canonical unsigned decimal string
- training_cost_eligible:false; whole_worker_retired_instructions:null
- whole_worker_coverage_verified:false; independently_verified:false
- platform_admitted:false; clock_valid_for_comparison:false
- interrupted_phase_readback:false; consumer_v2_admitted:false

ErrorSummary has message_preview:string, message_bytes:u64, message_sha256:hex,
truncated:bool. Preview is at most4096 UTF-8 bytes without splitting code points;
length/hash cover the full original error. The full original operation error is
still returned to the caller. If metrics persistence also fails, the returned
combined error preserves both failures; sensor failure alone never overwrites an
operation error or causes a second invocation. The error-summary digest cannot
prove the unavailable suffix's contents; mark truncated honestly.
Implement error summarization with a bounded fmt::Write sink that streams Display
formatting into SHA256/byte length and the bounded UTF-8 preview. Do not allocate
an unbounded new full error.to_string merely to truncate it. Preserve the original
Result and the separate persistence failure.

Observations contain event, attempt_index, event_index, status (scoped_raw,
scoped_estimate, unavailable), raw:null|u64, estimate:null|u64, lifecycle_clean:bool.
The projection is recomputed, not trusted. A raw observation requires successful
open/ID/reset/enable/disable/read with exact event identity and owner context,
positive equal enabled/running and read_bytes32. Multiplexing uses checked
floor(u128(raw)*enabled/running) to u64. Zero is a valid measured value only when
the counter ran. Invalid/control/context/error states preserve native diagnostics
but expose no admitted observation. Cycles assessment cannot promote instructions.
Cleanup failures remain visible and exclude a clean lifecycle. No exact-in-scope
legacy label is emitted or silently copied into v2.

Native nested validation requires the exact field sets, enum statuses, policy,
OS/architecture and attrs, owner PID equal to the start PID, valid context order,
attempt/fallback sequence, opened descriptor/group relationships, event-ID calls,
control/read/close outcomes, raw/assessment consistency and each false claim flag.
A failed pre-work attempt can precede one standalone fallback only under the
native policy; cleanup failure forbids fallback. No post-work rerun/fallback.
Kernel-reported events are still producer observations, not hardware proof.

Native status grammar is exactly: not_attempted, prepared, unsupported_platform,
unsupported_event, permission_denied, open_failed, identity_failed,
cancelled_before_start, not_started, context_mismatch, context_unavailable,
control_failed, read_failed, short_read, identity_mismatch, invalid_tuple,
not_running, observed_unmultiplexed, observed_multiplexed_estimate,
estimate_overflow. IdentityFailed is pre-work unavailable identity evidence and
can trigger the retained optional-cycles fallback; IdentityMismatch is a bad
read ID and cannot trigger a post-work fallback. ControlFailed may retain a valid
tuple assessment diagnostically, but lifecycle failure blocks its projection.
Native Operation is exactly open/event_id/reset_group/enable_group/disable_group/
read/close/repeated_start. Lifecycle Result wire format is the native externally
tagged one-key object {"Ok":i64} or {"Err":{"errno":null|i32,"message":string}};
both keys, absent keys, extra keys and unknown operation/status strings reject.
The phase wrapper invokes start only once, so a repeated_start call is invalid
under this supported measurement policy even though the native grammar names it.
Valid unsupported/identity-failed/placeholder/fallback fixtures precede tamper
tests, so strict parsing does not silently drop unavailable samples.

## Interval, persistence and bounded memory

Sequence: validate immutable binding/policy and preceding phase -> get usage ->
atomically install start -> open checked counters -> start diagnostic clock ->
start counter -> invoke operation exactly once -> finish/cleanup -> usage/elapsed
-> atomically install metrics -> return original Result. If start persistence
fails, neither open nor operation executes. If the operation panics, native RAII
cleans up under its own contract; only start evidence is promised. SIGKILL/OOM may
also leave start-only; no fabricated end, zero, total or success. Metrics write
failure preserves the original operation failure and the already-written start.

PMU interval is successful enable through disable, including wrapper/context
work. Start/end serialization occurs outside it, while solver serialization inside
the operation remains included. Resource and elapsed diagnostics have separately
stated process/clock scopes. No subtraction of instrumentation overhead.
Supervisor retired_instructions stays null/whole_worker_not_counted.

Bounds: policy16KiB, start64KiB, metrics512KiB, message preview4096 bytes, native
attempts<=2, events per attempt<=2, lifecycle calls per event<=12, context checks<=2.
Use bounded file reads and bounded serialization before allocation/install;
reject oversized or malformed objects. All four phases together have a fixed
small metadata bound. Existing snapshot16MiB context and cumulative store/budget
limits remain unchanged; count these bytes rather than raising a cap. These
bounds do not bound the solver's pre-existing allocations or original Result.
Rechecking acknowledged context is bounded by the same total16MiB context byte
envelope and at most19 finite file identities; a metadata replacement or growth
is rejected before an unbounded read. This is a tighter v2 admission rule, not a
change to legacy v1 snapshot byte semantics.

## Partial snapshot v2 and role recovery

Manifest schema ml-dataset.partial-snapshot/v2 selects CheckedPhaseV2 for an entire
attempt. V1 manifests remain byte-compatible and use the old finite allowlist.
A typed SnapshotProfile selects the new request path; legacy serialization omits
its default field. Concrete profile values are legacy_v1 and checked_phase_v2.
New helper request/prepared schemas are crypto.ml-dataset.snapshot-helper/v2 and
crypto.ml-dataset.prepared-snapshot/v2. The helper result carrying acknowledged
identities is crypto.ml-dataset.snapshot-helper-result/v2. V2 requests/results
require acknowledged_context:Vec<Artifact>; its path set must equal copied_context,
with no duplicate identity. V1 keeps this new field absent and its original bytes.
Schema/profile/identity-field mismatches reject. Generic receipt/catalog/
anchor hash-pointer envelopes can remain v1 only with byte-golden and binding
compatibility checks. The chain must reject any manifest-profile change.

Initial v2 context includes the original five mandatory records plus the exact
policy installed before worker launch. The v2 finite extra path/role mapping is:
- context/measurement-policy.json -> restricted-measurement-policy
- context/telemetry/{one-of-four-phases}-start.json -> phase-start
- context/telemetry/{one-of-four-phases}.json -> hardware-and-resource-telemetry
All are JSON. The corresponding local paths omit context/. No arbitrary context
prefix, extra file, role substitution or encoding substitution is admitted.

Maintain v2 acknowledged context identities as finite Artifact descriptors,
not only strings: path, role, encoding, hash, length. Before a subsequent snapshot,
revalidate every acknowledged original file against that identity; missing,
changed, symlink or replaced files fail. Preserve v1 helper semantics/bytes.
New descriptors become acknowledged only after the verified snapshot receipt;
a failed helper cannot advance identity state.

For each scan, membership is dependency-closed over acknowledged plus current
records. Policy precedes any phase. A start precedes its end; a later phase start
requires all preceding completed successful phases. If start is absent when
inspected, do not include its end even if it appears later during this scan.
A later snapshot can capture the newly complete prefix. Validate file contents
against that closed prefix before uploading. This prevents an interleaving from
manufacturing an end-only snapshot or a later phase without earlier evidence.

After a panic/kill with no worker-artifacts registry, recognize only those same
finite local paths/roles after strict v2 content/binding/prefix checks. Preserve
malformed bytes as incomplete diagnostics; never promote them based on path.
A v2 policy or phase failure cannot yield a sealed admitted observation. Report
start-only as interrupted_or_unfinished, even when it was recovered from the
supervisor's registry fallback. Unknown unrelated files retain their legacy
unregistered-partial classification.

Restore preflights chain structure, roles, profiles and cumulative budgets before
artifact GET. After installation it reconstructs AttemptBinding using actual
restored context files, validates the exact policy, and checks every available
start/end pair, order and recorded start hash. Hash-only agreement is insufficient.
No missing phase file is synthesized. Existing object/anchor lineage remains
immutable. Restored files do not restore solver memory, PMU descriptors, counters
or execution; no summing counts across attempts. Remote persistence is bounded
and polled; a host may lose records created after the last acknowledged prefix.
The restore namespace is artifacts/context/... under the restored attempt, not a
complete cell with manifest.json. A partial-attempt adapter passes that context
directory to authoritative binding/phase verification and retains original
path/role identities. It must not claim sealed-cell verification from a restore.

## Readers, leakage and acceptance tests

Producer verify_cell and restricted coverage reporting dispatch explicitly by
v1/v2 schema, check actual bindings and preserve original v1 report bytes. V2
restricted rows are versioned and retain source artifact identity plus diagnostic
interpretation. Static public features remain identical and exclude policy,
telemetry, source/binary/attempt identity, counters, status and outcomes.
The independent consumer is not edited; consumer_v2_admitted=false until its
separately owned checker/interoperability gate passes.

Deterministic tests before delivery:
- start-write failure means zero counter opens and zero op calls; sensor failures
  mean exactly one op; combined operation/end-write errors preserve both; panic
  produces only a valid start and cleanup runs;
- strict missing/extra-field and policy-byte tampering; ID/context/control/time/
  cleanup/status/estimate tampering; independent per-event classification;
- measured zero versus no running, overflow, unsupported and start-only;
- exact v1 phase/snapshot/helper/report goldens, unknown schemas reject, no mixed
  profile chain or malformed-policy downgrade;
- first pre-worker snapshot is already v2 with exact policy; all worker routes
  cannot create v2 phases behind a v1 initial flush;
- deterministic scan interleaving (start absent then end appears) retains a
  dependency-closed prefix, including policy/start interleaving and loss after
  acknowledgement; mutate/delete acknowledged file and reject;
- absent worker registry recovers valid start-only with exact role; malformed
  path-matching bytes never promoted; report and restore retain interruption;
- local mocked store upload/restore validates authoritative restored context and
  rejects rehashed binding/role/policy substitutions; budgets remain enforced;
- restricted view changes do not alter static feature bytes for equivalent
  instances. Consumer v2 remains unvalidated, not silently accepted.

Parent approved this contract including the exact helper/identity clarification
and bounded Display sink. Freeze it in Git before runtime changes. The parent
reserved the shared crypto Cargo slot for clean-source focused tests/strict lint
when implementation is ready; copy the immutable test binary before release.
The parent owns the full suite. Report any existing CLI integration route that
would open actual counters before running it; mock gates alone are not platform
admission. No Cargo or fixture execution has run for this contract yet.
No live PMU/platform/cloud campaign or independent-consumer claim is authorized.
