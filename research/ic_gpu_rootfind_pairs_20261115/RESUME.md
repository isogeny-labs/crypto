# Resume: crypto @ claude/index-calculus-gpu-optimization-otouqv (m = 3 pair table on the device; pooled multi-threaded CPU sweep)

## Goal
User (2026-11-15), "do these", on the open items of PR #4276: (1) the m = 3 pair table built on the
device; (2) a tuned multi-threaded CPU sweep; (3) the validator check of
research/ic_gpu_rootfind_big_20261115 (validator agent running).

## Lead / hypothesis
(1) RF_LOOKUP_PAIRS_DEVICE / rf64_session_create_pairs: every pair sum x(P_ai +/- P_bi) computed on the
device in the CPU build's order, deduplicated keeping the least build index (the CPU map's
or_insert), compacted and hashed; hits carry the build index. rootfind-subtract-gpu probe_build=pairs
charges the build analytically (per-case GroupOps of one add, x a(a-1) / a / a) and confirms hits with
PairTable::probe's logic; degenerate rows search the first pair by enumeration. Expected at |F_x| =
4096: oracle set-up from ~5.9 s to well under 1 s; solve then bound by linear algebra.
(2) EmuPool: persistent threads for the emulation backend, sized by rf_cpu_budget() (RF_EMU_THREADS, else
hardware_concurrency capped by the cgroup CPU quota). The round-3 validator found the per-batch thread
creation costs <= 10% and the arm sweeps ~1/7-1/9 the candidates per *host thread* of the Rust arm;
hypothesis: the pod's cgroup quota is far below its 96 visible threads (job.sh now records
cgroup-cpu.txt), so 96 threads oversubscribed it. The run tests that.

## Spec
specs/smoke.json, specs/rf3pairs.json (34-40 bits, |F_x| 2048/4096; rf3-cpu, rf3-mt, rf3-gpud,
rf3-gpup; cpu A/A; rho, rho A/A), specs/rf2mt2.json (28-32 bits; rf2-cpu, rf2-mt, rf2-gpud; rho).
job.sh on one A100 (daemon with RF_SERVE_TIMING=1; kills the daemon on every exit path).

## State
2026-11-15 08:00Z: done; pod deleted. 260/260 verified+replayed, 156 pairs equal S. Pair table on the
device: m=3 set-up 0.02-0.12 s; solve 4.0-57.3x one CPU thread, 4.7-10.9x a 14-thread arm (cgroup
quota 13.6 CPUs confirmed). m=2 pooled multi-thread 7.5-10.5x one thread. REPORT.md; record closed.

## Runs in flight
None (icgpu-rpair deleted 07:55Z).

## Stop and success conditions
Any S/relation/logarithm/ledger mismatch or failed replay voids that comparison (the smoke session's
rf3-gpup arm is the first CUDA check of the pairs build). Wall figures L0, descriptive; A/A arms give
the floor. Cap ~2.5 GPU-hours, $6. S3: same waiver as earlier rounds.

## Next steps
Track PR #4276. Next: linear algebra (now dominant at |F_x| = 4096); a vectorised CPU sweep; validator.
