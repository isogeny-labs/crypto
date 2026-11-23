# Native N83 1M continuation, preregistered 2026-10-20 UTC

The user requests resumption of the native N83 factor-base sweep, successive
conflict-cap increases for K2048/K4096, and all raw evidence in
setec-astronomy-artifacts. Previous 800K/8GiB observations remain immutable.

| Requirement | Frozen action | Acceptance |
|---|---|---|
| Higher native cap | One K2048 arm then one K4096 arm, each 1000000 conflicts / 6000 worker seconds | Complete native replay or explicitly preserved failure/censor |
| Resource custody | 8GiB, zero extra swap, retained immutable ELF/image/inputs | Spec, live inspection and worker declaration agree |
| Memory diagnosis | Once per 60 seconds, guard reads Docker-init PID RSS/HWM and unified cgroup memory counters | Raw samples and missing-counter errors retained; no allocation attribution claimed |
| Evidence | All actual attempts, exact cards/receipts/relays, sources/ELFs/inputs, native summary and inspected report | Complete locally verified raw archive, requested S3 PUT/GET/replay when access permits |

Run K2048 once and have the director assess its exact relay at unchanged HEAD
before separately compiling K4096. Each card command has a 6600-second limit;
maximum new worker allocation12000seconds. No automatic timed retry. A failed
attempt is an observation to preserve, not authorization for a replacement.
No tracked edits or HEAD changes while a card awaits assessment.

Retain the worker SHA256
3a31bd2bedee5519d30b517f39ec27f72561ce898213657688c55b51bbcdf7b6,
Blake3 9470bf4e009bcfe4dfbe642817282abea9dd116d170307d84f92451f725cd737,
and its nine watched sources matching660f0f4b10. No worker rebuild. Retain
image sha256:541096ae9b05d3b850341b78790e2ed31e94d5aa0ea086933c6ff0c09ffa2e63.
Use existing public fixtures and K2048/K4096 panels from
/home/ec2-user/scratch/n83-docker-sweep-20261018/inputs; bind each file.

Single public_x_hash query,seed2026100801,m5,identitymask0;
primary a=0 icv1-f2m83-tm6151469093347-debefd74,
x^83+x^45+x^2+x+1, subgroup2417851639230796216685689. The positive-trace
a=1 holdout remains distinct; EC1 equivalence is unresolved. Native replay
must verify the original group/query aG+bQ and full phase/input/log custody.
No unknown-scalar recovery, UNSAT, cold/rho or fastest-tool claim.

Guard-v15 raises only the natural budget ceiling to1000000/6000. Explicit
8GiB admission accepts exact natural800000/4800 and1000000/6000 pairs;
witness100000/900 and external1000000/900 remain unchanged. Unit tests
cover the new bounds and memory envelope. Memory sampling is diagnostic,
including the initial sample; samples are written JSONL and bound by a
Blake3 field in outer.json and the card's file digests. Missing procfs/cgroup
fields are errors, never zero. No sampling exec enters the worker container.
Kernel RSS/HWM and cgroup charges do not establish internal allocation causes.
Source inspection shows detached learnt clauses remain in the arena, but no
measured attribution to that arena or benefit from changing it is claimed.

Retain strict CPU reservation[14,30],runCPU14,full SMT core,1CPU quota,
networknone,read-only root/source/input/worker,capdropALL,no-new-privileges,
64PIDs,8GiBmemory+memoryswap (zero extra swap),exclusive/tmp/crypto-bench.lock.
Same native topology/cgroup/sibling/quiet checks, bounded30-second settling,
movable user-thread eviction/restoration only. Do not modify peers' cpusets.
Docker inspect60seconds,otherRPC15seconds,no retry. No builds/tests/heavy
analysis while either timed arm holds the reservation. Fixed kernel threads
and missing worker context-switch/fault counters preclude automaticL2.
n=1 per base; clocks descriptive only, no pooled timing/ranking or comparison
with old hosts. Host uses migrated EFS checkout and retained local Docker image.

OOM, wall censor, refusal, wrong answer and missing terminal counters remain
explicit failures; nonterminal counters never become terminal zeros.
Natural positive models fail closed unless independent original-group/rank/
scalar checks support them. A bounded zero-yield Unknown is not UNSAT.
Use native Rust/C/C++ for research, cards, execution, replay and summaries;
Python only for plots/rendering/PDF inspection. Deliver source-linked report,
SVG and same-source150dpiPNG, inspected PDF, benchmark copies/index. Canonical
cold/rho/identity graphs stay unchanged absent their separate admission gates.

S3 remains an open obligation: at15:42UTC the actual pending800K full-archive
conditional PUT was denied by the current worker role for missing s3:PutObject.
Preserve the new exact receipt beside this protocol. Do not change IAM, invent
credentials, claim successful upload or substitute local replay for fresh S3
readback. Retain sealed archives; publish exact bytes if authorized access
returns. New raw collection uses fixed tar UTC2026-10-20 timestamp.
Commit finished work and update existing PR2036 without force; existing broader
PR readiness/CI reconciliation holds remain separate from these native checks.
