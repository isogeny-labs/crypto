# Skeptic review: Lead D, settling run for target-parametric incremental SAT (idea 8)

2026-11-15. Independent review, one process at a time under `nice`, about 20 minutes. Everything
here is a reviewer assertion [pcc:unverified]: no pcc claim, no replay certificate, no matched rho
arm. Reviewer scripts and logs (scratch, not in the repository):
`/tmp/claude-0/-home-user/570c81c4-197a-558d-aa76-c9ff505d9731/scratchpad/skeptic-D/`
(`verify.py`, `verify.log`, `stats.py`, `stats.log`, `asm.py`, `rerun.log`). No producer file was
edited; the rerun ran from a scratch copy.

## Verdict

- **Reproduced: yes.** The small-cell rerun is bit-identical in every counter, and every headline
  number recomputes from the raw rows.
- **Under the pre-written rule: INCONCLUSIVE follows.** No interval I can build has its upper
  bound below 0.9 or its lower bound above 1.0.
- **Plain reading: the idea is a practical negative at this scale.** There is a real but small
  saving in a counter that under-prices the incremental arm (about 5% fewer search propagations,
  2% to 8% across the three V), it has stopped moving by position 444, and it comes with a
  solver-time penalty in every seed at both dimensions (1.4x to 3.0x). "Inconclusive" is the
  rule's word; the data do not leave the question of usefulness open at l = 4, 5.
- The producer's write-up is honest about most limits. It is weaker than stated in three places:
  the "95%" interval, the power-law exponent, and the comparability of the propagation counter.

## 1. Labels and the rationality clauses (own arithmetic)

Own F_2^n arithmetic (shift-xor multiply, Euclid inverse, a z^2+z lookup table instead of the
half-trace, own affine addition with an on-curve assertion after every operation). The modulus in
each meta row passes my irreducibility test.

| check | result |
|---|---|
| Labels of all 6,000 listed targets (l=4 and l=5, three V each) against my own triple-sum table | 0 mismatches; all targets rational, distinct, outside V |
| Meta fields: signed base size, non-liftable x in V, exact decomposable abscissae | all equal to my values (e.g. l=5: 3,205 / 2,197 / 1,827) |
| l=3, n=11, three producer V, exhaustive (1,052 / 1,050 / 1,054 abscissae): parametric CNF with rationality clauses vs my truth | 0 mismatches, 0 models that fail my lift |
| Same, CNF without rationality clauses | 0 mismatches, 0 verdicts changed |
| l=3 constant-target CNF, every decomposable and every 4th other (1,029 solves) | 0 mismatches |
| l=4 seed 0, all 1,000 measured targets, parametric CNF vs my truth | 0 mismatches; 15 SAT, all models lift to points summing to the target |
| l=4 seed 0, CNF without rationality clauses, planted + first 200 (210 solves) | 0 mismatches |
| l=4 seed 0, constant-target CNF, planted + every 10th (113 solves) | 0 mismatches |
| Non-liftable pairs x, 1/x in the nine V used | 0 in all six measured V (own count) |

**Can the rationality clauses change a label?** Labels come from real points, so no. They can
change a CNF verdict, and only in one case, which I confirmed with a positive control the producer
did not have. On this curve x is non-liftable exactly when 1/x is, so a V that contains both x and
1/x admits the spurious solution P1 + P2 = (0,1) from two non-rational summands. I built three
such V at l=3 (e.g. V = [665, 1958, 1943], target x = 271): without the clauses the CNF says SAT on
a truly non-decomposable target (2, 3 and 2 wrong verdicts out of about 1,055); with the clauses 0
wrong verdicts in all three. So the clauses are necessary in general, sound here, and changed no
verdict in the measured cells. Not checked by me: an independent SAT check of the l=5 formulas
(only the labels, and the producer's rows report 0 verdict errors against those labels).

## 2. Rerun of the small cell

`rerun_small.sh` from a scratch copy: l=4, seed 0, all arms, 1,000 targets, 57 s. 3,301 rows;
conflicts, decisions and propagations identical to `raw_l4_s0_ALL.jsonl` in every row (my own
field-by-field comparison, 0 differing rows). `results.json` `raw_rows` equals the twelve raw files
(14,531 rows, 0 error rows). Fresh-arm rows match the schedule recomputed from positions alone.
l=5 was not rerun (about 50 minutes).

## 3. Recomputation from raw rows

l=5, last third (idx 444..666), non-decomposable, propagations INC/SPEC:

| quantity | producer | recomputed |
|---|---|---|
| pooled geometric mean | 0.951 | 0.9507 (n = 216, 213, 211) |
| per seed | 0.976, 0.952, 0.924 | 0.9764, 0.9516, 0.9241 |
| two-stage seed-level bootstrap | 0.925..0.976 | 0.925..0.976 |
| plain range across seeds | n/a | 0.924..0.976 |
| target-level interval within each seed | n/a | 0.969..0.984, 0.944..0.959, 0.917..0.932 |
| target-level, seeds held fixed, pooled | n/a | 0.946..0.955 |
| t interval on the three seed means (2 d.f.) | n/a | 0.888..1.018 |
| sum INC / sum SPEC (total work, not GM) | n/a | 0.956 |
| conflicts | 0.601 | 0.601 (per seed 0.637, 0.600, 0.568) |
| decisions | not reported | 0.405 |
| solver seconds | 2.29 | 2.29 (per seed 2.46, 2.98, 1.64) |
| A/A (AA/SPEC) | 0.998 (0.992..1.005) | 0.998 (0.992..1.005) |
| INC/FP | 0.465 | 0.465 (0.473, 0.472, 0.450) |
| FP/SPEC | 2.04 | 2.036 (2.057, 2.011, 2.040) |
| l=4 INC/SPEC | 0.879 | 0.879 (0.684, 1.058, 0.937) |

**Is the seed-level bootstrap meaningful with 3 seeds?** Not as a 95% interval. It returns the
range of the three seed means (0.925..0.976 against a range of 0.924..0.976): with ten seed
multisets it cannot do anything else. It says nothing about a fourth V. The honest statements are:
(a) within each of these three V the ratio is below 1 beyond target noise and beyond the A/A floor
(worst seed 0.969..0.984); (b) between V it spans 0.924..0.976; (c) a t interval on three seed
means is 0.888..1.018, which contains 1.0. The producer flags the coarseness in the protocol and in
the README, but still prints "95%".

**Power law.** Reproduced: 1.160 p^-0.0323, crossing 1.0 at p = 99 and 0.5 at 2.0e11. It is not a
good description of the data and the exponent is overstated:

- R^2 = 0.16. The slope is carried by the first ten positions (binned ratio 2.27 at position 0,
  1.84 at 1..2, 1.14 at 3..9, then 1.03).
- Fits that start later give beta = 0.021 (idx >= 10), 0.019 (>= 30), 0.020 (>= 100), 0.021
  (>= 222): about 0.02, not 0.032, with the 0.5 crossing at 1e16 to 1e17. Fits on early windows
  give 0.087 (idx < 100) and 0.055 (idx < 222). A single exponent does not exist in these data;
  it falls with position.
- The two halves of the last third are equal: 0.952 (idx 444..554) and 0.950 (555..666). Binned:
  0.964, 0.964, 0.956, 0.960, 0.943, 0.950, 0.950 from idx 222 on.
- Seed 2 ran INC to 1,000 targets: geometric-mean INC propagations 1.778M (444..666), 1.791M
  (667..733), 1.767M (734..999). No further fall in absolute INC cost over the 333 unanalysed
  positions (no SPEC rows there, so this is INC alone).
- Every fit variant puts the ratio at the last available abscissa (p about 63,000) at 0.81 to
  0.87. "Never reaches 0.5 inside the cell" holds for every variant; "crosses 1.0 near 100" is
  about right from the bins (0.996 at 30..59, 0.988 at 60..99).

The quoted crossing interval (1.4e9..4.1e15) is a seed-level bootstrap of an extrapolation over
eight orders of magnitude beyond the data; it carries no information beyond "far outside the cell".

**Subsampling of the fresh arms.** Not a source of bias for the primary: SPEC solved every
non-decomposable target of the last third, so the primary uses no subsample. FP and AA use every
second non-decomposable last-third position, fixed by position; INC/SPEC on that half against the
other half is 0.973/0.980, 0.950/0.954, 0.919/0.929 (within target noise, same direction in all
three seeds, at most 1%). The fit has double point density in the last third; thinning it to equal
density gives beta 0.035, so this does not matter.

**Is the propagation counter comparable between arms?** Partly, and where it is not, the bias
favours INC.

- Assumptions: black-box test on CaDiCaL 1.5.3 through python-sat. A 5,000-literal chain implied
  by one assumption is re-counted on every `solve` call (5,003 propagations per call), so INC and
  FP count the assumption cone once per target and SPEC does not (its constants are folded at build
  time). It is not re-counted on restarts: with 1,303 restarts and 43,567 conflicts the chain was
  counted 3 times. The cone is 391 variables against about 2 million propagations per target, so
  this effect is below 0.1% and does not explain anything.
- Formula size: the parametric formula has 8,965 variables and 34,607 clauses, the specialised one
  4,393 and 17,170 (2.04x and 2.02x). Propagations count assigned literals, so the same search
  step costs more counted propagations on the larger formula: median propagations per conflict are
  1,787 (SPEC), 3,055 (FP), 2,828 (INC).
- Cost per propagation is not equal: median solver time per counted propagation at l=5 is 120 ns
  (SPEC), 155 ns (FP), 302 ns (INC). At l=5 the arms ran in different processes on a loaded
  machine, but l=4 ran in one process and shows the same thing: INC/FP is 0.384 in propagations and
  0.664 in seconds, INC/SPEC 0.879 and 1.84. INC pays about twice as much time per counted
  propagation as a fresh solver on the same formula. Plausible causes, not separated by me: a
  larger retained clause database and longer watch lists, and inprocessing (vivification, probing)
  that an incremental solver reaches because its conflict count accumulates and whose propagations
  this counter may not include. I could not read CaDiCaL's per-phase counters through python-sat,
  so the size of the uncounted part is unknown.

Consequence: the primary metric is a count in which INC's work is under-priced relative to SPEC.
A 5% saving in that count is not evidence of a 5% saving in work.

## 4. The rule, the thresholds, and what the data support

INCONCLUSIVE follows from the rule as written: upper bound 0.976 (or 1.018 with the t interval,
0.984 for the worst seed) is not below 0.9; no lower bound is above 1.0.

The thresholds were not well chosen, and the producer's own prediction shows why:

- KILL required INC to be *worse* than SPEC in propagations. An idea that is 1% better in an
  under-priced counter and several times slower can never be killed by this rule.
- ALIVE asked for only a 10% propagation saving, with no condition on time or on a common cost
  unit, and no condition on the trend.
- The band between (0.9..1.0) is exactly where a small, real, useless effect lands. The rule had no
  outcome for "real but too small to matter", which is what was measured.
- The rule's interval is a three-seed bootstrap, so its width is the seed range.

What the data support, stated plainly:

1. Carrying solver state across targets does reduce search: 40% fewer conflicts and 60% fewer
   decisions than SPEC, consistently in all three V, well outside the A/A floor. That is a real
   effect.
2. It converts to about 5% fewer counted propagations (2% to 8% by V), because each conflict costs
   1.6x more propagations on the parametric formula.
3. It does not convert to time: INC is slower than SPEC in every seed at both dimensions (l=5:
   1.6x to 3.0x, cross-process; l=4: 1.4x to 2.3x, same process), and on 99% of l=5 targets.
4. The saving is not growing at a useful rate: flat across the last third, flat in INC's own cost
   to position 1,000 in the one seed that got there, and every fitted law stays above 0.8 at the
   end of the cell.
5. From l=4 to l=5 the carried-state benefit weakens (INC/FP 0.38 to 0.47; exponent about 3x
   smaller), confounded with n and with base size as the producer says.

So: as a route to a material saving in point-decomposition cost, idea 8 is not supported at
l = 4, 5, m = 3, with this encoding and solver. I would record it as "negative in practice,
INCONCLUSIVE by the pre-written rule", and not carry "inconclusive" forward as if the question
were open. Limits of that statement: three V per l, one curve, one solver, default options, l <= 5.

**Is "the benefit is state carried on the parametric formula, which itself costs 2x SPEC" correct?**
As arithmetic in propagation units, yes: 0.465 x 2.036 = 0.947, against 0.951 measured on the full
set. As a mechanism statement it needs three corrections:

- FP is not INC with learnt clauses dropped. It also drops the inprocessed formula, variable
  scores, phases and scheduler state. INC/FP bounds everything carried, as the producer says.
- "Costs 2x" is unit-dependent. FP/SPEC is 2.04 in propagations, 1.19 in conflicts and 2.65 in
  seconds. The 2.04 matches the size ratio of the two formulas (2.04x variables); the search
  itself is only 19% longer. The specialised formula is cheaper mainly because constants are
  folded away, not because the parametric search is twice as hard.
- In time the carried state buys far less than half: INC/FP is 0.86 in seconds at l=5 (0.62 to
  1.11 by seed, cross-process) and 0.66 at l=4 (same process). That cannot repay a 2.65x formula
  penalty.

Also relevant: INC/FP is already 0.488 in the first third. Most of the carried benefit is in place
within tens of targets and then barely grows. That is the signature of a one-off amortised gain,
not of knowledge accumulating over a long target sequence, which was the claim under test.

## 5. The fix to analyze.py after the data

`PREREG.sha256` records `a4163730…` for `analyze.py`; the file now hashes `4396ce4d…`. No earlier
copy exists and there is no version control, so I reconstructed it: replacing
`str(k).startswith('_')` with `k.startswith('_')` on line 116 (function `clean`) reproduces the
pre-registered hash exactly. That one expression is the whole change.

`clean` only strips private keys and converts infinities while writing `analysis.json` and
`results.json`; it runs after every statistic is computed. Before the fix it raised on the integer
keys of `nstar`, so the pre-registered file could not produce any output on this data. The fix
cannot move the primary number or any other statistic. The producer's description ("a JSON key
type error, no change to any statistic") is accurate. All other pre-registered files still match
their hashes.

`analyze.py` was hashed at 07:35:11Z, about a minute after the first INC process started and
before any SPEC row existed (first fresh-arm file written 08:04), so it could not have been tuned
on a ratio. This rests on file times only.

## Problems, in order of weight

1. The primary metric (search propagations) is not a common cost unit across arms: INC pays about
   2x the time per counted propagation, in-process at l=4 as well. The 5% saving favours INC by
   construction of the unit.
2. The "95%" interval is the range of three seed means. With a t interval the primary spans
   0.888..1.018.
3. The decision rule could not return a negative for a small real effect; INCONCLUSIVE here hides
   a practical negative.
4. The power law is a poor fit (R^2 0.16), its exponent depends on the window (0.02 to 0.09), and
   the headline 0.032 is inflated by the first ten positions. The crossing interval is not
   informative.
5. The mechanism sentence treats FP as INC-DROP and treats 2x as a property of the parametric
   search; it is mostly formula size.
6. Sequence length 667 instead of 1,000 at l=5 (declared). The primary window is therefore
   positions 444..666, not 667..999. The one seed with INC to 1,000 shows no further change.
7. Encoding changed from the previous study (rationality clauses, about 6x cheaper instances), so
   this run does not settle the earlier run's numbers; it replaces them (declared).
8. Wall time at l=5 is cross-process on a shared machine (declared); the sign is confirmed in the
   same-process l=4 cell.
9. Cosmetic: the `A` meta rows carry `last_third_from: 666` (for 1,000 targets) while the analysis
   uses 444; the l-trend uses bootstrap seeds 1510 and 1511 where the protocol names 1510
   (declared).
10. Repository obligations not met by the pilot (declared): no Conductor task, knowledge-base
    receipt, benchmark record, validation gate or PDF. Nothing here may reach a leaderboard.

## Not completed (treat as weaker than stated)

- No rerun of any l=5 process; l=5 counters are taken from the stored rows.
- No independent SAT check of the l=5 formulas, only of the labels.
- CaDiCaL counter semantics were probed black-box, not read from source; the share of INC's work
  in uncounted inprocessing is not measured.
- No isolated-core timing.
