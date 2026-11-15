# SKEPTIC review: subcube refutation pilot (2026-11-14)

Independent review, about 25 minutes, one core. Every number here is `[pcc:unverified]`.
Scratch scripts and outputs (not in the repository):
`/tmp/claude-0/-home-user/570c81c4-197a-558d-aa76-c9ff505d9731/scratchpad/skeptic-fsub/`
(`own.py`, `check1.py`, `check_twist.py`, `claimA.py`, `claimA2.py`, `rows.py`, `probe5.py`,
`probe5.jsonl`, `l4.jsonl`).

## Verdict

| Item | Verdict |
|---|---|
| Reproduction | Yes for l = 4 (bit-identical). Keep arms at l = 5..8 re-measured with my own DFS loop and match. Discard, alpha and plain arms at l >= 5 not rerun. |
| Claim A, the numbers | Holds. Not an artefact of solver options. "1.6 per leaf" is the warm incremental figure only; a fresh solver needs 18 to 30. |
| Claim A, the diagnosis | Partly. The mechanism (the equivalence must be in the XOR set) is confirmed. The attribution to the `c_i & u_j` gates is not: fixing those alone leaves the cost and its growth. |
| Claim B, KILL | Follows from the rule as written. The rho leg is solid at l <= 8. The alpha leg is weak (l = 4 only, cold solver, 2 of 8 targets below 0.9). |
| "Every subcube-attempting arm is worse than budget 0" | False outside the producer's budget grid: budget 3 has lower rho than budget 0 at l = 5..8. True in conflicts only. |
| Protocol before data | Not verifiable. No git history; `PROTOCOL.md` and `run.py` carry the same mtime as the start of the first measured run. |

## 1. Arithmetic and ground truth (own code)

Own field multiplication, inversion, brute-force y search (no half-trace) and addition law.

- Field polynomial irreducible, factor base equal to the producer's (l = 3: 4 signed points;
  l = 4: 18).
- 102 targets (l = 3: 30 own; l = 4: the producer's 12 plus 60 own random or planted). My
  enumeration of all signed triples agrees with the producer's `decomposable` flag on all 12
  producer targets, and every planted target decomposes.
- CMS free solve (plain and dual) agrees with my enumeration on all 102 targets. All 110 SAT
  models lift, with my arithmetic, to points with x in V \ {0} summing to R, and the model's u
  equals x(P2 + P3).
- 1 152 per-x1 leaf answers (dual, incremental) equal my enumeration of decompositions using
  that x1.
- Infinity: targets have x(R) outside V, so no summand pair cancels and R - P1 is never
  infinity. Checked, sound.
- **x = 0 is not fully handled.** The clauses exclude x_i = 0 but not u = 0. When
  R = +-P1 + T (T the 2-torsion point), S3(x2, x3, 0) = 0 reduces to x2 x3 = sqrt(B), which
  non-rational (twist) x2, x3 in V also satisfy. I built three such instances at l = 4 (for
  example B = 1281, R = (4117, 4420)): brute force says non-decomposable, CMS says SAT with
  x2, x3 that do not lift. The measured cells are unaffected, since `run.py` asserts
  `sat == isdec` and the lift, and the event has probability about 2^(3l - 2n) per target.
  The README's "x = 0 is excluded on both sides" is too strong.

## 2. Rerun

`python3 run.py <tmp>/l4.jsonl 540 4 && python3 analyze.py <tmp>/l4.jsonl`: 336 rows, all
fields except `time` identical to `raw/l4.jsonl`, and the analysis output identical to
`analyze.py raw/l4.jsonl`. `results.json` equals the concatenated raw files for l = 4.
All 1 808 rows are accounted for: 1 744 OK, 64 SKIPPED_AFTER_CENSOR, 0 NOT_RUN. Targets,
seed and n are the same across arms in every cell.

My instrumented DFS (keep arm, dual encoding) gives the same mean rho as the tables at
l = 5, 6, 7 for budgets 0, 10, 100 and 0.25, 0.5, 1 * 2^j. At l = 8 on the first 4 targets
it gives 2.46, 3.71, 8.14, 3.55, 4.58, 6.11 against 2.48, 3.75, 7.95, 3.52, 4.39, 6.01 on 8.

Not independently checked: the conflict counter. It is parsed from the CMS verbose line by
the producer's wrapper, and I reused that wrapper. One consistency sign: at l = 8 the
budget-0 arm spends exactly 375 conflicts on most targets, which is 2 per liftable x1 (120)
plus 1 per non-liftable x1 (135).

## 3. Claim A

Own rendering of the clauses from `System.rows` and `System.gates`; mean conflicts per leaf,
all leaves, first 4 (l = 4), 3 (l = 5) or 2 (l = 6) non-decomposable targets.

Options. `run.py` sets `autodisablegauss=0, sls=0` once, globally, for every arm, so plain
and dual ran with identical options. The options do not create the effect:

| l | plain, default options | plain, producer options | dual, default | dual, producer |
|---|---|---|---|---|
| 4, incremental | 124 to 167 | 135 to 160 | 3.2 to 4.8 | 2.2 to 5.9 |
| 5, incremental | 479 to 519 | 444 to 525 | 1.5 to 1.8 | 1.5 to 1.8 |
| 5, fresh solver | 793 to 885 | 657 to 709 | 28 to 29 | 20 to 24 |

`autodisablegauss=0` alone changes nothing at these sizes; `sls=0` alone accounts for the
small differences. CMS reports 1 or 2 Gauss matrices in every arm.

Diagnosis. Producer options, fresh solver per leaf unless marked:

| arm | l = 4 | l = 5 | l = 6 |
|---|---|---|---|
| plain | 199 to 306 | 657 to 709 | 2 403 to 2 635 |
| dual, rail parity written as 4 CNF clauses instead of an XOR clause | 230 to 284 | 697 to 842 | not run |
| plain + this leaf's `t = u_j` added as XOR clauses | 228 to 296 | 637 to 709 | not run |
| stage 1 substituted out (x1 plugged in, no `c & u` gates), stages 2, 3 plain | 90 to 113 | 199 to 318 | 1 347 to 1 531 |
| dual rails on `c & u` gates only | 83 to 99 | 175 to 299 | 1 191 to 1 326 |
| dual rails on `u & p`, `u & s`, `s & z` only | 142 to 207 | 682 to 689 | 1 914 to 2 388 |
| dual everywhere | 15 to 25 | 21 to 25 | 18 to 21 |
| dual everywhere, incremental | 2.2 to 3.7 | 1.5 to 1.9 | 1.6 |

Reading:

- The parity must be an XOR clause. The same dual-rail gate with the parity in CNF costs as
  much as plain. That supports "the equivalence is invisible to the Gauss matrix".
- Making stage 1 exactly linear in u (substitution, the strongest form of the stated fix)
  gains a factor 2 to 3 and keeps the growth in l (about 100, 270, 1 440). The `c_i & u_j`
  equivalence is therefore not the main cause.
- Rails on the later gates alone gain nothing. Both are needed: the collapse to O(1) comes
  from the rails on `u_j & p_k`, `u_j & s_i`, `s_i & z_j`, whose selector is a decided
  variable and not an assumption, once stage 1 is also linear. The README mentions these
  gates in a parenthesis; the headline diagnosis names the wrong gate set as sufficient.
- Adding only the leaf's `t = u_j` XORs did not reproduce even the factor 2 to 3 that
  substitution gives. I did not find out why.
- The 1.6 figure needs the warm solver. Cold cost is flat at about 20, so "O(1) per leaf"
  holds either way, with a constant about 12 times larger.

## 4. Fairness of B

- Unit. rho charges 1 per visited leaf plus every conflict; the scan is charged 1 per leaf.
  The budget-0 arm is therefore >= 1 by construction, and its floor at l = 7, 8 is 2.47
  (1 + 2 conflicts per liftable x1 + 1 per non-liftable x1). The unit also favours pruning
  arms against the solver's own scan, because a pruned leaf pays no leaf unit (see the
  budget-3 result below). One conflict = one leaf solve is a convention, as the README says.
- Uncounted work. A budget-0 call at an internal node costs no conflicts. Wall time of the
  budget-0 DFS at l = 8 is 0.82 s against 0.85 s for the same 256 leaves without internal
  calls (A-dual-inc), so that work is negligible. rho' counts the calls.
- Wall time disagrees with the conflict unit. In the producer's rows the mean time per
  (leaf + conflict) at l = 8 is 1.30 ms for budget 0, 0.80 ms for budget 10 and 0.45 ms for
  budget 100. Budget 10 is faster in wall time than budget 0 at l = 6 (0.068 s against
  0.076 s) and l = 8 (0.77 s against 0.82 s). The runs shared a 4-core container, so this is
  weak, but the README's "every arm that attempts subcubes costs more" holds in conflicts,
  not in the only other work proxy recorded.
- Baseline level. About half the x1 values fail the lift condition (the engine reaches 47
  to 56 % of leaves). A real 2+1 scan pays almost nothing for those, so "2^l leaf solves"
  is generous to the SAT arms by up to a factor 2. This makes the KILL conservative.
- Budget grid. Budget 3, which the producer did not run, has lower rho than budget 0:

  | l | rho, budget 0 | rho, budget 3 | conflicts / 2^l, budget 0 | conflicts / 2^l, budget 3 |
  |---|---|---|---|---|
  | 5 | 3.41 | 2.89 | 2.41 | 2.82 |
  | 6 | 3.40 | 2.67 | 2.40 | 2.60 |
  | 7 | 2.49 | 2.44 | 1.49 | 2.41 |
  | 8 (4 targets) | 2.465 | 2.441 | 1.465 | 2.410 |

  The gain is the leaf unit that pruned leaves do not pay; in conflicts budget 3 is worse.
  It shrinks to 1 % at l = 8 and stays far above 1. In the producer's own rows, 16 of 240
  (target, arm) pairs at l = 4..6 also beat the same target's budget-0 run; none at l = 7, 8.
- Sizes and confound. Slopes use 5 sizes, but one curve and one seed per size, and n - (3l - 1)
  is 2, 3, 2, 3, 6. The budget-0 conflicts per leaf drop from 2.4 (l = 5, 6) to 1.47
  (l = 7, 8), a step and not a trend, so the negative slopes of the absolute-budget arms say
  little. The README states both points. The rule does not use the slopes for KILL.
- Arms. All protocol arms appear in the tables. The discard arm at l = 8 has 3 of 7 budgets,
  as the protocol states. The rand and alt controls have 4 budgets, as stated.
- Protocol timing. `PROTOCOL.md` and `run.py` have mtimes 4 ms apart (18:12:03.94), and
  `raw/l4.jsonl` was last written at 18:12:10 after a run of about 7 s. The protocol was
  last written at the moment the first measured run started. `analyze.py` is dated 18:14,
  after the l = 4 and l = 5 data. There is no git history to show an earlier protocol, and
  development on seed 999 is not preserved. I found no positive sign that the rule changed:
  code and protocol agree (budgets, sample sizes, alpha definition, kill rule).
- Does KILL follow? By the letter, yes: all 10 CMS std arms at l = 8 have rho >= 2.48, and
  alpha = 1.81 >= 0.9 "at the largest l that has it".
- The alpha leg is weak. It exists at l = 4 only, from 3 dimensions (j = 2, 3, 4). The
  per-target slopes are 1.36, 1.26, 2.30, 1.35, 2.84, 1.57, 0.73, 0.75, so 2 of 8 targets
  are below the 0.9 threshold; no interval is given. It uses a fresh solver, which costs
  16 conflicts at j = 0 against 1.5 warm, so it measures cold CMS on a partly assigned x1
  and not the mechanism of the keep arm. The README states the l = 4 limitation.

## 5. Probe: subcubes of dimension j >= 2 refuted for fewer than 2^j

The raw rows cannot answer this for the DFS arms: they hold per-run totals and a prune-depth
histogram, no per-node cost. The only per-subcube rows are the alpha rows (cold solver);
their best case is 48 conflicts at j = 2, l = 4, which is 12 times 2^j. No sub-2^j case
exists in the raw rows.

My instrumented keep-arm DFS (l = 5..7 on 8 targets, l = 8 on 4; budgets 0, 3, 10, 30, 100
and 0.25, 0.5, 1, 2 * 2^j) recorded every node:

- Marginal (warm) cost. Of 8 818 internal nodes with 2 <= j < l, 1 410 were refuted by their
  own call for fewer than 2^j conflicts: 1 038 at j = 2, 267 at j = 3, 80 at j = 4, 19 at
  j = 5, 6 at j = 6. 347 of them cost 0 conflicts, the largest a 64-leaf subcube (j = 6,
  l = 8, budget 2 * 2^j). Best non-zero case: 2 conflicts for 32 leaves (j = 5, l = 8,
  budget 2 * 2^j).
- These are marginal costs. The clauses that make them cheap were learnt in failed attempts
  higher up, which the root total includes.
- Inclusive cost. Counting a node's own failed attempt and everything below it, 38 subtrees
  with j >= 2 that were not refuted by their own call cost less than 2^j. All are small:
  best 5 conflicts and 0 leaves for 8 leaves (j = 3, l = 8, budget 0.25 * 2^j, ratio 0.625);
  none above j = 3. No sub-2^j subtree survives up to the root.

So the signal the pilot looked for exists locally, in learnt clauses over fewer than l
coordinates of x1 that exclude whole subcubes, and the pilot did not record it. It never
amortises: every run total stays at 2.4 or more times 2^l. The README's sentence "pruning
happens; it is not cheaper than the leaves it removes" is right in aggregate and wrong for
individual subcubes.

## Problems, in order of weight

1. Diagnosis of claim A attributes the cost to the `c_i & u_j` gates; my substitution test
   shows that fixing them alone gains a factor 2 to 3 and leaves the growth in l.
2. "Every subcube-attempting arm is worse than budget 0" depends on the budget grid and on
   the unit: budget 3 has lower rho at l = 5..8, and budget 10 is faster in wall time.
3. The alpha leg of the kill rule rests on l = 4, three points, a cold solver, and 2 of 8
   targets below the threshold.
4. Pre-registration cannot be verified: protocol mtime coincides with the first run, no
   history, development runs not kept.
5. Encoding unsound at u = 0 (false SAT through twist points); no effect on measured rows.
6. No per-node costs in the raw rows, so the pilot's own data cannot show or exclude cheap
   subcube refutations; they do occur, up to j = 6 marginally and j = 3 inclusively.
7. One curve and one seed per size, n excess not constant, l <= 8, one solver, one encoding
   family, conflict-limit budgets only. The KILL is a statement about that scope.
8. Conflict counts come from the producer's log parser, which I did not cross-check.

Checks not completed, to be read as "weaker than stated": rerun of `run.py` at l = 5..8,
discard and alpha arms, plain arms at l >= 7, bootstrap interval for alpha, the conflict
counter, any size above l = 8.
