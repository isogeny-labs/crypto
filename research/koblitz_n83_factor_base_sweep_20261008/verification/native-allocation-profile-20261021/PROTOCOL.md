# Native N83 storage-ownership diagnostic

Preregistered2026-10-21UTC, after the completed1M campaign and before any new
SAT worker launch. User requests continued progress and raw storage in
setec-astronomy-artifacts. First measure owners of SAT memory growth; do not
infer an allocation cause from container memory alone or raise caps blindly.

## Fixed first stage

Run K2048-p21 once, assess, then separately compile/run/assess K4096-p21 once.
450000conflicts/2400worker-seconds/3000command-seconds per arm; same1CPUquota,
full core14/30/run14,8GiB/zero-extra-swap, pinned image, public panels/query,
seed2026100801,m5,identitymask0, strict reservation/quiet/affinity restoration,
networknone/read-only/capdropALL/no-new-privileges/64PIDs, default benchmark lock.
Require precompile/predispatch CPU PSIavg10<=5.00 and the native bounded gate.
Additional maximum4800worker-seconds. No unused historical allowance is reused;
previous1M seven-command envelope42000seconds remains separate, not a full
campaign ledger. No automatic timed retries. Refusals, interruptions, drift,
OOMs or missing custody remain failed/unadmitted and require explicit assessment.

This is a separately declared diagnostic cap, not a substitute for the prior
completed1M arms. K2048 crosses the first reduction observed in its old trace;
K4096 profiles pre-reduction growth. An absence of detached clauses at this
cap is not evidence about their cost at larger caps.

## Profile and source

New native worker retains the SAT algorithm, adding read-only ownership
snapshots only under the existing opt-in N83_SAT_PHASE_TRACE=1. Solve entry,
return and each10000conflicts capture exact requested Vec capacities by owner:
clause arena/index, watch lists, XOR rows/matrix/support/reasons/scratch,
assignment/analysis/branch structures. Arena logical words partition originals,
active learnt and detached learnt clauses. Parent Vec headers and nested buffer
capacities are separate, with a native total and layout invariants.

These are requested capacities, not per-buffer resident bytes, malloc usable
sizes or allocation stacks. Native Linux/glibc mallinfo2 records process
allocator counters; /proc/self/status records actual SAT-worker RSS/HWM, distinct
from the launcher's timeout/init PID. Process values include non-solver storage
and small diagnostic serialization costs. Capacity-versus-RSS differences are
not leaks. Sampling and whole-process numbers do not identify exact pages.

Root compiler/test gates must pass before card compilation. Preserve the first
compile failure in the test harness (attempted cloning of a non-Clone Solver),
its correction, successful guard tests and all lock wait/refusal records.
No actual SAT experiment was run during these build attempts.
New worker is built at a new path; retained worker3a31bd2b is never overwritten.
Source starts at the published37679b35tree, including inherited one-word
PairSumTable updates. The GF83 retained query is rechecked natively. Compiler,
source/ELF/protocol hashes and exact frozen HEAD bind each card. No source/HEAD
changes from dispatch through assessment. Profilebaseline is not the old ELF;
no matched timing comparison is inferred from source overlap.

Mandatory native summarize-storage checks both current/reference full natural
audits and retained panel/fixture equality, all emitted conflict-snapshot
signatures (conflicts,decisions,propagations,XOR passes/propagations) against the
same base's original v20 1M trace. Every current snapshot must have reference
coverage and equal signature; this is observed counter-prefix agreement, not a
proof of every internal state. Validate nonnegative native integer capacities,
component sums/arena partitions/capacity, worker PID/RSS units/HWM, completed
profile boundaries and mandatory snapshot presence. Missing data fails closed.
No instrumented clock is a speed claim. Native cgroup/profile/startup/cleanup
and exact card/receipt/verbatim-relay gates remain mandatory.

## Decision after both profiles

Report measured owner totals/fractions, allocator totals, worker RSS and arena
active/detached words independently. If evidence implicates retained detached
arena storage, preregister a separately validated relocation/reclamation change;
do not silently alter forgetting, clause order, reason references or budgets.
Any candidate needs parity/CNF/reason-chain tests and same-input snapshot/model
checks before a new comparison. If another owner dominates, target that owner.
No candidate or higher-cap arm is yet executed or claimed by this protocol.

Every actual attempt, failed build and missing record remains raw evidence.
Produce native summaries, SVG/same-source150dpiPNG, inspected PDF and benchmark
copies as appropriate. Full release library/exporter/model-checker checks on
final code before publishing. Recheck canonical graphs and document unchanged
conclusions where no eligible promotion occurs. Seal/upload all raw evidence
and verification sidecar with fresh GET, payload/native positive/negative replay
and deterministic repack. Commit finished work/update PR2036 without force;
CI/readiness and historical full-campaign accounting remain separate obligations.

Readiness observations before freeze:31SAT tests,19exporter and2independent
model-checker tests pass; final35guard unit+7card tests pass. The final guard's
first link attempt failed because shared/tmp filled; logs are preserved.
Rebuilding under task-owned/var/tmp/n83-allocation-profile-tmp passes without
changing/removing other tasks' files. A no-argument loader smoke in the pinned
image/cap-drop context reaches the expected CLI usage refusal (worker exit1,
smoke wrapper0), with no query/SAT execution. New task-owned ELF copies are
mode755 so the cap-drop container can execute them. Retained binaries and the
shared lock's inode/ownership/mode are unchanged by this task.
