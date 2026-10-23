# ECC2K-95 class-walk launch geometry on the RTX PRO 6000

[`benchmarks/ecc2k-clmad/`](../ecc2k-clmad/RESULTS.md) put the gpu/ecc2k
class walk at 1.685 Gstep/s with the native carry-less product and named the
per-thread batch width as the next lever. This sweeps it, with the block size,
the `__launch_bounds__` min-blocks bound and the two stepper variants.

[`gpujob.sh`](gpujob.sh), through `ecc2k130/modal_job.py` with sources staged
by [`prepare.sh`](prepare.sh), on one RTX PRO 6000 Blackwell Server Edition
(driver 610.57.04, CUDA 13.3, `sm_120`, source in
[`results/host.txt`](results/host.txt)). `./bench rho --iters 2048`, 385,024
class walks at every width.

**Gate.** Each width was built with `-DSELFTEST_W=W` and passed `./bench
selftest`: device walk state identical to the host stepper after 64 steps,
step and restart counters and carried weights equal, the DP count equal,
device DPs replaying to the same class on the host, and the lowmem variant
matching ([`results/selftest-w*.txt`](results/)). W = 2 and 4 are new
instantiations in `bench2k.cu`; W = 16 was existing.

**Screen**, Gstep/s, median of two passes ([`results/screen.txt`](results/screen.txt)):

| block, min-blocks | reg W=2 | reg W=4 | reg W=8 | reg W=16 | lowmem W=2 | lowmem W=4 | lowmem W=8 | lowmem W=16 |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 128, unset (previous default) | 1.687 | 1.885 | 1.801 | 1.300 | 1.642 | 1.809 | 1.742 | 1.363 |
| 128, 4 | 1.696 | 1.720 | 1.593 | 1.336 | 1.518 | 1.643 | 1.565 | 1.373 |
| 128, 6 | 1.602 | 1.720 | 1.573 | 0.985 | 1.555 | 1.639 | 1.540 | 1.004 |
| 128, 8 | 1.657 | 1.732 | 1.536 | 0.967 | 1.571 | 1.647 | 1.516 | 0.978 |
| **256, unset** | 1.820 | **2.019** | 1.986 | 1.026 | 1.734 | 1.904 | 1.913 | 1.009 |
| 256, 2 | 1.756 | 1.897 | 1.683 | 1.027 | 1.648 | 1.800 | 1.647 | 1.014 |
| 256, 3 | 1.749 | 1.882 | 1.664 | 0.869 | 1.646 | 1.794 | 1.631 | 0.858 |
| 256, 4 | 1.712 | 1.865 | 1.583 | 0.843 | 1.628 | 1.772 | 1.571 | 0.825 |

Register and spill counts from ptxas for the W = 4 register-variant kernel
(`results/ptxas-*.txt`): the previous default is 168 registers with no
spills, so 65,536 registers hold three 128-thread blocks, 384 threads per SM.
256-thread blocks without a bound land at 128 registers with 80 bytes of
spill stores, two blocks and 512 threads per SM. The min-blocks bounds cut
registers to 128, 80 and 64 and add 32 to 136 bytes of spills; every bound
lost to the unbounded build of the same block size. The winner trades some
spill traffic for a third more resident threads; which of those is the
binding effect is inferred from the counts, not measured.

**Final**, five alternating repetitions of the three leaders against the
previous default ([`results/summary.txt`](results/summary.txt)):

| configuration | median Gstep/s | / previous default | paired ratios |
|---|---:|---:|---|
| 128-thread blocks, lowmem, W = 8 (previous default) | 1.775 | 1 | |
| **256-thread blocks, reg, W = 4** | **2.022** | **1.139** | 1.139-1.142, 5/5 above 1 |
| 256-thread blocks, reg, W = 8 | 1.993 | 1.123 | 1.122-1.124 |
| 256-thread blocks, lowmem, W = 8 | 1.914 | 1.078 | 1.078-1.084 |

`make bench ARCH=sm_120` now builds 256-thread blocks, and `./bench rho`
without `--w` or `--variant` uses W = 4 with the register variant on compute
capability 12.0. Every other part keeps 128-thread blocks, W = 8 and lowmem
until measured. The confirmation through those defaults is
[`confirm.sh`](confirm.sh). ECC2K-95 is a retired challenge; these rates say
nothing about ECC2K-130.
