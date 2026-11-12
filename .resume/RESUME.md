# Resume: crypto @ claude/index-calculus-gpu-optimization-otouqv

## Goal
User (2026-11-12): "Work on optimizing my index calculus code for gpu". Scope: the IC CUDA
directories audited today in docs/perf/20261112-ic-gpu-audit.md (gpu/spmv, gpu/fes,
gpu/macaulay, gpu/rootfind, gpu/semaev; explicit-relations profile only). That audit merged four
kernel commits (e1810bfafb spmv, 9872924cfe fes, ba14c7d9f0 macaulay, 6b25b221ee rootfind) that
were never run on a GPU.

## Lead / hypothesis
Step 1: run the audit's cards S/F/M/R/Z on one real GPU (Runpod, sm_89 RTX 4090 preferred),
BASE=e9d5dbdccd vs CAND=main: device selftests must pass on both; ABAB timing with an A/A arm.
Step 2: from the measured profile, further kernel optimizations (rootfind sweep unroll R-B,
semaev clmad fallback, etc.), each measured the same way.

## Spec
research/ic_gpu_measure_20261112/: job.sh (runs on the pod), rp_gpu.sh (GPU podserver client,
derived from research/runpod_kit_20261112). Record benchmarks/records/<stamp>-ic-gpu-cards.json.

## State
2026-11-12: kit and job script being written; no pod started yet.

## Runs in flight
None. Pods are named icgpu-*; delete with research/ic_gpu_measure_20261112/rp_gpu.sh delete NAME.

## Stop and success conditions
Any selftest/identity failure on the candidate = correctness bug, fix first. A speed change counts
only outside the A/A spread (median of >= 5 ABAB rounds). Hard ceiling: 3 GPU-hours, ~$5.

## Next steps
1. Launch pod, run job.sh, fetch results tarball, delete pod.
2. Tabulate, write report, benchrec close, session log, PR.
