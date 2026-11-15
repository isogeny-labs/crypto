---
name: dreg-triage
description: Iterate an index-calculus parameter sweep on its advisory degree-of-regularity forecast - run factor-base-sweep, read the per-cell forecast and triage (predicted degree of regularity with band, semi-regular yardstick, projected relation-phase cost with linear algebra, hope band against the rho floor), write the next plan with `refine`, and loop, under the repository's evidence, checkpoint and model-tier rules. Also reads the same forecast off ecbench ic.pipeline records and N83 msweep cells. Use with ic-autolab when choosing which factor bases, dimensions and seeds to sweep next; never to promote a result.
---

# Triage a sweep on its degree-of-regularity forecast

Work from this skill's containing repository. Read `AGENTS.md` ("Success
criterion", "Sabotage is assumed", "Model tiers", "Resume checkpoints",
§5a) and
[`docs/ic/factor-base-sweep/README.md`](../../../docs/ic/factor-base-sweep/README.md)
("Degree-of-regularity forecast and cost triage") first. The mechanism is
`crypto_lib::cryptanalysis::dreg_forecast` (`dreg_forecast/v1`), described
in [`docs/ic/FRAMEWORK.md`](../../../docs/ic/FRAMEWORK.md).

## What the forecast is, and is not

Every solve of a cell's decomposition system is one draw of its solving
degree. Decided solves observe the degree; exhausted solves are
right-censored at the highest Macaulay degree built. The Kaplan–Meier
estimate of that distribution gives the **predicted degree of regularity**
(median) with a 95 % band, `>bound` when the median lies above the F4 cap;
the **semi-regular degree** of the system shape is the yardstick
(`structural_gain` is true when the whole band lies below it). The cost
side prices one more solve at the observed per-degree work, projects the
remaining and whole relation phase from the relation yield, charges the
relation matrix from its own row operations (fitted as `a + b·k` per row,
priced into the solver unit at a recorded price), and reports the rho floor
over that work at 1, 10 and 100 multiplications per group addition. The
**advice** (`promising`, `undecided`, `doubtful`, `hopeless_at_bound`) is
given after four real solves and ranks only where the order holds across
the price band.

It is advisory. It never stops, skips or shortens a candidate; the sweep
runs every candidate on its frozen budget. `certified_degree_of_regularity`
stays null everywhere. A forecast is `[pcc:unverified]` in prose unless a
pcc claim carries it. It is a stage diagnostic of the decomposition oracle,
never a complete-DLP result, a speedup, a leaderboard row or a benchmark
row: those still go through ecbench, the validation gate and the autolab's
own promotion protocol (`ic-autolab`).

## The loop

One iteration is: plan → run → triage → refine → next plan. Each step is a
native command; nothing here is Python.

1. **Plan.** `factor-base-sweep plan --out plan.json --preset binary|prime|both
   [--seed N --seed-count K --dimensions 3,4]`, or edit the candidate list.
   Budgets (`train`, `holdout`, `max_degree`, `candidate_seconds`,
   `max_matrix_cells`, `memory_mib`, `threads`) are in the plan.
2. **Run.** A development smoke runs locally and is labelled not evidence.
   Anything meant as evidence runs on the default route (isolab-eks, AGENTS.md
   §9a) through the model tiers: the reasoning tier decides the plan,
   `experiment-director` compiles the card, `executor-mechanical` runs it,
   the director assesses. Before a run, fill `.resume/RESUME.md`
   (`scripts/checkpoint.sh init`); the hooks refuse run launchers until the
   checkpoint is pushed. `factor-base-sweep run --plan plan.json --out runs/<name>`
   prints `Info:Regularity forecast:` lines per trial and writes the forecast
   on every trial record, in `result.json` and in `summary-NNNN.json`
   (candidates ranked under `triage`).
3. **Triage.** `factor-base-sweep triage --run runs/<name> --out triage.json`
   ranks every attempt from the records on disk: advice first, then predicted
   degree, then projected relation-phase work (with priced linear algebra
   where observed). It takes no lock, so it reads beside a live sweep;
   records being written count as unreadable, never guessed. Read, per row:
   `advice` and `advice_reasons`, `predicted_degree_of_regularity` and
   `band_95` against `semi_regular_degree`, `p_exceeds_bound`,
   `relation_yield`, `projected_relation_phase_work_with_linear_algebra`,
   `hope_ratio_band_with_linear_algebra`, `solves` (fewer than four means
   the advice is not yet given).
4. **Refine.** `factor-base-sweep refine --run runs/<name> --out next.json
   [--drop hopeless_at_bound,doubtful] [--seed-count K] [--dimensions 3,4]
   [budget flags]` writes the next plan: candidates whose latest attempt was
   inapplicable, or whose forecast has at least four solves and an advice in
   `--drop`, are dropped; the rest are kept and expanded over consecutive
   seeds and the dimensions given. The provenance (`next.refine.json`) lists
   every kept and dropped candidate with its advice and reasons. Refine runs
   nothing. If it drops everything it fails rather than writing an empty
   plan: widen `--drop` (keep `doubtful`) or change the budgets.
5. **Loop.** Run `next.json` in a new directory, triage, refine. Stop when
   the kept set is stable across two iterations, when every candidate is
   doubtful under the whole price band, or when the budget is spent. Record
   each iteration in the session log with the triage file and the
   refine provenance, and in the PR that carries the sweep directory.

## Reading the other consumers

- **ecbench `ic.pipeline`**: the record's `solver` block carries
  `degree_of_regularity_forecast` (or `forecast_unavailable` for engines that
  report no degree, such as `sat-cdcl`). Capped engines are censored at
  `max_degree`; the uncapped `f4-f2` is bounded by the ring's ceiling; the
  Gaudry quartic and line-S4 oracles observe their fixed Macaulay degree,
  degenerate systems censored there. Linear algebra is charged from the
  run's matrix total.
- **N83 msweep cell** (`examples/n83_msweep_cell.rs`): Gröbner engines carry
  `degree_of_regularity_forecast`; others say why not in
  `forecast_unavailable`. The cell keeps no matrix, so its rank is
  approximated by the relations collected and no linear algebra is charged.

## What to say afterwards

Report each iteration as: candidates kept and dropped with the advice that
decided it, the predicted degree and band of the kept ones against the
semi-regular degree, and the projected relation-phase work with its hope
band, all labelled as forecasts. Do not call a kept candidate an
improvement, a speedup or a result: the success criterion counts verified,
measured improvements over the index-calculus baseline, and the validation
gate decides what may be shown as one.
