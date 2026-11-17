# Resume: crypto @ claude/index-calculus-gpu-optimization-otouqv (rootfind m = 3 above 32 bits, device-built probe table, multi-threaded CPU arm)

## Goal
User (2026-11-15), "do these", on the open items of PR #4217: (1) the m = 3 probe table built on the
device; (2) m = 3 above 32 bits; (3) a validator check of research/ic_gpu_rootfind_m3_20261115
(validator agent running); (4) a multi-threaded CPU comparison.

## Lead / hypothesis
(1) RF_LOOKUP_HASH_DEVICE: packed hash slots filled on the device (atomicCAS), the probe set no longer
sorted in Rust or on the worker host. Expected: m = 3 GPU set-up close to the CPU arm's (pair-table
build only) plus a transfer of 8 B per abscissa.
(2) prime_explicit curves at 34/36/38/40 bits (make_curves.py, orders checked): there the m = 3
relation phase (~ |F_x| * p / |F_x|^2 ... probes) grows past the set-up and the device should win.
(4) backend=emulate runs the kernel code on every host thread: a multi-threaded CPU sweep, the
fairer wall baseline for the GPU arms.

## Spec
specs/smoke.json, specs/rf3big.json (34-40 bits; |F_x| 2048, 4096; rf3-cpu, rf3-mt, rf3-gpud,
rf3-gpuh (host-built table, F2048), rf3-cpuaa A/A; rho, rho A/A), specs/rf2mt.json (28/30/32 bits,
|F_x| 4096; rf2-cpu, rf2-mt, rf2-gpud; rho). job.sh on one A100 (daemon with RF_SERVE_TIMING=1).

## State
2026-11-15 06:15Z: done; pod deleted. 240/240 verified+replayed, 136 pairs equal S. m=3 at 39.3 bits,
|F_x|=2048: GPU 11.1x one CPU thread, 2.6x multi-thread; device-built table -0.46..0.77 s set-up;
at 4096 the CPU pair table and LA bound the solve. m=2: GPU 4-24x multi-thread (weak baseline). REPORT.md.

## Runs in flight
None (icgpu-rbig deleted 06:12Z).

## Stop and success conditions
Any S/relation/logarithm mismatch or failed replay voids that comparison. Wall results are L0 and
descriptive; A/A arms give the noise floor. Cap ~2 GPU-hours, $5. S3: same waiver as earlier rounds.

## Next steps
Track the PR. Next: pair table on the device; a tuned multi-threaded CPU sweep; validator check.
