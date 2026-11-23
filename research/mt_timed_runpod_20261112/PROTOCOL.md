# Timed IC vs rho on a RunPod CPU pod, std vs fast hasher (protocol, frozen before data)

Follows `../mt_timed_20261112/` (PR #3980, claim PCC-20261112-ec2f3f [pcc:unverified]). On one CPU of the session
VM the fastest multi-target IC took 0.78-0.83x the solve time of the fastest multi-target rho at 2^24 and 1.11-1.29x
at 2^32, with the std (SipHash) hash tables of this harness. The two follow-ups the owner asked for (2026-11-12):

1. **Another host.** The lab's EKS route has no working AWS identity; the owner directed the rerun to RunPod. Does
   the size pattern (IC faster at 2^24, slower at 2^32) reproduce on a RunPod CPU pod?
2. **A fast hasher.** Does the pattern depend on the std hasher? The same comparison with every arm's hash tables
   switched to a fast multiplicative hasher.

## Instruments (one source commit: the commit that adds this file)

- `examples/mt_fieldops.rs`, built twice from that commit: the default build (`std`, std HashMap with SipHash), and
  `--features mt-fasthash` (`fast`, every arm's tables use the multiplicative hasher `MtFastH`). Counted operations
  are identical in both builds (checked before the freeze on all seven arms at 32 b, T = 4096: I, M, S, H, L, N and
  ok equal); only wall time can differ.
- Both binaries are built in this session (x86_64, glibc 2.39) and shipped to the pod in a deterministic payload
  (`run.py`, `bin/mt_fieldops_std`, `bin/mt_fieldops_fast`, the 12 instances), whose sha256 is recorded in
  `launch/payload.json` and pinned in the pod's environment.
- Host: one RunPod Secure Cloud CPU pod, flavor `cpu5c`, 4 vCPU, image `runpod/base:1.0.2-ubuntu2404`, reserved (not
  interruptible). The pod runs `pod/podjob.py`: it accepts only the pinned payload, runs the fixed command
  `python3 run.py holdout` (set at creation; there is no command endpoint) and serves the results read-only.
  `launch.py` creates the pod, uploads the payload, collects the results and deletes the pod; receipts in `launch/`.
  The seconds are diagnostic: a cloud CPU pod is not an isolated lab node (CPU model, cpuset, cpu.max and per-execution
  steal ticks are recorded).

## Design

- Configurations: as `../mt_timed_20261112/`: the v5 curves W{24,28,32}-{0..3}, seed 43, T in {1024, 4096, 16384}:
  36 configurations; 17 run keys each (ic5, ic5ks, ic5w, ic5rw; rho2xks kappa 0.05/0.1/0.25/0.5; rhodpu kappa
  0/0.25 x offset -1/0/1; bsgs2 mf 0.7/1/1.4), for each build: 1224 keys.
- 3 repeats per key; sequential, one process at a time, pinned to CPU 2 (`taskset -c 2`); within each round the
  order of all keys of both builds is a fixed-seed shuffle (seed 20261112), so builds and repeats are interleaved.
- `opcost` (std binary) once per size before and after.
- Every target of every run must verify; failures are reported, not dropped.
- Budget: one pod, about USD 0.14 per hour, ceiling USD 5 (deleted after collection or at 4 hours).

## Analysis (`analyze.py`, the rule of `../mt_timed_20261112/`)

Per build: time of a key = median of its repeats (`ms_arm`); per configuration icbest/rhobest (and bsgs); per cell
(4 configurations) median ratio, bootstrap 95% interval (with n = 4 the interval is the range: a 4-of-4 sign rule),
faster / slower / unranked; fixed-key control; A/A. Cross-build: per side, fastest key's time fast/std per
configuration. The size-level pattern is also reported as the count of configurations below or above 1 per size.

## Predictions (from one local run of each arm at 32 b, T = 4096 on the session VM, std vs fast; not data)

- `std` on RunPod: the size pattern reproduces (IC faster at 2^24, slower at 2^32), magnitudes within about 10% of the
  session VM's.
- `fast`: hash-heavy arms gain most (ic5rw about 0.6x, ic5ks about 0.8x of std); ic5 and rhodpu change little
  (0.94-0.96x); the IC/rho ratio falls (IC relatively faster) at every size; whether 2^32 turns faster is open.
- The IC stays slower than BSGS everywhere in both builds.

## Limits known in advance

One cloud pod, not an isolated lab node; one implementation per build; 4 configurations per cell; reused v5 curves
(not fresh) with the seed of the earlier timed study, so the std build is a host replication, not new data on new
configurations.

## Amendment A1 (2026-11-12 16:32Z, after attempt 1, before any measurement)

Attempt 1 (pod fukok3vh2qhsum, AMD EPYC 4564P host with 32 CPUs) produced no measurement: every execution failed at
`taskset -c 2` ("Invalid argument"), because the container may use only its own allowed CPU set. Its receipts and
outputs are kept in `attempts/a1/`. Change: `run.py` pins each execution to one CPU of the container's allowed set
(the middle element of `sched_getaffinity`, recorded with the set in `host.json`); nothing else changes. The payload
is rebuilt (new sha256 in `launch/payload.json`) and a new pod is created. The launcher also sends an explicit user
agent (RunPod's edge refuses urllib's default; no workload change).
