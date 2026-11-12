# LEAD-A-selector-counting-law: preregistered grid, 2026-11-12

Lead: `research/leads_20261111/lane_A_falldegree.md`, Lead A2. All results are
`[pcc:unverified]`.

## What was run

- **Command.** `run_counting_grid.sh`. For each preregistered cell `(type, k, n, e)`
  it takes the first 3 unsatisfiable instances and runs `emin solve` with `EMAX = e`.
  A cell is feasible at `e` exactly when `e_min <= e`.
- **Instances.**
  - R-type: `emit_instance.py N SEED K` (`lfd_small.make_instance`).
  - T-type: `emit_instance_T.py`, with `V` random inside `ker Tr` and
    `Tr(a6/z^2) = 0`.
- **Raw output and score.** `grid.jsonl`; `score.txt`, written by
  `score_counting_grid.py` against the preregistered table.

## Result

| cell (type, k, n, e) | observed (3 instances) | Model A | Model B |
|:--|:--|:--|:--|
| R 8 17 2 | infeasible ×3 | infeasible ✓ | feasible ✗ |
| R 9 22 2 | infeasible ×3 | infeasible ✓ | feasible ✗ |
| R 10 26 2 | infeasible ×3 | infeasible ✓ | feasible ✗ |
| R 10 27 2 | infeasible ×3 | infeasible ✓ | feasible ✗ |
| R 8 18 2 | infeasible ×3 | ambiguous | feasible ✗ |
| R 11 23 3 | infeasible ×3 | ambiguous | feasible ✗ |
| R 12 28 3 | feasible ×3 | ambiguous | feasible ✓ |
| T 13 25 4 | infeasible ×3 | feasible ✗ | infeasible ✓ |

**Controls, all as expected.**
- Negative: R 11 22 3, R 12 26 3 and R 13 25 3 are infeasible.
- Positive: R 13 25 4 is feasible.

**Score.** Model A holds 4 of 5 non-ambiguous predictions; Model B holds 2 of 8.
The preregistered rule ("A wins if ≥ 4 of 5 hold and B holds ≤ 4 of 8") gives
**Model A wins**. No cell was mixed.

## Reading

- **Random `V`.** The plain counting ratio
  `ρ_A = (k+1) M(k,e+1) / (n M(k,e))`, with a threshold in (1.2194, 1.2698),
  predicts every random-`V` cell it was confident about. That includes the
  off-diagonal cells (n ≥ 2k).
- **Where B is too generous.** Model B's Koszul-plus-hyperplane correction
  over-predicts feasibility on random `V`: 6 of its 7 R predictions fail.
- **The trace hyperplane matters.** A's one miss is the T-type cell. With `V`
  inside `ker Tr`, k = 13 at e = 4 is infeasible on all three instances, while
  random `V` at the same (k, n, e) is feasible. B's T-type adjustment, k in place
  of k − 1, gets that cell right.
- **Combined with lead 2's nullity result.** Below `e_min` the selector system's
  dependencies are exactly the trivial family. At `e_min` extra syzygies appear.
  The feasibility threshold follows the plain ratio for random `V` and shifts for
  `V` inside `ker Tr`. Both readings are conjectures fitted to n ≤ 29 and are
  `[pcc:unverified]`.
