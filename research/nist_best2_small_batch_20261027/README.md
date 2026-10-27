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

**GPU rate** (`gpu-ab-v1.json`; rows in `gpu-ab-v1.rows.jsonl`, summary
`gpu-ab-v1.summary.jsonl` from `rho_walk_leads gpuab`). Setup: one RTX PRO
6000 (driver 610.57.04) on Modal, PTX build, global table, 256 threads,
2,048 iterations per cell, three repetitions in alternating order.

Both builds pass the CPU suites in the image. The `k_rho` instantiations
sit at the same register counts as before (255 at W = 32 and 64).

| curve | W | plain Gstep/s | best of P±M | rate ratio (min–max) | GPU time per solve (range) |
|---|---:|---:|---:|---:|---:|
| P-256 | 16 | 1.450 | 1.296 | 0.894 (0.893–0.894) | 0.979 (0.936–1.023) |
| P-256 | 32 | 1.772 | 1.603 | 0.905 (0.903–0.905) | 0.967 (0.925–1.013) |
| P-256 | 64 | 2.021 | 1.867 | 0.923 (0.919–0.924) | **0.948 (0.906–0.994)** |
| P-384 | 16 | 0.474 | 0.423 | 0.892 (0.888–0.895) | 0.981 (0.936–1.029) |
| P-384 | 32 | 0.635 | 0.561 | 0.884 (0.881–0.884) | 0.991 (0.948–1.038) |
| P-384 | 64 | 0.756 | 0.676 | 0.894 (0.893–0.894) | 0.979 (0.936–1.023) |

How the last column is derived: GPU time per solve = 1 / (1.143 ×
rate ratio). The range combines the iteration interval (1.094–1.194)
with the min–max of the rate ratio.

![GPU](reports/gpu.svg)

**Verdict.** Best of P±M costs 8–12% of the kernel's rate. Against 1.143×
fewer steps, that nets a 1–5% saving of GPU time per solve.

- **P-256 at W = 64**, the kernel's fastest cell, saves about 5%, and its
  whole range lies below 1.
- **The other cells** save 1–3%, but their ranges cross 1.
- **Cost against batch size.** Against the CPU model's expectation, the
  relative cost is smallest at W = 64, where the kernel already spills
  registers.

This is an engineering gain at the kernel's best configuration, not a
change to the rho exponent.

**Open observations, reported as measured.**

- **Distinguished-point counts on P-256.** With best of P±M they ran
  2–8% below the 2^-24 expectation: observed/expected 0.923, 0.946 and
  0.974 at W = 16/32/64, about 2–3σ, with each W one sample because the
  repetitions replay identical walks. The plain build (0.981–0.996) and
  P-384 (0.991–1.019) sit at the expectation. The score (x limb 1) and
  the DP test (limb 0) read different limbs, so no cause is identified.
- **Cycle iterations the step model doesn't count.** The kernel walks up
  to `RHO_CYCLE_DEPTH + 1` = 6 extra iterations per detected fruitless
  cycle, and the harness's step count leaves these out. At the measured
  cycle rates that is about 1.2% (plain) against 1.5% (best of two) of
  iterations: roughly 0.4% more GPU time for best of P±M (derived).

**Not in `benchmarks/README.md`.** The validation gate
(`docs/VALIDATION_GATE.md`) admits a benchmark row only with a replay
certificate, a validator report and a matched rho reference. This run has
the matched reference (the plain kernel, same binary family and unit) but
neither of the other two, so the result stays in this research directory.

## Reproduce

```sh
make -C gpu/nist test BEST2=1
cd research/rho_walk_leads_20261014 && modal run modal_run.py --plan nistbest2 --out ../nist_best2_small_batch_20261027/iterations-v1
cat ../nist_best2_small_batch_20261027/iterations-v1/*.jsonl | harness/target/release/rho_walk_leads summary --ref neg_canon
cd ../../gpu/nist && modal run modal_app.py::best2_main --iters 2048 --reps 3 --out <json>
# rows: one JSON object per line, compact separators; then
rho_walk_leads gpuab --in gpu-ab-v1.rows.jsonl --speedup 1.1428 --lo 1.0944 --hi 1.1943
```
