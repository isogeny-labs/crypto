# Pre-registration: does the eliminated target-free S₄ solve below the S₄ target system on the same base? (lead L1, the presentation the lemma is about)

Registered 2026-10-18, before any cell below was measured. Follows
`research/target_free_relations_20261018/RESULTS.md`, which measured the
*chained* target-free system (intermediate `u`). There Lemma 1 already
predicts independent leading forms, and the system solved at the target
system's degree. The corollary to Lemma 1
(`research/ic_leads_20261018/LEMMAS.md`) places the leading-form
dependencies in the *eliminated* `S₄`, with no intermediate unknown. This
study measures that presentation.

## Instrument

- **Systems.** `build_eliminated_s4_system(basis, b = 1, target, twists, st)`
  (`src/cryptanalysis/koblitz_groebner.rs`): the `n` Boolean coordinates of
  `S₄ = (A₁C₂ + A₂C₁)² + (A₁B₂ + A₂B₁)(B₁C₂ + B₂C₁)`, with
  `A = (x + x')²`, `B = x·x'`, `C = (x·x')² + 1` per pair. This is the
  resultant formula of `binary_semaev_s4`. The test
  `eliminated_s4_matches_the_field_formula` checks it on every Boolean
  assignment at five small cells.
  - **target-free arm:** `x₁..x₄ ∈ V` (twisted by `x_i^{2^{a_i}}`), `4ℓ`
    unknowns, `n` equations;
  - **target arm:** `x₁, x₂, x₃ ∈ V` (twisted by `a₁, a₂, a₃`), `x₄ = x_R`
    a fresh random constant per draw, `3ℓ` unknowns, `n` equations.
- **Measure.** The F4 solving degree (`pq_f4_f2::groebner_basis_f4`, field
  equations included): the highest step degree that added a basis element.
  Run with a wall budget of 1,800 s and step matrices capped at 4 GiB
  (`F4_F2_MAX_MATRIX_WORDS = 2^29`). A record with `timed_out` or `oversize`
  is unmeasured; its `solving_degree` is a lower bound only. Also recorded,
  before F4 runs: the equations' degree histogram, the top degree, and
  `top_forms − top_rank`. That difference counts the `F₂`-dependencies
  among the top-degree homogeneous parts, which is the quantity the
  corollary is about.
- **Bases.** `gp`: the progression `span(1, θ, …, θ^{ℓ−1})`. `random`:
  `random_subspace_basis`.
- **Draws.** Seed `20261018`. On `gp` the target-free system is
  deterministic, so it is solved once (draw 0); the target arm runs on 4
  draws, which differ only in `x_R`. On `random`, both arms run on 4 draws.
- **Arms per cell.** gp `[0,0,0,0]`, gp `[0,1,2,3]`, random `[0,0,0,0]`.
- **Engine.** `examples/dreg_eliminated_s4.rs` via `run.sh`, one process
  per (cell, basis, twists), `prlimit --as=12000000000 timeout 10800`, 3
  rayon threads. The scorer is `dreg_score s4_free runs/cell-*.jsonl`.

## Cells (frozen)

| (n, ℓ) | target-free unknowns | target unknowns | no wrap (`4ℓ − 3 < n`) |
|---|--:|--:|---|
| (13, 3) | 12 | 9 | yes |
| (16, 4) | 16 | 12 | yes |
| (17, 4) | 16 | 12 | yes |
| (18, 4) | 16 | 12 | yes |
| (19, 4) | 16 | 12 | yes |
| (20, 5) | 20 | 15 | yes |
| (24, 6) | 24 | 18 | yes; likely over the cap |

**Already seen, excluded.** `(14, 3)` and `(15, 3)` were smoked while the
instrument was being built (gp and random, plain, `--seed 7`, records not
kept). There the target-free arm solved at 7 on gp and 8 on random, the
target arm at 6, and the gp top deficiency was 2 against 0 on random. The
smoke at `(10, 2)` and `(11, 3)` is excluded likewise. Those observations
motivate P2 and P3. They do not enter any reading.

## Readings and verdict rules

Per (cell, basis, twists): the *target-free value* is the measured draw on
`gp`, or the median of at least 3 measured draws on `random`. The *target
value* is the median of at least 3 measured target draws. The reading is
**lower** (target-free value at least one below the target value),
**equal**, **higher**, or **unmeasured** when a value is missing. The
**bounded reading** (labelled, not registered) uses the unfinished records'
lower bounds: "not lower" when every target-free record is at least the
measured target median.

- **Primary (the lead's falsifier).** *The lead survives* if gp reads
  lower at two or more (cell, twists) where random does not read lower. It
  is *killed* if no gp reading is lower and at least one gp reading is
  measured. Otherwise it is *undecided*.
- **Secondary (a shape effect inside the target-free arm).** At a cell with
  both values, *shape lower* when the gp target-free value is at least one
  below the random target-free value. A shape effect is recorded if two or
  more cells read shape lower. This reading does not rescue the lead: an
  index calculus needs the target-free system to beat the target system,
  not a random base.

## Predictions

- **P1.** The target-free arm reads *higher* than the target arm at every
  measured (cell, basis, twists). Its equations have Boolean degree 7
  against 6 for the target system, and it has `ℓ` more unknowns. About 85%.
- **P2.** The gp target-free top deficiency is above the random median at
  every cell where both are recorded. About 80%.
- **P3.** Secondary shape effect at two or more cells. About 50%.
- **P4.** The twist `[0,1,2,3]` brings the gp top deficiency down to the
  random plain median or below at every cell where both are recorded. A
  twisted progression is no longer a progression. About 60%.

## Stop conditions

A cell whose target-free records are all unfinished stops the larger cells.
The queue runs one process at a time.

## Scope and class

Stage diagnostic, exploratory lead. No `S`, no rho ratio and no yield. The
yield price noted in the first study still applies: `ℓ ≳ n/3 − 7` is needed
for enough relations, against `ℓ < n/4` for no wrap. No Conductor task: the
CLI is not installed and the hooks fail open.
