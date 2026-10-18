# BLINDSPOT-09: learned F4 pair selection and pivot order, with per-shape trace transfer

## Verdict (repair pass r2 assessment)

**BLOCKED**, level **constant**, by the frozen rule of `PROTOCOL.md` (branch
"BLOCKED if the instrument cannot complete the incumbent on a cell"). The item
is **not decided**: the rule needs at least 3 of the 4 cells, and only one
cell was evaluated.

| cell | state at the end of the r2 stage-1 run | evaluated |
|---|---|---|
| (17,4,3) | trained (first run) and evaluated: 32 of 32 usable targets, all arms | **yes** |
| (19,4,3) | all 32 train targets done (r2 chunks 0, 8, 16, 24); model not fitted, no test target solved | no (stage 2 never frozen or run) |
| (21,5,3) | all 32 train targets done (first run 0, 6, 18; r2 12, 24, 30); model not fitted, no test target solved | no (stage 2 never frozen or run) |
| (13,3,4) | train card 24-r2 **stopped**: 13 of 13 incumbent runs observed ended `cap_touch` at the frozen 3e9 cap; no sample, trace or model file | **blocked** |

What blocks cell (13,3,4): in `raw/train-13-3-4-00-16-r2.jsonl`, targets 0 to
12 each reached 3.02e9 to 3.93e9 touches (cap 3e9, checked every 256 rows, so
it overshoots) with `outcome: cap_touch`, at step 16 to 18, `degree_reached` 4,
and no reduced basis. Target 12 also has 24 solutions (satisfiable, expected
censored); targets 0 to 11 have 0 solutions and are unsatisfiable, so they
are refutations the incumbent does not finish within the frozen cap. Card
24-r2 then hit its 200 s timeout (13 of 16 targets, 12 to 20 s each against
the 4 s per target the card was sized for). Only 13 of the 32 train targets
were seen; the other 19 are unmeasured. What would unblock it: a new freeze
with the (13,3,4) cap raised above the incumbent's true cost (unknown, at
least 3.9e9 on target 0) and cards sized to 15 to 60 s per target, which is a
protocol change that this assessment does not make.

* The gap to matched Pollard rho is **not computed here**. This is a component
  measurement of one F4 step on refutation (decision) systems; it computes no
  `S`, no rho ratio and no GAE. No cross-method row exists, so the 0 / 0.1 / 1
  GAE band of AGENTS.md section 5a has nothing to price except the policy work
  of the learned arms (see below, unchanged ranking across the band). The
  distance to rho is carried by item 02's priced D rows.
* **Nothing here is a verified improvement over the index-calculus baseline.**
  The one-cell observations below do not meet the frozen "at least 3 cells"
  condition and are not claimed as a finding.

## First run (superseded)

The first run stopped while card 13 of 26 was executing and no assessment or
README was written for it, so there is no first-run verdict to preserve. Its
receipts: cards 02, 09, 10, 12 completed (cell-17 train chunks and fit, cell-21
train chunks 0, 6, 18; their outputs are reused as sha-bound inputs); cards
01, 05, 06, 07, 08 stopped (01: selftest 78 s against a 62 s timeout; 05:
`fit` exit 101, `samples.0` absent; 06 to 08: `eval` exit 101, model absent);
cards 03, 04, 11, 13 to 26 never ran. No number from a stopped first-run card
is used. The r2 repair pass (see `PROTOCOL.md`, "Repair pass r2") re-froze the
unfinished work without an instrument change (sha256 `7e46e27e...` unchanged).

## Method

As frozen in `PROTOCOL.md`: std-only Rust Boolean F4 replica
`examples/blindspots_09_learned_f4.rs` on chained Semaev `S_3` systems, arms
`a` (incumbent), `b50`/`b25` (learned pair subset), `c` (learned row order),
`d` (trace transfer), nulls `r50`, `r25`, `p`, headroom `o`. Metric
`touches = xors + mat_words` per refutation; cost at weight `w` is
`touches + w * policy_ops`. Decision statistic per cell and arm: ratio of
sums over usable test targets with a paired bootstrap (2,000 resamples, 95%
interval); cell win needs ratio >= 1.3 with the interval excluding 1 at each
`w`, equal bases, and `rows_a >= rows_X`; matched-null and 3-cell conditions as
frozen. The statistics below are computed from the committed `raw/*.jsonl` by
`analyze_cell17.py` (its output is `raw/analysis-cell17.txt`, figure data
`figures/cell17.json`); bootstrap seeds are fixed (seed 1) and are my
choice, since the protocol fixes the resample count but not the seed.

## Numbers, cell (17,4,3), 32 usable targets (all `a` runs refuted, 0 censored)

Ratio = sum touches(a) / sum cost(arm); measured counts, derived ratios.
Intervals are 95% paired bootstrap. Policy work moves every ratio by under
0.01 between `w = 0` and `w = 1` (policy_ops about 1e6 against 1e9 touches).

| arm | ratio at w=0 | 95% interval | median per target | rows_a / rows_X | censored at 1.5x |
|---|---|---|---|---|---|
| b50 learned half | 1.992 | [1.966, 2.020] | 1.999 | 0.801 | 0 |
| b25 learned quarter | 2.811 | [2.706, 2.942] | 2.712 | 0.644 | 0 |
| c learned row order | 1.000 | [0.992, 1.007] | 1.000 | 1.000 | 0 |
| d trace transfer | 0.842 | [0.810, 0.878] | 0.809 | 0.603 | 0 |
| p random row order (null of c) | 0.946 | [0.936, 0.957] | 0.939 | 1.000 | 0 |
| r50 random half (null of b50) | 7.278 | [7.149, 7.402] | 7.309 | 0.862 | 0 |
| r25 random quarter (null of b25) | 4.103 | [3.859, 4.365] | 4.467 | 0.468 | 0 |
| o oracle subset (3 targets only) | 7.151 | not computed | n/a | 0.63 to 0.67 | 0 |

Reading at this one cell, by the frozen rule (not a verdict, since 3 cells are
required):

* `b50` meets the touches condition (1.99, interval above 1.3) but **fails the
  rows tightening** (builds 25% more rows) and **fails the matched null**:
  `r50` beats it, `sum cost(r50) / sum cost(b50) = 0.274 [0.269, 0.279]`.
  `b25` likewise (0.685 against `r25`).
* `c` equals `a` (1.000 [0.992, 1.007]) and beats `p` only because `p` is worse
  than `a` (`p/c` 1.057 [1.044, 1.070]). The fit's row-order target was the
  default order on 26 of 32 sampled matrices (`raw/fit-17-4-3.json`).
* `d` regresses (0.842, interval below 1) and also builds more rows
  (rows_a / rows_d = 0.603).
* The random half `r50` reaches 7.28 and the oracle subset 7.15 (3 targets,
  its labelling cost of 4.2e9 to 4.7e9 xors per target is unpriced and
  offline). A random half of the minimal-degree pairs per step captures all of
  the headroom the oracle shows at this cell, so nothing here is attributable
  to learning. The frozen clause "r50 or p beats a by >= 1.3 on >= 3 cells is
  a measured improvement of the incumbent's pair selection on refutation" is
  **not met** (1 cell evaluated); it is a lead for cells 19 and 21 only.
* Break-even for the offline cost of `b50` at this cell: training labelled
  1.13e11 xors over 32 targets against a mean saving of 0.96e9 touches per
  target, about 118 targets to repay (derived, a rough figure).
* Descriptive wall (sum over the 32 targets, shared host, L0): a 46.9 s, b50
  27.2 s, r50 11.3 s, c 49.2 s, d 113.1 s.

## Controls

* **P1 (engine against committed degrees)**, card 01-r2: 92 of 92 refuted
  draws have degree equal to the committed closure degree, 40 of 40 equal to
  the committed msolve degree, 5 draws not refuted (cap or wall). Passed.
* **P2 (selftest)**, card 01-r2: monomial order PASS and three cell checks
  PASS (wrong 0, basis differs 0), `SELFTEST OK` (81.9 s). Passed.
* **P3 (basis identity, non-trivial bases)**, card 01b-r2, cells (8,3,3) and
  (8,2,4), 32 test + 8 planted targets, all 9 arms: no `check` false in 720
  runs, reduced-basis hash equal across arms on every target, standard-monomial
  count equal to the solution count on all 225 complete (`sat`) runs. Passed
  (my recomputation from `raw/eval-p3-*-r2.jsonl`; the card's own lines do not
  print this).
* **Cell (17,4,3)**: every `check` true (256 eval runs plus 3 `o` runs), basis hash
  equal to the incumbent's on every usable arm run.
* Positive control for detecting an effect (`o` headroom): 7.15, so the
  instrument can see a large effect, and it is a pair-selection effect a
  random half also produces.

## Budget and cards (assessor's tally)

Summed measured-command seconds in all receipts (build and cleanup excluded):
2,221 s, of which r2 stage 1 is 1,443 s, against the 7,200 s cap. Stage 2
(cards 05b, 06 to 08, 14 to 23b, 25, 25b) and stage 3 (26-r2) were never
frozen or run; they need the director pass named in the task (`make_r2_cards.py
stage2`), and for cell (13,3,4) a re-freeze first.

## Cards and relay status

All nine r2 stage-1 cards have receipts; eight `completed`, card 24-r2
`stopped at command 3` (timeout). I saved each executor reply as
`raw/relay-<n>.txt` (copied from the returned text) and ran `check-relay`:

| relay | card | check-relay |
|---|---|---|
| 1 | 01-r2 | verbatim |
| 2 | 01b-r2 | **deviated**: the executor said it did not paste the full output (omits the train/fit/eval stdout lines). The tool could not even evaluate it: `CARD FAIL`, because the card's own outputs (`model-p3-*-r2.*`) now exist unbound in the repository and lint refuses them |
| 3 | 03-r2 | verbatim |
| 4 | 03b-r2 | **deviated**: omits the receipt's `status: completed` line |
| 5 | 04-r2 | verbatim |
| 6 | 05-r2 | **deviated**: omits the receipt's `status: completed` line |
| 7 | 11-r2 | **deviated**: first line reads `CARD-BLINDSPOT-09-11-r2 (...)` without the `card: ` prefix |
| 8 | 13-r2 | verbatim |
| 9 | 24-r2 | verbatim (stopped card reported as stopped) |

The deviations are omissions or formatting; every number used above is read
from the committed `raw/*.jsonl` and the receipts, never from the executor
text, and the receipt outputs' sha256 values in the relays match the files.
Deviated relays are reported, not repaired. The leftover binary
`target/blindspots-09/card-24-r2/b09` (left by the stopped card) was deleted so
that lint would not refuse the card.

## Deviations and caveats

* **Host:** shared 4-vCPU container, isolation L0; wall times are descriptive,
  and two lanes ran concurrently.
* **Toy scale:** 17 to 38 unknowns, refutation of random unsatisfiable targets
  at D = 4, one factor base per cell; no link to the 2^k ECDLP sizes where
  index calculus competes with rho.
* **One cell of four evaluated.** The ADVANCES / TRADES / MATCHES conditions
  cannot be reached or excluded for cells 19 and 21; the (13,3,4) cell is
  blocked, so at most three cells can contribute, which makes ADVANCES for
  any arm require wins on all of (17,4,3), (19,4,3), (21,5,3). At (17,4,3)
  `c` and `d` already fail, so only `b50` can still reach a 3-cell literal
  win, and it fails the null at (17,4,3), so it cannot beat its null on 3
  cells either. Under the frozen rule that excludes ADVANCES for `b50`, `c`
  and `d`, but TRADES or MATCHES for `b50` still depends on cells 19 and 21.
  This is stated as reasoning from the frozen rule, not a measured verdict.
* Cell (17,4,3) evaluation used the first-run model and traces (card 02, bound
  by sha256), evaluated in r2 cards 03-r2, 03b-r2 and 04-r2; no first-run
  stopped card contributes.
* The bootstrap seed and the choice of `median` as the mean of the 16th and
  17th ordered ratios are mine.
* No benchmark entry: the protocol makes one warranted only for ADVANCES or a
  standing random-subset finding, and neither holds. No leaderboard change.

## Files

`PROTOCOL.md`, `card-*-r2.yaml`, `raw/` (records, receipts, relays,
`analysis-cell17.txt`), `analyze_cell17.py`, `figures/ratios.{svg,png}` with
`figures/make_figure.py` and `figures/cell17.json`, `REPORT.pdf`.

![cell 17 ratios and cell 13 block](figures/ratios.png)
