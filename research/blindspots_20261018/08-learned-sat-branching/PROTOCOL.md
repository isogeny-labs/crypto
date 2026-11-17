# Item 08 protocol: learned branching for WDSat and XOR-CDCL on Semaev CNFs

Frozen 2026-10-18, before any measured run. Source design: section "08" of
`research/blindspots_20261018/EXPERIMENTS.md` and its "Rules every item
follows". Written by the experiment-director lane. The design is tightened in
places and loosened nowhere; every deviation and every filled-in gap is listed
at the end.

Class: **stage diagnostic** (the branching rule of one SAT-style oracle on toy
fields). It computes no `S`, no rho ratio and no GAE, so the section 5a
cross-method ranking does not apply to its own rows. The section 5a band is
nevertheless applied the way it is meant: every unpriced native operation of a
scorer (`W`, defined below) is priced at 0, 0.1 and 1 search events per
operation, the same for every arm, and a conclusion is drawn only where it
holds at all three weights. The gap to matched rho is unchanged by this item and
is reported as the unchanged distance remaining.

## Question

Can a learned variable-selection policy cut WDSat's work on `m = 3` Semaev
instances relative to the static order WDSat uses?

## What already exists, and what is reused (read before designing)

* **The incumbent, the instance family and the CryptoMiniSat arm are already
  measured on 60 instances; that result is reused and cited, not re-measured.**
  `crypto-autoresearcher/experiments/EXP-ICPERF-66fd51/runs/RUN-ICPERF-305ca3/results.jsonl`
  (sha256 `429f455a...e0f0`, in the tree after PR 2331) holds, for the 60 Trimoska
  `EC-Index-Calculus-Benchmarks` instances `n15l5`, `n17l6`, `n19l6` (ids 1-10
  `S`, 11-20 `U`; git `a053971c`), stock WDSat's printed decision counter under
  the default order (and `-g 1..3l`, which equals it) and CryptoMiniSat 5.11.15
  on the CNF+XOR form (conflicts, decisions, propagations). They are transcribed
  into `corpus60.tsv` (this directory) with provenance. Findings used here:
  an unsatisfiable instance costs 30,874 to 31,448 WDSat decisions at `l = 5`
  and 254,599 to 255,969 at `l = 6`, i.e. 0.94 of the `2^(3l)` leaves: the
  default order prunes only at the last levels; a bad order (`e`-variables
  first, "noncore_first") costs 10^2 to 10^3 times more; CryptoMiniSat
  5.11.15 needs 2.4 to 11.4 times more decisions than WDSat on the 28
  unsatisfiable instances it finished (two timed out at 300 s) and 0.9 to 254
  times on the satisfiable ones; satisfiable instances cost WDSat 10 to 144,329
  decisions, depending on where the first model lies.
* **No learned branching rule, no non-default static order and no dynamic rule
  has been measured on this family.** `research/pdp_oracle_speedup_20261011/protocols/PS-7.md`
  (dynamic order on a different oracle, "expected negative", not run),
  `research/chain_split_order_20260924/` (split variable of an inherited-F4
  tree, a different object) and `pdp_smt_oracle_20261008` (SMT, not WDSat)
  are the nearest work; none measures this. Nothing is skipped on their account.
* **WDSat is not installed and its source cannot be linked.** The vendored source
  (`crypto-autoresearcher` ICPERF `builds/wdsat_987b7f5ef9/src`) is C with static
  allocation recompiled per instance size, prints only the decision counter (no
  propagation count), and its `-g` option takes a static order only, so a dynamic
  scorer cannot be hosted. The instrument is therefore a **native Rust replica of
  the WDSat search**, and the replica is validated against the stock binary's
  committed counts (Controls, `corpus`): it must reproduce all 60 decision
  counts exactly. In the smoke test it did (20/20 at `n15l5`; the other 40 are
  run by the first card).
* `src/cryptanalysis/wdsat_oracle.rs` (the repository's oracle wrapper) passes
  `-g 1..m*l` and a chained `S_3` system; it is not the corpus formulation and
  has no learned order. It is the reference for the incumbent's `-g` semantics
  only.
* The crate does not compile at this branch base (`cargo build --release --lib`:
  25 errors in `ecbench` and `gaudry_quartic`, re-checked 2026-10-18), so no
  `cargo build --example` can link. The instrument is std-only
  (`examples/blindspots_08_learned_branching.rs`) and each card builds it with
  `rustc`, as items 04, 05, 07 and 11 do.

## Objects

* `F = GF(2^n)`, `n` odd, modulus the repository's sparse irreducible (same
  candidate order as `find_irreducible_sparse`); the 60 corpus instances use the
  corpus modulus (Conway). Curve `y^2 + xy = x^3 + x^2 + 1` (the corpus curve;
  `S_3` does not depend on `a_2`). Factor base `V = span{1, z, .., z^(l-1)}`,
  one base per `(n, l)`, so "same factor bases" holds within a cell.
* **The system** is the corpus `S'_4` formulation (Trimoska, Ionica, Dequen):
  unknowns `x1, x2, x3 in V` as `3l` bits, then the bits of `e1 = x1+x2+x3`
  (`l`), `e2 = x1x2+x1x3+x2x3` (`2l-1`), `e3 = x1x2x3` (`3l-2`): `9l-3`
  unknowns; the `4l-1+3l-2` definition rows plus the `n` bit equations of
  `S_4 = t^4 + t^3 e3 + (1+t^4) e3^2 + t e3^3 + e3^4 + t^2 e2^2 + t^3 e2^2 e3 +
  t^4 e2^4 + t e1^2 e3 + t^2 e1^2 e3^2 + e1^4`, `t = x_R` (the resultant of
  `S_3(x1,x2,u)` and `S_3(u,x3,x_R)` written in `e1,e2,e3`; found by an exact
  `F_2` fit with full rank and re-checked in the self-test on 300 random field
  points against the resultant). It requires `n >= 3l-2`.
* **The solver** (`Sys`, `St`, `dfs`): atoms for monomials of degree >= 2 with
  `a -> x_i` and `x_1 & .. & x_d -> a`, XOR rows with constants, unit propagation
  on both to a fixpoint, depth-first search over a branching order, value 0
  first, one decision per node whose left branch is tried (WDSat's `conf[0]++`).
  Candidates are the `3l` subspace bits (WDSat `-g 1,..,3l`); the `e` bits are
  determined by propagation, and a model is accepted only after every ANF row is
  re-evaluated on it. **No Gaussian elimination** (WDSat without `-x`, the
  default the corpus results were taken with).
* **Counted unit** per instance and arm: decisions `D`; propagated assignments
  `P` (atoms included, decision literals excluded); probe assignments `Q`
  (lookahead a scorer performs, charged in full); and `W`, the unpriced native
  work of the arm (incidence scans while extracting features, model arithmetic,
  sorting, order-pointer steps). **Cost at weight `k`** is `D + P + Q + k W`,
  `k in {0, 0.1, 1}`. `D + P` at `k = 0` is the design's "decisions +
  propagations"; `k = 0.1, 1` are the section 5a band, priced identically for
  the incumbent (its `W` is the order-pointer steps). Wall time is descriptive.
* **Target populations** (the design says "held-out targets"; this fills the
  gap): **P**, planted: `x_R = x(P1+P2+P3)` for random factor-base points, built
  through the chain roots, so a decomposition exists; **R**, uniform random
  `x_R != 0`, almost surely unsatisfiable (the corpus `U` class). Both are in
  every measured cell, half and half. Satisfiability is cross-checked by the
  exact chain count and, with `--truth`, by the exact count over all `2^(3l)`
  triples by field arithmetic.

## Arms

All arms solve the same system by the same engine; only the branching rule
differs. All are exact (a verdict that disagrees with another arm or with the
exact count aborts the card).

| arm | rule |
|---|---|
| `a` | incumbent: WDSat's order, the `3l` subspace bits in index order (block-major: `x1` bits, `x2` bits, `x3` bits). Equals stock WDSat default (Controls). |
| `a2` | **non-learned structural order** (addition): bit `t` of all three blocks before bit `t+1` (`e`-coordinate `t` depends only on bits `<= t`). A learned rule must beat this too to be called learning. |
| `b` | **(b) WDSat with a learned scorer**: a static order, computed once at the root by scoring each candidate with a learned model over per-variable features (block, bit position, open quadratic and cubic atoms, rows holding the variable, rows through its atoms, smallest open row, free fraction, and the **current propagation counts** of setting it to 0 and to 1 with their conflict flags, from probes charged to `Q`) and sorting ascending by predicted log-cost. This is exactly what stock WDSat's `-g` could host. |
| `b2` | learned scorer **re-evaluated at every decision** (addition; structural features only, no probes, because probes cost `Q` at every node). Its scoring work is `W`. |
| `c` | **(c) learned cube selection**: for the first `dcut` levels, shortlist `k` candidates with the `b2` scorer, rescore the shortlist with a model that also sees the probe counts, split on the best; below the cutoff the incumbent order. Sequential cube-and-conquer: cubes in lexicographic order, stop at the first model. |
| `d` | **(d) random order** over the `3l` bits (negative control), one permutation per target. |
| `bs`, `b2s` | **shuffled-label controls** (addition): the same pipeline for `b` / `b2` with the per-state centred labels permuted among candidates, so the model is fitted to noise but has the same features, probes and cost. If a shuffled-label arm passes the rule, a pass of its learned twin is not attributable to learning. |
| `e` | **(e) CryptoMiniSat with XOR as external**: CMS 5.11.21 built from source (`git 5.11.21`, `-DSTATICCOMPILE=ON -DNOZLIB=ON -DNOM4RI=ON -DSTATS=OFF`, Release), flags `--verb 1 --threads 1 --maxtime 40`, input the same AND clauses plus native `x` lines. Counters are CMS's own (`decisions`, `propagations`, `conflicts`; CMS prints large counters rounded to K/M), so the row is descriptive and is never in the rule. Run on a subset (3 P + 3 R at `n = 31`); on the 60 corpus instances the committed CMS 5.11.15 rows are reused. |

**Learners** (Rust, in the instrument): ridge regression on standardised
features (`lambda = 1`), and gradient-boosted regression trees (40 trees,
learning rate 0.15, depth 2 for `b` and 3 for `b2` and the rescoring model,
minimum node 10, 24 quantile thresholds per feature, squared loss). **Labels**:
for a state (the root, or the state after a seeded random walk of
`d in {1,2,3,4,6,8}` decisions) and each unassigned candidate `v`, the label is
`log2(1 + D + P)` of the *complete enumeration* of the subtree below "branch on
`v` first, then the incumbent order" (so labels do not depend on where a
solution happens to lie), centred within the state. `b` is fitted on root states,
`b2` on all states, the rescoring model of `c` on states with depth `<= 8`
(with probe features). **Selection** among ridge and trees, and of `dcut in
{2,4,6}`, `k in {3,5}`, uses only validation instances (disjoint streams, sizes
of the training set), by the mean of `log2(1 + cost_{0.1})`; test targets never
touch training or selection.

## Controls

* **Instrument self-test** (`selftest`, first card and smoke test): field
  identities; the `e`-form of `S_4` equals the resultant and is symmetric in its
  four arguments (300 random points, `n = 31`); on six small cells (`n` 7 to 15,
  `l` 2 to 4, 42 draws) the engine's **find-all model count under three orders
  equals the brute-force count over the `2^(3l)` cores equals the count by field
  evaluation**, first-solution verdicts agree and every model re-evaluates; the
  chain count never exceeds the direct count; both SAT and UNSAT draws occur;
  ridge and trees recover synthetic laws; model text round-trips.
* **Positive control 1 (replica fidelity, `corpus`)**: regenerating the 60
  committed corpus instances from their (modulus, `x_R`) and running arm `a`
  must reproduce the stock WDSat decision count **exactly** on all 60, the
  SAT/UNSAT status, and the shipped certificates (`S` instances) must satisfy
  `S_4 = 0` and the generated rows. Any difference fails the card and voids the
  item (the instrument would not be WDSat).
* **Positive control 2 (planted targets)**: every P target is satisfiable by
  construction; every arm must find a model that re-evaluates; an arm that
  returns UNSAT on a P target aborts the card.
* **Negative control 1 (R targets)**: uniform random targets are the null of "a
  decomposition exists"; all arms must refute them, and they are the targets on
  which every ordering has (almost) the same tree, so a learned rule that beats
  the incumbent there is a genuine pruning gain, not search luck.
* **Negative control 2**: arm `d` (random order) and the shuffled-label arms
  `bs`, `b2s`.
* **Learning-pipeline control**: `fit` prints the oracle gain of the best first
  variable over the incumbent's first variable on the training roots (an upper
  bound for any first-variable rule) and the validation cost of the incumbent
  next to every learned candidate.

## Cells

`l = 5` throughout the rule (Deviation 2). Train `n in {23, 25, 27, 29}`: per
`n`, 3 P + 3 R sampling instances (role 3) at 7 states each; validation 2 P + 2 R
per `n` (role 5). Test `n in {31, 33, 35, 37}`: **32 targets per size, 16 P +
16 R** (role 7), 128 targets in the rule. Secondary, outside the rule:
`l = 6` at `n in {31, 37}`, 3 P + 3 R each (role 8), the ICPERF cell shape
(`n = 3l + 13`), with the `l = 5` models. Seeds: `20261018` for everything
measured; the smoke test uses seed 1, so no smoke instance recurs. The factor
base is fixed by `l`; the field modulus by `n`.

## Decision rule (frozen; `summarize` applies it mechanically)

For a candidate arm `X in {b, b2, c}` and the incumbent `a`, with per-target
ratio `r_i = cost_k(a) / cost_k(X)` over the 128 test targets, for `k in {0, 0.1, 1}`:
the median of `r_i` and its 95% **paired bootstrap** interval (target pairs
resampled with replacement, 10,000 resamples, seed `20261018 + index of k`,
percentile interval).

* **ADVANCES (constant)** iff, **at every `k`**, median `r >= 1.3` and the
  interval lies above 1. (The design: >= 1.3x median, interval excluding 1;
  tightening: all three weights, and pooled over all four held-out sizes.)
* **TRADES (constant)** iff the condition holds at `k = 0` but not at all three
  weights (a gain that exists only if the scorer's native work is free).
* **REGRESSES (constant)** iff at `k = 0` the interval lies below 1 and the
  median `r <= 1/1.1`.
* **INCONCLUSIVE** if fewer than 100 paired targets remain.
* **MATCHES** otherwise.
* **Attribution**: an ADVANCES/TRADES of `b` or `b2`/`c` is reported as not
  attributable to learning if a shuffled-label control (`bs`, `b2s`) also passes
  at `k = 0`.
* The same table is printed by size, by population (P, R) and against `a2`
  (descriptive; the verdict is by the rule against `a`, and a pass over `a` that
  does not survive `a2` is called a re-discovery of structure, not learning).
* Level is **constant** (a branching rule changes the cost of a tree walk, not
  the exponent of the oracle). Capped runs enter at their measured cost (a lower
  bound), are counted per arm, and a verdict with any capped candidate target is
  marked as a lower bound.

## Budget and execution

One thread, `prlimit --as=4294967296` per job; peak memory is a few MB (the
largest system has about 2,000 atoms at `l = 6`). At most 2 CPU-hours measured;
the card plan below is about 25 CPU-minutes. Each card finishes in under nine
minutes (per-command timeouts sum to less than that). Build products go to
`target/blindspots-08/<card>/` and are removed at the end of each card; the CMS
binary (static, 15 MB) is built once outside the cards in `target/` and bound by
sha256. Nothing in `src/` or `Cargo.toml` is touched.

Card plan (all `experiment_id: BLINDSPOT-08`, `background: false`; each card builds
the instrument with `rustc` into its own `target/blindspots-08/card-NN/` and
removes it; per-command timeouts of every card sum to under 540 s):

1. `selftest` and `corpus` on all 60 corpus instances (replica fidelity);
2. `samples` at `n = 23, 25`; 3. `samples` at `n = 27, 29`;
4. `fit` (model file, validation, shuffled-label controls);
5. test cells `n = 31, 33`; 6. test cells `n = 35, 37`;
7. secondary `l = 6` cell `n = 31`; 8. secondary `l = 6` cell `n = 37`;
9. arm (e), CryptoMiniSat on 3 P + 3 R at `n = 31` (binary bound by sha256);
10. `summarize`: primary table (the rule), secondary table, CMS table.

Cards 1 to 4 and 10 are in order; 5 to 9 need card 4's model file. The expected
total is about 25 CPU-minutes.

### Pilot calibration disclosed (not reused)

Before this freeze the engine was run unmeasured, on seeds 1 and 5 and on the
committed corpus, to choose `l` and the caps: unsatisfiable trees cost 30.8k
decisions (4.4M decisions+propagations, 0.4 s) at `l = 5` for every `n` in
23..37, 255k (44M, 3.7 s) at `l = 6`, a factor 8.3 per unit of `l`, and anything from 0.03%
to full for satisfiable targets, depending on where the first model lies;
identity, structural and random orders differ by 2% on unsatisfiable trees; the
first-variable rollout labels at a root span 0.06 bit (4%). Dynamic scoring
costs about 450 `W` per decision. A smoke pipeline (`smoke/`) ran the whole
chain at `n = 23` on 2 P + 2 R targets with seed 1. A timing dry run of the same
chain on seed 7 (6 sampling instances at `n = 23`: 59 s; a fit with 4 validation
instances: 14 s; one 32-target primary cell at `n = 31`: 66 s; CMS 5.11.21 on one
`l = 5` target: 12 to 30 s) set the card timeouts; its outputs are discarded and
no seed 7 or seed 1 instance is in a card.

## Deviations from the frozen design

1. **`rustc` instead of `cargo build --example`** (the crate does not compile at
   the branch base). The first card builds with `rustc`; each later card builds
   its own copy (a built binary cannot be bound by sha256 before it exists).
2. **`l` is fixed at 5 (secondary 6) instead of `l ~ n/3`.** The design gives the
   field degrees only. The incumbent's tree has `0.94 * 2^(3l)` leaves for every
   `n`, `P/D ~ 140`, and the replica runs at about 11 million assignments per
   second, so `l = 11` to 12 (`n ~ 33..37`, the factor base for which a
   decomposition is expected) would cost about `8e9..6e10` decisions, about
   `10^5` s (a day) per instance at `l = 11` and 8 times that at `l = 12`;
   `l = 7` already costs 2M decisions.
   The cells therefore keep the design's field degrees and sample sizes with a
   small factor base, which makes almost every random target unsatisfiable (hence
   the planted half). The published corpus is also `l = 5, 6`.
3. **Target mix**: 16 planted + 16 uniform per size (the design is silent).
4. **Arm (a) is the native replica**, validated against 60 committed stock
   counts; the stock binary is not re-run (not installed, prints no propagation
   count, cannot host a dynamic scorer). Arm (b) is therefore also measured in
   the replica, but is a static order, i.e. exactly a `-g` list.
5. **"Frobenius orbit index" is replaced by block index and bit position.** The
   factor base `span{1,..,z^(l-1)}` is not Frobenius-stable, so no orbit exists.
   Also "occurrence in XOR vs ANF clauses" is realised as rows holding the
   variable vs open atoms of degree 2 and 3 and rows reached through them.
6. **Additions** (stronger incumbents and controls, never a loosening): `a2`,
   `b2`, `bs`, `b2s`; the rule is applied at three weights of `W` and pooled
   over four sizes; ADVANCES must also survive the attribution check.
7. **Arm (e) on a subset** (3 P + 3 R at `n = 31`, 40 s cap each) because CMS
   needs 10 to 60 s per `l = 5` instance versus 0.1 to 0.5 s for the replica; the
   committed 60-instance CMS rows are reused for the corpus cells.
8. **Cube-and-conquer is sequential** (one thread): its cost is the sum over the
   cubes visited before the first model, with no parallel credit.
9. **Field modulus** is the sparse irreducible for new `n` (as items 05, 07, 11),
   the corpus Conway modulus for the 60 control instances.
10. **Held-out means disjoint instances and larger `n`** (`n = 31..37` vs training
    `n = 23..29`, different targets, field and base); the base is the same
    polynomial base for a given `l`, so what is held out is `n`, not `l`.
11. **Repair pass (`-r2` cards), 2026-10-18; question, arms, cells, metric and
    decision rule unchanged, no instrument change.** First-run status: cards 01 to 04
    completed (receipts `completed`; card 02's relay text said "not completed" but its
    receipt and outputs show it finished); cards 05 to 09 were **refused at lint** (each
    argv names `raw/model.txt`, written by card 04, which `binds` did not pin; no receipt,
    no output); card 10 **failed** (its `summarize` found no `raw/test-n31.jsonl`, because
    its inputs come from cards 05 to 08). The first run proved no instrument bug, so
    `examples/blindspots_08_learned_branching.rs` is unchanged (sha256 `02478185...a280a`,
    bound as before). `raw/model.txt` (sha256 `00f0d29d...b60809`, committed) is now pinned in
    `binds` of every card that reads it: the model is the one card 04 fitted, not refit.
    Revision cards `card-05-r2`, `card-06-r2` (test cells `n = 31, 33` and `35, 37`),
    `card-07-r2`, `card-08-r2` (secondary `l = 6`, `n = 31` and `37`), `card-09-r2` and
    `card-09b-r2` (arm (e), CryptoMiniSat) and `card-10-r2` (the three `summarize` calls).
    Outputs go to new `-r2` paths (`raw/test-nNN-r2.jsonl`, `raw/sec-l6-nNN-r2.jsonl`,
    `raw/cms-n31-{planted,random}-r2.jsonl`, `raw/summary-*-r2.txt`, `raw/receipt-card-NN-r2.json`);
    no first-run file is overwritten. Same commands, seeds (`20261018`), roles, draw counts
    and caps as the frozen cards 05 to 10; only the paths, `binds` and timeouts differ.
    Two changes in how work is carved up, neither a change of design: (i) **arm (e)** was
    run from a CMS 5.11.21 binary built outside any card and pinned by sha256; a card may
    run only binaries it builds, so each CMS card now clones tag `5.11.21`, checks out
    revision `4c9a6b6b...1794b` (the card fails if the tag moved), configures with the same
    flags as `cms-build.txt` (`-DSTATS=OFF -DNOZLIB=ON -DNOM4RI=ON -DSTATICCOMPILE=ON`, Release,
    `make -j2 cryptominisat5-bin`) and runs it; the build takes 35 to 50 s. The CMS arm is
    split in two cards so each stays under eight minutes in the worst case (`--cms-maxtime 40`
    times three targets): `card-09-r2` the planted targets (`--only 0,1,2`), `card-09b-r2` the
    random targets (`--only 16,17,18`), which together are the frozen 3 P + 3 R at `n = 31`.
    The rebuilt CMS binary is byte-different from the earlier one (build paths) but the same
    source revision and flags. (ii) **`summarize` is staged**: it reads the outputs of cards 05
    to 09b, and a card may read an earlier card's output only if that file is committed and
    bound by sha256, so `card-10-r2` cannot be frozen before they exist.
    `make_card_10_r2.py` writes it after those outputs and receipts are committed (it refuses an
    uncommitted input and omits a summary group whose receipts did not complete, printing
    what it omitted). Per-card timeouts now sum to at most 455 s. Smoke test before freezing
    (seed 1, `--np 1 --nr 1`, not a measured run, nothing enters the rule): the rebuilt
    instrument ran every cell shape (`l = 5` at `n = 31, 33, 35, 37`, `l = 6` at `n = 31, 37`)
    with `--truth` and `model.txt`, all arms agreed and no panic; the CMS card's command chain ran
    end to end (clone, pin, cmake, make, version, build, cell, cleanup: 86 s); the three
    `summarize` argument sets ran on those outputs (including `--cands none --controls e`).
    `make_cards_r2.py` regenerates the stage-1 cards.

## Outputs

`research/blindspots_20261018/08-learned-sat-branching/`: `PROTOCOL.md`,
`corpus60.tsv`, `card-NN.yaml` (frozen, linted), `smoke/`, `raw/` (JSON lines,
sample files, the model file, receipts, `summary-primary.txt`,
`summary-secondary.txt`), then `README.md` (verdict by the frozen rule), an SVG
and PNG graph and `REPORT.pdf` after the runs. A speed claim is not made (a
branching rule on toy fields); no `benchmarks/` entry or leaderboard row is
warranted; the integration step decides.
