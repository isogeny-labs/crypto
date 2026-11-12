# Exponent study, extension: m = 3 and m = 4 at larger |V| under raised limits (frozen 2026-11-10)

Follow-up to [`../curve25519_exponent_study_20261108/PROTOCOL.md`](../curve25519_exponent_study_20261108/PROTOCOL.md)
(the "parent study"). The parent study left m = 3 and m = 4 "undetermined": its limits (3.5 GB per
process, 1,200 s per target) censored every chain before the preregistered estimator (at least 4
complete cells with d >= 9 spanning at least 4x in |V|) could apply. This extension does two things,
both frozen here before any cell runs:

1. **Raised limits on the censored cells.** The first censored cell of each chain is rerun with 12 GB
   of virtual memory per process (`ulimit -v 12000000`) and 10,800 s of wall time per target, and the
   chain continues under the parent's rule (d ascending, stop at the first censored cell).
2. **Densified fit ranges.** The parent grid used odd divisors of p - 1 only. The even divisors of
   p - 1 = 2^2 * 3^3 * 7 * 19 * 73 between the parent's smallest fitted cell and its first censored
   cell are added at k = 20: m = 3, d in {12, 14, 18, 28, 36, 38, 42, 54}; m = 4, d in {12, 14, 18}.
   With them, the m = 3 chain at k = 20 can reach the estimator's span (9 to 36 is 4x) even if
   d = 57 censors again.

Everything else is the parent's: the same binary, sha256 `dc2c30a4...3faf` (`data/frozen.sha256`,
archived at `/home/user/bin-archive/curve25519_family-dc2c30a4`, checked by the runner at launch),
frame `mont`, coset gidx = 0, form `sym`, 6 targets per cell with seed 20261108 + 1000 m + d (so a
rerun cell meets the same targets the parent censored), `RAYON_NUM_THREADS=1`, one process at a time.
The engine stops a cell at its first capped target and the cell is censored (never fitted).

## Cells and order (sequential, one process at a time)

Chains run d ascending and stop at the first censored cell; a cell completed in the parent is not
rerun. Order of execution, cheapest first so that results accrue if the run is interrupted:

1. k = 20, m = 3: 12, 14, 18 (parent complete: 3, 7, 9, 19, 21, 27)
2. k = 20, m = 4: 12, 14, 18 (parent complete: 3, 7, 9)
3. k = 20, m = 3: 28, 36, 38, 42, 54, then 57 (parent: censored at 1,200 s), 63, 73, ...
4. k = 20, m = 4: 19 (parent: out of memory at 3.5 GB), 21, 27, 28, ...
5. k = 22, m = 3: 55 (parent: out of memory at 3.5 GB), 75, 93

Chains 3 to 5 each end at their first censored cell under the new limits. A cell that censors in the
parent's chain position is still "censored" for the chain rule: in chains 3 and 4 the even cells
below the parent's censored d run first, then the parent's censored d, so the chain rule applies to
the union of parent and extension cells in d order.

The runner (`scripts/run_ext.py`) records `uptime` load averages before and after each cell and the
wall time of each cell, writes `cells/<name>.json` in the parent's format, and is resumable (a
complete cell is skipped, a censored cell ends its chain, a cell with no file is run).

## Limits and host

12 GB virtual memory per process; 10,800 s per target; the host is the same shared 4-core, 15 GB
container as the parent's. Wall-clock censoring under host load is a known weakness (parent README,
section 2); the load averages are recorded so that a censored cell can be read against them. No
EKS, no Conductor, no S3 checkpoint (same disclosure as the parent).

## Estimator and verdict rules (unchanged)

The parent's section 5 applies verbatim to the union of parent and extension complete cells per
(k, m, sym): OLS of ln T on ln |V| over complete cells with d >= 9, when there are at least 4 such
cells spanning at least 4x; slope s with a two-sided 95 % t-interval; cells that capped excluded.
Lever thresholds: m = 4 iff s < 0.5 with upper bound < 0.5; m = 3 iff s < 0 with upper bound < 0.
Balanced-total exponent 2/(m + 1 - s) under the paper's model, reported beside the verdict. Fewer
cells or smaller span: "undetermined" with two-point local slopes.

## Predictions (written before data)

- **P1.** k = 20, m = 3, d = 57 censors again at 10,800 s. Basis: its first target had spent 8.3e10
  field operations at 1,200 s with the matrix still growing (777,369 x 798,826 at degree 23), and the
  parent's local slope between d = 21 and 27 is above 10.
- **P2.** At least 4 of the m = 3 even cells {12, 14, 18, 28, 36, 38, 42, 54} complete, the estimator
  applies to the k = 20, m = 3 chain, and the fitted s is above 3 with a lower 95 % bound above 0.
  Verdict: no lever at m = 3.
- **P3.** k = 20, m = 4, d = 19 censors (time or memory) under 12 GB and 10,800 s. Basis: the parent's
  m = 4 cells grow from 2.5e7 (d = 7) to 1.4e11 (d = 9) field operations.
- **P4.** The m = 4 cells d = 12, 14, 18 complete, and every two-point local slope among the complete
  m = 4 cells with d >= 9 is above 2. If the estimator applies (it needs d = 36 or beyond, which P3
  says it will not reach), s > 2 with lower bound above 0.5. Verdict: no lever at m = 4; the
  preregistered estimator is expected to remain inapplicable at m = 4.
- **P5.** k = 22, m = 3, d = 55 censors again (memory: the parent's exit was an allocation failure at
  3.5 GB, and F4 matrix memory grows faster than linearly in |V|).
- **P6.** The frozen binary's sha256 matches `data/frozen.sha256` at launch, and every rerun cell's
  first target has the same `target_x` as the parent's censored cell (same seed): 610025 for
  k = 20, m = 3, d = 57.

A failed prediction stays failed. Every number in the write-up is an unverified assertion
[pcc:unverified] until a validator report says otherwise.
