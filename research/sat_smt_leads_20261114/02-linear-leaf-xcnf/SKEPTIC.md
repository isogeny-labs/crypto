# SKEPTIC review: idea 2, linear-leaf XCNF (2026-11-14)

Independent review of the pilot in this directory. Every number here is a pilot-level
assertion, [pcc:unverified]. Scratch (my scripts and outputs):
`/tmp/claude-0/-home-user/570c81c4-197a-558d-aa76-c9ff505d9731/scratchpad/skeptic-02-linear-leaf-xcnf/`
(`indep.py`, `indep_l3.out`, `cmp.py`, `slopes.py`, `rerun_l3.json`, `rerun.log`).

## Outcome

- Reproduced: yes, on the smallest pre-registered cell (l = 3).
- Stated verdict "inconclusive": not supported. It follows only from the protocol's own
  rule, which replaced the assigned kill criterion with one that depends on a weak,
  censored baseline.
- Corrected verdict: **kill criterion met** for the arms where the solver does any of the
  search (T1, T2, T3). Limits on that kill are in section 4.

## 1. Instance generator and answers (independent check, l = 3)

My own field arithmetic, curve addition, point lifting and brute force, none of the
producer's functions; inputs were the `R` values and models stored in `results.json`.

- All 16 targets are on the curve with x(R) outside V.
- Brute force over all signed triples of factor-base points (4 x-values, 7 signed points,
  repetition allowed): the 8 planted targets decompose, the 8 random ones do not. This
  equals the `decomposable` flag on every row.
- All 144 rows at l = 3 (9 arms x 16 targets): SAT/UNSAT equals brute force; every stored
  SAT model has x1, x2, x3 in V that lift to points with a sign choice summing to R.
  0 mismatches, 0 failed lifts.
- Encoding semantics, checked on 4 targets only (planted idx 0, 1; random idx 8, 9):
  exhaustive search over all 2^23 values of u and all x1, x2, x3 in V for a common zero of
  S3(x1, xR, u) and S3(x2, x3, u) gives the same answer as the point brute force. The
  other 12 targets were not checked this way (time).
- Not checked: l >= 4 answers, the l = 2 cell, and that the seeds regenerate the stored
  `R` (that would need the producer's generator).

## 2. Rerun

`ARMS=B0,T1,T2,T3,T4 python3 pilot.py <scratch>/rerun_l3.json 20261114 300 3` ran in 5.9 s.
All 96 rows (L, B0, T1, T2, T3, T4 x 16 targets) are identical to `results.json` in
status, conflicts, calls, R and model: 0 differing rows. The Glucose arms were not rerun
(not in the rerun command).

## 3. Fairness and bookkeeping

What holds:

- Same instances for every arm: each (l, idx) has one R, one seed and one
  decomposable flag across all arms.
- Decomposable and non-decomposable rows are tabulated and fitted separately. Planted and
  decomposable coincide for l <= 7.
- Every arm in the raw files appears in `summary_tables.md`; the 34 NOT_RUN rows are
  listed; censored cells are excluded from fits and flagged.
- The strongest cheap baseline, the native linear 2+1 scan L (2|F| - 1 leaf solves), is
  reported next to every arm, and T4 is correctly described as that scan with a SAT leaf.
- I recomputed the slopes from `results.json`: B0 2.96 (l = 2..4), 3.84 (l = 3, 4);
  T1 4.63; T2 2.34 (l = 2..7), 3.77 (l = 2..4); T3 3.32; T4 1.23 and 1.00. These match.

Problems:

1. **The kill criterion was changed.** The assigned criterion is absolute: conflicts per
   target stay at or above 0.5 * 2^{(m-1)l}/m!, slope not reduced by at least 0.7 bits
   per l (for m = 3: not below 1.3). PROTOCOL.md replaced it with a slope difference
   against the SAT arm B0, plus an "inconclusive if B0 has fewer than 3 uncensored sizes"
   clause. B0 is weak (121,420 conflicts against 683 tuples at l = 4) and censors at
   l = 5, so the substituted rule could hardly return anything but inconclusive. The
   assigned criterion needs no SAT baseline and is decidable from the T-arm data.
2. **The stated verdict contradicts the protocol once l = 2 is counted.** The l = 2 cell
   was added to give B0 three uncensored sizes. With it, B0, T1 and T2 each have three
   matched uncensored sizes, the inconclusive clause no longer applies, and the protocol's
   rule reads KILL (T1 1.67 above B0, T2 0.81 above B0). The README adds the cell and then
   sets it aside as degenerate.
3. **Deviations were decided after seeing data.** The "validation run" used the same
   deterministic seeds and code as the measured run and already showed B0 censoring at
   l = 5, so it was a first look at the measurements. Its output was not kept. The file
   times (PROTOCOL.md 07:54:35, pilot.py 07:56:15, measured runs starting about 07:59) are
   consistent with a protocol written before the measured runs, but there is no version
   history and I cannot tell whether it predates the validation run.
4. **No symmetrised or WDSat baseline** was run, although the assignment names the
   chained/symmetrised encoding as the baseline. The README says so.
5. **n is fixed at 23 for every l.** The solver always carries 23 free bits of u, so the
   SAT arms' level at small l is dominated by u and their slope in l is not the slope of
   a matched instance family (n near 3l). This affects all SAT arms and both criteria.
6. **Thin cells at the top.** T2 at l = 7 has 2 targets; T3 and T4 at l = 7 and 8 have 1
   each. The T3 and T4 fits over l = 2..8 lean on single targets.
7. Small errors: T2 on l = 3..7 fits 1.77, not the reported 1.80. "One Gauss matrix in
   every run" is wrong for 2 T1 rows at l = 4 (2 and 3 matrices). PROTOCOL.md says
   pycryptosat 5.11.x; the installed version is 5.18.0.
8. Not counted in any arm's work: encoding construction, and for T3/T4 a fresh solver per
   leaf. This favours the treatments, not the baseline, and does not change the direction.

## 4. Does the verdict follow from the kill criterion?

Against the assigned criterion (reference line 0.5 * 2^{2l}/6, slope 2 bits per l,
required reduction 0.7), non-decomposable targets, mean work, 8 targets per cell unless
noted:

| arm | slope (sizes) | reduction from 2.0 | work / (0.5 * 2^{2l}/6) |
|---|---|---|---|
| T1 free, (s,p) encoding | 4.63 (2..4) | none | above at every size |
| T2 assumptions on x1 | 1.67 (3..6), 1.65 (4..6), 1.77 (3..7, l = 7 has 2 targets), 2.34 (2..7) | 0.35 at best | 1,040x to 2,010x for l = 3..7 |
| T3 x1 substituted | 3.26 (2..6), 3.32 (2..8) | none | 13x at l = 3 rising to 239x at l = 6 |
| T4 x1 and u substituted | 1.00 (2..6) | 1.0 | below, but it is the native scan by construction |

No arm in which the solver performs part of the search gets its slope down by 0.7, and
all of them sit one to three orders of magnitude above the 0.5 * 2^{2l}/6 line at every
size. T2 has four full cells (l = 3..6) and T3 five, so the three-size requirement is
met without B0. The kill condition is met. The only arm at 2^l is T4, where an outer
Python loop enumerates x1 and both roots u; the protocol itself says T4 does not decide
the verdict.

The producer's prose conclusion (levels point against the idea, only the scan-by-
construction arm reaches 2^l) is right. The label "inconclusive" understates it.

What the kill does not cover:

- Decision restriction was only emulated (assumptions, outer loop). A solver with real
  decision-variable control, or branching on x1 plus the root choice of u with XOR
  reasoning through the substituted ANDs (WDSat style), was not tested.
- One solver (CryptoMiniSat, default options), m = 3 only, n fixed at 23, l <= 7.
- T2's best window is 0.35 below slope 2. That is short of 0.7, not zero, and the
  margin to the bar (about 0.35 to 0.5 bits per l) rests on 8 targets per cell.

## 5. Not done in this review

- Sizes above l = 3 were not rerun; l = 4..8 numbers are taken from `results.json`.
- Glucose arms were not rerun.
- The full algebraic brute force over u covered 4 of 16 targets at l = 3.
- No `docs/session-logs/` entry was written: this review was confined to this one file.
