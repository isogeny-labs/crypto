# Skeptic review: pilot 03, cut-variable branching (m=4)

Reviewed 2026-11-14 by an independent session. Everything here is an unverified assertion
[pcc:unverified]; no pcc claim was filed. Scratch (scripts and logs of this review):
`/tmp/claude-0/-home-user/570c81c4-197a-558d-aa76-c9ff505d9731/scratchpad/skeptic-03-cut-variable-branching/`
(`indep.py`, `satchk.py`, `noise.py`, `noise.log`, `rerun_n11_l1.log`).

## Verdict

- The producer's verdict "kill-criterion-met" follows from the criterion as written, on the amended design.
- It is weaker than the headline suggests. It kills one implementation (an n-bit cut driven through
  CaDiCaL's IPASIR-UP `decide()` at n=11, l=1..3), not the idea card's bounded-nogood variant, which was not run.
- The fitted slope rests on a degenerate l=1 point, and the protocol's size, l range and target counts were
  changed after calibration data existed.
- Corrected verdict: kill-criterion-met, narrow scope, low power.

## 1. Instance generator: correct

Checked with my own field arithmetic, curve-point enumeration, point addition and S3 (`indep.py`; no producer
function used), at n=11.

| Check | Result |
|---|---|
| Reachable-u set size for l=1,2,3 | 2, 12, 44; matches the producer's `U_size` |
| All 16 distinct n=11 targets in `results.json` | Label and solver answer agree with my exhaustive scan of the relaxed system; 0 of 80 rows disagree |
| n=7, l=2, all 128 targets | Producer's CNF solved with Glucose4 (a different solver) agrees with my brute force 128/128; 24 SAT; every model satisfies my S3 |
| n=11, l=1 and l=2 non-decomposable targets | UNSAT in Glucose4 as well |
| My own planted targets at n=11, l=3 (sums of four rational points with x in V) | x(R) in {87, 1022, 1826, 1858}, exactly the producer's four "decomposable" targets |
| Planted decomposition fixed by assumptions (x_i, u, v) | SAT on the producer's CNF for all four; a wrong u is UNSAT |
| Free models of those four targets | All lift to curve points whose signed sum has x = x(R) |

Caveats:

- **Decomposable cell is degenerate.** The only rational base x in V at l=3 are 0 (the 2-torsion point) and 6,
  so the four targets are multiples of one point plus 2-torsion. It says little about generic decomposable
  targets.
- **Relaxation is loose.** At l=3, 556 of 2048 x(R) values are relaxed-solvable, but only 7 of the 991
  rational ones are, and 4 are truly decomposable.
- **Amendment 2 is wrong on one point.** It says models are "mostly non-rational"; all 20 SAT rows have
  `lift = lifted`. The README has it right.
- **`validate.py` mislabels a block.** The comment says "planted n=19 l=3"; the code runs n=11.

## 2. Rerun of the smallest cell: reproduced

The producer's rerun command (n=11, l=1, five arms, 4 targets) took 0.3 s. All 20 rows are identical to
`raw_logs/p_n11_l1.log` and to `results.json` on every field except wall time.

A further run of arms D and C with CaDiCaL `seed` 1..5 on the same n=11 targets (`noise.py`):

| l | D median conflicts | C median conflicts | C/D | Producer D / C |
|---|---|---|---|---|
| 1 | 375 | 232.5 | 0.62 | 375 / 233 |
| 2 | 3180 | 6637 | 2.09 | 3180 / 6633 |
| 3 (seeds 1,2; 7 of 8 D runs finished when read) | 85635 | 104244 | 1.22 | 108287 / 101937 |

- Fitted slopes from this run: D 3.92, C 4.40. Cut-first is again not lower.
- The seed option barely perturbs l=1 and l=2 (counts are identical to the producer's), so this is not a
  real noise estimate for those cells.
- At l=3 the default arm moved by about 20% with the seed, which is about the size of the C-versus-D gap there.

## 3. Fairness

**Holds**

- Same CNF, targets and seeds for every arm, with arm order rotated per target (confirmed in `run.py` and
  the rows).
- Seeds follow the stated formula.
- Decomposable and non-decomposable rows are fitted separately.
- All five arms that ran are reported; 92 rows match the logs.
- Nothing is uncounted in the treatment's favour: there is no precomputation, lemma generation or symmetry
  breaking, and cut decisions are inside the conflict counts.
- Both SAT arms are about 2^10 above the 2^(2l) MITM / linear-leaf reference at l=3, and the README says so.

**Problems**

1. **The 3-point fit is an endpoint difference.** With l=1,2,3 the least-squares slope equals
   (y3 - y1)/2. Cut-first's higher slope (4.39 against 4.08) comes entirely from it being cheaper at l=1
   (per-target C/D 0.54 to 0.76), where V={0,1} and only 2 values of u are reachable. At l=3 the arms are
   level (per-target C/D 0.94, 0.95, 1.45, 0.85).
2. **Without l=1 the sign flips.** The l=2 to 3 step is D 5.09 against C 3.94 at n=11, and 6.14 against 3.07
   at n=13 (2 targets). Read naively that is in the "alive" direction. The producer discloses it and
   attributes it to cut-first being 2x to 8x worse at l=2, which the rows support. It is still two points,
   so neither reading is a slope.
3. **The noise floor is the size of the threshold.** The idle-propagator control against default (same
   branching) differs by 0.20 bits/l in fitted slope; my target-resampling bootstrap of that difference spans
   -0.41 to +0.62. For C minus D the bootstrap gives +0.12 to +0.61 (median +0.32) and never reached -0.3
   in 20000 draws. So "not alive" is stable on these 12 targets, but "C is steeper than D" is not established.
4. **The design was changed after data.** n=19 became n=11/13, l=2..5 became 1..3, 8 targets became 4, and
   planted targets were replaced. The kill criterion text is unchanged and the amendments are dated and
   reasoned. Pre-registration cannot be confirmed: there is no git history, and `PROTOCOL.md` was last
   written at 08:20:50, after the l=1 and l=2 rows (08:18:40 and 08:18:51). That is consistent with
   Amendment 2 and does not prove the earlier text was untouched.
5. **Calibration rows are typed by hand.** They are literals in `analyze.py` with no log behind them (one
   has `cap_conflicts=10`), so the facts used to justify dropping n=19 cannot be checked. One of them is
   unfavourable to the treatment and is not in the headline: at n=17, l=3, default finished at 73502
   conflicts and cut-first hit the 150001 cap.
6. **No positive control.** No arm shows this instrument can register a 2 bits/l slope on this encoding.
   Cost at fixed l grows with n, so the l-slope at fixed n=11 may measure mostly the n-bit XOR system, in
   both arms.
7. **The idea was only partly tested.** Not run: a hard learnt-clause cap, a DPLL with a cut-nogood store,
   a 2l-bit cut, branching on v. The producer lists all of these. The "memory dial" is CaDiCaL's reduction
   schedule with no observed database size, so nothing can be said about a time-memory trade.
8. **The baseline is a reference count.** Enumeration, the linear-leaf scan and MITM were not run as arms.
   This does not affect the kill, because the comparison is C against D.

## 4. Does the verdict follow

Yes by the letter: Amendment 1 applies the unchanged criterion to n=11, l=1..3; slope(C) = 4.39 is above
slope(D) - 0.3 = 3.78, with three uncensored sizes in both arms. The rerun and the bootstrap agree.

What the data support is narrower than "cut-first branching is dead":

- On this chained CNF at n=11 and n=13, cut-first on n u-bits through CaDiCaL shows no advantage at the
  largest solved size (C/D between 0.76 and 1.22 at l=3). It is 2x to 8x worse at l=2 and at least 2x worse
  on the single n=17 calibration instance.
- The slope comparison itself has little power: three sizes, one of them degenerate, and an instrument
  noise of about the threshold.
