# Item 09 protocol: learned F4 pair selection and pivot order, with per-shape trace transfer

Frozen before any measured run. Source design: section "09" of
`research/blindspots_20261018/EXPERIMENTS.md` and its "Rules every item
follows". Written by the experiment-director lane, 2026-10-18. Cells use the
frozen seed `20261018`; the calibration described under "Calibration
disclosed" used seed `1` and no seed-`1` instance is in a card.

Class: **speed claim at the component level** (the Boolean F4 step of the
chained Semaev `S_3` descent oracle), measured in counted operations. It
computes no `S`, no rho ratio and no GAE, so the 0 / 0.1 / 1 GAE band of
AGENTS.md section 5a is applied to the one unpriced native item this
comparison has, the **policy work of the learned arms**, at 0, 0.1 and 1
touch-equivalents per policy operation (see Unit). The gap to matched Pollard
rho is **not computed here**; item 02 prices the `D = 4` rows. Nothing here is
an improvement over the index-calculus baseline by itself.

## Question

Does a learned critical-pair selection or pivot-order policy in Boolean F4
cut column touches on chained `S_3` descent systems, and does a trace recorded
on one target transfer to other targets of the same shape?

Cell notation is `(n, l, m)`: field degree `n`, factor-base subspace dimension
`l`, `m` summands; unknowns `N = m*l + (m-2)*n`, equations `(m-1)*n`, as in
item 07. The four cells are the design's: `(17,4,3)` N = 29, `(19,4,3)` N = 31,
`(21,5,3)` N = 36, `(13,3,4)` N = 38.

## What already exists, and what is reused (read before designing)

Checked against the committed record first.

* **No committed measurement of pair-selection or pivot-order policies on
  these systems exists.** The Boolean F4 experiments under
  `research/boolean_f4_*` and `research/f4_f5_*` measure engine speed
  (queues, tables, packing) at fixed pair sets, not which pairs to select.
  Nothing is reused as a result for the arms; every cell and arm below is
  measured.
* **Reused as the positive control, not re-derived:** item 07's
  `controls.tsv` (committed draws with their committed closure degree and,
  where run, the msolve F4 degree: item 06 / `dreg_*` records). The engine
  here must reproduce those degrees (Controls, P1).
* **Reused as a reference for the incumbent's logic:** `src/cryptanalysis/
  pq_f4_f2.rs` (Gebauer-Moeller update, field pairs, symbolic preprocessing,
  forward elimination counted in 64-bit word XORs). The instrument replicates
  that engine's per-step algorithm and counting convention (Deviation 2).
* **Reused system builder:** the field arithmetic, the symbolic `S_3` chain
  and the exact solution counter are copied from
  `examples/blindspots_07_blocked_closure.rs`, whose counts item 07 checked
  against 104 committed solution counts.
* **Not reusable:** the (17,4,3), (19,4,3), (21,5,3), (13,3,4) draws do not
  exist in the repository; they are generated from the frozen seed.

## Instrument

`examples/blindspots_09_learned_f4.rs`, std-only Rust, one thread. Built by
each card with `rustc --edition 2021 -C opt-level=3 -C codegen-units=1
-C target-cpu=native` into `target/blindspots-09/card-NN/` and removed at the
end of the card.

The engine is a Boolean F4 over `F_2[x]/(x_i^2 + x_i)`, DegRevLex with
`x_0 > x_1 > ...`: an initial echelon of the equations; Gebauer-Moeller pair
update with field pairs `(g, x_v)` at degree `deg LM(g) + 1`; per step, select
pairs of the minimal degree `d`, form both halves of every critical pair and
every field product, add a reducer row for every monomial an active leading
monomial divides (symbolic preprocessing, the reducer with fewest terms), and
reduce the matrix by row-by-row forward elimination; rows whose pivot is a
monomial no active leading monomial divides are the new elements. The run stops
when the constant `1` is found (**refutation**, the target is unsatisfiable) or
the pair queue is empty (**complete**: reduced Groebner basis, standard
monomial count). Unselected pairs stay queued and are selected later, so any
selection policy yields the same ideal; the reduced basis is unique.

Pair features (`NF = 15`): bias, degree, field flag, leading degrees of both
elements, log term counts, leading-monomial overlap, number of derived
elements among the two, recency, log step index, multiplier degree, coordinate
and intermediate variables in the lcm, smaller leading degree. Row features
(`NR = 9`): bias, log nonzeros, log span in words, lead column fraction, field
flag, log group size at the lead, default position, density, lead degree.

### Arms

| arm | definition |
|---|---|
| `a` | **incumbent**: all minimal-degree pairs per step (normal strategy); S-rows eliminated in lead-descending order (the order `pq_f4_f2` uses) |
| `b50`, `b25` | **learned subset (b)**: logistic regression on pair features, trained by imitation of the label oracle (below); each step selects the top `ceil(q * n)` minimal-degree pairs by score (`q` = 0.5 primary, 0.25 secondary, at least one pair); the rest are deferred |
| `c` | **learned pivot order (c)**: ridge regression on row features predicting each S-row's position in the cheapest of five candidate orders on recorded matrices; S-rows eliminated in predicted order; pair selection as `a` |
| `d` | **trace transfer (d)**: train target `A_i` (index `i` of the 32 train targets) records the pairs its run found useful, keyed by the unordered pair of leading monomials; test target `B_i` selects, at each step, the minimal-degree pairs whose key is in `A_i`'s trace (all pairs if none match); S-row order as `a` |
| `r50`, `r25` | **negative control for `b`**: a uniformly random `ceil(q * n)` pairs per step, same fractions |
| `p` | **negative control for `c`**: uniformly random S-row order |
| `o` | **oracle subset** (headroom bound, cell `(17,4,3)` targets 0 to 2 only): the pairs the label oracle marks useful on the very state being solved; its labelling work is unpriced and offline |

Label oracle (the design's "knows which pairs produced new leading terms"):
on a recorded incumbent step, the selected pairs are processed one at a time
(by degree, lcm, index) against the reducers, and a pair is **useful** when its
rows add a pivot on a leading monomial no active element divides. Row-order
oracle: on up to two matrices per train target (at least 500 S-rows), the five
orders (default; nonzeros ascending; span ascending; reverse default;
nonzeros x span ascending) are each eliminated and the cheapest is the target.

Training uses the incumbent's runs on the 32 train targets of the cell only
(about 1,000 pair samples and 300 row samples per target), then the models are
frozen and written to `raw/` before any test target is solved.

## Unit and cost accounting

Per solve (counted, deterministic):

* `xors` = 64-bit word XORs of the forward eliminations (initial echelon
  included), counted as `pq_f4_f2` counts them: the words between a pivot's
  lead word and its last non-zero word, once per row-add.
* `mat_words` = rows x words summed over every matrix built.
* **`touches` = `xors` + `mat_words`** is the design's "column touches" in
  64-column words, and the primary metric. `rows` = rows built (reducers and
  S-rows over all matrices) is the design's second metric; steps, pairs,
  deferred pairs, divisor tests, matrix maxima and wall time are descriptive.
* `policy_ops` = native work of the learned or random policy: 60 operations
  per scored pair (feature construction and dot product, counted generously)
  plus `2 n ceil(log2 n)` for the ranking sort, 8 per trace-membership test,
  `2 NR` per scored S-row plus a sort. The cost of an arm at weight `w` is
  `touches + w * policy_ops`, `w in {0, 0.1, 1}` touch-equivalents per
  operation. Arm `a` has no policy work.
* Training and labelling work (`label_xors`, fit) is offline: reported per
  cell, never in the per-solve cost. The break-even number of targets is
  reported beside it (training cost divided by the per-target saving).

Wall time is descriptive: shared 4-vCPU host, isolation L0, and the instrument
does not use M4RI tables (the counted unit is the row-by-row convention).

## Draws and cells

Per cell one factor base `V` (an `l`-dimensional `F_2`-subspace of
`F_{2^n}`, sparse irreducible modulus as items 05, 07, 11) and 64 distinct
random nonzero targets `x_R`, drawn from `Rng(seed ^ n<<40 ^ l<<32 ^ m<<48)`
with seed `20261018`: targets 0 to 31 are **train**, 32 to 63 **test**. One
`V` per cell is what the index-calculus oracle faces (one factor base, many
targets) and what makes "same shape" meaningful for trace transfer. The exact
solution count is computed for every target inside the run (a draw whose F4
result disagrees with it fails the item).

Most random targets are unsatisfiable. A satisfiable target needs the
degree-5 matrices (above the 1 GiB matrix cap here; calibration: cell
`(13,3,4)` has satisfiable targets with 24 to 106 solutions that exceed it).
Such a target is **censored at the incumbent** and no arm is run on it; it is
counted and listed. The solve measured at the design cells is therefore the
**refutation** (decision) cost. Basis equality at the design cells is the
identity of `{1}`; non-trivial bases are checked on the control cells below.

Caps (touches, per run; reached means the run is censored, not a result):
`(17,4,3)` 8e9, `(19,4,3)` 1.2e10, `(21,5,3)` 4e10, `(13,3,4)` 6e9; any arm
other than `a` is stopped at 2.0 times the incumbent's touches on the same
target and recorded `cap_touch` (a lower bound on its cost: it cannot win
that target, and is counted as a censored loss); matrix cap 2^27 words;
wall cap 120 s per solve.

## Controls

* **P1, engine against the committed degrees.** `b09 control` runs arm `a` on
  every unsatisfiable draw of item 07's `controls.tsv` with at most 34
  unknowns (m = 3 and m = 4, cells (4,1) to (13,4)); the degree of the last
  step (`degree_reached`) must equal the committed closure degree on every
  draw the run refutes, and the committed msolve F4 degree wherever one is
  recorded. Calibration: 92 of 92 and 40 of 40 (3 draws hit the cap).
* **P2, instrument self-test.** `b09 selftest`: the monomial key equals the
  committed DegRevLex comparison; on three small cells that contain
  satisfiable and unsatisfiable targets (planted decomposable targets
  included), every arm's result equals the exhaustive solution enumeration
  or the exact counter, the standard-monomial count of every completed run
  equals the solution count, and all arms return an identical reduced basis.
* **P3, basis identity with non-trivial bases.** Control cells `(8,3,3)` and
  `(7,2,4)` (32 test + 8 planted targets, models trained on their own 32
  train targets) with all arms and `o`: identical reduced-basis hash on every
  target, standard-monomial count equal to the exact solution count.
* **N1, negative controls.** `r50`, `r25` (random pairs, same fraction) and
  `p` (random row order) are measured on every design cell and the learned
  arm must beat its own null, not only `a`. A deterministic rerun of `a` is
  exactly equal (selftest).
* **Headroom (positive control that the instrument can see an effect).** `o`
  on cell `(17,4,3)`: its ratio to `a` bounds what any selection policy can
  gain there. Reported, not a gate.

## Decision rule (frozen)

Statistic per cell and arm `X` against `a`: ratio of sums over the usable test
targets, `sum touches_a / sum (touches_X + w policy_ops_X)`, with the 95%
paired-bootstrap interval (2,000 resamples of targets, fixed seeds); the
median per-target ratio and the rows ratio `sum rows_a / sum rows_X` are
reported. Usable: the incumbent completed and `X` completed or was censored at
2x (its censored cost is a lower bound).

**Cell win** for `X in {b50, c, d}` (and `b25` as a secondary row): the ratio
is `>= 1.3` **and** its interval excludes 1, at **each** of `w = 0, 0.1, 1`
(the section 5a rule: an order is ranked only where it holds across the band),
and the reduced bases are identical on every target both completed.

The design's literal rule: **ADVANCES (constant)** if `X` wins on `>= 3` of the
4 cells. Tightenings, registered here and never loosening it:

1. **Rows.** A cell win also needs `sum rows_a >= sum rows_X`. If `X` wins on
   touches on `>= 3` cells but builds more rows, the verdict is **TRADES
   (constant)**.
2. **Matched null.** `b50` must also beat `r50` (ratio `sum cost_r50 /
   sum cost_b50` above 1 with the interval's lower bound above 1 at each `w`)
   and `c` must beat `p` on the cell. If `X` meets the literal rule on `>= 3`
   cells but beats its null on fewer than 3, the verdict is **MATCHES (the
   null, constant)**: the gain is deferral or shuffling, not learning. `d` has
   no matched null (its selection fraction is data-dependent); `r50` is
   reported beside it.
3. **Censoring.** Censored incumbent targets are excluded and counted; a
   verdict computed with any censored arm target says so.

Other verdicts: **REGRESSES** when `X`'s ratio interval lies below 1 on `>= 3`
cells; **INCONCLUSIVE** otherwise or when a control fails; **BLOCKED** if the
instrument cannot complete the incumbent on a cell (what blocked it and what
would unblock it is then named). Each verdict states its level (constant).
The random-subset and random-order arms are also reported as measured results
in their own right: if `r50` or `p` beats `a` by `>= 1.3` with an interval
excluding 1 on `>= 3` cells, that is recorded as a **measured improvement of
the incumbent's pair-selection step on refutation**, with its rows and policy
cost, not as a learned-policy result.

## Budget and execution

One thread per job, `prlimit --as=4294967296`. CARD_PLAN_PLACEHOLDER

## Calibration disclosed (not reused)

CALIBRATION_PLACEHOLDER

## Deviations from the frozen design

DEVIATIONS_PLACEHOLDER

## Outputs

`research/blindspots_20261018/09-learned-f4-policy/`: `PROTOCOL.md`,
`card-NN.yaml` (frozen, linted), `raw/` (JSON lines, sample files, model and
trace files, receipts, `report-*.txt`), then `README.md` (verdict by the frozen
rule), an SVG and PNG graph and `REPORT.pdf` after the runs. A
`benchmarks/20261018-blindspots-learned-f4/` entry and a leaderboard component
section are warranted only if a verdict reads ADVANCES or the random-subset
finding stands; the integration step decides.
