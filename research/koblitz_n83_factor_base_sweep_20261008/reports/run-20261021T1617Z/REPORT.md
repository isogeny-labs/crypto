# Native N83 SAT storage ownership, 21 October 2026

Both separately frozen diagnostic arms completed **450,000 conflicts**, zero
models, relations and rank. Native replay, memory profiles, storage invariants,
retained search-counter prefixes, exact mechanical relays and cleanup pass.
The clause arena dominates requested solver buffer capacity. K2048 retains
1,787,792,088 bytes of detached learned clauses; K4096 has not reduced its
learned database at this cap. Reclamation remains an untested next change.

These diagnostics follow the [completed original 1M arms](../run-20261020T1546Z/REPORT.md);
they neither replace those runs nor extend their search cap. No natural relation,
UNSAT result, recovery, selected base, minimum-memory or speedup claim follows.
Nominal report start is K2048's retained container start `2026-10-21T16:17:57.760744925Z`.
The earlier shell failure has no measured start/duration.

## Request and actual scope

| Requirement | Status and evidence |
|---|---|
| Continue native N83 sweep after both 1M terminals | Two additional allocation diagnostic arms complete at explicitly declared 450K; original 1M results unchanged |
| Identify SAT memory owners before another cap increase | Native requested Vec capacities, arena logical partitions, process allocator and actual worker RSS/HWM measured and validated |
| Preserve search behavior | All 451 emitted conflict snapshots per arm, including zero, match each original 1M reference's conflicts/decisions/propagations/XOR passes/propagations |
| Retain failures and every raw record in S3 | Shell failure, lock refusal, build/link failures, cards/receipts/streams/source/ELFs/inputs retained; [publication proof](../../verification/native-allocation-profile-20261021/PUBLICATION.md) |
| Optimize the memory owner and raise cap again | Not attempted by this diagnostic protocol; separately freeze/test arena reclamation next |
| Complete finite factor-base grid and full cold selection | Open; no useful natural relations/rank or verified unknown-scalar recovery |

## Observations

All bytes below come from native `summarize-storage`, n=1 per base; no spread or
confidence interval was measured. Requested capacity is distinct from initialized
logical storage, resident bytes and process allocator totals.

| Terminal observation, bytes | K2048-p22 | K4096-p22 |
|---|---:|---:|
| All requested solver Vec capacities | 4,611,260,673 | 4,673,328,420 |
| Clause arena requested capacity | 4,294,967,296 | 4,294,967,296 |
| Arena capacity / all requested capacities | 0.9314084803637385 | 0.9190381907719638 |
| Arena logical used storage | 2,263,844,356 | 2,399,593,924 |
| Original clause storage | 14,108,924 | 24,717,564 |
| Active learned clause storage | 461,943,344 | 2,374,876,360 |
| Detached learned clause storage | 1,787,792,088 | 0 |
| Detached / used arena storage | 0.7897151070751438 | 0 |
| Detached learned clauses | 190,400 | 0 |
| Actual SAT-worker RSS at return | 2,763,960,320 | 3,151,409,152 |
| Actual SAT-worker HWM at return | 2,764,464,128 | 3,151,409,152 |
| Process glibc in-use arena + mmap | 4,830,762,864 | 5,109,477,536 |
| XOR reason literal capacity | 2,136,400 | 2,108,032 |
| XOR matrix capacity | 114,163,712 | 135,135,232 |
| XOR row mask capacity | 109,613,120 | 118,112,320 |

The native sums/fractions are derived from exact recorded capacities and checked
arena partitions. Bytes include length words for stored clauses. All 24 component
keys must be present with unsigned integer values. Parent vector headers and
nested allocations are separate. The inline Solver struct, stacks, other objects,
allocator fragmentation and per-buffer residency are outside this inventory.
`mallinfo2` is process-wide and can include reserved, uncommitted mmap capacity;
it is not a resident-memory counter. `/proc/self/status` is read by the actual
SAT worker, namespace PID11/10 respectively, rather than the init/timeout process.
No allocation stack trace, exact resident-page attribution or leak is established.

[Storage growth](storage-growth.svg), [arena partition](arena-partition.svg) and
[terminal owners](terminal-owners.svg) render native JSON in [data/](data/).
The same plotting source emits SVG and 150dpi PNG; decimal-GB tick conversion is
display only. [PDF](report.pdf) contains the visuals. No new scientific result is
computed in Python.

## Execution, resource and counter checks

Each worker had 450000conflicts/2400seconds, seed2026100801, m5, identitymask0,
1CPU quota, full core14/30 reserved and workerCPU14,8GiB memory/no extra swap,
strict quiet/eviction/affinity restoration, networknone, read-only source/inputs/
worker/root, capdropALL, no-new-privileges and64PIDs. Pinned Docker image:
`sha256:541096ae9b05d3b850341b78790e2ed31e94d5aa0ea086933c6ff0c09ffa2e63`.
Host `ip-172-31-26-135.us-east-2.compute.internal`, Xeon6975P-C,32logicalCPUs.
Missing per-worker context-switch/fault counters and fixed kernel threads prevent
automatic L2. No task build/test/compression ran during either worker.

| Recorded execution | K2048-p22 | K4096-p22 |
|---|---:|---:|
| Command seconds, including supervision | 1721.016573635 | 1731.917490259 |
| Command deadline seconds | 3000 | 3600 |
| Conflict snapshots matched | 451 | 451 |
| Storage snapshots | 47 | 47 |
| Memory samples | 29 | 29 |
| Greatest sampled cgroup charge/kernel peak, bytes | 2746028032 | 3022114816 |
| Last sample to outer-end gap, ms | 27008.30878400011 | 38622.77860700013 |

Both exit0, no timeout/OOM; all sampled memory.max8589934592 and swap.max0.
Samples approximately60seconds apart cannot exclude between-sample changes or
establish full-run peak. Worker terminal RSS can exceed the last cgroup sample.
Owned startup synchronization, native release barrier and container/runtime-file
cleanup validate. Read-only supplemental Docker streams equal final raw logs
byte-for-byte. Commands describe diagnostic conditions only: no matched clocks,
uncertainty, pooling, ranking, fastest-tool or end-to-end gain.

Native storage comparison requires full current and reference audits, identical
panel/fixture inventories, complete traces, all storage boundaries and every
emitted conflict signature covered by the original1M trace. Agreement of these
counters is an observed prefix check, not proof of every internal state.
The historical old trace with no storage fields is a negative control and must
be rejected. Full native memory-profile/startup/cleanup checks remain separate.

## Sources and retained failures

Worker source `674d2075958c5cb388cbfcb271e8ebcd6dea646c` starts at published
`37679b3586992e52e44cd1e994f589aa2a9b204b`, adding opt-in read-only telemetry.
SAT propagation, branching, learning and forgetting are unchanged. Inherited
one-word PairSumTable changes are present; the exact GF83 query and trace prefix
are revalidated. No timing equivalence with the older retained ELF is inferred.
Worker SHA256 `c4b2062df1bd14e13c2c3d9adae28bdb552be103a0c6dd3f2dbc272ce3b1337d`.
K2048 guard-v21 SHA256 `cad94b2cb6209e0235ecdfacb755b1c0c1823be3feda3c70eba092adf524b74f`.
K4096 guard-v22 source `7e8ae96c4af60ac63311b4c3c29efcb83321861e`, SHA256
`36e40caf480f0d119ff39a1e2a2f3660d6647e7df46bd596c6ac0f59dab431ba`.
Each card remained at its clean frozen HEAD through independent assessment.

- Initial development compile failed because a new test tried to clone a
  non-Clone Solver; constructing the identical reference separately fixes it.
  Original log retained; exact full source of that first failed development
  attempt was not snapshotted. No result relies on the failed build.
- A final-guard link failed because shared `/tmp` was full. Rebuild under a
  task-owned `/var/tmp` passes. No peer files/processes were removed or changed.
  Queued build/lock readiness records and preliminary unused guard ELFs remain.
- K2048-p21 shell redirection failed before card-run because exec parent was
  absent. No native receipt, worker, duration or terminal exists; director
  classified impeded. Root created the intended ignored parent; separately
  compiled K2048-p22 completed. Original card/prompt/spec remain immutable.
- K4096-p21 sole command exited1 after10.985894322seconds without timeout:
  another benchmark acquired the lock after instantaneous readiness. All14
  outputs absent, empty run dir; no CPU planning/eviction/container/worker or
  cleanup receipt. Original receipt and exact relay remain unadmitted.
- [QUEUED-V22](../../verification/native-allocation-profile-20261021/QUEUED-V22.md)
  preregisters optional unsigned lock_wait_seconds<=600, default0. A bounded,
  interruptible native queue precedes CPU planning. Open errors propagate; only
  contention is queued. No SAT retry. Same lock/inode/ownership/mode retained.
  K4096-p22 requested600seconds, actually acquired after0.000012404seconds.
  Its lock-wait.json is hashed in config/custody, verified in audit and among15
  required card outputs. Command deadline3600 includes queue; worker budget fixed.

The amended conservative attempted-native-card envelope is7200worker-seconds,
including the full2400 allocation for refused K4096-p21; actual two-worker query
allowance is at most4800seconds. The earlier shell failure has missing measured
cost, not a fabricated zero. This is not full historical campaign accounting.
Original800K OOMs, rejected1M swap profiles and interrupted old-host custody remain
in their separately verified archives, never retrospectively admitted here.

Primary exact model: `icv1-f2m83-tm6151469093347-debefd74`, a=0,
`y²+xy=x³+1`, modulus `x^83+x^45+x²+x+1`, subgroup2417851639230796216685689.
Public retained K2048/K4096 panels/query/fixtures, no deployed/private target or
cross-model equivalence inference. Original witness/selector external controls
remain linked in the [prior XOR report](../run-20261019T1720Z/REPORT.md).
CMS5.17.0 remained faster there; no new external timing comparison was run for
this instrumented ownership diagnostic.

## Validation and publication

Focused readiness:31 SAT tests,19 exporter and2 independent model-checker tests
passed; first final guard35+7, queued guard38+7. Final full release checks and
fresh native positive/negative replay are recorded in
[VALIDATION.json](../../verification/native-allocation-profile-20261021/VALIDATION.json).
[PUBLICATION.md](../../verification/native-allocation-profile-20261021/PUBLICATION.md)
records all actual S3 names/digests/bytes and fresh GET/payload/source-bundle/
native audit/profile/prefix/rejection/repack proofs. Upload receipts live outside
immutable archives. Source history retains both unpublished experimental freezes.

Canonical `docs/ic-leaderboard.html`, `docs/index-calculus-scoreboard.html`,
`docs/ic/progress-timeline.json`, performance-gain history and curve diagrams were
checked and left unchanged: no new eligible cold/rho/identity/component speed
promotion. The [memory problem](../../../../problems/n83-native-800k-memory-limit.md)
now has measured ownership evidence; minimum memory and reclamation remain open.
Broader PR2036 base reconciliation/CI/readiness hold remains separate.

Interpretation: test arena reclamation at a reduction boundary while preserving
clause order, forgetting schedule, all live reason/watch references and search
signatures. K2048's detached storage supports that experiment. K4096's active
pre-reduction growth means reclamation alone has no demonstrated benefit at450K.
No reclamation candidate or higher-cap arm has run under this protocol.
