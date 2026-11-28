# PROTOCOL: lead C, cheap block refutations (written 2026-11-15, before any measured run)

Local pilot. Every number that follows from this protocol is `[pcc:unverified]`. No benchmark record,
leaderboard row or pcc claim follows from it.

Follow-up to `../../sat_smt_leads_20261114/followup-subcube/` (producer README and SKEPTIC.md). Three
items the skeptic left open are settled here by fixed rules: (i) cheap warm subcube refutations,
(ii) the scaling exponent alpha of subcube refutation cost, (iii) budget 3 against budget 0.

## What was run before this file was written

- `validate.py 3 4` (cell seeds 2026111503, 2026111504 and random B for the u = 0 instances): the
  fixed encoding against an independent brute force. Not a measured cell.
- One development probe on seeds 999 + l, l = 6, 8, 9, 2 targets: wall time of one DFS (0.05 s, 0.5 s,
  1.2 s), cold and warm cost of one subcube per j (for the conflict cap), and whether the CMS `seed`
  option changes conflict counts (it does not: identical counts for seed unset, 1, 2). No measured seed
  (2026111506..09) was run.

## Instances

- As in the earlier pilot: F_{2^n}, curve y^2 + xy = x^3 + x^2 + B, V = span(1, t, .., t^(l-1)), factor
  base = rational points with x in V \ {0}, targets with x(R) outside V. (l, n) = (6, 19), (7, 23),
  (8, 29), (9, 29). n is not constant; n - (3l - 1) = 2, 3, 6, 3: a confound for trends in l.
- Cell seed 2026111500 + l. Per cell: the first 24 random targets that the pair-table ground truth says
  are non-decomposable, then 4 planted (decomposable) targets, reported separately. Same targets in
  every arm.
- Encoding: `dual` (dual-rail gates) with the u = 0 fix: clause "u != 0" in the SAT instance and the
  u = 0 case decided outside it (`Cell.u0_decomposable`). CryptoMiniSat 5.18.0 via pycryptosat, one
  thread, options `autodisablegauss=0, sls=0` as before.
- Baseline named throughout: the linear 2+1 scan, counted as 2^l leaf solves.

## Units

Primary: leaves visited and conflicts. rho = (leaves + conflicts) / 2^l per target. Also reported for
every arm: conflicts / 2^l alone and rho' = (solver calls + conflicts) / 2^l, because a budgeted
internal call pays no leaf unit under rho (the skeptic's point; the development probe shows budget 1
visiting 8 of 64 leaves). Wall time secondary.

## Arms

DFS over x1's coordinates in index order, value 0 first, one incremental (warm) solver per DFS,
conflict limit b at every internal node (dimension j >= 1), no limit at leaves (`lib.rec_dfs`). Every
node is recorded: depth, prefix, result, conflicts of its own call, CMS's final assumption
conflict (`get_conflict`, as a set of coordinates), leaves and conflicts spent at and beneath it.

- rep 0: canonical variable numbering. rep 1, 2, 3: the same CNF/XOR instance with variables
  renumbered and clauses and literals reordered at random (seed cellseed * 31 + 7 * target + rep).
  This is the A/A device, since the CMS seed has no effect.
- l = 6, 7, 8: budgets 0..10 at rep 0 and rep 1; budgets 0 and 3 also at rep 2 and rep 3.
- l = 9: budgets 0, 1, 2, 3, 5, 10 at rep 0 and rep 1; budgets 0 and 3 also at rep 2 and rep 3.
- Control `rand`: seeded random coordinate order (seed cellseed * 13 + target), rep 0, budgets 0 and 3.
- Planted targets: rep 0, budgets 0, 3, 10; reported separately, used in no rule.

## (2) Cheap-refutation census

Event: a DFS node of dimension j >= 2 whose subcube was disposed of for an inclusive cost
(leaves visited + conflicts, at the node and beneath it, including its own failed attempt) below 2^j.
Covered fraction of one DFS: leaves lying under at least one event, divided by 2^l. Non-decomposable
targets only. Reported for every budget in {0, 1, 2, 3, 5, 10}, rep 0 and rep 1.

Primary series: budget 3, rep 0, mean over the 24 targets, at l = 6, 7, 8, 9 (bootstrap interval over
targets).

Rule:
- "signal" if the covered fraction is >= 10 % at l = 9 and not decreasing over >= 3 sizes
  (the last three sizes non-decreasing);
- "no signal" if it is < 2 % at l = 9 or strictly decreasing over the four sizes;
- otherwise "undecided".

The events are marginal: clauses learnt elsewhere in the DFS are not charged to the node. The rule is
therefore not a statement about the total; the root total (rho) is reported next to it, and so are:
- the largest event dimension and the share of leaves under events of dimension >= 3, >= 4, >= l - 2;
- lift-adjusted events: inclusive cost below the number of leaves in the subcube whose x1 lifts to a
  rational point (the scan pays next to nothing for the others);
- conflict-only comparison: conflicts spent on the event subcube against the conflicts the budget-0
  DFS of the same target and rep spent on the same subcube.

Characterisation (no rule):
- per refuted node: conflicts, j, and the size of the final assumption conflict. A size below the
  depth k means the refutation covers a larger cube than asked. For 0-conflict refutations the negated
  assumption conflict is a clause over x1-coordinate literals only that already fires by propagation.
  pycryptosat exposes neither the learnt clause database nor clause provenance, so clause length is
  the assumption-conflict size and "XOR-derived" cannot be read off directly.
- trivial cause, checked exactly for every leaf of every target (`lib.leaf_class`): class 0 = x1 does
  not lift (S3(x1, xR, u) = 0 has no root u in the field); class 1 = P1 rational but for both
  u = x(R +- P1) the affine F2-linear system p^2 + u p + u^2 s^2 + B = 0 in the 3l - 1 bits of (s, p)
  is inconsistent; class 2 = consistent for some u, no split (stage 3). An event is "trivially
  explained" if all its leaves are class 0, and "linear-stage explained" if all are class 0 or 1.
  Compared with the same statistic over all aligned subcubes of the same j (base rate). This is the
  "for all completions" check done by enumeration; a function-field computation is not attempted.
- the `rand` control: do the covered fractions survive a change of coordinate order.

## (3) Alpha

l = 6, 7, 8, 9; first 12 non-decomposable targets; j = 0..6 (j <= l - 1); 2 seeded random subcubes per
(target, j): the first l - j coordinates fixed at random values (seed cellseed * 19 + target). The same
subcube in every arm. Conflict cap 50 000 per query.
- cold: fresh solver, the subcube query only. j <= 5. Once both samples of a target at some j are
  censored, larger j are skipped for that target.
- warm32: fresh solver, first 32 distinct random leaves outside the subcube (x1 fully assumed), then
  the subcube query. Warm-up conflicts recorded, not charged.
- warmq: the same with 2^(l-2) leaves outside the subcube.

A j is uncensored if no sample at that j hit the cap or was skipped. alpha(l, arm) = least-squares
slope of log2(mean conflicts) against j over uncensored j >= 2 (needs >= 3; a mean of 0 is replaced by
1/16 and flagged). Interval: 2 000 bootstrap resamples of the targets, 2.5 and 97.5 percentiles.
The fit over j >= 1 and mean conflicts / 2^j are reported too.

Rule: "sub-scan exponent" only if the upper bound of the warm alpha interval is < 0.9 at l = 9, for
warm32 and for warmq; otherwise "no sub-scan exponent". A warm exponent excludes the warm-up cost by
construction, and the level (conflicts / 2^j) is reported with it.

## (4) Budget 3 against budget 0

For configurations X, Y = (budget, rep): d(X, Y) = mean over the 24 targets of rho_X - rho_Y.
- Edge estimates at each l: E_r = d((3, r), (0, r)), r = 1, 2, 3 (r = 0 reported as well).
- A/A interval at each l: 4 000 bootstrap resamples of the targets; in each, a budget b in {0, 3} and
  an ordered pair r != r' in {1, 2, 3} drawn uniformly; d((b, r), (b, r')) recorded; 2.5 and 97.5
  percentiles.
- The edge is outside at size l if E_1, E_2, E_3 are all below the A/A interval.
- Rule: "budget-3 edge real" only if it is outside at two of l = 6, 7, 8. l = 9 is reported, not used.
- The same quantities in conflicts / 2^l and in rho' are reported, with no rule, and budgets 1..10
  against 0 at rep 0 and rep 1.

## (5) Exploratory arm

Chosen after the census, if it shows a mechanism that is not trivial. Labelled exploratory, used in
no rule, reported as (leaves + conflicts) / 2^l on the same targets with rho' and conflicts next to it.

## Limits fixed in advance

One process, one core, under `nice`; each process under 10 minutes; about 80 minutes in total. Cells
not reached are listed as NOT_RUN. No parameter is changed after a measured run; a deviation is listed
in the README.

## Addendum: the exploratory arm (5), written after the l = 6 and l = 7 sweep files were analysed and before any exploratory run

Seen so far (l = 6, 7, rep 0): the events are almost all dimension-2 nodes refuted by their own
budget-3 call (59 to 78 % of dimension-2 attempts succeed), they spend about 0.5 to 0.6 conflicts per
leaf, and the budget-3 DFS spends about 1.1 conflicts per leaf on failed attempts at dimensions >= 3.
Exploratory question: do the dimension-2 refutations stay cheap when the failed attempts above them
are not made? Variants (`explore.py`), budget 3, rep 0 and rep 1, the same 24 non-decomposable targets,
l = 6..9: `j2only` (calls at dimension 2 and at leaves only), `jle2` (dimensions 1, 2), `jle3`
(dimensions 1, 2, 3). Reported as rho, conflicts / 2^l and rho'; reference = the budget-0 and budget-3
arms of the same rep. No rule; nothing in decisions (2) to (4) depends on it.

Second exploratory variant, added after the l = 6, 7, 8 sweep files were analysed and before any
exploratory run: in the budget-0 leaf scan about a quarter of the leaf refutations end in a final
assumption conflict that omits a coordinate at which the leaf has value 0, i.e. the solver has refuted
leaves the DFS has not reached. `coreskip` = the budget-0 scan in which a leaf inside the
final-conflict cube of an earlier leaf refutation is not submitted to the solver (no leaf unit, no
conflicts). It triggers the mechanism by bookkeeping only. Same reporting, no rule.
