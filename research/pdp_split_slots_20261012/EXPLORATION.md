# Exploration notes (not preregistered, not results)

Two stage-driver sweeps run on 2026-10-18 while the frozen Part A/B/C runs
were in progress, niced under them. Seeds 77 and 78, two targets per cell,
four slot tuples per target on the closure. They size round 3; nothing
here is a measured result of this protocol, and nothing here changes it.

## Sweep 1: ℓ downward on the closure (`--base closure`, plan `3`)

| cell `m:a:n:ℓ` | closure abscissae | columns | slot tuples | λ per system (512 random systems) | XORs per target, g = 4 | g = 6 |
|---|--:|--:|--:|--:|--:|--:|
| 3:1:17:3 | 18 | 2 | 153 | 0 | 3.8e3 | 1.9e3 |
| 3:1:17:4 | 69 | 5 | 153 | 0.0078 (≈ 1.2 per target) | 2.5e5 | 6.5e4 |
| 3:1:19:3 | 20 | 2 | 190 | 0 | 3.5e3 | — |

- At n = 17 the closure of the dimension-4 subspace has yield where V
  alone had none in round 1 (0 in 256 targets): about 1.2 decompositions
  per target spread over 153 systems. Per relation that is about 128
  systems at 6.5e4 XORs (g = 6), about 8e6 XORs, against the measured
  ℓ = 5 arm's 1.5e10 per run. The rough projection is a 100–200× cheaper
  run at ℓ = 4 than the frozen ℓ = 5 arm, with a 5-column base.
- ℓ = 3 is too sparse at both sizes: the closure has 18–20 abscissae and
  no decomposition in 512 random systems. Expected decompositions per
  target scale like `|closure|³ / (6 r)`, which puts the useful ℓ at
  about `n/3 − log₂ n + 2.3`: 3.9 at n = 17, 4.4 at n = 19, 5.4 at
  n = 23. The closure buys about `log₂ n − 1.3` of ℓ against the plain
  subspace, i.e. about 3·that fewer Boolean unknowns per unsplit system.
- Every run was complete and exact against exhaustive enumeration.

## Sweep 2: guess count g on the standard base (plan `3`, g ∈ {4, 6, 8})

XORs per target, all `2^g` branches charged; two targets each, both exact.

| cell | g = 4 | g = 6 | g = 8 | solving degree 4 → 8 | F4 wall 4 → 8 (descriptive) |
|---|--:|--:|--:|---|---|
| 3:1:19:4 | 1.56e5 | 3.3e4 | 1.6e4 | 4 → 0 (linear after substitution) | 0.013 s → 0.003 s |
| 3:1:17:5 | 1.07e8 | 7.0e6 | 8.4e5 | 6 → 4 | 0.8 s → 0.11 s |
| 3:0:13:5 | 6.0e7–8.1e7 | 3.0e6–4.4e6 | 8.6e5–9.9e5 | 6 → 4 | 0.4–0.6 s → 0.09 s |

- At ℓ = 5 the XOR count falls 128× from g = 4 to g = 8, and the frozen
  round-2 arms (g = 4) sit far from the hybrid optimum.
- Wall time fell only 7×. F4's fixed per-call work (symbolic
  preprocessing, pair selection, divisor tests) is not in the
  elimination-only XOR count, and at 256 calls per target it dominates.
  The XOR figure therefore overstates the gain as g grows, and the priced
  S of a high-g arm would be unreliable in the same direction.
- g = mℓ is brute force over V³. Its fully charged relative is the
  pair-table MITM oracle, which the repository already measures at 4.27×
  strong rho on n = 17 (dimension-9 invariant base). The hybrid
  interpolates between F4 and that table.

## Sweep 3: ℓ-ladder on the closure at n = 23 (`K_0`, plan `3`, g = 8)

Two targets, two slot tuples each (seed 79), every run complete and exact.

| ℓ | unknowns | residual k | closure abscissae | columns | λ per system | XORs per branch | XORs per target | growth per unknown |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 4 | 12 | 4 | 186 | 10 | 0.0078 | 8.0e1 | 2.0e4 | — |
| 5 | 15 | 7 | 370 | 18 | 0.0156 | 4.2e3 | 1.07e6 | 3.74 |
| 6 | 18 | 10 | 761 | 35 | 0.0625 | 2.1e5 | 5.5e7 | 3.71 |
| 7 | 21 | 13 | 1428 | 64 | 0.297 | 4.6e7 | 1.18e10 | 5.99 |
| 8 | 24 | 16 | — | — | — | 5.2e8 | 1.33e11 | 2.25 (one target, one slot; 443 s; exact) |

The ℓ = 8 step is the first below 3: the solving degree has been 5 since
k = 10, and once it stops rising F4's cost in k should turn polynomial, so
the per-unknown factor would keep falling. Whether it falls below
enumeration's 2 is the degree-of-regularity question in measurable form;
ℓ = 9 (k = 19) is the next point.

The same growth read from sweep 2 at fixed ℓ: 8.7 (n = 13, ℓ = 5, k 9 → 11),
6.8 and 7.9 (n = 17, ℓ = 5, k 7 → 9 → 11). Enumeration grows by 2 per
unknown. `pdp_split_analyze --ladder` produced these columns.

## Sweep 4: g at fixed ℓ = 7, n = 23 closure (one target, two slot tuples, seed 79)

| g | branches | residual k | XORs per branch | XORs per target | F4 wall per target (descriptive) | solving degree |
|--:|--:|--:|--:|--:|--:|--:|
| 8 | 256 | 13 | 4.6e7 | 1.18e10 | 26–28 s | 5 |
| 10 | 1,024 | 11 | 4.2e5 | 4.35e8 | 13–21 s | 5 |
| 12 | 4,096 | 9 | 2.9e4 | 1.18e8 | 29–63 s | 4 |
| 14 | 16,384 | 7 | 1.9e2 | 3.1e6 | 45–108 s | 0–2 |

Growth per unknown from these: 12.3 (k 7 → 9), 3.8 (9 → 11), 10.5
(11 → 13). From g = 8 to g = 14 the XOR count falls 3,700× while the wall
time **rises** 2–4×: at 16,384 F4 calls per target the per-call work that
the elimination-only count omits is the whole cost. The XOR-priced S of a
high-g arm is therefore not a cost at all; only the fully charged table
arm (round 3's bracket) and, descriptively, wall time bound it.

## What this suggests for round 3

1. An oracle that **enumerates** the residual unknowns after guessing, with
   every polynomial evaluation counted at a pinned price, as the fully
   charged bracket; F4 stays as the lower bound. The two together bound the
   true cost from both sides.
2. g swept to the per-cell optimum, with XORs, F4 calls, divisor tests and
   wall all recorded.
3. ℓ = 4 on the closure at n = 17 (and ℓ = 5 at n = 19 and 23) in ecbench,
   to put the closure's "yield for free" where it matters.
4. An ℓ-ladder at one n large enough for yield (closure), ℓ = 4…7 at a
   fixed guess fraction, for the per-system cost exponent in ℓ. That
   exponent, against the table's `2^{2ℓ}` and rho's `2^{n/2}`, is what
   decides whether any of this scales, and it is not visible at one ℓ.
5. n = 15, where Frobenius-invariant subspaces of dimension 4–6 exist:
   invariant base vs closure vs standard, same oracle.

Raw lines: scratch only (`x-ell.jsonl`, `x-g.jsonl`), not committed; the
commands are in the text above and reproduce from the committed binaries.
