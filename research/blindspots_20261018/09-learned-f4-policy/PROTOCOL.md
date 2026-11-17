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
`F_{2^n}`, sparse irreducible modulus as items 05, 07, 11) and 96 distinct
random nonzero targets `x_R`, drawn from `Rng(seed ^ n<<40 ^ l<<32 ^ m<<48)`
with seed `20261018`: targets 0 to 31 are **train**, 32 to 95 the **test**
pool (the first 32 test targets are the design's; the rest are used only at
`(13,3,4)`, Deviation 10). One
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
`(17,4,3)` 6e9, `(19,4,3)` 1e10, `(21,5,3)` 3e10, `(13,3,4)` 3e9; any arm
other than `a` is stopped at 1.5 times the incumbent's touches on the same
target and recorded `cap_touch` (a lower bound on its cost: it cannot win
that target, and is counted at the cap, a censored loss); matrix cap 2^27
words; wall cap 150 s per solve.

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
  `(8,2,4)` (32 test + 8 planted targets, models trained on their own 32
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
1.5x (counted at the cap, a lower bound on its cost).

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
   verdict computed with any censored arm target says so (censored arm
   targets are counted at 1.5 times the incumbent, a loss for the arm).

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

One thread per job, `prlimit --as=4294967296`. Expected measured work is about 82 CPU-minutes (calibration wall times:
cell `(17,4,3)` 520 s, `(19,4,3)` 1,180 s, `(21,5,3)` 2,700 s, `(13,3,4)` 520 s,
controls about 100 s) against the 2 CPU-hour cap (7,200 s). The per-command
timeouts of the measured commands sum to 7,189 s, so even if every command ran
to its timeout the cap holds. There are 26 cards, `experiment_id: BLINDSPOT-09`,
`background: false`; each builds the instrument with `rustc` into
`target/blindspots-09/card-NN/` and removes it, and the timeouts of every card
sum to under 530 s including the build (under 9 minutes). Per-command
`--wall 150` and the touch caps bound a runaway. Peak resident memory is below
1 GiB at every design cell. Nothing in `src/` or `Cargo.toml` is touched.

Order: card 1 (controls P1, P2, P3), then per cell train chunks (each writes a
sample file and a trace file), `fit` (writes the frozen model), then test
chunks; the last card runs `report` on the raw JSON lines. The model of a cell
is fitted from its 32 train targets alone and is not changed afterwards. Card
contents, in run order:

* card 01: selftest, control-p1, train-p3-8-3-3, fit-p3-8-3-3, eval-p3-8-3-3-test, eval-p3-8-3-3-planted, train-p3-8-2-4, fit-p3-8-2-4, eval-p3-8-2-4-test, eval-p3-8-2-4-planted
* card 02: train-17-4-3-00-16, train-17-4-3-16-32, fit-17-4-3
* card 03: eval-17-4-3-test-00-16, eval-17-4-3-test-16-32
* card 04: eval-17-4-3-test-00-03-o, train-19-4-3-00-08, train-19-4-3-08-16, train-19-4-3-16-24
* card 05: train-19-4-3-24-32, fit-19-4-3, eval-19-4-3-test-00-07
* card 06: eval-19-4-3-test-07-14
* card 07: eval-19-4-3-test-14-21
* card 08: eval-19-4-3-test-21-28, eval-19-4-3-test-28-32
* card 09: train-21-5-3-00-06
* card 10: train-21-5-3-06-12
* card 11: train-21-5-3-12-18
* card 12: train-21-5-3-18-24
* card 13: train-21-5-3-24-30, train-21-5-3-30-32, fit-21-5-3
* card 14: eval-21-5-3-test-00-03
* card 15: eval-21-5-3-test-03-06
* card 16: eval-21-5-3-test-06-09
* card 17: eval-21-5-3-test-09-12
* card 18: eval-21-5-3-test-12-15
* card 19: eval-21-5-3-test-15-18
* card 20: eval-21-5-3-test-18-21
* card 21: eval-21-5-3-test-21-24
* card 22: eval-21-5-3-test-24-27
* card 23: eval-21-5-3-test-27-30, eval-21-5-3-test-30-32
* card 24: train-13-3-4-00-32, fit-13-3-4
* card 25: eval-13-3-4-test-00-32, eval-13-3-4-test-32-64
* card 26: report-17-4-3, report-19-4-3, report-21-5-3, report-13-3-4, report-p3

Per-cell settings: `(17,4,3)` cap 6e9, arms a b50 b25 c d p r50 r25 on test
targets 0 to 31 plus `o` on 0 to 2; `(19,4,3)` cap 1e10, same arms, 0 to 31;
`(21,5,3)` cap 3e10, arms a b50 c d p r50 (the `q = 0.25` arms are dropped here
for budget: they are secondary), 0 to 31; `(13,3,4)` cap 3e9, same arms as
`(17,4,3)`, test targets 0 to 63 (about half are satisfiable and censored, so
64 are drawn to leave about 32 usable). Every eval is run with `--rel-cap 1.5`.
Train chunks use the same cap (satisfiable train targets run to the cap, and
their partial runs still give labels).

## Calibration disclosed (not reused)

Before this freeze the instrument was run unmeasured on seed `1` (factor
bases and targets that appear in no card) to choose the caps, the card sizes
and the policy form; the outputs are discarded. What was seen, so a reader can
judge the freeze:

* The engine refutes every unsatisfiable calibration target at step degree 4
  (`D = 4`), costs 1.8e9 touches (1.7 s) at `(17,4,3)`, 3.8e9 (3.2 s) at
  `(19,4,3)`, 1.7e10 (9 s) at `(21,5,3)`, 0.9e9 (1 s) at `(13,3,4)`, and the
  last degree-4 step is almost all of it. Satisfiable targets (24 to 106
  solutions at `(13,3,4)`, 6 to 35 at `(17,4,3)` planted) need degree-5
  matrices above the 1 GiB cap or run past 8e10 touches.
* With 4 to 8 train targets: the learned subset (`b50`) gave ratios of about
  2.0, 2.3 and 2.7 at `(17,4,3)`, `(19,4,3)`, `(21,5,3)` and lost at `(13,3,4)`
  (0.9); a random half (`r50`) gave about 7.0 and 6.8 at the first two, 0.7 at
  `(21,5,3)` and 0.6 at `(13,3,4)`; the learned row order (`c`) gave 1.5, 1.04,
  1.00 and 1.1; trace transfer (`d`) lost everywhere (0.2 to 0.7); the oracle
  subset reached 6.8 at `(17,4,3)`. These are 4 to 6 targets per cell, seed 1.
  The first policy (a score threshold keeping 98% of useful pairs) kept most
  pairs and gave 1.00; it was replaced by the top-fraction rule and the random
  null was added after seeing that a random half already beats the incumbent
  at two cells. The primary fractions were fixed to `q = 0.5` (primary) and
  `0.25` before any frozen target was solved.
* P1 on item 07's table (max 34 unknowns): degree equal to the committed
  closure degree on 92 of 92 refuted draws and to the committed msolve degree
  on 40 of 40; 3 draws reached the calibration cap.
* The first calibration of `d` ran past its cap because the cap was checked
  between steps; the cap is now checked every 256 rows inside the
  elimination.
* The fit's row-order target is dominated by the default order at the larger
  cells (the cheapest of five orders was the default on the sampled matrices
  at `(21,5,3)`), so `c` is expected to equal `a` there; it is measured anyway.

## Deviations from the frozen design

1. **`rustc` instead of `cargo build --example`** (the crate library does not
   compile at the branch base: 25 errors). The instrument is std-only, copies
   the item-07 system builder, and each card builds its own copy (a built
   binary cannot be bound by sha256 before it exists); the first card builds it
   too.
2. **The incumbent is a replica of `pq_f4_f2`'s step, not a call to the crate.**
   The crate cannot be linked (Deviation 1) and has no selection hook. The
   replica follows its pair update, field pairs, symbolic preprocessing and
   counting convention; it has no M4RI tables, so `xors` is the row-by-row
   logical count (the quantity `pq_f4_f2` reports as `word_xors`), and wall time
   is descriptive only. Sparse pivot storage speeds the XOR without changing
   the count. Fidelity is checked by P1 (committed degrees) and P2, not by a
   side-by-side run of the crate engine.
3. **"Column touches" is defined as `xors + mat_words`** (above); the design
   does not define it. `xors` alone and `rows` are reported beside it.
4. **Refutation, not complete solves, at the design cells.** Satisfiable
   targets need degree-5 matrices above the matrix cap; they are censored at
   the incumbent. Basis equality is therefore checked as `{1}` identity at the
   design cells and on non-trivial bases at control cells P3 (an addition).
5. **One factor base per cell, 64 targets** (32 train, 32 test), instead of
   64 independent systems: the oracle's real workload and the only reading of
   "same shape" under which a trace can transfer.
6. **Selection is a top-fraction rule, not a threshold.** The first version
   thresholded the classifier at 98% recall of useful pairs; on calibration
   targets that kept about 50 to 80% of the pairs and changed nothing, so the
   policy is now "the top `ceil(q n)` pairs by score", with the same-fraction
   random null `r<q>`. `q = 0.5` is the primary, `q = 0.25` secondary; the
   primary is fixed here and not chosen after the test data.
7. **Oracle definitions are mine** (the design names the pair oracle only):
   greedy useful-pair label for (b) and (d); cheapest of five candidate
   orders for (c). `d` pairs train target `i` with test target `i`.
8. **Additions (stronger incumbents and controls, never a loosening):**
   nulls `r50`, `r25`, `p`; headroom arm `o`; control cells P3; P1/P2; the
   matched-null and rows tightenings; the rule applied at three weights of
   `w`; censoring at 1.5x the incumbent.
9. **Costs of a pair selection that defers pairs are charged in full**: every
   extra step's matrix, reducers and elimination are in `touches`; policy work
   is in `policy_ops`; only offline labelling and fitting are outside, and
   they are reported per cell.
10. **Cell `(13,3,4)` draws 64 test targets** (design: 32) because about half
    of its targets are satisfiable and censored; all usable targets are
    measured. The m = 3 cells use the design's 32.
11. **The `q = 0.25` arms are not run at `(21,5,3)`**, for the CPU budget; they
    are secondary and never decide a verdict.
12. **Relative cap 1.5.** Arms other than `a` are stopped at 1.5 times the
    incumbent's touches on the same target and counted at that cap (a lower
    bound), so a slow arm cannot burn the budget. An arm that wins is not
    affected.

## Outputs

`research/blindspots_20261018/09-learned-f4-policy/`: `PROTOCOL.md`,
`card-NN.yaml` (frozen, linted), `raw/` (JSON lines, sample files, model and
trace files, receipts, `report-*.txt`), then `README.md` (verdict by the frozen
rule), an SVG and PNG graph and `REPORT.pdf` after the runs. A
`benchmarks/20261018-blindspots-learned-f4/` entry and a leaderboard component
section are warranted only if a verdict reads ADVANCES or the random-subset
finding stands; the integration step decides.

## Repair pass r2 (2026-10-18, experiment-director)

The first run stopped while card 13 of 26 was executing. This section records
what happened and how the unfinished work is re-frozen. **The question, arms,
cells, draws, caps, metric, controls and decision rule above are unchanged.**
No instrument change: the first run did not prove an instrument bug, and
`examples/blindspots_09_learned_f4.rs` is byte-identical (sha256
`7e46e27eaf9f5901f74772e9fd2a5f95457ab3182cda7ffcad320fcd820e5743`, bound by
every r2 card). A fresh `rustc` build was smoke-tested before freezing: `selftest`
(4 PASS lines, SELFTEST OK, 78 s), `control` P1 (92 of 92 and 40 of 40, 24 s),
the full P3 chain on both control cells (train, fit, eval test and planted, all
nine arms, 37 s, all `check` true), and at every design cell
`(17,4,3)`, `(19,4,3)`, `(21,5,3)`, `(13,3,4)` a one-target train, fit and eval
with the cell's arm list (no panic, every `check` true), plus the `o` arm and
the two-file trace list at `(17,4,3)` with the real first-run model. Smoke
outputs were scratch and are not records.

### What the first run left (receipts under `raw/`)

| cards | first-run receipt | cause |
|---|---|---|
| 02, 09, 10, 12 | completed | none; their outputs are reused as bound inputs |
| 01 | stopped at command 3 | `selftest` took 78 s against a 62 s card timeout; 3 PASS lines printed; the 11 later commands never ran |
| 05 | stopped at command 4 | `fit-19-4-3` exit 101: `model-19-4-3.samples.0` absent (card 04 never ran) |
| 06, 07, 08 | stopped at command 3 | `eval` exit 101: `model-19-4-3.model` absent |
| 03, 04, 11, 13 to 26 | none, never ran | 03, 04 and 26 name existing first-run outputs without a bind, so `lint` refuses them; 11 and 13 were not reached (cell-21 chunks 12 to 18 and 24 to 32 are missing); 14 to 25 need the cell-21 and cell-13 model files, which no completed card produced |

Cards 05 to 08 therefore failed on missing inputs, not on the instrument; no
number from a stopped card is used. The 0-byte `eval-19-4-3-*.jsonl` files and
the `train-19-4-3-24-32` output of the stopped card 05 are not reused (that
chunk is re-run in r2). Nothing from the first run is overwritten; every r2
output carries the suffix `-r2` and every r2 card id the suffix `-r2`.

### Card rules applied to r2

Notes read "The repository root is /home/user/crypto. Run the command through the
Bash tool with timeout 600000."; `background: false`; the per-command timeouts
of each card sum to at most 460 s (card actual time estimates 100 to 300 s);
each card builds its own binary and removes it; a card reads a file from
another card only if that file is committed and bound by sha256; list-valued
arguments (`--arms`, `--trace`, `--samples`) are single quoted strings.

### Layout (replaces the card-content list in "Budget and execution" for the unfinished work)

Cards are written by `make_r2_cards.py` in three stages, because a consuming
card cannot be frozen before the file it binds exists and is committed.

* **Stage 1 (frozen now):** `01-r2` selftest and P1; `01b-r2` P3 control cells
  (train, fit, eval, self-contained); `03-r2`, `03b-r2` cell `(17,4,3)` test
  0 to 16 and 16 to 32, binding the completed card 02's `model-17-4-3.model`,
  `.trace.0`, `.trace.16`; `04-r2` the `o` arm at `(17,4,3)` targets 0 to 2 and
  cell-19 train 0 to 16; `05-r2` cell-19 train 16 to 32; `11-r2` cell-21 train
  12 to 18; `13-r2` cell-21 train 24 to 32; `24-r2` cell-13 train 0 to 32 (two
  chunks). Train chunks write `model-<cell>-r2.samples.<lo>` and `.trace.<lo>`.
* **Stage 2 (frozen after every stage-1 receipt is committed):** the evaluation
  cards `05b`, `06`, `07`, `08` (cell 19, 8 targets each), `14` to `23`, `23b`
  (cell 21, 3 targets each, last 2) and `25`, `25b` (cell 13, 32 each). Each
  begins with `fit` from the bound sample files (cell 21 binds the first-run
  chunks 0, 6, 18 and the r2 chunks 12, 24, 30), then evaluates with the
  model it just wrote. The fit is deterministic: refitting the cell-17 first-run
  samples reproduced the committed `model-17-4-3.model` and `fit-17-4-3.json`
  byte for byte, so every card of a cell obtains the same model, and the model
  digest is in each receipt (a differing digest within a cell fails the item).
  No fit input contains a test target.
* **Stage 3 (frozen after every stage-2 receipt is committed):** `26-r2`, the
  `report` runs, binding every `-r2` eval record.

Eval chunk sizes follow the measured wall time per test target (smoke:
`(17,4,3)` 11 s, `(19,4,3)` 27 s, `(21,5,3)` 66 s) so that each card stays under
8 minutes; chunk boundaries do not enter any statistic (the report pools the
records of a cell).

### Budget

Expected r2 CPU time is about 1,450 s (stage 1), 3,300 s (stage 2) and 10 s
(stage 3), plus about 715 s already spent by the completed first-run cards, near
5,500 s against the 7,200 s cap. The per-command timeouts of the r2 measured
commands sum to 8,790 s, so unlike the first freeze the timeouts alone do not
guarantee the cap; the executor's receipts give the running total and the
assessor stops the item if the summed `seconds` of measured commands reach
7,200.

## Repair pass r3 (2026-10-18, experiment-director; stage 2 and the cell-(13,3,4) re-freeze)

State after the r2 stage-1 run (receipts under `raw/`): completed are 02, 09,
10, 12 (first run) and 01-r2, 01b-r2, 03-r2, 03b-r2, 04-r2, 05-r2, 11-r2, 13-r2
(r2); stopped are 01, 05, 06, 07, 08 (first run) and 24-r2 (timeout, 13 of 13
incumbent runs `cap_touch`); never run are the cell-19, cell-21 and cell-13
evaluation cards and the report card. **Question, arms, cells, draws, metric,
controls and decision rule are unchanged, and the instrument is byte-identical**
(sha256 `7e46e27e...0e4743`, bound by every card). No receipt or output from an
earlier pass is overwritten: new cards carry `-r2` ids, new cell-13 outputs the
suffix `r2b`.

### Deviation 13: cell (13,3,4) touch cap raised from 3e9 to 6e10

Proven by the r2 run, not assumed: `raw/train-13-3-4-00-16-r2.jsonl` has all 13
incumbent runs observed at `cap_touch` (3.02e9 to 3.93e9; 12 unsatisfiable, 1
with 24 solutions). The frozen 3e9 was set from seed-1 calibration (0.9e9
touches per refutation), which did not predict the frozen-seed factor base. A
scratch run (not a record) of train target 0 at cap 4e10 refuted it at 2.19e10
touches in 20 s, `check` true; one eval target (test 0, all eight arms, cap
4e10, 97 s) gave `a` 3.2e10 touches, all arms `check` true. Cap 3e9 would censor
every target, so the cell could not be measured at all. The new cap is **6e10**
for training (cards 24b to 24e) and for the later evaluation cards; the
relative cap 1.5 and every other setting are unchanged. A satisfiable train
target (idx 12, 24 solutions) ends `cap_matrix` at 2.96e10 touches in 36 s, so
satisfiable targets stay censored as before. This changes the censoring
threshold only; a target whose incumbent exceeds 6e10 is censored and counted.

### Budget, with the cell-13 cut-off stated in advance

Measured-command seconds in all receipts so far: 2,221 (of the 7,200 s item
cap; the assessor sums the receipts' `seconds`). Smoke times per target with the
cell's full arm list: `(19,4,3)` 26 s, `(21,5,3)` 62 s, `(13,3,4)` 97 s (train
target about 20 s, satisfiable about 36 s). Stage 2 for cells 19 and 21 (32
targets each) is about 830 s + 2,000 s, and the cell-13 train 32 targets about
700 s, which brings the running total to about 5,750 s. That leaves roughly
1,400 s, enough for about 12 `(13,3,4)` test targets (4 cards of 3), not the 32
usable the design asks for. The cell-13 evaluation cards are therefore **not
frozen in this pass**: they bind the sample files of cards 24b to 24e, which do
not exist yet, and are sized afterwards from the receipts' actual seconds (stage
2c). If the remaining budget supports fewer than the design's 32 usable test
targets, that is a further, named deviation of that pass, and the assessor
reports the cell as measured on fewer targets, not as the design's cell. The
cell-13 cards run last, so the 7,200 s stop cannot cut a cell-19 or cell-21
evaluation.

### Layout (stage 2 frozen now; 2b frozen now; 2c and 3 later)

* **Stage 2 (this pass):** `05b-r2`, `06-r2`, `07-r2`, `08-r2` (cell 19, test 0
  to 8, 8 to 16, 16 to 24, 24 to 32), `14-r2` to `23-r2` (cell 21, 3 test
  targets each) and `23b-r2` (targets 30 to 32). Each card first runs `fit` on
  the bound sample files (cell 19: the four r2 chunks; cell 21: first-run
  chunks 0, 6, 18 and r2 chunks 12, 24, 30), then `eval` against the model it
  just wrote and the bound trace files. A refit is byte-deterministic (the
  cell-17 refit reproduced the committed model), so every card of a cell obtains
  the same model; each card writes its own `model-<cell>-r2-c<id>.model` and the
  assessor checks the digests agree. The 19 and 21 flows were smoke-tested with
  the real bound files at one test target each (arm lists as in the cards, all
  `check` true, no panic) before freezing.
* **Stage 2b (this pass):** `24b-r2` to `24e-r2`, cell-13 training in four
  chunks of 8 targets (0 to 32), cap 6e10, outputs `model-13-3-4-r2b.samples.<lo>`
  and `.trace.<lo>`, records `train-13-3-4-<lo>-<hi>-r2b.jsonl`.
* **Stage 2c (after 24b to 24e are committed):** the cell-13 `fit` plus `eval`
  cards, sized to the remaining budget.
* **Stage 3 (after every evaluation receipt is committed):** `26-r2`, the
  `report` runs.

`make_r2_cards.py` gained `stage2b` and the `cap` and `sfx` parameters of
`train`; `stage2` no longer writes the cell-13 cards (`25`, `25b`), which
cannot be frozen before their inputs exist.

## Repair pass r3b (2026-10-18, experiment-director; restart during card 18-r2)

A container restart at 19:42 killed the run while card 18-r2 was executing.
Receipts: completed are 01-r2, 01b-r2, 03-r2, 03b-r2, 04-r2, 05-r2, 05b-r2,
06-r2, 07-r2, 08-r2, 11-r2, 13-r2, 14-r2, 15-r2, 16-r2, 17-r2 (and first-run
02, 09, 10, 12); stopped are 01, 05, 06, 07, 08 and 24-r2; **no receipt** for
18-r2 (killed after its `fit`, before any eval record), 19-r2 to 23b-r2,
24b-r2 to 24e-r2, the cell-(13,3,4) evaluation cards and the report card.
**Question, arms, cells, draws, caps (incl. Deviation 13), metric, controls and
decision rule are unchanged; the instrument is byte-identical** (sha256
`7e46e27e...0e4743`).

* The killed card left a binary at `target/blindspots-09/card-18-r2/b09` (a
  build product, deleted; it made lint refuse the card) and committed partial
  outputs at the `-r2` paths (`card-18-r2-build.*.log`, `fit-21-5-3-c18-r2.*`,
  `model-21-5-3-r2-c18.model`). None is a record; they are left in place, not
  overwritten. Card 18 is re-frozen as **`card-18-r2b.yaml`** (id
  `CARD-BLINDSPOT-09-18-r2b`), same command line, every output under the suffix
  `r2b` (`model-21-5-3-r2b-c18.model`, `fit-21-5-3-c18-r2b.*`,
  `eval-21-5-3-test-12-15-r2b.jsonl`). The suffix `r2b` is used because the
  `-r2` paths of card 18 are taken. `card-18-r2.yaml` must not be run.
* Cards 19-r2 to 23b-r2 and 24b-r2 to 24e-r2 already obey the card rules and
  lint OK; they are unchanged and are run as frozen. `make_r2_cards.py`
  regenerates every committed r2 card byte for byte (checked), gained the
  `stage2r` stage and a card suffix, and its `stage3` template binds
  `eval-21-5-3-test-12-15-r2b.jsonl` for the 12-to-15 chunk of cell (21,5,3).
* Smoke test before freezing (scratch, not a record): refitting the cell-21
  model from the bound sample files reproduced `model-21-5-3-r2-c17.model` and
  the partial `...-c18.model` byte for byte; one-target eval of test 12 with
  the card's arm list ran with every `check` true and no panic (55 s).
* Still staged (rule: a card is frozen only after its input is committed): the
  cell-(13,3,4) fit-plus-eval cards after 24b to 24e have run, and the report
  card `26-r2` (stage 3) after every evaluation receipt is committed.
* Budget: measured-command seconds in all receipts so far 3,583 of 7,200; the
  assessor sums them again and stops the item at 7,200.
