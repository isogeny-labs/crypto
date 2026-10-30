# GPU index-calculus baseline: preregistered protocol (draft, no data)

Status: **design only. No card exists and nothing has been run.**
Class: engineering stage diagnostic. A kernel throughput number is not a
whole-DLP result and cannot move the leaderboard (AGENTS.md §5a, Validation gate).

## 1. Question

How fast do `gpu/semaev` and `gpu/rootfind` run on real hardware, what limits
them, and does the device agree with the verified host path? Neither kernel has
ever run on a GPU (`gpu/semaev/README.md`, `gpu/rootfind/README.md`,
`docs/ic/perf/OPTIMIZATION_PLAN.md` §1).

## 2. Route: blocked on a lab change (owner decision, 2026-10-27)

The owner chose to add a GPU pool to Iso Lab on EKS before any card is written.
Today the lab cannot run this:

- `isolab-eks` covers x86 CPU only ("GPU and ARM builds are separate work").
- `taskq/deploy/eks-karpenter/nodepool.yaml` has CPU pools only: C/M/R gen 6+, 8/16/32 vCPU, Spot,
  with CPU-isolation kernel args and a 32-vCPU cap per pool.
- AGENTS.md §9a forbids silently routing an unsupported workload elsewhere.

Prerequisites, each an administrative action that needs its own authorization
and is **not** done by this document:

1. A GPU `EC2NodeClass`/`NodePool` (Spot, `g7e` for sm_120; `p5`-class for sm_90 only if asked),
   an NVIDIA GPU AMI, and the device plugin. Nodes must carry a dedicated taint so CPU jobs never land on them.
2. Quota. Spot G/VT vCPU quota is separate from the 96-vCPU Standard Spot quota and is unverified. Do not request an
   increase implicitly.
3. A CUDA build image digest-pinned from the tested commit (`Dockerfile.ecbench` is CPU).
4. GPU-side admission replacing the CPU L2/A/A gates: exclusive device, locked clocks or recorded clocks,
   `nvidia-smi` clock/power/ECC trace, no co-resident process. The CPU thresholds are not reused or lowered.
5. A committed owner approval (`EXP-*`) for `task_card.py lint`, and `.resume/RESUME.md` filled
   (`scripts/checkpoint.sh init`) before launch.

Until 1 to 5 hold, `task_card.py lint` cannot pass and the card is not written.

## 3. Arms (one build per arm, same commit, same host, interleaved)

| id | binary | purpose |
|---|---|---|
| S0 | `gpu/semaev` `bench2`, `GF2N_CLMAD=1` (default, sm_80+) | native carry-less product |
| S1 | same source, `-DGF2N_CLMAD=0` | software product: isolates the `clmad` contribution |
| S2 | `sweep_kernel` vs `sweep_kernel_flat` | per-row vs per-pair mapping (the README leaves this open) |
| R0 | `gpu/rootfind` `bench`, lookup `sorted` | |
| R1 | same, lookup `hash`, chunk in {16, 32, 64} | `RF_BLOCK` and `RF_SHARED_HASH_CAP` left at defaults |
| C0 | host emulation / `bench_host` on the same inputs | correctness reference and CPU matched cost |
| A/A | each arm against itself, separate launches | noise floor |

Semaev's `l` is compile-time (generated header), so each `l` in {12, 14, 16, 17, 18} is its own build and its
own arm. Rootfind bits=61, factor base 2^12, 2^16 targets, plus the P-256 instantiation (`bench p256`).

## 4. Order of work and gates

1. `./bench2 selftest`, `./bench selftest 61 1024 4096`, `./bench p256 256 1024`. These compare device with host on
   every target and fail if only one verdict occurs. **If any fails, stop**: no throughput is reported, and the
   failure is the result.
2. `make ptxas-info`, `occupancy`: registers, spills, shared memory, blocks per SM. Recorded, not tuned.
3. Throughput, at least 5 interleaved rounds per arm after a warmup, arm order randomized by a derived seed.
4. One `ncu` pass per kernel (speed-of-light, pipe use, warp stalls) via the `profile` skill. A profile explains
   a cost and never proves a speedup.

## 5. Metrics and units

- Primary: field multiplications per second, counted in the kernel (not wall time of the sweep).
  Secondary: pairs/s (semaev), cells/s and targets/s (rootfind).
- Ratio to the matched CPU path on the same inputs. Report as the distance remaining, never pass/fail.
- Price unpriced native work of every arm, rho references included, at 0, 0.1 and 1 GAE per operation (AGENTS.md §5a).
  Rank two arms only where the order holds across that band.
- Device facts per run: GPU model, driver, CUDA version, SM clock min/median/max, power, ECC, host CPU, commit, binary SHA-256.

## 6. Preregistered predictions (each can fail)

- P1: the semaev device sweep matches the host sweep on every target (exact). Failure means the kernel or the
  launch structure is wrong.
- P2: S0 beats S1 on `gf_mul`-bound `l`. Prediction: at least 1.5x, since `ecc2k130` measured +22.4% end to end from `clmad`, and the
  sweep is more multiply-dense than the walk. If under 1.1x, the `clmad` argument for GPU over FPGA is weaker than claimed.
- P3: rootfind device is 100x or more over the host emulation, against the README's 300 to 1000x estimate.
  Under 30x falsifies the estimate model in `RESEARCH_GPU_ROOTFIND_DECOMPOSITION.md` §3.
- P4: S2 per-pair beats per-row at small `l` and loses at large `l` (setup redone per pair vs triangular imbalance).
  The crossover `l` is reported with its interval.
- Stop rules: any selftest failure; SM clock median under 90% of max in any round (round invalid, recorded, not dropped);
  spot interruption (interval invalid, new epoch, nothing merged across epochs).

## 7. Does not claim

No whole-DLP speedup, no exponent change, no leaderboard or `benchmarks/README.md` row (no replay certificate, validator
report or matched rho exists). The semaev note's own conclusion stands: a faster oracle is a constant, not an exponent.
