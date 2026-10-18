# Item 02 protocol: pricing binary curves at deployed sizes under the measured degree model

Frozen before any measured run. Source design: section "02" of
`research/blindspots_20261018/EXPERIMENTS.md` and its "Rules every item
follows". Written by the experiment-director lane, 2026-10-18.

This item is a **derived cost-model evaluation**, not a solve. Nothing in it
is measured on a curve; every number is a closed-form model evaluation, so it
may be labelled "derived" and never "measured" (AGENTS.md section 6,
"An extrapolation presented as a measurement"). Wall time is not a metric.

## Question

Under each measured solving-degree model, at which extension degree `n` does
the `m = 4` (and `m = 3`, `m = 5`) Weil-descent route cross Pollard rho? Does
any model flip the sign of the gap at a deployed binary size
(sect233 to sect571), and does the answer decide whether item 07's cells are
worth the machine?

## What already exists, and what is reused

* `examples/descent_degree_cost.rs` (the committed native port of
  `research/notes/ecc2k130/descent_degree_cost_20260930.py`) already evaluates
  this model, but only at `n = 131`, with `l` searched over `4..70`, rho fixed
  at the constant `60.81`, and no `K`. It is **not edited**; its n = 131 output
  is committed as
  `research/notes/ecc2k130/descent_degree_cost-output-20261016-scale.txt`.
* `research/ic_next10_20261014/06-equivariant-f4-m4/cost_d4.py` /
  `cost_d4-output.txt` evaluate constant `D = 3, 4, 5, 6` at `n = 131`. The
  design's "port cost_d4.py" step is therefore **already done by
  `descent_degree_cost.rs`** (it reproduces the `D = 6` rows); this item does
  not re-port it, it reproduces both committed tables as the positive control.
* The `n = 131` cells of the sweep (the `k1` family, ECC2K-130) are **reused
  results**: the instrument reproduces every one of the 30 + 16 committed rows
  identically (control below) and the sweep's `n = 131` cells are those same
  model evaluations. Nothing at `n = 131` is claimed as new; the new content is
  `n >= 163`, the `m = 5` and `m = 2` rows, `K`, memory, and the `r1` family.
* Measured degree evidence the arms rest on (not re-measured here):
  `RESEARCH_ECC2K130_DESCENT_DEGREE.md` and
  `research/ic_next10_20261014/06-equivariant-f4-m4/README.md` (F4 degree 4
  measured to `n <= 15`, `l <= 5`; XL 6 to 7; the `l + 4` fit at `m = 4`;
  the `ceil(l/2) + 4` fit at `m = 3`).

## Model (identical to the committed formulas)

For field degree `n`, arity `m`, factor-base dimension `l` (integer), Macaulay
degree `D`, oracle weight `w`, and touches-per-group-operation `K`:

```text
relations        2^l
targets          l + max(0, log2(m!) + n - m*l)            (log2)
unknowns         N = (m-2)*n + m*l
oracle / target  w * log2 C(N, <=D)  -  log2 K              (group operations)
linear algebra   log2(m) + 2*l                              (group operations)
total            log2( 2^(targets + oracle) + 2^algebra )
memory           max( w*log2 C(N,<=D) , l + log2 m )         (log2 words)
gap              total - rho
```

`K` divides the oracle term only, as the committed note states ("converting
would lower every row with an oracle by `log2 K`"). The `free oracle` floor has
no oracle term and is `K`-invariant. Each row is minimised over integer `l`
(first strict minimum, as committed). Memory is reported alongside time at the
minimising `l`; rho's memory is O(1) (not priced).

**Rho reference** per curve, from the real subgroup order `r` and the
automorphism group size `A` of `<G>`:

```text
rho = 1/2 * ( log2(pi/2) + log2 r - log2 A )     group operations
A = 2n  (Koblitz: negation x Frobenius)       A = 2  (random: negation)
```

This reproduces the committed ECC2K-130 reference `2^60.8090` (`r =
680564733841876926932320129493409985129`, `A = 262`).

## Arms

Degree models (design list): `D = 3`, `D = 4` (F4), `D = 6` (XL), `D = 7`
("levelled at 7"), `D = l + 4`. Arities `m in {3, 4, 5}`. Weights `w in {1, 2}`.
Also reported: the free-oracle **floor** (no oracle term; a bound, not a
degree model).

Controls and supplementary arms (not in the design list; listed so they are
not mistaken for design arms):

* `m = 2`: the negative control.
* `D = ceil(l/2) + 4`: the committed `m = 3` fit, reported as a
  **supplementary** arm and excluded from the design flip statement.
* `semi-regular D_reg(N)`: used only in the `n = 131` positive control (the
  committed table contains it); not an arm of the sweep.

## Cells

`n in {131, 163, 233, 283, 409, 571}`. Each `n` is evaluated for two matched
rho families, each with the curve's **real cofactor and order** taken from the
library's curve constants:

| n | k1 (Koblitz, A = 2n) | h | r1 (random, A = 2) | h |
|--:|---|--:|---|--:|
| 131 | ECC2K-130 | 4 | sect131r1 | 2 |
| 163 | sect163k1 | 2 | sect163r1 | 2 |
| 233 | sect233k1 | 4 | sect233r1 | 2 |
| 283 | sect283k1 | 4 | sect283r1 | 2 |
| 409 | sect409k1 | 4 | sect409r1 | 2 |
| 571 | sect571k1 | 4 | sect571r1 | 2 |

`K in {1, 10, 16, 256}`: `1`, `16 = 2^4`, `256 = 2^8` are the design's
`2^0, 2^4, 2^8`; `10` is the AGENTS.md section 5a weight of 0.1 GAE per touch
(`K = 1` is 1 GAE per touch; the floor is the weight-0 end of the band).
`l` is searched over `1..=n` (see deviations). The per-cell output carries
`best l`, `N`, `D`, targets, oracle, algebra, total, gap, memory, and flags for
`l` at a search edge, targets above the subgroup, and total above the subgroup
(a model that has left the regime where it describes index calculus).

**Crossover scan.** For the crossover `n` per model, a smooth-family scan
(`#E = 2^n`; Koblitz `h = 4`, `A = 2n`; random `h = 2`, `A = 2`) over
`n = 64..=1024` (step 1) and `1032..=4096` (step 8) for constant-`D` arms and
the floor, and `n = 64..=1024` (step 16) plus the six deployed `n` for the
growing-`D` arms. Reported: first grid `n` with gap `<= 0`, the smallest `n`
from which every larger grid `n` has gap `<= 0` (sustained), the gap at 571,
and the minimum gap. Values above `n = 571` are extrapolation of the closed
form and are labelled so.

## Controls

* **Positive control.** The instrument, run at `n = 131`, `l` in `4..70`,
  rho `60.81` (exactly the committed settings), reproduces **every row** of
  `research/notes/ecc2k130/descent_degree_cost-output-20261016-scale.txt`
  (all fields: `l`, `N`, `D`, targets, oracle, algebra, total, gap, to two
  decimals; includes the semi-regular and fit rows) and of
  `research/ic_next10_20261014/06-equivariant-f4-m4/cost_d4-output.txt`
  (`D = 3, 4, 5, 6`, `l` in `4..=70`), including the named `2^76.31`
  (`m = 4, D = 6, w = 1`) and `2^88.05` (`m = 3, D = 6, w = 1`) rows. It also
  checks the ECC2K-130 reference `2^60.8090` from the real order. Any
  mismatch fails the control and stops the item.
* **Range check.** At `n = 131` the constant-`D` minimisers are unchanged when
  the search widens from `4..70` to `1..=131`.
* **Negative control.** `m = 2` must never cross rho: over every cell, both
  families, every arm including the floor, every `w` and `K`, the minimum gap
  must be `> 0`. A crossing means the instrument or its rho reference is wrong.
* **Internal monotonicity.** gap(`w = 2`) >= gap(`w = 1`); gap non-increasing
  in `K`; gap non-decreasing in constant `D`. Violations fail the control.

## Unit and pricing (AGENTS.md section 5a)

Unit: log2 of ecbench-style group operations (GAE); the oracle term is column
touches converted by `K`. Wall time is not reported. Unpriced counters, listed
per arm because the model does not emit their counts:

* attack arm: Macaulay-matrix construction and memory traffic, verification and
  root extraction of decomposed targets, factor-base membership tests. Their
  counts are not emitted, so the attack side is a **lower bound**; the oracle's
  column touches are priced at weights 0 (floor), 0.1 (`K = 10`) and 1
  (`K = 1`) GAE; the linear-algebra term is charged one group operation per
  touch as committed.
* rho reference: orbit canonicalisation (negation/Frobenius minimum) per step,
  partition hash, distinguished-point table inserts and lookups. Not emitted
  by the formula, so the rho reference is also a **lower bound**.

Both sides being lower bounds, no gap is a ranking: every gap is reported as
the distance remaining, and a gap whose sign changes inside the band
(`K = 1, 10`, floor) is "unresolved at that boundary", never a win for either
side.

## Metric and decision rule

Metric: `gap = log2(total) - log2(rho)` per `(family, arm, m, w, K, n)`;
crossover `n` per `(family, arm, m, w, K)` from the scan.

Rule (from the design): an arm **flips** if its gap is `<= 0` at some deployed
`n`. The verdict, stated per family and separately for the design arms and the
supplementary arm, is:

* **ADVANCES (derived, model level)**: some design arm flips at `K = 1` (no
  unmeasured constant needed). Level: exponent if the flipping arity has a
  floor exponent below `n/2` (`m >= 4`: `2n/5`, `n/3`), constant otherwise.
* **INCONCLUSIVE (conditional on unmeasured K)**: no design arm flips at
  `K = 1` but one flips at `K` in `{10, 16, 256}`; name the smallest such `K`.
  `K` (column touches per GF(2^n) group operation) is not measured here.
* **REGRESSES at every deployed n**: no design arm flips at any `K` in the grid
  by `n = 571`. The output then says so plainly: "no plausible model flips by
  n = 571".

Item-07 gate: the gate is **open** if the first verdict holds, **conditional**
if the second holds, **closed** if the third holds. The gate is the output's
only downstream effect; this item changes no research state.

A 'flip' of the floor alone (no degree model) is reported but is not a design
arm and does not open the gate; it states that a free oracle would cross, which
is the committed section 5.3 reading.

## Budget

One thread, `prlimit --as` 4 GiB per job, at most 2 CPU-hours total
(expected well under 10 minutes: closed-form evaluation). Disk: build in the
shared `target/`; outputs are text, under 5 MB.

## Deviations from the frozen design

Each tightens or makes the design concrete; none loosens it.

1. **File.** The instrument is a new
   `examples/blindspots_02_deployed_binary_pricing.rs`; `examples/descent_degree_cost.rs`
   (which already is the cost_d4 port) is left untouched, because its committed
   `n = 131` outputs are cited by other results and another lane owns it.
2. **`l` range** is `1..=n` instead of the committed `4..70`. A superset: it
   can only lower a cost (favours the attack), and the `n = 131` constant-`D`
   minimisers are verified unchanged. The positive control uses the committed
   `4..70` exactly.
3. **`K` placement.** `K` divides the oracle term only (the committed note's
   statement); the design says "as a parameter" without placement. `K = 10` is
   added for the section 5a band.
4. **Two matched rho families** (Koblitz with `A = 2n`, random with `A = 2`),
   because the design says "each curve's real cofactor" without naming the
   curve family and the model does not use Koblitz structure; the two are
   never merged and the flip statement is made for each.
5. **`m = 5`, `m = 2`** are as designed (`m = 2` the negative control);
   `D = ceil(l/2) + 4` is added as a supplementary arm; semi-regular `D_reg`
   appears only in the control.
6. **Memory column** definition above is mine (the design asks for the
   column, not its formula).
7. **Floating point.** The monomial-count recurrence is the committed one,
   rescaled by exact powers of two to avoid overflow at `N` up to ~2800; it is
   bit-identical to the committed evaluation wherever that is finite.
8. **Dependency-free build, `rustc` instead of `cargo`.** At the branch base
   (`1d0d9617d`, also `origin/main`) the `crypto` library does not compile
   (`cargo build --release --lib`: duplicate `Instance::Fp4` variant in
   `src/cryptanalysis/ecbench/workload.rs`, `ecbench::fbm` not declared in
   `ecbench/mod.rs`, undefined `OnbWeightBase` and `sparsest`; 25 errors), and
   `target/` holds no `crypto_lib` rlib, so `cargo build --example` cannot
   succeed for any example. The instrument therefore depends on `std` only:
   the curve orders and cofactors are copied from `src/binary_ecc/curve.rs`
   (each checked against `#E = h*r` within the Hasse interval of `2^n`, and
   ECC2K-130's against its decimal order), and the committed
   `semi_regular_dreg` is re-implemented on 768-bit two's-complement integers
   (it is exercised by the positive control: 30/30 rows including the
   semi-regular ones). Each card compiles the example with
   `rustc --edition 2021 -C opt-level=3 -C codegen-units=1` to a path that does
   not yet exist, because a binary path that exists is a repository file that
   `tools/task_card.py` would require to be bound by a sha256 unknowable before
   the build. The example source and the two committed reference tables are
   bound by sha256. This is a change of build tool only; no formula changes.
   Not fixed here: the broken library belongs to other lanes.
9. **Conductor.** `conductor` is not installed on this host (CLAUDE.md asks for
   a task check); the hooks warn and let edits through, per docs/CONDUCTOR.md.

## Outputs

`raw/` holds `control.txt`, `cells.tsv`, `cells.md`, `scan.tsv`, `scan.md`, the
receipts, and the run logs. `README.md` (verdict by this rule), the SVG and PNG
graph and `REPORT.pdf` are produced by the assess stage, not here.

## Smoke test disclosure (before the cards were frozen)

The positive and negative controls were run once on the built instrument
(30/30 and 16/16 committed rows identical; `m = 2` minimum gap `+24.50` at
the sect131r1 floor; 1392 monotonicity checks, 0 violations). The `cells` and
`scan` code paths were then run once with `--smoke`, which restricts them to
the smallest cell (`n = 131`) and the grid up to `n = 131`. That output was
written to the scratchpad, not to `raw/`, and was inspected only for crashes;
it shows that at `n = 131` some `m = 5` rows already have a gap at or below
zero (for example the sect131r1 `D = 3` and `D = 4` rows). The decision rule,
arms, cells and verdict vocabulary above were fixed before this and are not
changed by it; the measured cards below re-run everything into `raw/`.
