# LEAD-A-selector-eta-kernel: preregistered nullity test, 2026-11-12

Lead: `research/leads_20261111/lane_A_falldegree.md`, Lead A1, conjecture C1. All
results are `[pcc:unverified]`.

## What was run

- **Code.** `emin nullity E`, a new mode in `emin.c`. It builds the homogeneous
  selector system at degree `E` over all `2^k` points (no right-hand side, no
  sampling), echelonises it with M4RI, and prints `cols − rank`.
- **Instances.** The first three unsatisfiable seeds of `lfd_small.make_instance` at
  each of `n = 23, 25, 27`, at `e = 3` and `4`. That is 18 runs.
- **Raw output.** `nullity_grid.jsonl`.

## Result

| n | k | e | predicted M(k−1,e−1) | inside the lead's non-binding region | observed nullity (3 seeds) | verdict |
|--:|--:|--:|--:|:--|:--|:--|
| 23 | 12 | 3 | 67 | yes | 67, 67, 67 | holds |
| 23 | 12 | 4 | 232 | yes | 1144, 1145, 1143 | **fails, excess about 911** |
| 25 | 13 | 3 | 79 | yes | 79, 79, 79 | holds |
| 25 | 13 | 4 | 299 | yes | 741, 740, 740 | **fails, excess about 441** |
| 27 | 14 | 3 | 92 | yes | 92, 92, 92 | holds |
| 27 | 14 | 4 | 378 | yes | 378, 378, 378 | holds |

- **"Inside the region"** means the lead's own condition holds:
  `E_eff = (k+1)M(k,e+1) − M(k,e+2) + 1 >= n M(k,e) − M(k−1,e−1)`.
- **Independent check.** `emin verify` (a second row builder plus PLUQ elimination)
  on all `2^k` points reproduces the coefficient-matrix rank in two cells:
  - `n = 23` seed 4, `e = 4`: 17,118 = 18,262 − 1,144;
  - `n = 25` seed 5, `e = 4`: 26,584 = 27,325 − 741.

  Both systems are also consistent there, so these are feasible degrees.
- **The 9 earlier certificates** at `n = 33–41` (infeasible cells, logged rank
  deficits) also equal `M(k−1,e−1)`.

## Reading

- **C1 is falsified as stated.** The lead's falsification criterion was "any cell
  with nullity strictly above `M(k−1,e−1)` after a full-point run is a non-trivial
  `c`-only syzygy". Two cells exceed it, and both are inside the region where the
  lead claimed equality.
- **The A/A control fails in the same cells.** The nullity there varies by 1–2
  between instances, so it is not a function of `(k, n, e)` alone, as the A/A
  control assumed.
- **Pattern (an observation, not a law).** The excess appears exactly in the
  feasible cells: `e = e_min = 4` at `k = 12` and `13`. Every infeasible cell
  measured so far equals the trivial kernel exactly. That is 3 × 4 cells here plus
  the 9 certificates.
- **What this suggests (conjecture).** The trivial kernel accounts for all the
  linear dependence below `e_min`. At `e_min`, extra `c`-only syzygies of degree
  `<= e` appear, and these could be what makes the system solvable. Extracting a
  few of them (a kernel basis modulo the trivial family) is the natural next step.
  It is not done here.
- **Unaffected.** The proved part of the lead (F1–F4: the trivial family exists, so
  the nullity is at least `M(k−1,e−1)`) is consistent with every cell.
- **Not tested here:**
  - the secondary prediction C2 (`e'_min = e_min`), which needs the
    hyperplane-restricted mode;
  - the T-type cells (`V` inside `ker Tr`);
  - the planted and wrong-trace controls.
