# secp256k1 rho on one RTX PRO 6000 (2026-10-17)

`modal_app.py::selftest` and `modal_app.py::tune --min-blocks 0,2,3,4` from
this tree (`c6d64436a` plus the Makefile change these results justify), one
RTX PRO 6000 Blackwell Server Edition, `sm_120`, CUDA 12.8.1 image. Logs:
[`selftest.log`](selftest.log), [`tune.log`](tune.log).

**Correctness.** `./bench selftest` passes for the portable build and for
`FP_PTX=1`: every kernel against the host reference, including the full rho
walk state after 64 batched fold-6 iterations. This is the device check
OPTIMIZATION_BLACKWELL.md section 2 asks for before `FP_PTX` can be a default.

**Throughput**, `./bench rho` at its defaults, one sample per configuration:

| build | Mstep/s | vs portable default |
|---|---:|---:|
| portable, min-blocks unset | 733 | 1.000 |
| portable, min-blocks 2 / 3 / 4 | 754 / 769 / 791 | 1.029 / 1.049 / 1.079 |
| `FP_PTX=1`, min-blocks unset | 1,336 | 1.823 |
| **`FP_PTX=1`, min-blocks 2** | **1,377** | **1.879** |
| `FP_PTX=1`, min-blocks 3 / 4 | 1,372 / 1,256 | 1.872 / 1.714 |

An earlier run of the same sweep on 2026-10-09, from an older tree, ranked
the configurations the same way: `FP_PTX=1` at min-blocks 2 first at 1,397,
min-blocks unset at 1,329 and min-blocks 4 last of the PTX rows at 1,255.
The min-blocks 2 lead over unset is about 3% in both runs and rests on one
sample per configuration each time. The 1.82-1.88x from `FP_PTX` is far
outside that noise.

`make bench ARCH=sm_120` now selects `FP_PTX=1 MIN_BLOCKS=2`; every other
architecture keeps `FP_PTX=0 MIN_BLOCKS=0`, and either variable can still be
set on the command line. The portable build keeps improving up to min-blocks
4 while the PTX build peaks at 2 and loses 9% at 4. Why the two builds turn
at different points is not measured here; ptxas resource reports per build
would be the first check.
