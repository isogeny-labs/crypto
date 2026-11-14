# Resume: crypto @ claude/index-calculus-gpu-optimization-otouqv (fes-f2-gpu per-call overhead)

## Goal
User (2026-11-13, "Yes onwards"): cut fes-f2-gpu's per-call overhead (1.0-3.6 ms per call against
0.1-1.3 ms of kernel, research/ic_gpu_e2e_20261113).

## Lead / hypothesis
Half the overhead was Rust-side: AnfRow::from_poly (0.42 of 0.57 ms at 30 unknowns, local probe).
Fixed by a direct QuadraticForm::from_poly (0.041 ms). The rest: text protocol and four synchronous
pageable copies; replaced by --serve-bin and one pinned copy each way (commit 16e7903d26).
Expect identical counted S and calls; per-call wall down by roughly half or more.

## Spec
research/ic_gpu_overhead_20261114/job.sh on one A100: ABBA old (abecd26901 ic + worker, text serve)
vs new (this branch, binary serve) on koblitz-n23 (fes-f2, fes-f2-gpu), char2-n25 and char2-n29
(fes-f2-gpu). Record benchmarks/records/<stamp>-ic-fes-gpu-overhead.json.

## State
2026-11-14: code, tests and local probe done; pod not started.

## Runs in flight
Pods icgpu-*; research/ic_gpu_measure_20261112/rp_gpu.sh delete NAME.

## Stop and success conditions
Identity: every row verified and S/calls equal across arms. Speed: per-call solver wall outside the
old-vs-old / new-vs-new spread. Cap: ~1 GPU-hour, $3. S3: same waiver as before.

## Next steps
Launch, fetch out_ab.tgz, delete pod, analyze, report, record, merge main, PR.
