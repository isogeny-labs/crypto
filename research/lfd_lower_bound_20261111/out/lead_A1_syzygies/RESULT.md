# FOLLOWUP-selector-extra-syzygies (follow-up to LEAD-A-selector-eta-kernel), 2026-11-12

All result statements below are `[pcc:unverified]`.

## Preregistration (written before any computation in this directory)

Frozen at 2026-11-12, before `syz.c` was written or run.

**Object.** For an unsatisfiable instance `(n, k = (n+1)/2, V, z, a6)` and a degree `e`, `K_e` is the
kernel of the homogeneous selector system (all `2^k` points, rows `Tr(lambda(C) Phi*_C(v_i))`,
`Tr(lambda(C) h_C)`, unknowns the ANF coefficients of `lambda: V -> GF(2^n)` of degree `<= e`).
`T_e = { q (1 + ell_0) zeta : deg q <= e-1 }`, `zeta = z^-2`, `ell_0(C) = Tr(C) + Tr(a6/z^2)`
(lane_A_falldegree.md, Lead A1, F3), `dim T_e = M(k-1, e-1)`.
The **excess** is `x_e = dim K_e - dim T_e`, computed only after checking `T_e ⊆ K_e`.

**Prediction P (from the task card).** The excess is nonzero only at `e = e_min`.

Precise reading, fixed now: because `K_e ⊆ K_{e+1}` and `T_{e+1} ∩ (deg <= e) = T_e`, a nonzero
excess at `e_min` forces `x_e >= x_{e_min} > 0` for every `e > e_min`. So P is only testable at
`e <= e_min` and is read as:

- P1: `x_e = 0` for every tested `e < e_min`;
- P2: `x_{e_min} > 0` for every tested instance.

**Cells** (instances: the committed unsatisfiable seeds of `lfd_small.make_instance`, `e_min` from
`out/c/emin_n*.jsonl`):

| n | k | e_min | seeds | e tested |
|--:|--:|--:|:--|:--|
| 15 | 8 | 3 | 1, 5, 6 | 2, 3 |
| 17 | 9 | 3 | 1, 2, 8 | 2, 3 |
| 19 | 10 | 4 | 4, 7, 10 | 3, 4 |
| 21 | 11 | 4 | 3, 11 | 3, 4 |
| 23 | 12 | 4 | 4, 8, 11 | 3 (control), 4 |
| 25 | 13 | 4 | 5, 6, 7 | 3 (control), 4 |
| 27 | 14 | 5 | 3, 6 | 4 (control, already known 0), 5 if it fits in 3 GB / 20 min |

**Falsifiers.** P is falsified by any cell with `x_e > 0` at `e < e_min` (P1), or any instance with
`x_{e_min} = 0` (P2). Any instance with `T_e` not inside `K_e`, or `rank T_e != M(k-1,e-1)`, or a kernel
vector that fails the independent point-evaluation check, is a bug and voids the cell.

**Controls.**
- Reproduction: the kernel dimension at `n = 23, 25` must equal the committed `nullity_grid.jsonl`
  values (1144/1145/1143, 741/740/740) to the digit.
- Independent check: every kernel basis vector is evaluated on all `2^k` points by a Moebius
  transform and every homogeneous row is re-tested by parity (code path independent of the
  echelon form).
- Trivial family: `T_e ⊆ K_e`, `rank T_e = M(k-1,e-1)`.

**Characterisation (descriptive, no thresholds; reported as found):** degree filtration of the
excess, support on the two halves `H_0 = {ell_0 = 0}`, `H_1 = {ell_0 = 1}`, fibre ranks against the
pointwise annihilator dimension, the V-independent ("universal") subfamily
`{lambda : lambda(C) ⊥ Phi*_C(GF(2^n)) and ⊥ h_C for all C}`, and closure under Frobenius
(coefficientwise squaring) and under multiplication by the field generator `x`.

### Addendum to the preregistration (written 06:46 UTC, after n = 15-27 seed 3 data, before the n = 27 seed 6, e = 5 run)

Post-hoc observation from the first 36 cells: at `e = e_min` the kernel dimension equals a per-n
base plus `h`, the number of points `C` in `V` with `C = X1 + X2`, `X1, X2` in `V`, `X1 X2 = z`.
Out-of-sample prediction for `n = 27`, seed 6 (h = 2), `e = 5`: kernel dimension `10400 + 1 = 10401`
(seed 3 has h = 1 and 10400). Falsified by any other value.

Result of the out-of-sample check: `n = 27`, seed 6, `e = 5` gave kernel dimension **10401**, as
predicted (`[pcc:unverified]`).

---

## What was run

All commands are run from `research/lfd_lower_bound_20261111/`. Each job ran single-threaded.
`emin.c` is not modified (`git diff` on it is empty), and every new file is in this directory.

- **Tool.** `out/lead_A1_syzygies/syz.c`, compiled with
  `gcc -O2 -march=native -o syz syz.c -lm4ri -lm`. It has three modes:
  - `syz E PERMILLE SEED`: the kernel analysis.
    - It builds the homogeneous rows of emin.c (same row builder, same column layout).
    - It extracts a kernel basis from the reduced echelon form (M4RI).
    - It cuts that basis to the exact kernel by evaluating every vector on all `2^k` points.
      This step only matters when `PERMILLE < 1000` (a point sample).
    - It re-checks the final basis by Moebius evaluation and per-row parity on all `2^k (k+1)`
      rows.
    - It builds `T_e` from the F3 formula and checks `T_e ⊆ K_e` the same way.
    - It reports ranks, the degree filtration, support on `H_0`/`H_1`, the V-independent
      subfamily, fibre ranks against `dim A_C`, and closure under coefficientwise squaring and
      under multiplication by `x`.
  - `syz dual D`: the dual section count `dim S_D` (see the reading below).
  - `syz coset H E`: the sections over one half `H_H` only, in its `k − 1` coordinates.
- **Main grid** (`grid.jsonl`, `run.log`):
  - `out/lead_A1_syzygies/run_grid.sh "15:1,5,6:2,3" "17:1,2,8:2,3" "19:4,7,10:3,4" "21:3,11:3,4"`
  - `out/lead_A1_syzygies/run_grid.sh "23:4,8,11:3,4" "25:5,6,7:3,4"`
  - `out/lead_A1_syzygies/run_grid.sh "27:3:4" "27:3:5:460"`
  - `out/lead_A1_syzygies/run_grid.sh "27:6:5:460"`
  - The `n = 27, e = 5` runs use a sample of 7,536 of the 16,384 points. The sample kernel had
    violation rank 0 on all points, so it is the exact kernel. The cost was 1.53 GB peak and
    744 s / 761 s.
- **Post-hoc tools:**
  - `high_points.py N SEED` (`high_points.jsonl`) gives `h`.
  - `syz dual D` at `D = k − e − 1` (`dual.jsonl`) covers 20 cells at n = 15–23.
  - `syz coset H E` (`coset.jsonl`) covers 46 half-cells.
- **Tables, figure and PDF:** `RESULT.pdf` is a pandoc render of this file, with Unicode symbols spelled out in ASCII. `summarize.py` writes `tables.md` and `plot.py` writes
  `kernel_vs_trivial.svg` and `kernel_vs_trivial.png`.

## Controls

- **Reproduction:** passes. The kernel dimension at `n = 23` (1144 / 1145 / 1143) and `n = 25`
  (741 / 740 / 740) equals the committed `out/lead_A1/nullity_grid.jsonl` to the digit, and so
  does `n = 27, e = 4` (378).
- **Independent evaluation check:** every final kernel basis vector passes in all 37 cells
  (`kernel_check_ok`).
- **Trivial family:** in all 37 cells, `T_e ⊆ K_e` and `rank T_e = M(k−1, e−1)`. F1/F2 hold at
  every point (`F1F2_violations = 0`).
- **Default unchanged after adding the dual/coset modes:** the rebuilt `syz` reproduces the grid
  JSON of `n = 19` seed 4 and `n = 23` seed 4 at `e = 4` field for field (only time and RSS
  differ).

## Results `[pcc:unverified]`

### Excess per cell, and the preregistered test

`h` is the number of `C` in `V` with `C = X1 + X2`, `X1, X2 ∈ V`, `X1 X2 = z`.

| n | k | seed | e | e_min | kernel dim | M(k-1,e-1) = rank T | T in K | excess | P | h | cols - E_eff | dim S_{k-e-1} (dual) | identity |
|--:|--:|--:|--:|--:|--:|--:|:-:|--:|:-:|--:|--:|--:|:-:|
| 15 | 8 | 1 | 2 | 3 | 8 | 8 | yes | 0 | P1 ok | 0 | -120 | 1502 | yes |
| 15 | 8 | 1 | 3 | 3 | 146 | 29 | yes | 117 | P2 ok | 0 | 146 | 800 | yes |
| 15 | 8 | 5 | 2 | 3 | 8 | 8 | yes | 0 | P1 ok | 0 | -120 | - | - |
| 15 | 8 | 5 | 3 | 3 | 146 | 29 | yes | 117 | P2 ok | 0 | 146 | - | - |
| 15 | 8 | 6 | 2 | 3 | 8 | 8 | yes | 0 | P1 ok | 1 | -120 | 1501 | yes |
| 15 | 8 | 6 | 3 | 3 | 147 | 29 | yes | 118 | P2 ok | 1 | 146 | 800 | yes |
| 17 | 9 | 1 | 2 | 3 | 9 | 9 | yes | 0 | P1 ok | 0 | -263 | 3836 | yes |
| 17 | 9 | 1 | 3 | 3 | 87 | 37 | yes | 50 | P2 ok | 0 | 31 | 2486 | yes |
| 17 | 9 | 2 | 2 | 3 | 9 | 9 | yes | 0 | P1 ok | 0 | -263 | - | - |
| 17 | 9 | 2 | 3 | 3 | 87 | 37 | yes | 50 | P2 ok | 0 | 31 | - | - |
| 17 | 9 | 8 | 2 | 3 | 9 | 9 | yes | 0 | P1 ok | 1 | -263 | 3835 | yes |
| 17 | 9 | 8 | 3 | 3 | 88 | 37 | yes | 51 | P2 ok | 1 | 31 | 2486 | yes |
| 19 | 10 | 4 | 3 | 4 | 46 | 46 | yes | 0 | P1 ok | 1 | -265 | 6942 | yes |
| 19 | 10 | 4 | 4 | 4 | 1164 | 130 | yes | 1034 | P2 ok | 1 | 1163 | 4070 | yes |
| 19 | 10 | 7 | 3 | 4 | 46 | 46 | yes | 0 | P1 ok | 2 | -265 | 6941 | yes |
| 19 | 10 | 7 | 4 | 4 | 1165 | 130 | yes | 1035 | P2 ok | 2 | 1163 | 4070 | yes |
| 19 | 10 | 10 | 3 | 4 | 46 | 46 | yes | 0 | P1 ok | 1 | -265 | - | - |
| 19 | 10 | 10 | 4 | 4 | 1164 | 130 | yes | 1034 | P2 ok | 1 | 1163 | - | - |
| 21 | 11 | 3 | 3 | 4 | 56 | 56 | yes | 0 | P1 ok | 2 | -849 | 17711 | yes |
| 21 | 11 | 3 | 4 | 4 | 1211 | 176 | yes | 1035 | P2 ok | 2 | 999 | 11936 | yes |
| 21 | 11 | 11 | 3 | 4 | 56 | 56 | yes | 0 | P1 ok | 1 | -849 | 17712 | yes |
| 21 | 11 | 11 | 4 | 4 | 1210 | 176 | yes | 1034 | P2 ok | 1 | 999 | 11936 | yes |
| 23 | 12 | 4 | 3 | 4 | 67 | 67 | yes | 0 | P1 ok | 1 | -1860 | 42342 | yes |
| 23 | 12 | 4 | 4 | 4 | 1144 | 232 | yes | 912 | P2 ok | 1 | 153 | 32034 | yes |
| 23 | 12 | 8 | 3 | 4 | 67 | 67 | yes | 0 | P1 ok | 2 | -1860 | - | - |
| 23 | 12 | 8 | 4 | 4 | 1145 | 232 | yes | 913 | P2 ok | 2 | 153 | 32034 | yes |
| 23 | 12 | 11 | 3 | 4 | 67 | 67 | yes | 0 | P1 ok | 0 | -1860 | - | - |
| 23 | 12 | 11 | 4 | 4 | 1143 | 232 | yes | 911 | P2 ok | 0 | 153 | 32034 | yes |
| 25 | 13 | 5 | 3 | 4 | 79 | 79 | yes | 0 | P1 ok | 1 | -3473 | - | - |
| 25 | 13 | 5 | 4 | 4 | 741 | 299 | yes | 442 | P2 ok | 1 | -1900 | - | - |
| 25 | 13 | 6 | 3 | 4 | 79 | 79 | yes | 0 | P1 ok | 0 | -3473 | - | - |
| 25 | 13 | 6 | 4 | 4 | 740 | 299 | yes | 441 | P2 ok | 0 | -1900 | - | - |
| 25 | 13 | 7 | 3 | 4 | 79 | 79 | yes | 0 | P1 ok | 0 | -3473 | - | - |
| 25 | 13 | 7 | 4 | 4 | 740 | 299 | yes | 441 | P2 ok | 0 | -1900 | - | - |
| 27 | 14 | 3 | 4 | 5 | 378 | 378 | yes | 0 | P1 ok | 1 | -5903 | - | - |
| 27 | 14 | 3 | 5 | 5 | 10400 | 1093 | yes | 9307 | P2 ok | 1 | 6538 | - | - |
| 27 | 14 | 6 | 5 | 5 | 10401 | 1093 | yes | 9308 | P2 ok | 2 | 6538 | - | - |

### Characterisation per cell

| n | seed | e | degree filtration [d, dim K_<=d, dim T_<=d] (last two) | rank on H1 | dim vanishing on H1 | universal | Frobenius-closed | x-closed | fibre rank = dim A_C at every point |
|--:|--:|--:|:--|--:|--:|--:|--:|--:|:-:|
| 15 | 1 | 2 | [[1, 1, 1], [2, 8, 8]] | 0 | 8 | 8 | 0 | 0 | no (0 on all of H1) |
| 15 | 1 | 3 | [[2, 8, 8], [3, 146, 29]] | 117 | 29 | 29 | 0 | 0 | yes |
| 15 | 5 | 2 | [[1, 1, 1], [2, 8, 8]] | 0 | 8 | 8 | 0 | 0 | no (0 on all of H1) |
| 15 | 5 | 3 | [[2, 8, 8], [3, 146, 29]] | 117 | 29 | 29 | 0 | 0 | yes |
| 15 | 6 | 2 | [[1, 1, 1], [2, 8, 8]] | 0 | 8 | 8 | 0 | 0 | no (0 on all of H1) |
| 15 | 6 | 3 | [[2, 8, 8], [3, 147, 29]] | 118 | 29 | 29 | 0 | 0 | yes |
| 17 | 1 | 2 | [[1, 1, 1], [2, 9, 9]] | 0 | 9 | 9 | 0 | 0 | no (0 on all of H1) |
| 17 | 1 | 3 | [[2, 9, 9], [3, 87, 37]] | 50 | 37 | 37 | 0 | 0 | yes |
| 17 | 2 | 2 | [[1, 1, 1], [2, 9, 9]] | 0 | 9 | 9 | 0 | 0 | no (0 on all of H1) |
| 17 | 2 | 3 | [[2, 9, 9], [3, 87, 37]] | 50 | 37 | 37 | 0 | 0 | yes |
| 17 | 8 | 2 | [[1, 1, 1], [2, 9, 9]] | 0 | 9 | 9 | 0 | 0 | no (0 on all of H1) |
| 17 | 8 | 3 | [[2, 9, 9], [3, 88, 37]] | 51 | 37 | 37 | 0 | 0 | yes |
| 19 | 4 | 3 | [[2, 10, 10], [3, 46, 46]] | 0 | 46 | 46 | 0 | 0 | no (0 on all of H1) |
| 19 | 4 | 4 | [[3, 46, 46], [4, 1164, 130]] | 1034 | 130 | 130 | 0 | 0 | yes |
| 19 | 7 | 3 | [[2, 10, 10], [3, 46, 46]] | 0 | 46 | 46 | 0 | 0 | no (0 on all of H1) |
| 19 | 7 | 4 | [[3, 46, 46], [4, 1165, 130]] | 1035 | 130 | 130 | 0 | 0 | yes |
| 19 | 10 | 3 | [[2, 10, 10], [3, 46, 46]] | 0 | 46 | 46 | 0 | 0 | no (0 on all of H1) |
| 19 | 10 | 4 | [[3, 46, 46], [4, 1164, 130]] | 1034 | 130 | 130 | 0 | 0 | yes |
| 21 | 3 | 3 | [[2, 11, 11], [3, 56, 56]] | 0 | 56 | 56 | 0 | 0 | no (0 on all of H1) |
| 21 | 3 | 4 | [[3, 56, 56], [4, 1211, 176]] | 1035 | 176 | 176 | 0 | 0 | yes |
| 21 | 11 | 3 | [[2, 11, 11], [3, 56, 56]] | 0 | 56 | 56 | 0 | 0 | no (0 on all of H1) |
| 21 | 11 | 4 | [[3, 56, 56], [4, 1210, 176]] | 1034 | 176 | 176 | 0 | 0 | yes |
| 23 | 4 | 3 | [[2, 12, 12], [3, 67, 67]] | 0 | 67 | 67 | 0 | 0 | no (0 on all of H1) |
| 23 | 4 | 4 | [[3, 67, 67], [4, 1144, 232]] | 912 | 232 | 232 | 0 | 0 | yes |
| 23 | 8 | 3 | [[2, 12, 12], [3, 67, 67]] | 0 | 67 | 67 | 0 | 0 | no (0 on all of H1) |
| 23 | 8 | 4 | [[3, 67, 67], [4, 1145, 232]] | 913 | 232 | 232 | 0 | 0 | yes |
| 23 | 11 | 3 | [[2, 12, 12], [3, 67, 67]] | 0 | 67 | 67 | 0 | 0 | no (0 on all of H1) |
| 23 | 11 | 4 | [[3, 67, 67], [4, 1143, 232]] | 911 | 232 | 232 | 0 | 0 | yes |
| 25 | 5 | 3 | [[2, 13, 13], [3, 79, 79]] | 0 | 79 | 79 | 0 | 0 | no (0 on all of H1) |
| 25 | 5 | 4 | [[3, 79, 79], [4, 741, 299]] | 442 | 299 | 299 | 0 | 0 | yes |
| 25 | 6 | 3 | [[2, 13, 13], [3, 79, 79]] | 0 | 79 | 79 | 0 | 0 | no (0 on all of H1) |
| 25 | 6 | 4 | [[3, 79, 79], [4, 740, 299]] | 441 | 299 | 299 | 0 | 0 | yes |
| 25 | 7 | 3 | [[2, 13, 13], [3, 79, 79]] | 0 | 79 | 79 | 0 | 0 | no (0 on all of H1) |
| 25 | 7 | 4 | [[3, 79, 79], [4, 740, 299]] | 441 | 299 | 299 | 0 | 0 | yes |
| 27 | 3 | 4 | [[3, 92, 92], [4, 378, 378]] | 0 | 378 | 378 | 0 | 0 | no (0 on all of H1) |
| 27 | 3 | 5 | [[4, 378, 378], [5, 10400, 1093]] | 9307 | 1093 | 1093 | 0 | 0 | yes |
| 27 | 6 | 5 | [[4, 378, 378], [5, 10401, 1093]] | 9308 | 1093 | 1093 | 0 | 0 | yes |

The `cols − E_eff` column uses the lead's rank bound `E_eff = (k+1)M(k,e+1) − M(k,e+2) + 1`.

**Kernel dimensions at e_min, with bases.** The base is the excess minus `h`.

| n | k | e_min | kernel dim at e_min | excess | excess − h (base) |
|--:|--:|--:|:--|:--|--:|
| 15 | 8 | 3 | 146, 146, 147 | 117, 117, 118 | 117 |
| 17 | 9 | 3 | 87, 87, 88 | 50, 50, 51 | 50 |
| 19 | 10 | 4 | 1164, 1165, 1164 | 1034, 1035, 1034 | 1033 |
| 21 | 11 | 4 | 1211, 1210 | 1035, 1034 | 1033 |
| 23 | 12 | 4 | 1144, 1145, 1143 | 912, 913, 911 | 911 |
| 25 | 13 | 4 | 741, 740, 740 | 442, 441, 441 | 441 |
| 27 | 14 | 5 | 10400, 10401 | 9307, 9308 | 9306 |

## Verdict against the preregistered falsifiers

**P holds in every tested cell** `[pcc:unverified]`.

- **P1:** the excess is 0 at all 18 instances with `e = e_min − 1`: n = 15, 17 at e = 2;
  n = 19, 21, 23, 25 at e = 3; n = 27 at e = 4.
- **P2:** the excess is positive at all 19 instances with `e = e_min`: n = 15, 17 at e = 3;
  n = 19–25 at e = 4; n = 27 at e = 5 (2 seeds).
- **No bug condition fired.**
- **Scope.** P is only testable at `e <= e_min`. Above `e_min` a positive excess is forced by
  `K_e ⊆ K_{e+1}` (see the preregistration).

## Reading: what the extra kernel is `[pcc:unverified]`

Points 1–4 below are measured. Points 5–8 are post-hoc: two of them come with a short proof and
one with an out-of-sample check. None of it is machine-checked.

1. **Degree profile.**
   - At `e = e_min`, every extra element has degree exactly `e_min`:
     `dim (K ∩ deg <= e_min − 1) = dim T` in every cell.
   - So the extra kernel is a set of new top-degree generators. Since `K_{e_min−1} = T_{e_min−1}`,
     none of them is a multiple of a lower-degree kernel element.
2. **Not V-independent and not field-structured.**
   - Take the subfamily with `lambda(C) ⊥ Phi*_C(GF(2^n))` and `⊥ h_C` at every `C`. It equals
     `T_e` exactly in all 37 cells. The only V-independent syzygies are the trivial ones.
   - The subspace mapped back into the kernel by coefficientwise squaring (Frobenius) has
     dimension 0 in every cell, and so does the one for multiplication by `x`. This holds even
     for `T_e`, because `zeta` is fixed.
   - So the extra kernel is not a Frobenius orbit or a `GF(2^n)`-subspace.
3. **Support.**
   - The trivial family is exactly the set of kernel elements that vanish on `H_1` (in all cells).
   - Every extra element is nonzero somewhere on `H_1`: `rank_on_H1 = excess`.
4. **Pointwise behaviour ("global generation"), measured in all 37 cells.**
   - At `e = e_min`, the evaluations of `K_e` at `C` span the whole pointwise annihilator
     `A_C = W_C^⊥`, `W_C = span(Phi*_C(V), h_C)`, at every one of the `2^k` points.
   - At `e = e_min − 1`, they span `zeta` on `H_0` and nothing on `H_1`.
   - So the extra syzygies appear at the same degree as global generation of the annihilator
     bundle.
5. **A recognisable sub-family on H_1: the mirror of the trivial family.**
   - A kernel element supported on `H_1` is `ell_0 · s`, with `s` a section over `H_1` of degree
     `<= e − 1` in its `k − 1` coordinates. So `dim {lambda ∈ K_e : lambda|H_0 = 0}` equals the
     dimension of the `H_1`-sections at degree `e − 1`.
   - The data agree in every cell where both were computed:
     - n = 19, seeds 4 / 7 / 10: 105 / 108 / 105;
     - n = 27, seeds 3 / 6: 843 / 846;
     - 0 at n = 15, 17, 21, 23, 25.
   - So `K_e = T_e ⊕ E_e ⊕ G_e`:
     - `T_e`: supported on `H_0`, the F3 family;
     - `E_e`: supported on `H_1`, nonzero only at n = 19 and n = 27;
     - `G_e`: a complement of elements nonzero on both halves.
   - Sizes of `G_e`:
     - n = 19: 1164 − 130 − 105 = 929;
     - n = 27: 10400 − 1093 − 843 = 8464;
     - otherwise all of the excess.
   - The `H_0`-sections at degree `e_min − 1` are exactly `zeta · q`. Their dimensions are
     `M(k−1, e_min−1)`: 130 at n = 19, 232 at n = 23, 1093 at n = 27.
6. **Exact duality identity** (proved by hand below; checked numerically in 20 of 20 cells).

   `nullity_e = n M(k,e) − (k+1) 2^k + Ann0 + dim S_{k−e−1}`, where:
   - `Ann0 = Σ_C (k+1 − rank W_C)` is the dimension of the pointwise syzygies of the rows;
   - `S_D = { G : V → GF(2^n), deg G <= D, G(C) ∈ W_C for all C }`.

   Proof sketch:
   - The annihilator of the row space is `{g : V → F_2^{k+1} : deg Σ_i g_i w_i <= k−e−1}`,
     by the duality between Reed–Muller codes of orders `e` and `k−e−1`.
   - The map `g ↦ Σ g_i w_i` has kernel the pointwise syzygies and image `S_{k−e−1}`.
   - With the obvious sections `Σ g_i w_i`, `deg g <= D−1` (dimension
     `(k+1)M(k,D−1) − M(k,D−2)`), the identity becomes
     `nullity = cols − E_eff + h + extra_S`.
   - So the lead's `E_eff` is exactly this count with no extra sections. Its `+1` is the point
     `C = 0`, where `rank W_0 = k+1`.
7. **The A/A variation is exactly `h`.**
   - `Phi*_C(A) = A (A + C)(A² + CA + z)`, so `rank W_C` drops by one exactly when `V` contains a
     root of `A² + CA + z`. Then `C = X1 + X2` with `X1 X2 = z` and `X1, X2 ∈ V`.
     `high_points.py` checks that set equality on 19 instances, n = 15–27.
   - Hence `Ann0 = 2^k − 1 + h`, given `sqrt(z) ∉ V`, which holds in every instance.
   - The measured `dim S_{k−e−1}` at `e_min` does not depend on the instance:
     800, 2486, 4070, 11936, 32034 at n = 15, 17, 19, 21, 23. So at `e_min` the kernel is a
     per-n base plus `h`.
   - At `e_min − 1`, `dim S` drops by exactly `h`: 1502 / 1501, 3836 / 3835, 6942 / 6941,
     17712 / 17711. That cancellation keeps the nullity at `M(k−1, e−1)`.
   - The out-of-sample prediction (n = 27, seed 6: 10401) held.
   - These `h` points are partial Semaev decompositions: pairs in `V` with the right product
     `X1 X2 = z` (not solutions of `S = 0`; the instances are unsatisfiable).
8. **Generic count against special sections.**
   - At n = 15 and 19, `extra_S = 0`: the kernel at `e_min` is exactly
     `cols − E_eff + h` (146 + h, 1163 + h). There the excess is a plain count of unknowns minus
     independent equations, not a special syzygy.
   - At n = 17, 21, 23 there are sections of `W` beyond the obvious ones: 56, 210, 990 (measured).
     The identity gives 2640 at n = 25 and 3861 at n = 27, seed 3.
   - 56 = C(8,3) and 210 = C(10,4), but 990 matches no such binomial. No closed form is claimed.

**Summary.** The extra syzygies at `e_min` consist of three parts:
- an instance-dependent `+h`, explained by the curve-point structure (one per point
  `X1 + X2` with `X1 X2 = z`);
- an `H_1`-supported mirror of the trivial family, when the `H_1`-sections at `e_min − 1` are
  nonzero;
- a large global part that is no V-independent, Frobenius or `GF(2^n)`-linear family. At n = 15
  and 19 that part is fully accounted for by the plain count, and elsewhere it is driven by
  extra low-degree sections of `W`.

## What is blocked or not done

- **`n = 25, 27` dual counts (`syz dual`) are not computed.** They need 177k and 349k columns,
  more than 2.2 GB, above the 3 GB / 20 min job budget. Their `extra_S` values come from the
  identity, not from a measurement.
- **`n = 29` and larger are not run** (`e_min` is 5 there): EKS/S3 are unavailable because the
  AWS identity is rejected. No resume checkpoint or bench record was written: no S3, and this
  is not a benchmark.
- **`research-kb` and `ecdb` were not queried** from this subagent. No novelty claim is made.
- **Not tested:** whether global generation (point 4) is equivalent to feasibility. It is the
  same degree in all 19 instances, but no argument links them. The planted and wrong-trace
  controls from Lead A1 were not run here either.

## Next step

1. Prove or refute: `e_min` = the least `e` at which `K_e` generates `A_C` at every point. Test
   it at the T-type instances (`V ⊆ ker Tr`) and at `n = 29` on a larger box.
2. Identify the extra sections of `W` (56, 210, 990, …), for example by checking whether they
   are `H_0`/`H_1`-supported sections of the dual bundle. A closed form for `dim S_{k−e−1}`
   would give `e_min` as a counting statement through the identity in point 6.
3. Lean obligations: the factorisation `Phi*_C(A) = A(A+C)(A²+CA+z)`, and the duality identity
   (Reed–Muller duality plus rank–nullity).
