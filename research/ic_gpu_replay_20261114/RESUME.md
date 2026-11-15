# Resume: crypto @ claude/index-calculus-gpu-optimization-otouqv (replay, batching bound, rootfind profile)

## Goal
User (2026-11-14, "All"): (1) independent replay of the fes-f2-gpu measurements, (2) batching several
systems per launch, (3) profile rootfind on a host with GPU counters.

## Lead / hypothesis
(1) Replay the e2e sweeps from current main on a fresh host; a separate validator agent re-derives
every reported number from the raw outputs with its own code. (2) Upper bound for batching = round
trip minus kernel GPU time per call (FES_SERVE_TIMING=1); if small at the sizes the GPU engine is
used for, batching is not worth its extra counted work. (3) Runpod and Modal give no counters here;
a sampled clock64() phase profile (RF_PROFILE=1) instead.

## Spec
research/ic_gpu_replay_20261114/job.sh on one A100; ic built from fa75167c45 (Rust identical to main
9b88564250). Record benchmarks/records/<stamp>-ic-gpu-replay.json.

## State
2026-11-14 07:30Z: done; pod deleted; validator review in; record 20261114T0620Z-ic-gpu-replay closed; PR open.

## Runs in flight
Pods icgpu-*; research/ic_gpu_measure_20261112/rp_gpu.sh delete NAME.

## Stop and success conditions
Replay: rows verified; S/rho and calls compared with the original runs. Cap ~1.5 GPU-hours, $4.
S3: same waiver as before.

## Next steps
Track the PR. Next round candidates: rootfind lookup layout (see REPORT.md section 3).
