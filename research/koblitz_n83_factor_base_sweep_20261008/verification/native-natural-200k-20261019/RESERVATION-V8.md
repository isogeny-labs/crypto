# Separately frozen full-core reservation correction

The v7 K2048 card stopped before launching any timed worker. Its actual
relay, receipt, stderr and isolation-before.json remain under exec/ and
are included in the raw archive. Director card-check assessed that refusal.
Four user-thread affinity evictions failed because an unrelated container
had an effective cpuset containing only CPU31; CPU31 was fully busy in the
2-second settle sample. No timed solver budget was spent. No peer workload
is killed, paused or moved out of its fixed cpuset.

The original guard hardcoded Auto, which always selects full core15/31.
Taskset on the controller does not alter its cgroup's effective cpuset.
Guard v8 adds optional cpu_reservation as a nonempty ascending array of
u32 CPU IDs. Absence retains Auto. Existing native topology checks require
online CPUs inside the cgroup and every SMT sibling; no none/inherit option
is exposed. Existing lock, movable-thread eviction/restoration, strict
quiet thresholds and bounded readiness wait are unchanged. Invalid type,
empty, unordered, duplicate and negative requests fail closed.

Fresh specs request [14,30], a whole SMT core on this host. The controller
keeps other CPUs for housekeeping; actual before/after conditions and any
failed samples remain raw evidence. This is a placement change, not a solver
change or relaxation of isolation. It is not pooled with earlier CPU15/31
observations to infer speed or base ranking. The retained worker, query,
public inputs, 200000-conflict/1200-second budgets and maximum new worker
allocation2400seconds remain unchanged. The failed v7 attempt launched no
worker; these are the first timed arms of this round if readiness passes.

Immutable runner-v8 SHA256:
2ec47f22528893777941de5dc4a68e8b2ac02d920c87449ed835137ddb2c0273.
Guard17unit+7card tests pass, including explicit-request validation plus
existing missing-SMT-sibling/cgroup/online checks. Build and tests complete
before freezing fresh source/specs/cards. Each arm gets new write-once paths
and director compile -> exact mechanical -> director assessment. The unused
K4096 v7 card is superseded without execution; the v7 files remain intact.
Any new refusal is assessed before any further card is compiled.
