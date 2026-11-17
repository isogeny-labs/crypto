# CUDA port of the ECCp-131 and ECC2-131 rho walks (2026-10-23)

Both walks of the [CPU parity study](../certicom131_rho_parity_20261023/README.md), written once as
per-thread functions (`gpu_walk.cuh`) that compile as CUDA device code and run unchanged on the CPU.
Read [`report.pdf`](report.pdf) (source `report.typ`); every value is in [`results.json`](results.json).

**Not run on a GPU.** The container that wrote this has no GPU and no `nvcc`. There is no device parity
result and no GPU rate. `walk_cuda.cu` holds the kernels, the device parity gate and a timed launch; it
compiles (clang 18) and assembles (`ptxas` 12.9) for sm_80, sm_86, sm_89, sm_90 and, with a retargeted PTX
header, sm_120, but has never executed.

What was checked, all passing: the per-thread code run on the CPU against the header arithmetic (field
oracle, one-step oracle on every lane, on-curve end points, end states independent of the thread/batch
split, zero exceptional lanes) at 12/6 and 200/40 iterations; and `ptxas -v` statistics (ECCp-131 kernel:
64 registers, 1,992 B stack, no spills; ECC2-131 kernel: 255 registers, about 28.5 KB stack, some spills
in the thread function). No speed claim, no row in `benchmarks/README.md` or the leaderboard.

On a CUDA machine: `make cuda-run` (set `SMARCH`); it exits nonzero unless the device end state equals the
CPU end state word for word. `make host-test` and `make cuda-compile` repeat the offline checks.
