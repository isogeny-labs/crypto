# Results: the target-free chained system solves at the same F4 degree as the target system at (16, 4), with ten times the matrix

Scored natively by `dreg_score target_free runs/cell-*.jsonl`
(`score-output.txt`) against `PREREGISTRATION.md` (commit `fd61f5d27`).
Every value below comes from a committed record in `runs/` or
`runs_highcap/`. Run report with graphs and PDF:
`reports/run-20261018T1645Z/`. *Class:* stage diagnostic; no `S`, no rho
ratio, no speed claim.

**Registered verdict: undecided, as scored.** The registered rule needs at
least three measured draws per arm, and no target-free draw finished under
the registered 1 GiB step-matrix cap. The bounded reading is not undecided:
every target-free draw, on every base and twist, reached a degree-4 step
that added basis elements before the cap stopped it, so its solving degree
is at least 4, the target arm's value. No cell can read *lower*, which is
the only reading that keeps the lead alive. A raised-cap pass (a deviation,
below) finished two target-free draws and both read 4, *equal* to the
target arm.

**What this means for L1.** In the chained presentation (intermediate
`u ∈ F_{2^n}`) the target-free relation gives no degree drop at the
smallest registered cell, on the progression base or on a random one. It
costs about ten times the columns and eleven times the rows of the target
system's largest step, and about 650 times the wall time, for the same
degree. This is what Lemma 1 predicts for this presentation (see "Why the
chained presentation could not show the effect"). The eliminated `S₄`
presentation, where the lemma's corollary does give leading-form
dependencies, was not tested here; it is the next study.

## What was measured

| arm | base | twists | draws | F4 outcome | solving degree | largest step matrix (rows × cols) | wall s |
|---|---|---|--:|---|---|---|--:|
| target `m = 3` | gp | — | 1 | finished, unsatisfiable (basis {1}) | 4 | 25,458 × 23,951 | 3.1–4.5 |
| target `m = 3` | random | — | 4 | finished, unsatisfiable on all 4 | 4, 4, 4, 4 | 24,342–25,211 × 22,735–23,706 | 2.4–3.8 |
| target-free | gp | `[0,0,0,0]` | 1 | stopped by the 1 GiB cap in the degree-5 step | ≥ 4 | 70,184 × 41,300 when stopped | 155 |
| target-free | gp | `[0,1,2,3]` | 1 | stopped by the cap at degree 5 | ≥ 4 | 58,641 × 41,330 when stopped | 168 |
| target-free | random | `[0,0,0,0]` | 4 | stopped by the cap at degree 5 on all 4 | ≥ 4 ×4 | 70,097–72,445 × about 41,300 | 125–133 |
| target-free | random | `[0,1,2,3]` | 4 | stopped by the cap at degree 5 on all 4 | ≥ 4 ×4 | 54,841–58,913 × about 41,300 | 137–144 |
| **deviation:** target-free, 8 GiB cap | gp | `[0,0,0,0]` | 1 | finished, degree reached 5 | **4** | 278,739 × 235,381 (8.2 GB step) | 2,023 |
| **deviation:** target-free, 8 GiB cap | random | `[0,0,0,0]` | 1 | finished, degree reached 5 | **4** | 279,021 × 236,407 (8.2 GB step) | 2,038 |
| **deviation:** target-free, 8 GiB cap | gp | `[0,1,2,3]` | 1 | aborted on allocation under the 12 GB address-space limit; empty record | unmeasured | — | — |

The finished target-free bases have 1,187 elements (gp) and 1,205
(random): the diagonal family `x₁ = x₃, x₂ = x₄` and the other solutions
keep the ideal far from zero-dimensional-and-small. The two bases' F4 runs
are the same size to within 0.5%, so the run shows no shape effect of the
progression base in this presentation.

Cells `(18, 4)`, `(20, 5)` and `(24, 6)` were not run. The registered stop
condition ("a cell whose target-free arm is unmeasured on every draw stops
the larger cells") fired at `(16, 4)`.

## Predictions

| prediction | outcome | confidence | evidence |
|---|---|---|---|
| P1: gp plain reads higher or equal at every measured cell | **unmeasured** under the registered cap; on the bounded reading not lower; *equal* on the deviation pass | about 60% | `score-output.txt`, `score-output-highcap.txt` |
| P2: gp twisted reads equal at every measured cell | **unmeasured** (registered cap; the deviation run aborted on allocation); bounded not lower | about 55% | same |
| P3: random never reads lower | **holds on the bounded reading** (8 of 8 random target-free draws have solving degree ≥ 4 = the target median); unmeasured under the registered rule | about 85% | same |
| P4: target-arm F4 solving degree = 4 on unsatisfiable draws at (16, 4), (18, 4) | **holds at (16, 4)**: 10 of 10 unsatisfiable target records (5 draws, each recorded once per twist file) read 4; (18, 4) not run | about 80% | `runs/cell-16-4.*` |

## Why the chained presentation could not show the effect

The lead's mechanism (`research/ic_leads_20261018/LEMMAS.md`, corollary
to Lemma 1) gives leading-form dependencies only in the *eliminated*
target-free `S₄`, where every multiplier is a constant and the top forms
live in `V^{(a)}V^{(b)}V^{(c)}V^{(d)}`, of dimension `≤ 4ℓ − 3 < n`. The
same corollary states that in the chained presentation each link's leading
forms are the coordinates of `u·x·x'` on `K × V × V`, independent for
every `V` because `u` ranges over all of `K`. The registered instrument
built the chained presentation (`build_target_free_system`), so Lemma 1
already predicted the outcome measured here. The preregistration named the
eliminated `S₄` as the motivating system and then measured the chained
one; that mismatch is a design error in this study, recorded here, and it
is why the study does not kill L1.

## Deviations from the preregistration

1. **The `gp` arm can never have a registered reading.** The progression
   base is deterministic, so `run.sh` ran it with one draw, while the
   verdict rule needs three measured draws per arm. The registered rule
   cannot read `gp` at any cell; the bounded and single-draw readings above
   are labelled as such.
2. **Raised-cap pass.** `run_highcap.sh` (written after the registered
   queue stopped, before its runs) reran three `(16, 4)` arms, one draw
   each, with `F4_F2_MAX_MATRIX_WORDS = 2^30` (8 GiB step matrices), a
   3,600 s budget and the same 12 GB address-space limit, using 3 threads.
   Its records are in `runs_highcap/`, scored separately
   (`score-output-highcap.txt`) and never pooled with `runs/`.
3. **Queue-log exit statuses are wrong.** Both scripts read `$?` after a
   `$(date …)` substitution, so every logged status is that of `date`. The
   twisted raised-cap run logged `exit 0` but aborted on allocation (its
   `.log` and empty record). Fixed in both scripts afterwards; a hand note
   at the end of each queue log says so.
4. **The queue was stopped by hand** at the registered stop condition, a
   few seconds into `cell-18-4.gp.tw0000`; that process was killed and its
   empty record removed. The stop is noted in `runs/queue.log`.

## Scope

One cell, `(16, 4)`, `b = 1`, seed 20261018, the 4-core 15 GB container
(not isolated; wall times are practicality notes). Nothing here bears on
the eliminated `S₄`, on larger cells, or on yield.

## Next

Preregister and run the eliminated presentation: Weil-descend
`S₄(x₁, x₂, x₃, x₄)` (the resultant of two `S₃` in `u`, as
`binary_semaev_s4` computes it) with all four `x_i ∈ V`, against
`S₄(x₁, x₂, x₃, x_R)` on the same base. At `(16, 4)` that is 16 Boolean
unknowns against 12, cheap enough to run at `(16, 4)`, `(20, 5)`, `(24, 6)`
and `(28, 7)`.
