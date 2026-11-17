# SKEPTIC review: lead A, dual-rail encoding at m = 4 (2026-11-15)

Independent review, one `nice` process at a time, about 20 minutes. Every number here is
`[pcc:unverified]`. Producer files were not edited. Scratch (not in the repository):
`/tmp/claude-0/-home-user/570c81c4-197a-558d-aa76-c9ff505d9731/scratchpad/skeptic-A/`
(`own.py` own arithmetic and model enumeration, `probe.py` leaf-cost probe, reruns).

## Verdict

- **Reproduced: partly.** l = 2 and l = 3 rerun and match `raw/` row for row (168 + 168 rows,
  every field except wall time). l = 4, 5, 6 were not rerun; their numbers are taken from
  `results.json` and are unchecked.
- **KILL is supported** under the rule as written. It is a KILL of "this encoding beats the
  pair scan count at l <= 6", nothing wider.
- **Weaker than stated:** the answer to question 1 ("warm: yes, O(1) in the mean") and the
  reading of the budget-0 trend. Details in sections 3 and 4.

## 1. Labels, infinity, repeated points (own arithmetic)

Own field (irreducibility by trial division), own curve addition, own lift by exhaustive y.
The pilot's code was used only to build the CNF; models were enumerated with direct
pycryptosat calls and blocking clauses on (x1, x2, z, s), for `dual`, `plain` and `dual2`.

| check | l = 2 (B = 380) | l = 3 (B = 6194) |
|---|---|---|
| pilot's 12 targets: on curve, pass the filter, label equals own truth | 12 of 12 | 12 of 12 |
| further targets | every curve point with x != 0 (496) | 150 random points |
| CNF solution set, projected on (x1 <= x2, {x3, x4}), equals own set of decompositions | 508 of 508, all three encodings | 162 of 162, all three encodings |
| models lifted to four signed curve points summing to R | 455 of 455 per encoding | 87 of 87 per encoding |
| planted targets SAT | 4 of 4 | 4 of 4 |

How the ground truth treats the edge cases, and whether the CNF agrees:

- **Through infinity or opposite points** (P1 + P2 = O, or any pair Pi = -Pj): not counted.
  `decomposable` requires both pair sums in S2 without O. For targets that pass the filter
  (R not in S2, x(R) not in V) such tuples cannot exist; own count over the 24 pilot targets:
  0. The CNF cannot represent them either. Agreement. On the 24 filter-excluded points at
  l = 2 the CNF still equals the own "no O pair sum" set, and no point had a
  through-infinity-only decomposition.
- **Repeated points** (Pi = Pj, a doubling): counted as decomposable, and the CNF represents
  them (d = 0 gives u = x(2P1)). At l = 2 all 4 planted targets and 66 of the 472 valid
  targets decompose only with a repeated point (3 base points, 4 summands); at l = 3, 2 of 4
  planted. The task statement does not say whether repeats are allowed; the pilot allows them.
- **Sums of three base points**: not treated as decomposable unless also a sum of four.
- **x = 0 intermediate (the old defect)**: the producer tested no such cell. Tested here on
  cells the pilot rejects (sqrt(B) in V.V, 2-torsion point in S2). l = 2, B = 4: CNF UNSAT
  while a decomposition exists, for 4 points, all excluded by the target filter. l = 2,
  B = 20: 6 points (2 valid targets) where the CNF misses solutions through the 2-torsion
  point; SAT/UNSAT still correct. l = 3, B = 4: no difference in 150 samples. No false SAT
  seen. So with the clauses u != 0, v != 0 the encoding is incomplete on such curves, and the
  restriction on B is needed.

## 2. Rerun

`python3 dr4_run.py <scratch>/dr4_l2.jsonl 300 2` (0.9 s) and the same for l = 3 (9.8 s), in
a scratch copy: 168 of 168 rows identical to `raw/l2.jsonl`, 168 of 168 to `raw/l3.jsonl`,
ignoring `time`. `analyze.py` on them gives the l = 2 and l = 3 columns of `tables.md`.

## 3. Fairness and integrity

**Reference count.** 2^(2l)/2 counts unordered pairs over all of V, x = 0 and x1 = x2
included (2 048 at l = 6). The DFS and the free arms do use the x1 <-> x2 symmetry
(`driver_skip`, lex clauses), so there is no 2x penalty; the DFS visits 2 016 leaves.
Two things the README gets wrong or leaves out:

- The floor of the budget-0 ratio is 1 - 2^-l (0.984 at l = 6), not 1 as the README says.
- "Generous to the SAT arms by up to about 4x" holds only if one scan leaf handles a whole
  x-pair. A SAT leaf covers four sign branches (two roots u, two roots v); the scan needs one
  linear solve per branch. In linear solves the scan over liftable pairs costs 2N(N+1), which
  is 24, 60, 220, 480, 2 112 for l = 2..6 against 8, 32, 128, 512, 2 048. The reference is
  therefore about right at l = 5, 6 and too small by 1.7x to 3x at l = 2..4.

Budget-0 arm (leaves + conflicts: 63.5, 141, 522, 2 027, 7 786) under other references:

| reference | l=2 | l=3 | l=4 | l=5 | l=6 |
|---|---|---|---|---|---|
| 2^(2l)/2 (pre-registered) | 7.94 | 4.41 | 4.08 | 3.96 | 3.80 |
| 4 linear solves per liftable x-pair, 2N(N+1) | 2.65 | 2.35 | 2.37 | 4.22 | 3.69 |
| one unit per liftable x-pair, N(N+1)/2 | 10.6 | 9.4 | 9.5 | 16.9 | 14.7 |
| 4 solves per pair, no lift test (unreasonable scan) | 2.65 | 1.26 | 1.09 | 1.02 | 0.97 |

The KILL holds for every reasonable reference. Only the last row, a scan that skips a
one-trace lift test, puts the budget-0 arm at parity, and that arm is the scan itself.

**Factor-base density confound.** The share of V \ {0} that lifts is 3/3, 5/7, 10/15, 15/31,
32/63. Own probe (warm solver, every leaf): leaves where both x lift take 81 % to 96 % of all
conflicts (8 to 15 conflicts each at l = 3, 4, 6); leaves where one lifts cost 0.4 to 0.8. Part
of the fall from 7.94 to 3.80 is the base thinning from 100 % to 50 %, not the solver
improving. Rows 2 and 3 of the table above do not fall.

**Leaves + conflicts as a unit.** A convention, as the producer says. Two additions.
Internal-node solver calls are not counted, only leaf calls; counting them moves l = 6 from
11.0 to 11.6 (B3), 4.95 to 5.28 (B10), 5.90 to 5.96 (B100). This favours the SAT arms and does
not touch the decision. Conflict counts come from the wrapper's parse of the verbose line and
were not cross-checked here either.

**Restriction on B.** V.V is 6 of 512 field elements at l = 2 and 22 of 8 192 at l = 3; the
rejected share falls with l. Replaying the seeds: no draw was rejected for sqrt(B) in V.V at
any size. One draw was rejected at l = 2 by the base-size amendment (empty base); none at
l = 3..6. The measured cells are the first draws for l >= 3. The distribution is not
materially changed. One curve per size remains the real limit.

**Rows.** `results.json` equals the concatenation of `raw/*.jsonl` (760 rows, all OK, no
duplicate (l, arm, target)). Every cell has 8 non-decomposable and 4 planted targets, one
(seed, B, polynomial, base size) per l, the same R per target index across arms, and
sat == label on every uncensored row. Arm sets are complete per the protocol (18, 18, 16, 14,
14 arms). The kills themselves cannot be checked: no logs; file times (l6 slices 08:04 to
08:22, `dr4_run.py` edited 07:58) are consistent with the account. Protocol-before-data rests
on a one-minute gap in file times (PROTOCOL.md 07:35, `raw/l2.jsonl` 07:36).

**Cap change at l = 6.** Free dual: 2 runs capped at 100x, 6 at 20x. Free plain: 8 capped at
20x. Each run is a lower bound of at least 20, so ">= 1.0" holds for every run and the
decision is unaffected. The quoted means (>= 40, >= 20) mix caps and carry no other
information. Free dual2 finished on all 8 (9.73 to 15.1), under both caps. Capped-run counts
in the README (5, 6, 7, 8 of 8) match the rows.

## 4. Does KILL follow, and extrapolation

Rule: KILL if every keep, core and free arm is >= 1.0 at l = 6. All 12 arms ran on 8 targets;
the lowest mean is 3.80; the lowest single run is 2.51 (keep-B3, target 3). Capped arms count
as their lower bounds, all >= 20. **KILL follows.**

Budget-0 trend. The ratio is (1 + c)(1 - 2^-l) with c = conflicts per leaf = 9.58, 4.04,
3.35, 3.09, 2.86.

- Slope of log2(ratio): -0.051 bits per l over l = 4..6, -0.068 over l = 3..6. Taken
  literally it reaches 1.0 at l of about 44 (n = 177) or 34. Not a foreseeable size.
- It cannot go below 1 - 2^-l in any case: the arm solves every leaf.
- Successive drops of c are 0.69, 0.26, 0.23. A geometric fit on l = 4..6 gives a limit of c
  near 1.6 (ratio near 2.6); on l = 3..5 it gives c near 2.9. The fit is not stable.
- Spread across the 8 targets is 0.36 to 1.38 per cell (standard error 0.13 to 0.49), so the
  4.08, 3.96, 3.80 steps are inside the noise, on one curve per size.
- With the density-corrected reference (section 3) the series is 2.37, 4.22, 3.69: not falling.

Reading: convergence to a constant above 1, like the m = 3 case. "Still falling" is not
supported. Budget 100 (slope -0.37) rose from l = 5 to l = 6 and one of its targets sits at
13.7. Free dual2 is rising (+0.22). No arm has a trend that reaches 1.0 at a foreseeable l.

## 5. Problems

1. **Question 1 is weaker than stated.** "Warm: O(1), flat in l" depends on the order of the
   assumption literals. Own probe, same warm solver, same leaves in lex order, but assumptions
   passed LSB first (`S.assump`): 4.83, 4.17, 32.0, 4.97 conflicts per leaf at l = 3..6,
   against the producer's 4.04, 3.35, 3.09, 2.86 (l = 6 probe on 3 targets only). The l = 5
   mean is ten times higher. The producer's own core arm uses that order and shows the same
   blow-up at l = 5 (>= 70). The mean is driven by rare leaves of about 2 000 conflicts.
2. The fall of the budget-0 ratio is partly a factor-base density artifact; the README reads
   it as flat, which is right, but the stated 7.94 to 3.80 series should not be read as a
   trend.
3. The README's "floor is 1 by construction" and "baseline generous by up to about 4x" are
   both inaccurate (section 3). Neither changes the decision.
4. Internal-node solver calls are uncounted in the metric.
5. One curve, one seed, one V per size; l = 2 is degenerate (every decomposition repeats a
   point).
6. Not checked here: l = 4, 5, 6 reruns, the conflict-count parse, the external kills, the
   protocol's time order. Treat those as unverified.
7. The old x = 0 defect was not exercised by the producer; on rejected curves the encoding
   loses solutions (section 1). The fix by choice of B is confirmed as needed and as harmless
   to the instance distribution.
