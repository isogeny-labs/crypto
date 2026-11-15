# Resume: crypto @ claude/index-calculus-gpu-optimization-otouqv (rootfind m = 3 on the GPU; daemon worker)

## Goal
User (2026-11-15), "do these" on the open items of PR #4178: (1) a validator check of
research/ic_gpu_rootfind_ic_20261114/REPORT.md (validator agent, running); (2) m = 3 on the GPU;
(3) remove the ~0.3 s per-solve GPU start-up.

## Lead / hypothesis
(2) Kernel probe mode: candidates x(R -/+ F_i) looked up in the pair table's abscissae, no j >= i
filter; rootfind-subtract-gpu:m=3 confirms reported positions through the CPU probe in sweep order:
identical relations/S to rootfind-subtract:m=3 by construction. The m = 3 relation phase is ~4|F_x|^2
candidate probes against an |F_x|^2 table build (charged, CPU, both arms), so the solve-wall gain is
expected to be smaller than m = 2's.
(3) worker=daemon: rootfind_serve --daemon SOCKET holds one CUDA context; a run connects and INITs
(table upload). Expected per-solve set-up ~ms instead of ~0.3 s; the daemon's start is reported.

## Spec
specs/smoke.json, specs/rf3.json (30/32/34/36 bits; |F_x| 1024, 2048; rf3-cpu/gpu/gpud; rho, A/A),
specs/rf2d.json (28/30/32 bits; |F_x| 4096; rf2-cpu/gpu/gpud; rho). job.sh on one A100.

## State
2026-11-15 03:20Z: done; pod deleted. 232/232 verified+replayed, 128 pairs equal S. Daemon: m=2 solve
20.8-127.8x CPU, 2.0-3.5x worker-per-run. m=3: correct, 0.41-0.86x CPU (device set-up). Amendment 1
(rf3 curves 26-32 bits). Record closed. REPORT.md.

## Runs in flight
None (icgpu-rm3 deleted 03:16Z).

## Stop and success conditions
Any cpu/gpu S, relation or logarithm mismatch, or failed replay, voids that comparison. Wall gains
reported per workload with the CPU arm single-threaded; daemon start reported beside the records.
Cap ~1 GPU-hour, $3. S3: same waiver as earlier rounds (session AWS keys rejected).

## Next steps
Track the PR. Next: m = 3 probe table built on the device; prime_explicit curves > 32 bits; validator.
