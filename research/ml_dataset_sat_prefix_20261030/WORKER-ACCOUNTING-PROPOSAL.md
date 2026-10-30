# Proposed worker accounting and intermediate-artifact contract

Read-only design audit, 2026-10-30, SAT worktree. Observed HEAD during inspection:
`ae28fe2ef873b9fac8bd318e0de83515db1827f4`; active-tree file hashes are in
WORKER-ACCOUNTING-SOURCE-MAP.md. No builds, solver runs, experiments, cloud actions or repository
edits were performed. This is an implementation proposal, not measured coverage.

## Prior evidence and documentation checked

Read the repository's `docs/ml-dataset/DESIGN.md` measurement/artifact contract,
`REMAINING-COVERAGE.md`, current SAT `COVERAGE-RECONCILIATION.md`, native adapter
capabilities, phase telemetry, checked telemetry policy/replayer, capture emitter
and worker/supervisor source. Historical validation counts are not rerun here.

Checked Linux interface documentation:

- https://man7.org/linux/man-pages/man2/perf_event_open.2.html : inheritance covers
  newly created tasks; `enable_on_exec` supports an exec boundary; event/time
  read formats and PMU availability must be preserved. Inherited group-read
  restrictions require individual event reads and explicit identities.
- https://docs.kernel.org/admin-guide/cgroup-v2.html : a dedicated cgroup supplies
  hierarchical CPU usage and memory peak; `cgroup.kill` handles descendant
  termination and concurrent forks. These facilities require actual delegation
  and available files; no permission or support is assumed.
- https://man7.org/linux/man-pages/man2/wait4.2.html : child-specific terminal
  resource usage can be collected when reaping. This is not automatically a
  complete record of every unjoined descendant.

## Observed boundaries

The checked path uses a fixed `calling_thread` PMU policy for four phases:
curve construction, known-answer replay, factor-base replay and solver. It
retains checked syscall/readback evidence and explicitly sets training cost,
whole-worker coverage, hardware-independent verification and platform admission
false. Its exact scope must not be expanded by relabelling old artifacts.

The legacy path tries inheriting counters then a noninheriting fallback, with
different validation strength. Neither path covers the worker executable's
startup and every out-of-phase serialization step. Phase process resource
snapshots are cumulative RUSAGE_SELF values; peak RSS is a high-water mark.
Current supervisor explicitly reports `whole_worker_not_counted` and null
retired instructions. It uses a process group, parent-death signal, per-process
RLIMIT_AS and a 100ms polling loop. RLIMIT_AS is an address-space bound, not an
aggregate descendant RSS bound. Sampled directory size can overshoot its cap
between polls; post-seal/publication bytes are separate.

Worker result writing, artifact serialization, supervisor sealing and independent
verification, upload/readback, checkpoint packaging and restore/replay require
their own intervals. The broad solver phase includes adapter work, event
serialization and some checking; it is not just native elimination. In addition,
curve generation already verifies the constructed objects internally, then the
worker explicitly replays known answers and factor bases. Preserve this actual
work rather than assuming each named verification happens only once.

Capture already saves acknowledged event prefixes and completed payloads with
attempt bindings, hashes, caps and immutable chunks. It does not expose private
native state that an engine never emits. A capture watermark is neither a
completed solver execution nor a resumable process checkpoint.

## Smallest native extension, in priority order

**P0: a parent-owned worker lifecycle/resource receipt.** Add a small
`worker_accounting` module and strict `worker-accounting-policy/v1`, launch-intent,
post-spawn identity and terminal schemas. Durably write the pre-spawn intent with
plan/cell/attempt, executable hash/source, host boot identity, monotonic time,
limits and collector version; PID and process-start identity are null because
the child does not yet exist. After spawn, write a separately bound identity
record with the returned PID and only independently observed process-start,
process-group/cgroup identities. The worker can execute or exit before this
record is persisted. P0 provides no PID-bound durable-start-before-work guarantee;
unavailable identities remain null with reasons, and a missing identity record
does not establish that no worker ran. The readiness/release handshake is P1.

Write a terminal receipt after the direct-child reap and attempted cleanup,
including on signal, timeout, cap, spawn failure or normal backend error. Record
exit code, signal, supervisor stop reason, reap evidence and cleanup attempts
separately. Current kill_group ignores signal-call errors and reaps only the
direct child; neither its return nor a direct-child exit proves descendant
quiescence. P0 records descendant cleanup as attempted/unverified unless a
separately admitted mechanism establishes quiescence. Retain observed kill
errors where collected; historical missing results stay unknown. Do not infer
OOM solely from SIGKILL; preserve cgroup memory.events evidence or unknown.

Collect child-specific terminal CPU/fault/context-switch/RSS observations via
one owning wait/reap implementation. Integrate into SupervisedChild rather than
calling wait4 alongside `Child::try_wait` and racing the reaper. Preserve the
same timeout/censoring behavior. A reap error is an accounting failure, never a
successful zero-cost run. Without dedicated descendant accounting, label the
resource scope as direct child plus only the kernel-reported waited descendants,
not aggregate worker-tree usage. Keep unsupported metrics null with reasons.

**P1: optional parent-owned instructions from exec through worker-tree exit.**
Use an internal native launcher with a readiness/release handshake: launcher
becomes the future worker via exec, creates no helper descendants, and waits
while the supervisor attaches disabled instructions/cycles to its verified PID.
The supervisor holds all counter FDs. Set enable_on_exec and inheritance before
releasing the launcher, so loader/startup and newly spawned threads/subprocesses
are included. This avoids an unmeasured attach-after-exec race and preserves
readback if the worker dies. Do not block Rust's pre_exec hook awaiting a
handshake that can only be serviced after Command::spawn returns; use a real
launcher protocol with finite readiness/exec-error deadlines and cleanup.

Implement this as a new explicit policy, not a silent change to checked phase
telemetry or HwCounters. Preserve exact attr bytes, event IDs, every open/reset/
enable/disable/read/close result, raw bytes, errno, enabled/running times and
scoped assessments. Instructions-only fallback may omit cycles but must never
silently omit inheritance or widen privileges. If required coverage cannot be
admitted, launch under the protocol's declared unavailable-counter policy or
stop before work; do not fabricate instructions from field operations/time.

Default declaration is `worker_tree_user_retired_instructions` with kernel and
hypervisor excluded. A separate admitted policy may include kernel work; do not
compare or sum unlike privilege scopes. Preserve PMU type/config, CPU/ISA/kernel,
virtualization and affinity provenance. Event meaning and availability require
the actual host's admission, not just schema validation.

Counters remain open until the tree is known quiescent, not merely the main PID
exited. On supported delegated Linux hosts, use a fresh dedicated per-attempt
cgroup, prevent workload migration out, kill remaining members at termination,
and confirm empty. Record cpu.stat, memory.peak, memory.events and io.stat with
their actual support/units. Existing container-wide values shared with other
tasks cannot be attributed to one cell. Without a reliable descendant boundary,
retain a scoped observation and `whole_worker_coverage_verified=false`.

Use raw unmultiplexed counts only after checked lifecycle and scope admission.
Preserve multiplexed raw counts as observed during scheduled intervals and a
separate checked scaled estimate, never an exact total. Reject zero-running,
running>enabled, read truncation, identity mismatch and arithmetic overflow as
admission failures. Inherited enabled/running aggregation must be validated on
the installed kernel with thread/process fixtures before asserting total scope;
do not assume a parent elapsed time proves child counter coverage. PMU sampling
is not an instruction-by-instruction replay certificate.

The worker-tree count includes tracing, in-worker checking and serialization.
Do not subtract phase readings to claim exact native-only cost: intervals,
thread scopes and instrumentation can differ. It excludes supervisor startup,
control-plane work, upload and later independent replay, all reported separately.
Non-Linux and PMU-denied hosts keep useful lifecycle receipts with null PMU data.

**P2: end-to-end attempt accounting ledger.** Add parent intervals for context
admission/launch, sealing+hashing, independent verification, archive/checkpoint,
publish+fresh readback, retry and restore/replay. Each cost observation carries
unit, scope, inclusive/exclusive relationship, measured boundaries, origin,
availability and bound artifact IDs. Use checked addition only for disjoint
intervals with the same metric scope. CPU sums are not elapsed-time sums; peak
memory is not additive. Parent process/thread cost is separate from worker cost.
I/O block counters are not network request or application-byte counters.

Reuse the native budget journal for actual charged S3/STS requests/retries and
bind it into accounting; do not reconstruct billed costs from successful final
objects alone. Explicitly record compression/input/output/temp bytes and local
fsync/capture work, remote PUT/HEAD/GET attempts and bytes, restore verification,
failed requests and orphan cleanup. Overlapping upload/capture intervals form a
timeline, not a sum of wall clocks. Run-level setup and shared cached artifacts
have a single cold cost plus explicit consumer/reuse links and amortization.

A final ledger necessarily excludes writing/uploading its own final receipt
unless that work is recorded by an outer run receipt. Define the cutoff; never
create a recursive self-accounting completeness claim. Similar cutoffs apply to
S3 receipts and their readback receipts. All losses/censored attempts remain
charged. A fully measured timeout is still a censored solve, not a zero cost or
evidence of impossibility. Known-answer construction is not ECDLP recovery.

**P3: close engine trace gaps one declared class at a time.** First add a common
trace-coverage declaration for every adapter: native event producer/version,
event classes retained/omitted, exact byte/event limits, acknowledged sequence,
terminal reason, native/reconstructed origin, replay level and checkpoint kind.
Use existing CaptureEmitter rather than a second publication stack. Replace
ambiguous generic `trace_complete` claims in new schemas with scoped fields:
`encoding_complete`, `native_event_prefix_complete`, `execution_complete`,
`certificate_verified`, `restorable_checkpoint_available`. Preserve legacy
fields but interpret them only within their recorded historical scope.

Complete intermediates means the enumerated algorithmic event classes in a
versioned trace contract, not every allocation/register state. Persist inputs
and append-only operation records sufficient to reconstruct each declared
matrix/polynomial/table state, plus bounded snapshots at checkpoints. Charge
snapshot reconstruction separately. Opaque native sections remain explicitly
opaque until instrumented; no amount of post-hoc domain reconstruction fills
that gap. Do not claim full instruction-level execution reconstruction.

Emit from inside long native calls before changing a state that must be durable,
or define atomic batch boundaries with replayable before/after data. A sink cap
or I/O failure is sticky and terminates the attempted trace-covered solve with
unknown/partial outcome while retaining verified prior certificates. Reserve a
small terminal-record budget outside payload admission. If the last mutation
was not acknowledged, stop the replay prefix before it. Keep unacknowledged
tail bytes for diagnosis without treating them as committed events. Multi-thread
events need task IDs and dependency order; a global sequence alone does not
establish causal correctness. No implicit continuation from a saved event prefix.

## Coverage matrix: current source versus next trace obligation

| Adapter/domain | Retained/exposed in inspected source | Priority gap to close; claim boundary |
|---|---|---|
| Construction, all families/twists | Orders/certificates, projected candidates, membership/orbit witnesses | Hasse baby/giant tables, factorization internals, repeated-r membership work, exact setup phase counters; computed logs remain restricted. |
| Enumerate, all families m2..15 | Visited group transitions, bounded outcomes | Preserve domain/order and per-transition counter dictionary; existing trace_complete refers to exposed bounded traversal, not every field operation. |
| BSGS, all families | Baby table and giant probes; verified recovery | Table construction/collision/hash operations and memory coverage are distinct from group-add counts; successful scalar must replay without witness. |
| Binary F4, m2 | Selector ANF, bounded Macaulay input/reduced rows, splitting | Stream descendant matrices, row/column order, pivots, reduction operands, zero rows and node transitions; private work currently not fully emitted. |
| Binary F5, m2 | Above plus F5 report | Signatures per row, rejected-pair/row criterion witnesses, syzygy/rewrite state; do not rename matrix criterion engine general incremental F5. |
| Prime F4, m2..15 p<2^32; supervised wide m2 | Exact domain/equations, bases/useful rows, recursive branch/candidate/lift events | Full critical-pair and symbolic-preprocessing stream, pivots/zero rows, native univariate states; rows currently can materialize before serialization. Wide event format needs its own verifier. Affine exceptions prevent general negative PDP claims. |
| F6, binary/Koblitz m4 | Pair sums independently reconstructed after native construction; final checked indices | Native pair insertion/order, batch arithmetic, lookup/probe/collision stream and query counters; reconstructed additions are charged diagnostic work, not native history. |
| Roots, all families m2 | Prime field/power/GCD/split and binary quadratic/trace/half-trace/matrix states; distinct-root certificates | Audit every omitted operation/class and independent counter replay; root completeness for a call does not imply complete execution trace, total cost or higher-m support. |
| SAT, binary/Koblitz m2..15 | Formula plus bounded native decisions, reasons, conflicts, learning/deletion/backjump/restart/XOR events | Producer gate-map integration and nonzero causal replay validation remain separate delivery gates; then minimization derivations, proof-checkable UNSAT, watch/heap updates if explicitly requested. Learned RUP replay alone is not whole UNSAT proof. |
| Weil, binary/Koblitz m2 | Exact selector equations and coordinate lift map | Record basis/coordinate conversion operations if full construction history desired; this adapter constructs an encoding and does not solve it. |
| PKM, prime m2/3 p<2^31 | Alternative Kummer base, quotient rules, prepared/specialized equations, F4 report, extracted roots | Tower/basis transforms, all private elimination states, per-query fallback traversal and separate fallback cost. Alternative rational domain remains PDP-only where outside target subgroup. |
| Double large prime, all families m2..15 | Residual domains/partial candidates, coefficients, sparse filtering and rank/recovery certificates | Native pivot selection and every graph/filter/matrix mutation, fill-in, dropped rows and recovery substitutions; certified final rank is not a complete native mutation stream. |

All requested n23..31 and47..61, p=2^61-1, both twists, generators/keys and
m2..15 remain plan axes. A coverage matrix entry is an implementation boundary,
not a solved-cell count. T887's independent dynamic wide-prime changes are not
assumed integrated. Current cloud corpus cells remain zero per retained status.

## Exact validation requirements before promotion

Software tests use fake syscalls/counters and deterministic fixture processes;
no hardcoded host instruction count is expected. Existing checked-counter tests
provide lifecycle/readback/error patterns to reuse. Required additional cases:

1. Readiness timeout, exec failure, counter open/enable/read/disable failure,
   short read, reordered/wrong ID, FD reuse, PID/start mismatch, unauthorized
   privilege change, wrong policy hash, overflow, unavailable PMU and cycles-only
   failure. Exactly one operation attempt, original error preserved, no orphan.
2. Worker normal exit, returned error, panic, signal, timeout, directory cap,
   address-space failure, supervisor death and restart reconciliation. Terminal
   records distinguish observed signal from inferred reason. Start-only records
   stay incomplete. No duplicate worker launch after ambiguous prior execution.
3. Threads created after attachment, fork/exec child, child outliving parent,
   process-group-changing child, immediate short-lived descendants and concurrent
   unrelated workload. Demonstrate admitted tree coverage or retain false; no
   inference from RAYON_NUM_THREADS=1 that the process cannot spawn threads.
4. Reaping occurs exactly once and preserves resource data; child cleanup cannot
   discard inherited final counts. Cgroup shared/absent/permission denied,
   migration attempt, missing memory.peak/io.stat and nonempty terminal cgroup
   each produce accurate scoped/unavailable outcomes.
5. Raw zero with positive running time versus never-scheduled event; multiplexed
   values preserved as estimates, invalid time relationships rejected; group
   and instructions-only fallback validated on actual supported kernels.
6. Trace cap before first event, at exact boundary, midway through native call,
   short write/fsync/rename failure, killed producer between payload and watermark,
   duplicate/reordered/omitted event, forged rehashed matrix operation, changed
   terminal counter and replaced source artifact. Verified prefix survives;
   complete/unsat/cost flags cannot survive missing obligations.
7. Every adapter's nonzero native trace fixture has an independent replayer of
   declared event semantics; a deliberately corrupted private-operation record
   fails even after hashes are recomputed. Native and reconstructed records must
   not exchange origin tags. Unknown/unsupported outcomes are retained.
8. Ledger overlap/double-count/shared-cache/retry tests, scope/unit mismatch,
   peak-RSS nonadditivity, missing terminal receipt, interrupted publication,
   fresh remote restore and archive corruption. These last custody checks require
   a separately admitted finite environment; this audit launches none.

Platform admission is separate from fixture/software acceptance. Freeze an
official experiment protocol before performance data: same public inputs,
reference/A-A/control arms, instrumented versus uninstrumented arms, fixed
concurrency, explicit CPU/PMU admission and all overhead charged. Cost targets
remain restricted labels; no timings, counts or outcome-derived metadata enter
causal model inputs. Final instruction counts describe instrumented work unless
an independently measured overhead experiment supports a narrower statement.

## Recommended first reviewable PR

Implement P0 with strict native lifecycle receipts and terminal child resources,
and the per-adapter trace-coverage manifest from P3. Keep whole-worker PMU totals
null. This delivers useful verified scope immediately without touching every
engine. Follow with the optional launcher/parent-held PMU policy plus actual
thread/subprocess admission. Then add accounting intervals and one engine's
internal trace at a time, starting with F4/F5 matrix operations because they
support the requested algebraic-structure and fall-degree research. No step
alone warrants "everything measured" or "every intermediate saved".

## Parallel corpus obligations

Versioned factor-base policies, checked Frobenius-orbit closure of the actual
selected population, and first/last-fall-degree certificate obligations remain
outstanding parallel work. Retained point-orbit witnesses do not establish
population closure. See FACTOR-BASE-POLICY-PROPOSAL.md and the dataset design;
this accounting proposal neither implements those policies nor certifies exact
fall degrees.
