# Whole-worker instruction eligibility proposal

Version 2 preserves the original packet and incorporates independent source
review: platform/event semantics admission is required for the whole-attempt
projection, and uninterrupted exec-to-exit coverage is an explicit verified fact.

This external proposal adds a bounded, allocation-free policy classifier and
synthetic native tests for instruction-cost eligibility. It reuses the existing
checked collector as the integration donor; it does not collect counters,
implement a second collector, alter producer source or qualify an existing cell.
All numerical test tuples are software fixtures, not observations.

The inspected current CLI already installs the fixed checked phase policy before
launch. The older instruction-retention note's request to integrate that collector
has been completed at this scope. The remaining implementation is a separately
versioned whole-worker collection boundary and verified task coverage.

## Existing components and exact boundaries

Source root: `/home/ec2-user/crypto-ml-ordinary-ownership-20261101`.
Exact inspected files are recorded in `evidence/source-files.sha256`. The recovered
SAT snapshot has identical checked phase adapter/policy bytes; its new search
implementation does not add a whole-worker collector.

| Component | Current source | Actual scope |
|---|---|---|
| Checked native collector | `src/cryptanalysis/ecbench/isolation/checked.rs:15,68,452` | Thread-bound `CheckedCounters`; captures fixed request bytes, event IDs, controls, reads, cleanup and owner PID/TID. `pid=0,cpu=-1`; configurable new-descendant inheritance; user space only; close-on-exec descriptor. |
| Dataset adapter | `src/ml_dataset/telemetry/checked/mod.rs:478,502,515` | Always `CallingThread`, optional cycles with standalone-instructions fallback. Durable phase-start precedes opening; successful enable through disable covers phase plus wrapper work. |
| Frozen policy | `src/ml_dataset/telemetry/checked/policy.json` | `checked-calling-thread-diagnostic-v1`, hash `45c9f43fa4df83c6c478094a84478690dd791e27fccaa19af5b87db33f6b4a50`; all whole-worker/training/platform flags remain false. |
| Strict producer replay | `src/ml_dataset/telemetry/checked/native.rs:229` and `mod.rs:234` | Recomputes lifecycle/tuple projection and rejects promoted claims. This is receipt consistency, not independent evidence of hardware truth. |
| Four phases | `src/ml_dataset/telemetry/checked/types.rs:39` | Construction, known-answer replay, factor-base replay and solver. Phase counters are not additive coverage of the whole worker. |
| Worker | `src/ml_dataset/mod.rs:1494,1515,1533,1553,1565,1578` | Admission/setup, original artifact work, solver output adoption, result/registry writes and owner destruction occur outside or between the measured phases. |
| Supervisor | `src/ml_dataset/mod.rs:1849,1889,1904,1913,1953` | Spawns worker with `RAYON_NUM_THREADS=1`, process group and memory limit; monitors deadline/output; explicitly records whole-worker `retired_instructions:null`. |
| Process reconciliation | `src/ml_dataset/mod.rs:1257,2324` | Process-group cleanup and stale-worker checks are operational safeguards, not a complete race-free descendant census. A transient task or a descendant that changes group cannot be ruled out by endpoint scans. |
| Report | `src/ml_dataset/report.rs:398,406` | Restricted checked observations; `training_cost_eligible:false`, whole-worker count null and complete coverage false. |
| CLI capabilities | `src/ml_dataset/solvers.rs:1032,1050` | Declares hardware-instruction gaps and lost readback on interruption. Method work/memory reservations and group calls are distinct units. |

The legacy `HwCounters` path remains for unbound legacy calls. It is not the donor
for new cost labels: it discards control return values and has weaker tuple
admission. Preserve old bytes and semantics instead of rewriting old measurements.

## Proposed pure API

`eligibility.rs` exports `classify(&Facts) -> Decision`, plus typed scope, unit,
objective, outcome and coverage states. It makes no allocation, system call,
clock read or iteration. It distinguishes:

- Raw diagnostic observations, even when lifecycle or coverage is inadmissible.
- A multiplexing estimate computed with checked integer arithmetic; this never
  becomes an exact instruction-to-solve label.
- An unmultiplexed whole-attempt user-instruction observation after admitted
  platform/event semantics and complete identity, lifecycle, uninterrupted
  interval, boundary and task-coverage checks. An interrupted or failed
  attempt can retain consumed work if its terminal read and all task exits were
  actually observed.
- A training cost label only after additional measurement-policy admission,
  independent receipt replay, matching verified objective/domain, successful exit
  before the original deadline, and absence of resource/interruption censoring.

The proposed unit is **worker exec-to-exit user-space retired instructions**. It
includes dynamic loader/startup, construction, in-worker replay, computation,
artifact serialization, measurement overhead and destruction in that worker
tree. Kernel/hypervisor work, supervisor, uploads and later independent validation
are separately accounted and are not this count. This unit is not a pure solver
kernel count or a complete campaign cost. The protocol must explicitly accept
that distinction before admitting labels.

`Facts` deliberately has no deserializer. Its booleans/enums are trusted facts
that a future strict verifier must derive, not a proposed worker self-attestation
schema. There is no current adapter capable of producing all affirmative facts.
An untrusted caller can construct a synthetic accepted fixture; that is why this
module alone cannot be used as an evidence admission boundary. Integration must
make verified facts private to the verifier or issue an opaque verified token.

Known-answer construction does not satisfy independent scalar recovery. A complete
finite-domain negative can finish a declared PDP decision, but not ECDLP recovery.
An UNSAT label requires its matching checked certificate. Timeout/error/unsupported
rows retain observed work and remain outside complete-to-solve labels. The module
does not impute missing counts, treat null as zero, subtract collection overhead,
sum phase records, pool cohorts or convert reservations/group calls into hardware
instructions. Censored work is not automatically a statistical lower bound.

## Source-mapped integration plan

1. **Version a whole-worker policy and receipt, preserving phase v2.** Add a
   separate restricted policy/receipt path and distinct record schemas; never flip
   existing false fields or mutate frozen policy bytes. Bind raw source/binary,
   protocol/plan/config/attempt identities, executable inode/hash, platform/kernel/
   PMU identity, target PID/TID with reuse-resistant identity, parent owner, selected
   event request, actual scope, full syscall transcript and terminal outcome.
   Reuse checked strict types, bounded encoding and immutable storage. Keep the
   current phase diagnostics opt-in/configured under a fixed measurement policy,
   because their extra PMU events can affect whole-worker scheduling.

2. **Extend the existing collector's target contract.** Current `EventRequest::new`
   is fixed to `pid=0`; `CheckedCounters` owns an in-process interval, so wrapping
   `worker()` simply creates another worker-body phase. Add a separately versioned
   supervisor-owned target mode using the existing request/ID/control/read/cleanup
   machinery. A candidate is a fresh child held before its first exec while the
   parent installs a disabled `enable_on_exec` event with inheritance on the exact
   target task. The parent retains descriptors through worker exit. This requires
   a verified spawn handshake and syscall adapter; it is not implemented here.

   `Command::spawn` waits for its exec-error handshake. **Do not place a blocking
   pre-exec barrier in the current `pre_exec` closure and expect the same thread to
   attach after `spawn()` returns**; that would deadlock. Design and test a bounded
   launcher/exec handshake or a safely coordinated spawning path first. Keep the
   original monotonic deadline, memory/output caps and no-retry contract. Changing
   attachment mechanism must not reset or relax admission.

3. **Prove task coverage at the actual boundary.** Open before any target task can
   create work. Inherit does not retroactively include preexisting threads or
   children. Require a verified complete descendant lifetime/exit contract before
   reading the aggregate, and account separately for preexisting external service
   work. Parent exit, `RAYON_NUM_THREADS=1`, a same-TID start/end check, an empty
   process group, or two `/proc` snapshots are insufficient. No existing approved
   route was established in this audit that supplies this proof. Keep whole-worker
   admission false until a reviewed mechanism is implemented and tested; do not
   deploy new tracing/privilege machinery as a side effect of this proposal.

4. **Read once after terminal closure and retain failure costs.** The supervisor
   owns final readback, including killed-worker outcomes. Verify all included
   tasks exited, disable/read/close results, IDs, tuple lengths and monotonic
   boundaries. Verify uninterrupted coverage of every included task throughout
   exec-to-exit: no early disable, reset or gap. Equal enabled/running times and
   a terminal read after exit alone do not establish that interval. Preserve
   missing readback when the supervisor itself dies; a saved
   start receipt or previous prefix does not reconstruct unseen instructions.
   Any periodic cumulative snapshots need sequence/counter identity and must
   never be summed. Retain partial/censored receipts with existing capture/custody
   infrastructure and finite byte/object caps, then verify fresh S3 readback under
   the existing grant. This proposal performs none of those operations.

5. **Independent verification and export.** Add the new schema to the independent
   consumer before label promotion. Verify exact request bytes, event identities,
   complete lifecycle/task evidence, source/input joins, policy and outcome/domain
   certificates. Replaying arithmetic or a receipt cannot reproduce the historic
   PMU observation or prove hardware truth; later hardware reruns are separate
   measurements. Keep full attempt/failure cohorts and restrict cost/terminal
   labels from causal feature channels. Add proposed CLI capability metadata from
   `capability-proposal.json` only when the producer's actual support matches it.

## Grouping, scaling, process and availability rules

The existing checked collector schedules instructions and optional cycles as a
group, reads each event with ID and enabled/running times, and permits a bounded
standalone-instruction fallback after clean teardown. Successful grouping does
not establish task coverage. A new inherited mode must retain compatible read
formats and verify every included task has terminated before its terminal
aggregate is accepted. Do not add per-thread counts to an inherited aggregate;
that would double count. Thread-local counters for existing pool threads would
need a separate exact, nonoverlapping coverage contract.

Raw values require positive running time, running no greater than enabled, and
the exact event identity. Equal times permit an unmultiplexed observation; a
shorter running time produces only a scaled estimate. Overflow remains explicit.
Cycles are optional to this instruction metric, and no failed optional-group
cleanup may be ignored. Failed open/reset/enable/disable/read/close, short reads,
context changes, descriptor reuse, missing terminal evidence and lost descendant
records remain ineligible. Zero is valid only when an actual otherwise-admitted
read returned zero. Unsupported ABI/PMU, permission denial and unscheduled
events remain unavailable; this plan does not change permissions.

Counter grouping, enabled/running interpretation, inheritance and exec behavior
were checked against the Linux [perf_event_open manual](https://man7.org/linux/man-pages/man2/perf_event_open.2.html).
Platform/permission boundaries were checked against the kernel's
[perf security documentation](https://docs.kernel.org/admin-guide/perf-security.html).
The blocked-before-exec constraint also follows Rust's documented
[`CommandExt::pre_exec`](https://doc.rust-lang.org/std/os/unix/process/trait.CommandExt.html#method.pre_exec)
boundary, the [standard library spawn handshake](https://doc.rust-lang.org/src/std/sys/process/unix/unix.rs.html#137-164)
and the current supervisor call order. These sources guide the proposed
implementation; they do not establish availability on this host.

## Acceptance tests and delivery status

The standalone synthetic tests passed: **15 passed, 0 failed**, compiled with
`rustc --test -D warnings` under a 30-second CPU limit, 1-GiB address-space limit,
45-second compile timeout and 15-second test timeout. No repository Cargo build
was run. The code hash before and after the tests matched.

The tests cover: accepted declared scope; zero versus absent
or never-scheduled; integer multiplexing; overflow; invalid time tuples and event
IDs; software/cycles rejection; phase/body scope refusal; insufficient task
snapshots/group closure; every lifecycle/custody gate; policy/platform/replay and
terminal gates; failed/censored/unsupported cost retention; missing terminal read;
objective/certificate mismatch; and SAT versus construction semantics.

Before integration can be called usable, add fake-syscall/launcher tests for
failed attach/exec, early exit, PID reuse, child/thread creation before attach,
descendant exit after root exit, group escape, incomplete event stream, OOM/
SIGKILL, missing parent read, deadline during handshake, descriptor cleanup,
duplicate cumulative prefix and concurrent phase PMU groups. Mock controls can
test bookkeeping without collecting hardware. A separately approved fixed
platform qualification is still needed for any actual instruction labels.

No full repository build, PMU probe, perf command, cloud request, scientific
measurement or frozen-input mutation belongs to this increment. Validation
status and source hashes are recorded alongside this file; synthetic fixtures
must never enter the scientific dataset.
