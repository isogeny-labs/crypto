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
  logs, under the 15 GB `prlimit`. G3 runs peaked at 6.74–7.04 GB. The
  previous study's 10.6 GB was a single outlier (draw 5, branch 5) among 68
  runs whose median was 7.0 GB, so 136,698 columns fit comfortably. The cap
  was not changed.
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
- **What the registered result changes.** It does not lower the measured upper
  bound (k ≤ 4 at ℓ = 8, previous study), and it shows this 3-set is not an
  upper bound. No row moves.
- **Interpretation (not registered; derived from committed records).**
  - **G3 is structurally an ℓ = 7 system.** Fixing the top coordinate of each
    summand leaves every summand in a 7-dimensional subspace or coset, with
    N = 43 unknowns. The unspecialised ℓ = 7 draws of the closure-at-scale and
    closure-driver studies (N = 40 and 42) did not refute at degree 4 on 8 of
    8 draws. G3 not refuting on draws 0, 3 and 5 is therefore what the
    "ℓ = 7 stops refuting" result predicts, not a separate surprise.
  - **The pattern fits the rule `m·(ℓ−7)+1`.** It is the pre-registered
    "K_marginal" rule of the previous study: `m·(ℓ−7)` guesses bring every
    summand down to ℓ = 7, plus about one more (the single-fix result at
    ℓ = 7). At ℓ = 8, m = 3 it gives 4. Four suffice (64 of 64 branches) and
    the 3-set fails on 3 of 4 draws. The rule `m·(ℓ−6)` (6) was already
    rejected by the previous study.
  - **Draw 6 is an exception:** all 8 branches of the 3-set refute, so the
    count varies by draw.
  - **What it would mean at the priced cell.** Extended linearly, the rule
    gives 4·(33−7)+1 = 105 guesses at m = 4, ℓ = 33, against the break-even
    k ≤ 6 with degree 5. That is an extrapolation from two values of ℓ
    (ℓ = 7: about 1 guess on 5 of 8 draws; ℓ = 8: 4 suffice, 3 do not for this
    set on 3 of 4 draws), not a result. It is the first evidence leaning
    toward a guess count that grows with ℓ.
  - **What would test it.** The matched null: unspecialised (22, 7) draws
    (N = 43, 136,698 columns, the same matrix size as G3) should not refute on
    most draws if G3 is "just ℓ = 7", and a 3-set that leaves one summand at
    dimension 8 (for example 7, 15, 24) should fail on most draws.
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
- **The matched null and the shape arms** described above. They fit in this
  container: 136,698 columns at about 7 GB. The unspecialised ℓ = 8 system
  (179,447 columns) and k = 1, 2 (164,221 and 149,986 columns) may fit too,
  since peak memory was 7 GB at 136,698; that is untested.
- **ℓ = 9 and degree 5 past ℓ = 7** still need a larger machine. m = 4 at
  ℓ = 6 is no longer open: another session measured it on a large host
  (`research/dreg_closure_m4l6_20261018/RESULTS.md`, refutes at degree 4 on 2 of 2
  unsatisfiable draws), which refutes the "m·ℓ ≥ 21" explanation and leaves
  "ℓ = 7". That makes the structural reading above (G3 is an ℓ = 7 system)
  the one the data support.
