# Item 13 protocol: the negation-map shortfall (H-RHO-bc6ea6) and an ECC2K-130 completion plan

Frozen 2026-10-18, before any measured run. Source design: section "13" of
`research/blindspots_20261018/EXPERIMENTS.md` and its "Rules every item
follows". Preregistered escape-rule space: hypothesis `H-RHO-bc6ea6`
(`/home/user/crypto-autoresearcher/ledger/hypotheses/H-RHO-bc6ea6.yaml`), whose
status this item does not change. Written by the experiment-director lane. The
design is tightened in places and loosened nowhere; every deviation and every
filled-in gap is listed at the end.

Class: **constant-factor cost measurement** of a generic-rho component (part A)
and **a costed plan from committed rates plus one native CPU rate** (part B).
Neither part touches an exponent: the generic square-root bound is fixed
(`KN-TECH-005`). The success criterion (CLAUDE.md) applies: the gap of every
row to the matched rho is the distance remaining, here the baseline is rho
itself, so the number reported is the gain of the negation walk over the
un-quotiented Teske walk and where that gain sits against the `sqrt 2` target.

## Questions

**A.** Over prime fields, does the negation map reach `sqrt 2` once
fruitless-cycle escape is charged, and where does it cross 1.0?

**B.** What is the full cost of completing ECC2K-130 with the repository's best
iteration function? Measure the CPU per-iteration rate of the current
`ecc2k130/` client natively, combine it with the committed GPU rate, write a
costed plan (GPU-hours, distinguished-point storage, cost at the committed
cloud prices). No claim beyond the measured rate.

## What already exists, and what is reused (read before designing)

* **`EXP-RHO-60da4f`** (`crypto-autoresearcher/experiments/EXP-RHO-60da4f/`,
  approved 2026-08-06, executed, `grid_analysis_final.json`,
  `controls_analysis.json`, `implementation.md`) is the earlier test of this
  same hypothesis. **None of its cells is reused**, for the reasons recorded
  there: it resolved the full grid only at `2^32`; at `2^40` and `2^48` it ran
  a reduced grid with 2 seeds per cell (`r` in {20, 32}, 8 DP bits only, 8
  rules at `2^40`, 4 at `2^48`); its baseline control failed at `2^40` and
  `2^48` (41.8% and 19.8% against its 0.886 sqrt N convention); its
  collision semantics were a single-walk proxy and then a multi-walk
  approximation, with escape rules that were pure functions of the current
  class, which re-absorbed in cycles of period above 2 (its own
  `implementation.md`, "specification gap"). No cell of this design
  (`2^40`, `2^44`, `2^48` times `r` in {16, 32, 128, 2048}, 64 targets, the
  rules below) exists there. Its finding that a 2-cycle-only rule re-absorbs
  in longer cycles is the prediction part A tests properly at `r` up to 2048.
  Its C1 convention (`0.886 sqrt N` for a plain van Oorschot-Wiener walk) is
  not reproduced here; see Deviation 2.
* **Ecbench toy records** `docs/bounds/records/prime-rho-neg.json` and
  `calibration-prime-rho-neg.json`: negation rho at the toy tier measures
  `S / sqrt(pi r / 2A)` = 1.466 [1.278, 1.653] and 1.453 [1.184, 1.772]
  (`canonicalisations_uncharged`). That is the ecbench harness's own ratio to
  a floor, with no Teske baseline in the same harness, no escape-rule variants
  and one escape rule fixed by the code. Context only; not a cell of this
  design.
* **`src/cryptanalysis/ecdlp_nist/rho.rs`** implements negation with doubling
  escape for 2-cycles and abandons longer cycles (its header). It is the
  incumbent code path the escape-rule arms stand in for; this item does not
  edit it.
* **The ECC2K-130 client and its committed numbers.** `ecc2k130/README.md`
  (status table, 12.6 M iterations/s per AVX-512 core, 15.7 M per M4 Pro core;
  sigma-table GPU rate; `2^60.9` iterations), `ecc2k130/WALK-CONSTANT.md`
  (`2^60.91` to `2^60.92` measured on completed trails),
  `ecc2k130/benchmarks/dp-interval/README.md` (`2^28.41` iterations per
  distinguished point at weight 32, `2^25.27` at weight 34, 132 workers),
  `ecc2k130/benchmarks/sigma-table/headline/result.json` (candidate median
  15884.369 M iterations/s on an RTX PRO 6000, paired ratio 1.0239 over a
  15513.653 control), `ecc2k130/RTX-PRO4500.md`, `FPGA-CEILING.md` section 5
  and `B200.md` (cloud price observations dated 2026-09-11 and 2026-09-16).
  **These GPU rates, work constants and prices are reused, cited by file, and
  not re-measured**: no GPU exists on this host. The CPU rate is measured
  natively here (the design requires it), not copied from the README.

## Part A: arms, controls, cells

### The walk engine

One r-adding walk engine (Teske), generic over a counted group, with a
van Oorschot-Wiener collision search: independent walks from a stride
sequence of start points, each run to its first distinguished point (DP), the
DP stored with its coefficients `(a, b)` for `aG + bQ`, the search ending the
first time two different walks report the same DP with different `b`, the
logarithm solved from the two coefficient pairs and **verified by a counted
double-and-add against `Q`**. A walk longer than `20 * 2^theta + 64` steps is
abandoned (a trapped walk). The run is censored when its operation budget,
`2.0 * sqrt(pi N / 2)` group operations, is exhausted. DP test: the low
`theta` bits of a keyed hash of the point (class) are zero; `theta = floor(bits/4)`
in the core cells (10, 11, 12). The branch of a step is another field of the
same hash. All arms of a target share the target, the jump table, the stride
and the hash salt: the arms are paired by target.

Groups (all in `examples/blindspots_13_rho_negation.rs`, std only):

* `ZGroup`: `Z/N` with the logarithm as the point, `N` the largest prime below
  `2^bits`. Used for every sweep cell. A group addition is counted, not
  executed as a curve operation; the dynamics are those of the cyclic group.
* `Curve`: a real short-Weierstrass curve `y^2 = x^3 + ax + b` over `F_p`,
  `p = 3 mod 4`, prime order found by point counting, affine arithmetic whose
  multiplications, squarings and inversions are tallied by the arithmetic
  itself.
* `TableZ`: `Z/N` whose DP keys and signs are those of the real curve's
  multiples of `G`. The **two-implementation agreement check** (hypothesis
  control C4): the same engine on `Curve` and on `TableZ` must agree in every
  counter, the DP digest and the recovered logarithm, for every arm, at
  `N ~ 2^20`, `r` in {16, 64}, 4 targets (selftest, run in card 1). The
  counted field work of the real curve must equal the per-operation constants
  times the group operations. The two implementations share the walk engine;
  they differ in the group arithmetic and in the source of the key and the
  sign, so this checks the group-level accounting and the equivalence of the
  `Z/N` model to a curve, not the engine against a second engine (Deviation 8).

### Arms (rules), 14 plus the A/A copy

Baseline, un-quotiented: `base` (Teske r-adding). Negation walk (the class
`{P, -P}`, canonical representative the one with the smaller sign key) with:

| name | rule | collision-preserving |
|:--|:--|:-:|
| `none` | no detection; positive control, expected absorbed | no |
| `dbl` | detect a 2-cycle by comparing with the point two steps back; escape by doubling the repeated point (design: "doubling escape") | no, the escape point depends on the entry phase |
| `min2` | the same detection; escape by doubling the smaller of the two cycle members (Bos-Kleinjung-Lenstra min-of-2) | yes |
| `la` | look-ahead: refuse a jump whose image would take the same jump back; try the next branch | yes |
| `per2`, `per4`, `per8` | running minimum over windows of k steps; two equal consecutive window minima are a cycle, escaped by doubling the minimum; k in {2, 4, 8} (from the hypothesis' k set) | yes |
| `per16`, `per32`, `per64` | the same, k in {16, 32, 64} (design: "periodic detection with period in {16, 32, 64}") | yes |
| `laper16`, `laper32`, `laper64` | look-ahead plus the periodic detection of k | yes |
| `baseb` | the baseline again on an independent target sample (A/A negative control) | n/a |

The hypothesis' `deterministic_representative_break` is `min2`'s escape (a
function of the cycle's content); its `lookahead_double_step` and
`branch_reselect_on_collision` are `la` (re-selection of the branch by
advancing it). "Collision-preserving" is this item's reading of the
hypothesis' memorylessness filter: two walks that fall into the same cycle
leave it at the same point, a function of the cycle's content and not of how
each walk entered, so distinguished-point collisions survive the escape.
`dbl` is measured and reported but **never counted as a winner**. A window of
k sees only cycles of length at most k (a window of 2 is a 2-cycle-only rule).

`r` in {16, 32, 128, 2048} (design), each jump `(m_j, alpha_j, beta_j)` random.

### Cells and unit

`bits` in {40, 44, 48} (the design's `log2 r`, read as `log2 N`, the subgroup
order), `r` in {16, 32, 128, 2048}, 14 arms plus `baseb`, **64 targets per
cell**: 12 cells (size by `r`), 840 records per size-r... precisely 64 targets
times 15 arms times 12 cells = 11,520 runs. Targets are `t` in 0..63 with seed
offset `7000000` (arms) and `9000011` (`baseb`), disjoint from the smoke and
pilot seeds (Deviation 7).

**Unit: counted group operations converted to field multiplications.**
Every group addition and doubling is counted, with set-up, the starting
stride additions, escape doublings, look-ahead trials and verification all
charged. Every unpriced native operation is counted separately: partition and
DP hashes (`hash`), canonicalisations (`canon`), cycle comparisons (`cmp`) and
DP-table probes (`lookup`). The summary prices both from the raw counts:

* group work, three models, in field multiplications (a squaring priced as a
  multiplication): **JAC30** (Jacobian mixed addition 11, doubling 10,
  inversion 30; every arrived point normalised to affine x for the hash, `I+S+M
  = 32`, plus 2 for the affine y when canonicalising), **AFF3** (affine,
  inversion 3 by batching: add 6, double 10), **AFF30** (affine, inversion 30:
  add 33, double 37). Field constants of the affine add (2M 1S 1I) and double
  (5M 2S 1I) are the ledger convention of `ic_boundary.rs`
  (`FIELD_OPS_PER_ADD`, `FIELD_OPS_PER_DOUBLE`) and are verified against the
  real curve's tallies in the selftest.
* native work at weight `nu` in {0, 0.1, 1} group-addition equivalents per
  operation (AGENTS.md section 5a), a GAE being the model's cost of one walk
  step's addition (JAC30: 43 including the normalisation, AFF3: 6, AFF30: 33).
  The same weights apply to every arm, the baseline included.

The nine pricing points (3 models by 3 weights) are the section 5a band. A
statement is **robust** only if it holds at all nine. Primary gain: **fully
charged JAC30, nu = 0** (set-up and verification included).
Gain = mean cost of `base` / mean cost of the rule, paired by target, with a
95% paired-percentile bootstrap interval (4,000 resamples). A cell where the
rule left any target unsolved at the budget cap is marked (a dagger): its cost
is a lower bound and its gain an upper bound; such a cell cannot support a
reach claim, and supports a loss claim conservatively.

## Controls

* **C1 baseline reproduction (blocking).** The plain walk, pooled over
  `r >= 128` and the two independent samples (`base`, `baseb`), must reproduce
  the **ideal random-map control** run under the identical protocol (same DP
  rule, walk cap and start protocol, no group and no coefficients, 512 targets
  per size): the 95% interval of the ratio of mean walk operations must overlap
  [0.92, 1.08]. The band is the old +-8 percent widened only by the sampling
  error of the two means. Reference value `sqrt(pi/2) = 1.253 sqrt N`
  (Deviation 2).
* **C2 ideal random-map control, positive control for the detector.** A walk
  driven by a keyed hash into the group, on `N` points and on its `(N+1)/2`
  classes: the ratio of mean steps to a DP collision must have an interval
  covering `sqrt 2` at every size. This proves the instrument can see a
  `sqrt 2` when there is one.
* **C3 no-escape positive control (blocking, hypothesis falsification
  condition 2).** `none` at `r = 16` must solve no target and have a
  capped-walk fraction at least half of the model `1 - exp(-2^theta / (2r))`
  at every size. If it does not, the cycle mechanism is wrong and the item
  reports that.
* **C4 two implementations** (above): selftest PASS in card 1.
* **A/A negative control.** `base` against `baseb`, unpaired, 12 cells:
  at most one of the 12 intervals may exclude 1 (a 95% interval misses about
  one cell in twenty by chance).
* **Positive controls for escape (selftest).** Windows of 4, 8, 16 and the
  three `laper` rules solve at least 5 of 6 targets at `2^40`, `r = 16`;
  baseline, `dbl`, `min2`, `la`, `per16`, `laper16` solve at least 5 of 6 at
  `r = 2048`; `none` is absorbed; no run in any selftest returns a wrong
  logarithm.
* **Per-step cycle rate (hypothesis prediction 3, HEUR-RHO-1).** Entries into
  a fruitless 2-cycle per draw, against `1/(2r)`, per size and `r`, for `dbl`,
  `min2`, `la`, `per16`; two estimators reported (DP-ending walks, and every
  step); the spread of the rate across sizes reported.
* **Size dependence (HEUR-RHO-2).** The fully charged gain per rule and `r`
  at each size, with a flag where the intervals of the smallest and largest
  size are disjoint.

An extension, **outside the frozen rule**: `theta` in {8, 14} at `2^44`, `r`
in {128, 2048}, arms `base`, `la`, `min2`, `per8`, `per16`, `laper16`, 64
targets. The shortfall depends on how long a walk runs before a DP, because a
walk trapped in a longer cycle is only abandoned at the cap; this extension
reads that dependence. It is reported separately and enters no verdict.

## Decision rule (frozen; applied mechanically to the summary by the assess stage)

The instrument's `summary` prints every quantity below. Gates first.

1. **Gates.** C1 at every size, C2 at every size, C3 at every size, the A/A
   count, the selftest. A failed gate at a size withdraws every comparison at
   that size (hypothesis falsification condition 1); the item is then
   INCONCLUSIVE at the constant level and says which gate.
2. **Reach.** A cell (size, collision-preserving rule, `r`) *reaches* 1.35 if
   its point gain is at least 1.35 at the most favourable of the nine pricing
   points; it *robustly reaches* if its point gain is at least 1.35 at all nine
   points, its JAC30 nu=0 95% lower bound is at least 1.35 and every target of
   the rule was solved.
3. **Hypothesis, first prediction** (no rule reaches 1.35):
   * **SUPPORTED** (at the constant level, for this walk and these sizes) if at
     every size no collision-preserving cell reaches 1.35;
   * **REFUTED** if at every size some collision-preserving cell robustly
     reaches 1.35 (the negation map then essentially achieves its target);
   * otherwise **INCONCLUSIVE**, stating per size which cells reach and at
     which pricing.
   `dbl` and any non-collision-preserving rule never count toward REFUTED; if
   one reaches 1.35 it is reported beside the verdict.
4. **Hypothesis, second prediction (a sign change).** A setting (rule, `r`) has
   a *clear net loss* at a size if its JAC30 nu=0 fully charged 95% upper
   bound is below 1.0, *robust* if also its best of nine points is below 1.0.
   If some collision-preserving setting has a clear net loss at all three sizes
   the sign change exists in the tested range and the settings are listed; if
   none does, the report uses exactly the words **no sign change inside the
   tested range**. The crossing is located per rule as the `r` at which the
   gain passes 1.0 (the tables list every `r`).
5. **Verdict vocabulary against the incumbent** (the un-quotiented Teske walk,
   level: constant). For each collision-preserving (rule, `r`) setting:
   **ADVANCES** if the lower bound of its fully charged gain exceeds 1.0 at all
   three sizes and its worst of nine points exceeds 1.0 at all three;
   **REGRESSES** if the upper bound is below 1.0 at all three sizes and its
   best of nine is below 1.0; **TRADES** if the gain is above 1.0 at some
   pricing points and below at others at some size (the order flips inside the
   band, so the setting is unresolved at that boundary, never a win);
   **MATCHES** if the intervals cover 1.0 at every size with no flip;
   **INCONCLUSIVE** otherwise or when the gate fails. The report also states
   each setting's distance to `sqrt 2` (the gain divided by 1.414) as the
   distance remaining; the gap to matched rho is that gain itself, since the
   baseline is rho.
6. **HEUR-RHO-1** is refuted if the primary-estimator rate departs from
   `1/(2r)` by more than a factor 2 at any tested `r` for `min2` or `la`, or
   moves across sizes by more than the sampling error. **HEUR-RHO-2** (size
   independence) is reported, not combined into the first verdict: it is
   refuted for a (rule, `r`) whose fully charged intervals at `2^40` and `2^48`
   are disjoint, and the report says how many of the collision-preserving
   (rule, `r`) pairs are flagged; a refutation caps that pair's constant to the
   size at which it was measured.

## Part B: arms, cells, rule

* **CPU rate.** The current `ecc2k130/` client, built from the committed
  `ecc2k130/src/main.cu` by the card with the Makefile's exact `ecc2k130-cpu`
  flags (the card's argv is the output of `make -n -B ecc2k130-cpu`, output
  redirected into `target/`), is run `--curve 131 --bench --threads 1 --steps 32
  --launches 800 --verify 0` five times with `OMP_NUM_THREADS=1`, one thread,
  `prlimit --as=4294967296`. The host load average before and after is recorded.
  Reported: the five rates, their median, minimum and maximum, in M
  iterations/s per core. The host is a shared 4-vCPU container at isolation
  level L0, and its load is recorded, not controlled: the rate is a statement
  about this host.
* **Plan.** `b13 plan` computes, from the five rates and the committed
  constants named above: expected work (`2^60.9`), DPs to collect and storage at
  weights 32 and 34 (32-byte records), CPU core-hours and core-years, GPU-hours
  and cost for the shipping sigma walk's sigma-table kernel on an RTX PRO 6000
  (15.884369 B iterations/s) priced at the four committed RTX PRO 6000 prices,
  and for the promoted x-only two-bridge map on an RTX PRO 4500 (12.499203 B/s,
  work factor 1.006696 from its frozen collision study) at its committed price,
  the break-even CPU core-hour price, the work at 50%, 90% and 99% completion
  under the Rayleigh first-collision model (labelled a model), and the
  wall-clock for fleets of 100, 1,000 and 10,000 GPUs.
* **Rule.** None. There is no hypothesis and no verdict: the output is the
  measured CPU rate and arithmetic on cited inputs. The plan states in its own
  text that it claims no GPU rate measured here, no live price, no campaign
  progress credit and no network or orchestration cost. A GPU row from a
  different iteration function or a superseded revision (the 20.078 B/s v2
  table walk, the 5.1 B/s v3 table walk) is not substituted for the shipping
  walk.

## Budget

One thread per job; `prlimit --as=4294967296` on every run argv. Part A
estimated from the smoke runs (all arms, 2 targets): about 1.1 s per target at
`2^40`, 4.9 s at `2^44`, 18.5 s at `2^48`, for all `r`, so about 30 CPU-minutes
for the core cells, 3 for the ideal-map controls, 2 for the extension, 4 for
part B: about 40 CPU-minutes against the cap of 2 CPU-hours. Every card is
sized for under 5 minutes of nominal wall time so that a loaded shared host
still finishes under 8; cards are split by size, by `r` and by halves of the
64 targets at `2^48`. Memory: the DP tables hold at most tens of thousands of
points; the 4 GB address-space cap is not approached.

Cards (frozen as `card-NN.yaml` in this directory, in this order; each builds
the binaries it runs):

| card | content |
|:--|:--|
| 01 | build, selftest (the two-implementation check and the positive controls), ideal-map controls at 40/44/48 (512 targets), `2^40` all `r` |
| 02 | `2^44`, `r` in {16, 32} |
| 03 | `2^44`, `r` in {128, 2048} |
| 04 to 11 | `2^48`, `r` in {16, 32, 128, 2048}, targets 0..31 then 32..63 |
| 12 | extension, `theta` in {8, 14} at `2^44` |
| 13 | part B: CPU build, five bench runs, plan |

The assess stage reads `raw/` and runs, from the repository root,
`target/blindspots-13/<bin>/b13 summary --dir <raw> --out-md <raw>/summary.md
--out-json <raw>/summary.json` (the binary rebuilt by `rustc` as in card 01's
build command); it applies the decision rule above to `summary.md`.

## Deviations from the design and from the hypothesis, and filled-in gaps

1. **Arms added beyond the design list** (tightening, more chances to reach
   1.35 and to find a crossing): `per2`, `per4`, `per8` (the hypothesis' k set
   below 16), `laper16/32/64` (look-ahead plus periodic detection, the
   combination the literature uses). All design arms are present.
2. **The baseline control.** The hypothesis and `EXP-RHO-60da4f` require the
   plain walk to reproduce `0.886 sqrt N` within 8 percent. That figure is
   `sqrt(pi N)/2 = sqrt(pi N / 2) / sqrt 2`, the *ideal negation* cost, not the
   cost of a plain random-map walk, whose first-collision law is
   `sqrt(pi N / 2) = 1.253 sqrt N`; the earlier experiment's C1 failed for that
   reason and the attempted explanation (a parallel search being `sqrt 2`
   cheaper than a single walk) is not what the ideal-map control measures. The
   control used here is the ideal random map under the identical protocol, with
   the +-8 percent band widened by the interval of the ratio. This is a
   correction of a convention, not a loosening: the old band was unreachable by
   any honest walk.
3. **Cell values.** The hypothesis lists sizes {32, 40, 48}, `r` {4, 8, 16, 20,
   32} and DP bits {8, 12, 16}. The design (this item's frozen text) fixes
   sizes {40, 44, 48}, `r` {16, 32, 128, 2048} and 64 targets; DP bits are not
   in the design and are `floor(bits/4)` in the core cells (10, 11, 12), with
   the extension at 8 and 14 at `2^44` only. The hypothesis' prediction 2
   ("at all three sizes") is read at the design's three sizes. `r` of 4 and 8
   are not run (the hypothesis already predicts absorption there and the
   design has none).
4. **Memorylessness.** The hypothesis demands every rule be a function of the
   current class only. The design's own rules (min-of-2, periodic detection)
   use short history, so the filter is read as the collision-preserving property
   defined above. `dbl` fails it, is measured because the design names it, and
   is never counted as a winner.
5. **Censoring.** Runs stop at `2.0 sqrt(pi N / 2)` group operations (the
   hypothesis' experiment used `64 sqrt(N/2)` steps). Cells with gain below
   about 0.5 are therefore bounded above, not resolved; every claim that rests
   on a censored cell is a loss claim, for which the bound is conservative, and
   a censored cell cannot support a reach claim.
6. **Group and cost model.** Sweep cells run on `Z/N` with the logarithm as the
   point (identical dynamics, counted operations), not on `N`-point curves
   with frozen `curves.json` as the hypothesis' experiment did; the equivalence
   is the agreement check at `2^20`. Cost is the counted group operations
   priced by three named models plus the native counts at three weights,
   instead of one named curve model. The Jacobian model charges a full
   normalisation per arrived point (conservative against the negation arms'
   extra work); AFF3 is the batched-inversion affine walk.
7. **Pilot disclosure.** Before this protocol was committed the instrument was
   debugged on pilot runs: 8 targets at `2^44`, `r` in {16, 2048}; 128 targets
   at `2^40`, `r` in {128, 2048}; then 2 targets at every cell (the card-rule
   smoke). All used seed offsets 0 and 1000003, disjoint from the measured
   offsets 7000000 and 9000011. The pilot changed the instrument in four
   ways, all recorded here: the arms in Deviation 1; the selftest's positive
   controls (asserting only the rules that can work at `r = 16`, with the
   reason in the check's text); the cycle-rate estimator (the draft counted
   every step including those spent in a trapped longer cycle; both estimators
   are now reported, the DP-ending-walk one being primary); the baseline gate
   (Deviation 2). No threshold of the decision rule was set from a measured
   gain.
8. **Two implementations** share the walk engine and differ in the group
   arithmetic and the source of keys and signs; a second engine was not
   written. The hypothesis' C4 asks for two independent implementations of
   the walk harness; this is weaker on the engine and stronger on the group
   accounting. It is stated here and not claimed to be more.
9. **Part B** reuses the committed GPU rates, work constants and prices (no GPU
   here), measures only the CPU rate, and prices the plan from price
   observations dated 2026-09, not from live quotes. The plan's work at 50, 90
   and 99 percent is a Rayleigh-law model, labelled as such.
10. **Gaps filled.** The design names no DP-bit rule, no cap, no pricing model,
    no definition of "reach" or of the verdict thresholds; they are in this
    document, fixed before any measured run.

## Addendum A (card split; written before any measured run; changes no cell, rule or threshold)

Timing the `2^48` cells (about 6.3, 5.6, 5.2 and 4.0 s per target for all 14
rules at `r` = 16, 32, 128, 2048) showed the card table above would leave a
loaded shared host too close to the 8-minute limit. The frozen cards are:

| card | content |
|:--|:--|
| 01 | build, selftest, ideal-map controls at 40 and 44 (512 targets), `2^40` all `r` |
| 02 | `2^44`, `r` in {16, 32} |
| 03 | `2^44`, `r` in {128, 2048} |
| 04 to 07 | `2^48`, `r` = 16, targets 0..15, 16..31, 32..47, 48..63 |
| 08, 09 | `2^48`, `r` = 32, targets 0..31, 32..63 |
| 10, 11 | `2^48`, `r` = 128, targets 0..31, 32..63 |
| 12, 13 | `2^48`, `r` = 2048, targets 0..31, 32..63 |
| 14 | ideal-map control at 48 (512 targets) and the `theta` extension |
| 15 | part B: CPU build, five bench runs, plan |

Every sweep card also runs the `baseb` A/A sample on the same target range
with seed offset 9000011. Each card builds the binary it runs and removes it.
The assess stage rebuilds `b13` with card 01's `rustc` argv and runs `summary`
on `raw/`.
