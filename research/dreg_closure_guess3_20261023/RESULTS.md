# Results: three guessed variables do not suffice at ℓ = 8 for this guess set

Scored natively by `dreg_score guess --k 3 runs/m3-22-8.*.jsonl`
(`score-output.txt`) against `PREREGISTRATION.md` (commit `821b33666`, no
amendments).

- **Records.** Every value comes from a committed record in `runs/`.
- **Run.** 2026-10-23, 05:16:20 to 06:06:03 UTC: 16 processes (1 N7, 4 G6 and
  11 G3), all exited 0 (`runs/queue.log`). No run was killed, empty or rerun.
- **Interruptions.** None in the queue log. `run.sh` took only `CLOSURE_BIN`
  and the step (`control`, then `measure`); the script is unchanged. A
  checkpoint commit (`39ba2ec01`) was made mid-queue.
- **Binary provenance.** `dreg_closure` SHA-256 `d7246453…2ec4`,
  `dreg_score` SHA-256 `efa15cb5…18b9` (`runs/binary.sha256`, taken before the
  first run). The `dreg_closure` hash differs from the previous study's
  (`a11dddbb…`) because of a rustfmt-only change; the N7 control below checks
  that this changed nothing.
- **Draws.** The frozen `research/dreg_closure_guess_20261022/draws/m3-22-8.jsonl`:
  m = 3, (n, ℓ) = (22, 8), N = 46, unsatisfiable draws 0, 3, 5, 6. No new draw
  was generated.
- **Memory.** Peak VmHWM 7,044,568 kB (7.04 GB, draw 5 G3b0) across all 16
  logs, under the 15 GB `prlimit` and the previous run's 10.6 GB. G3 runs
  peaked at 6.74–7.04 GB. The pilot G3b0 on draw 0 ran 252 s at 6.93 GB. The
  cap was not changed.
- **Degree-4 columns:** 136,698 per G3 branch (N = 43), 102,091 per G6 run and
  for N7. Degree-4 scan time: 193–252 s per G3 branch.
- **Run report** with graph and PDF: `reports/run-20261023T0516Z/`.

## Controls

- **N7 (instrument identity and negative control): valid.** The unspecialised
  m = 3, ℓ = 7 draw 0 was compared programmatically with the committed record
  `research/dreg_closure_scale_20261016/runs/fresh-m3-19-7.d0.jsonl`, on every
  field of both scan entries except timing: degree, `xl_refuted`,
  `closure_refuted`, `saturated`, `xl_rank`, `closure_rank`, `columns`,
  `closure_products` and `closure_word_ops`. All equal. Closure ranks 2027
  and 94230, XL ranks 797 and 16136, columns 7224 and 102091; degrees 3 and 4
  do not refute and saturate. Timings differ (131 s here, 148 s there).
- **G6 (positive control): valid.** Refutes at degree 4 on 4 of 4 draws
  (need at least 3); closure rank 1 on each.

## The measurement

| draw | G6 (6,6,6) | G3 branches refuting at 4 | not refuting | not run (early stop) | three guesses |
|--:|---|--:|--:|--:|---|
| 0 | refutes | 0 | 1 (b0: closure rank 129,221 of 136,698 cols) | 7 | do not suffice |
| 3 | refutes | 0 | 1 (b0: rank 128,213) | 7 | do not suffice |
| 5 | refutes | 0 | 1 (b0: rank 129,893) | 7 | do not suffice |
| 6 | refutes | 8 | 0 | 0 | suffice |

- **Registered verdict (as scored): "Draws: 1 suffice, 3 do not → three
  guesses do not suffice at ℓ = 8".** In the registered wording: three do not
  suffice for this guess set (H_grow; its prediction was at least 3 of 4
  draws failing, and 3 of 4 failed). H_flat is not supported.
- **Draws 0, 3, 5.** The first branch G3b0 (variables 7, 15, 23 all 0)
  saturates degree 4 without the constant `1`; by the registered stop rule the
  other 7 branches of the draw were not run. They are unmeasured, not
  refuting. One non-refuting branch is enough for "do not suffice" on a draw.
- **Draw 6.** All 8 branches refute at degree 4 (closure rank 1).
- **Controls valid** (above), so the verdict stands as scored.

## What it does and does not say

- **Three do not suffice for THIS set.** The set is the top coordinate of each
  summand (variables 7, 15, 23) with no intermediate coordinate. The previous
  study's four-variable set (7, 15, 23, 24) refuted on all 64 branches, so
  four suffice (an upper bound) and three of these do not on 3 of 4 draws.
  Whether the 4th variable (24, an intermediate coordinate) or merely a 4th
  guess is what mattered is not separated here.
- **One cell does not kill the route or fix a growth law.** For this set, three
  suffice on draw 6 only. A different three-variable set may suffice on all
  draws; none was measured. Two or three cells (ℓ = 6: 0, ℓ = 7: about 1, ℓ = 8:
  at most 4) do not determine whether the count stays bounded or grows.
- **Not measured:** the unspecialised ℓ = 8 system, other guess sets, ℓ = 9,
  m = 4.
- **Scope:** m = 3, one cell, four draws, one container (4-core, 16 GB, not
  isolated). Class: stage diagnostic. No S, no rho ratio, no speed claim.

## What this means for ECC2K-130 (derived)

Recomputed, not copied (`C(N, ≤d)` is the sum of binomials up to d):

- **The oracle price.** The guess-then-degree-4 oracle costs
  `2^k · C(N−k, ≤4)` per target; degree 5 costs `C(N, ≤5)`. At the priced
  m = 4, ℓ = 33 cell, N = 2·131 + 4·33 = 394, the ratio
  `C(394, ≤5) / (2^k · C(394−k, ≤4))` is 78.2, 39.5, 19.95, 10.08, 5.09 for
  k = 0..4, 2.57 at k = 5, 1.30 at k = 6 and 0.66 at k = 7. **The hybrid beats
  degree 5 there only if k ≤ 6.**
- **What this study changes: very little.** It does not lower the measured
  upper bound (k ≤ 4 at ℓ = 8, previous study) and shows this particular
  3-set is not an upper bound. The cost model's inputs are unchanged, so no
  row moves. The guess-then-degree-4 route stays open, with its k at ℓ = 8
  between 4 (suffices, measured) and a lower value not yet reached.
- **Against the success criterion.** This is a measurement that narrows what
  is known, not an improvement over the index-calculus baseline and not a row
  beating rho. No S, no speed claim.

## Successor

- **Another three-variable set at ℓ = 8**, for example the 3 variables from
  the four-set that include the intermediate coordinate 24 (such as 7, 15,
  24), to see whether the failure is about the set or about the count. Same
  columns (136,698), same memory (about 7 GB).
- **Draws 0, 3, 5 under other branches of G3** are unnecessary for the
  verdict, but would say how many branches fail.
- **Measurements that fit in 16 GB are now used up for guess counts.** ℓ = 9,
  m = 4 at ℓ = 6, and degree 5 past ℓ = 7 need a larger machine.
