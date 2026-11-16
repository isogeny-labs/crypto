# SKEPTIC review: lead C, cheap block refutations (2026-11-15)

Independent review of `PROTOCOL.md`, `README.md`, `results.json` and the raw rows in this directory.
Every number here is `[pcc:unverified]`: a local pilot review, no claim, no benchmark record.
Scripts and outputs of this review are outside the repository, in
`/tmp/claude-0/-home-user/570c81c4-197a-558d-aa76-c9ff505d9731/scratchpad/skeptic-C/`
(`sk_verify.py`, `sk_raw.py`, `sk_raw2.py` and their `.out` files; the scratchpad is not durable).
Producer files were read only; the 15 files in `raw/` have the same SHA-256 before and after.

## Verdict

- **Reproduced: yes** for the data and the three rule outcomes; **the reading of (2) is weaker than
  "signal"**, and one supporting statement in the README (budget 3 is below budget 0 in rho') is an
  accounting artefact.
- **(2) "signal" is a marginal-accounting artefact, not a trend toward sub-scan.** When the failed
  attempts at an event's ancestors are charged to it pro rata, the share of leaves under events at
  budget 3 falls from 0.247, 0.382, 0.404, 0.471 to 0.000, 0.008, 0.001, 0.004 (l = 6..9). In totals,
  budget 3 costs 1.09 to 1.33 times the same solver's scan at every size and renumbering, with no
  downward trend.
- **(3) "no sub-scan exponent": follows from the rule**, and is robust to the fit variant.
- **(4) "not shown above the A/A floor": follows from the rule.** In units that charge every solver
  call, budget 3 is worse than the scan at every size.
- **Data integrity: clean.** No truncated rows, 24 targets in every cell, duplicates identical.
- Pre-registration cannot be verified from the files (see Problems, 1).

## 1. Encoding and labels, checked with separate arithmetic

`sk_verify.py`: own field multiplication (shift and reduce), Fermat inversion, own half-trace solver,
own point addition, own pair table; the field polynomial is the producer's, checked irreducible by
trial division. It uses the producer's `lib.System`, `lib.CMS` and target generator, since those are
the objects under test.

| Check | Result |
|---|---|
| l = 3 (n = 11) and l = 4 (n = 13), 24 non-decomposable + 4 planted + 24 further random targets each: producer label against own ground truth | 56 of 56 labels agree; at l = 3 the pair table also equals the full triple enumeration (52 of 52) |
| Free solve (SAT or `u0_decomposable`) against own ground truth | 104 of 104 agree (95 UNSAT, 9 SAT) |
| SAT models lift, with own arithmetic, to three points with x in V summing to R | 9 of 9 |
| Per-x1 leaf answers against own enumeration (decomposition with that x1 and x(P2 + P3) != 0), and leaf models lift with the assumed x1 | 1 144 of 1 144 |
| u = 0 fix: 12 instances constructed here (B = (x2 x3)^2 with a non-liftable x among x2, x3; R = P1 + T; non-decomposable by own pair table) | old encoding: SAT on 12, none of the models lifts; fixed encoding (dual and plain): UNSAT on 12; `u0_decomposable` false on 12 |
| Measured cells l = 6, 7, 8: the 24 targets labelled non-decomposable, the 4 planted | 24 of 24 non-decomposable and 4 of 4 decomposable in each cell; every x(R) outside V |

Not checked: the l = 9 labels by own arithmetic (pure-Python pair table too slow in the time given).
The measured runs assert UNSAT against the label on every DFS, so a wrong l = 9 label would have
aborted the run unless the encoding is unsound in the same direction. The small cells have 3 and 7
base points, so they exercise few distinct structures.

## 2. Rerun of the smallest cell

`cmeasure.py <scratch>/c_l6.jsonl.gz 540 6 sweep,rand,planted,alpha,explore` on a scratch copy, one
process under `nice`, 85 s. All 1 332 rows have the same keys as the producer's l = 6 rows, and every
field other than wall time is equal, per-node records included (0 rows differ). `analyze.py` on the
rerun prints the producer's l = 6 values. The numbers in sections 3 to 5 below were recomputed from
the producer's raw rows with separate code and agree with `results.json` wherever both give the same
quantity (covered shares, rho, conflicts / 2^l, the E_r values, the alpha fits).

## 3. Accounting: is anything below the scan?

### 3a. The event definition leaves out the cost that makes events cheap

An event's cost is its own call plus what lies beneath it. It does not include the failed budgeted
calls at its ancestors. At budget 3 those calls are where the learning happens: per x1 value the DFS
spends 1.10 to 1.20 conflicts in failed calls at dimension >= 3 and 0.24 to 0.51 more in failed calls
at dimension 2, against 1.63 to 2.20 for the whole budget-0 scan (rep 0).

Share of leaves under events, budget 3, mean of 24 targets (95 % bootstrap over targets):

| l | producer's definition, rep 0 | rep 1 | ancestors' failed calls charged pro rata by leaves, rep 0 | rep 1 |
|---|---|---|---|---|
| 6 | 0.247 (0.198..0.299) | 0.240 | 0.000 | 0.000 |
| 7 | 0.382 (0.332..0.431) | 0.397 | 0.008 (0.003..0.013) | 0.010 |
| 8 | 0.404 (0.379..0.432) | 0.393 | 0.001 (0.000..0.002) | 0.005 |
| 9 | 0.471 (0.453..0.489) | 0.478 | 0.004 (0.001..0.007) | 0.007 |

With the ancestors charged, the series is below 2 % at l = 9, which is the protocol's own threshold
for "no signal". The same holds at budget 1 (0.02 at l = 9) and budget 10 (0.000).

### 3b. Totals in units where a scan leaf costs what it costs this solver

Unit H = solver calls + conflicts. The scan in the same solver is 2^l leaf calls plus the conflicts
of the budget-0 arm. Ratio of sums over the 24 targets, paired, 95 % bootstrap over targets:

| l | budget 3, rep 0 | rep 1 | rep 2 | rep 3 | budget 3 conflicts / scan conflicts, rep 0 / rep 1 |
|---|---|---|---|---|---|
| 6 | 1.148 (1.082..1.214) | 1.319 (1.265..1.381) | 1.267 | 1.331 | 1.31 / 1.65 |
| 7 | 1.091 (1.021..1.171) | 1.143 (1.106..1.179) | 1.148 | 1.144 | 1.31 / 1.47 |
| 8 | 1.177 (1.146..1.210) | 1.236 (1.205..1.269) | 1.182 | 1.235 | 1.49 / 1.61 |
| 9 | 1.100 (1.043..1.150) | 1.136 (1.118..1.152) | 1.151 | 1.146 | 1.37 / 1.46 |

Budget 3 is above the scan at every size on every renumbering, in H and in conflicts. Budgets 1, 2,
5 and 10 are above it too (1.06 to 1.73 in H). The sequence is not monotone in l; l = 7 and l = 9,
which share n - (3l - 1) = 3, give 1.14 and 1.14 on the three renumbered instances.

Best arm of the pilot, exploratory `jle2`, same unit:

| l | `jle2` / scan, rep 0 | rep 1 | `jle2` / `coreskip`, rep 0 | rep 1 | `jle2` conflicts / scan conflicts, rep 0 / rep 1 |
|---|---|---|---|---|---|
| 6 | 0.836 (0.787..0.890) | 0.997 (0.961..1.033) | 0.873 | 1.097 | 0.92 / 1.20 |
| 7 | 0.829 (0.784..0.869) | 0.941 (0.916..0.968) | 0.931 | 1.051 | 0.94 / 1.14 |
| 8 | 0.857 (0.838..0.878) | 0.899 (0.879..0.919) | 0.953 | 1.008 | 1.04 / 1.13 |
| 9 | 0.831 (0.791..0.864) | 0.879 (0.868..0.890) | 0.927 (0.896..0.955) | 0.976 (0.964..0.988) | 0.98 / 1.08 |

- `jle2` is 12 to 17 % below the scan in H at l = 9. All of that is call count (0.57 calls per x1
  value against 1.0); its conflicts are equal to the scan's or above. `coreskip`, which is the scan
  with bookkeeping only, already removes 10 % of H.
- Against `coreskip` the difference is -7 % (rep 0) and -2 % (rep 1) at l = 9, from +10 % at l = 6
  on rep 1 and from -13 % at l = 6 on rep 0. The two renumberings move in opposite directions, so
  there is no trend established. Two renumberings, one curve per size, an arm chosen after seeing
  the sweep.
- The saving is bounded: it prices a budgeted call at dimension 1 or 2 the same as a leaf call, and
  conflicts per x1 value do not go below 1.49 in any arm. Nothing in these data moves the exponent.
- Wall time does not settle the call price: `jle2` against `coreskip` is 0.85 against 0.92 of the
  budget-0 arm's time at l = 9, intervals overlapping, on a machine at load 4 to 7.
- Against the linear 2+1 scan, counted as 2^l units, every arm is at 1.68 or above as the producer says.

### 3c. Is the rise with l an artefact?

Tested here:

- Leaf units unpaid: not the explanation. In the budget-0 scan no aligned 4-leaf block is refuted
  for fewer than 4 conflicts at any size (minimum 4 to 5, mean 6.0 to 8.8). The budget-3 blocks are
  cheaper in conflicts on their own leaves, as the producer says.
- Warm-up along the DFS: partial at l = 9 only. Share in the first 64 leaves of the DFS: 0.247,
  0.414, 0.453, 0.346 at l = 6..9 (rep 0); 0.240, 0.419, 0.388, 0.375 (rep 1). At l = 9 the later
  leaves are at 0.46 to 0.50; at l = 8 they are lower than the first 64. At fixed position the
  series is not monotone in l.
- Ancestor attempts: the dimension-2 success rate is 0.55, 0.69, 0.72, 0.77 with them and 0.31,
  0.36, 0.43, 0.42 without them (`j2only`, producer's figure). Most of the rise in the share sits in
  the part that the ancestors pay for.
- Instrument threshold: the conflict limit is not enforced exactly. At budget 3 failed calls report
  4 or 5 conflicts (3 to 27), and dimension-2 own-call refutations report 0 to 17. In the first
  l = 9 file (rep 0) 551 of them report 3 conflicts and 466 report 4. The event test "below 4" cuts
  between these two bins, and the producer's own check puts the counter within "a few conflicts per
  call". The level of the share depends on a one-conflict boundary the instrument does not resolve.

Not separated: n is not constant, and one curve per size.

So the census trend is a reproducible description of where the marginal counter puts events. It is
not a trend in any total, and it disappears when the ancestors are charged.

## 4. Do the decisions follow from the rules?

**(2)** The rule was applied as written: 0.471 >= 0.10 at l = 9 and 0.382, 0.404, 0.471
non-decreasing give "signal". "By the letter only" is the right reading, with these additions:

- The same rule on the renumbered instance (rep 1: 0.240, 0.397, 0.393, 0.478) gives "undecided",
  because 0.397 to 0.393 is a decrease. The rule has no noise allowance, and the verdict changes
  between two renumberings of the same instances. The random-order control gives "signal".
- On the producer's lift-adjusted share (0.036, 0.130, 0.095, 0.128) the rule gives "undecided"; on
  events of dimension >= 3 (0.5 %, 3.9 %, 4.0 %, 7.4 %) it gives "undecided" (below 10 % at l = 9).
- With ancestors charged it gives "no signal" (section 3a).

What the rule needed to say to mean something: state it on a total, for example "the paired ratio
(calls + conflicts of the arm) / (2^l + conflicts of the scan in the same solver) is below 1 at
l = 9 with its interval, on at least two renumberings, and decreases over the last three sizes by
more than the A/A spread". If events are kept, charge ancestors' failed calls, require dimension
growing with l (the protocol lists "dimension >= l - 2" as a reported quantity; it is 0 everywhere),
and fix the event threshold away from the counter's error.

**(3)** Recomputed from raw: warm32 at l = 9: 1.44 (1.38..1.50), five uncensored dimensions
j = 2..6, 24 samples each, none censored. warmq: 1.26 (1.14..1.37), five uncensored. Both upper
bounds are above 0.9, so "no sub-scan exponent" follows. Geometric-mean fits give 1.39 and 1.28;
dropping j = 2 gives 1.56 and 1.31. All warm lower bounds are above 1 at every size. Limits: 12
targets and 2 subcubes per (target, j); j stops at 6; warm-up conflicts (97 and 275 on average at
l = 9) are not charged, which favours the warm arms and still leaves alpha above 1. Cold: two
uncensored dimensions at every l, so the fit is undefined, as reported.

**(4)** Recomputed E_1..E_3 and A/A intervals agree with the README to the bootstrap seed (for
example l = 6: A/A -0.223..+0.207 here, -0.214..+0.209 there). All of E_1..E_3 are below the
interval at l = 7 only among l = 6, 7, 8, so "not shown" follows. The README adds "In rho' it is
below at l = 7, 8, 9". That comes from the budget-0 arm as run: it makes 2^(l+1) - 1 solver calls,
of which 2^l - 1 are zero-budget calls at internal nodes that a scan would not make, and rho'
charges them (3.717 against 2.719 at l = 9, rep 0). Against the scan with 2^l calls, budget 3 is
above at every size (section 3b).

## 5. Data integrity

- All 15 gzip files read to the end; 0 unparsable lines.
- Every cell has 24 distinct non-decomposable targets, with the same R, B and seed for each target
  index across the sweep, continuation, alpha and exploratory files.
- l = 8 (relaunched): 624 sweep, 48 control, 12 planted rows, all OK, none missing.
- l = 9: `sweep_l9.jsonl.gz` has 362 OK sweep rows and 82 NOT_RUN rows (22 sweep, 48 control,
  12 planted). `sweep_l9_cont.jsonl.gz` adds 32 OK sweep rows for targets 22 and 23, of which 10
  repeat rows of target 22 from the first file, plus 2 repeated leaf-class rows. The 12 repeats are
  identical to the first copies in every field compared, node records included. `sweep_l9_cont2`
  holds the 48 control and 12 planted rows. After de-duplication 384 sweep rows, none missing.
- Exploratory: 192 OK rows per size. Alpha: 432, 476, 478, 480 OK rows; the rest are the declared
  skipped and not-in-protocol placeholders.
- Internal consistency of every DFS row: the node conflicts sum to the row total, the root's
  inclusive counts equal the row totals, budget-0 rows visit 2^l leaves, calls equal the number of
  node records, and no non-planted row is SAT. 0 inconsistent rows.
- No sign of rows from a killed process mixed in. The discarded partial l = 8 file is not present.
- Not checkable from here: whether the producer's `pkill -f "run.py raw/"` hit another agent's job.
  That question belongs to the other jobs' logs.

## Problems

1. **Pre-registration is unverifiable.** The directory is not under version control here.
   `PROTOCOL.md` was last modified at 07:52, after `sweep_l6` (07:36), `sweep_l7` (07:38) and
   `sweep_l8` (07:47) were complete. The producer discloses two addenda and one edit; the file alone
   cannot show that the thresholds of rules (2) to (4) predate the data. `explore.py` was last
   modified at 08:19:54, after its outputs (08:07 to 08:13) and after `results.json`; the l = 6
   rerun with that version reproduces the stored rows exactly, so behaviour at l = 6 is unchanged.
2. **The headline of (2) is produced by a rule that cannot fail for the reason it passed.** A budget
   near 2^j makes dimension-j events whenever the solver finishes inside the budget, and the cost of
   getting there is booked to other nodes. The producer says this in the README; the decision table
   still prints "signal" in bold. It should read "no signal in any total; marginal events only".
3. **The "signal" does not survive the pilot's own A/A device** (rep 1 gives "undecided").
4. **rho' overcharges the budget-0 arm** by 2^l - 1 zero-budget calls, which makes budgeted arms look
   12 to 19 % cheaper than the scan in rho' when they are 9 to 33 % dearer.
5. **rho undercharges budgeted arms**: an internal call pays no unit. The lowest quoted ratios
   (1.72 / 1.68 for `jle2`) are in this unit; in calls + conflicts `jle2` is 2.26 / 2.23 per x1 value.
6. **The event threshold sits inside the conflict counter's error**, and the conflict limit of 3 is
   in practice 4 to 5 reported conflicts.
7. **Confounds left open**: n varies with l, one curve and one seed per size, the canonical
   numbering is an outlier against budget 0 (rep 0 budget-0 rho is 0.14 to 0.52 above reps 1 to 3),
   and primary series were fixed on that numbering.
8. **The exploratory arm** was chosen after the data, has two renumberings, and its small advantage
   over `coreskip` changes direction of travel between them.
9. Process: this pilot has no session-log entry, Conductor task, research-kb receipt or PDF report
   (the producer lists this). This review also wrote only this file, as instructed.

## Not done in this review

- l = 9 labels by separate arithmetic; reruns of l = 7, 8, 9.
- An independent conflict counter, or a run with the event threshold moved.
- A second curve per size or fixed n, which the trend question needs.
- Wall-time comparison on an idle machine.
