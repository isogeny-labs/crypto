# Best of P+M and P−M in the gpu/nist small-batch rho kernel (2026-10-27)

Question: does stepping to the better of `P + M_j` and `P − M_j` pay in the
`gpu/nist` P-256 / P-384 rho kernel, whose batched inversion covers only
W = 16–64 walks per thread? This is the open lead of
[`research/rho_walk_leads_20261014`](../rho_walk_leads_20261014/) (PR #1829),
and the prime-field counterpart of ECC2K-130's `SIGMA_SIGN` (`ecc2k130/BEST-OF-TWO.md`).

## The change

`RHO_BEST2=1` (`gpu/nist/nist_rho.cuh`, `make ... BEST2=1`, default 0):

- An ADD step computes both images from the one inverse `1/(x_M − x_P)`:
  the slopes differ only in the sign of `y_M`. That costs one extra
  multiplication and one extra squaring, and `y` is computed only for the
  image kept.
- It keeps the image with the higher score, with ties going to `+`. The
  score is limb 1 of `x`, which the partition and the distinguished-point
  test (limb 0) do not read.
- The choice rides in bit 1 of the aut code. The host replay
  (`nist_rho_host.hpp`) subtracts `(c_j, d_j)` when `P − M` was taken.
- Doubling and cycle-escape steps are unchanged.

## Results so far

**Correctness.** `make -C gpu/nist test` passes with `BEST2=0` and with
`BEST2=1`:

- the batched and single steppers agree bit for bit at every W;
- `batch_inv_w` matches `batch_inv`;
- the toy-curve end-to-end solve recovers the planted `k` through the new
  subtract path.

**Iterations** (`iterations-v1/`, Modal CPU). The model walk mirrors the
kernel's: rho_walk_leads harness `e4` with the negation map and the
kernel's table size r = 256, 64 walks with distinguished points at 2^-8,
N ≈ 2^36, 1,000 paired trials per arm.

| arm | mean steps | escapes / 10^6 steps |
|---|---:|---:|
| no negation map | 304,222 | 0 |
| negation map (the kernel's walk) | 227,092 | 1,931 |
| negation map + best of P±M | 198,711 | 2,570 |

Best of P±M against the kernel's walk, paired: **1.143× fewer steps,
95% CI 1.094–1.194**. The in-degree prediction is √(4/3) = 1.155.
Fruitless 2-cycles rise by a third; the kernel's existing doubling escape
handles them.

![iterations](reports/iterations.svg)

**GPU rate** (RTX PRO 6000, PTX build, global table, 256 threads,
W = 16/32/64, both curves, three alternating repetitions): running on
Modal (`gpu/nist/modal_app.py::best2_main`). Best of P±M pays at a given W
only if its rate costs less than the 1.143× iteration gain.

## Reproduce

```sh
make -C gpu/nist test BEST2=1
cd research/rho_walk_leads_20261014 && modal run modal_run.py --plan nistbest2 --out ../nist_best2_small_batch_20261027/iterations-v1
cat ../nist_best2_small_batch_20261027/iterations-v1/*.jsonl | harness/target/release/rho_walk_leads summary --ref neg_canon
cd ../../gpu/nist && modal run modal_app.py::best2_main --iters 2048 --reps 3 --out <json>
```
