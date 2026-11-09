# ML-aided SAT for the PDP: the ceiling is real, the signal is not

**Status: closed by the preregistered rule** (2026-10-27). Toy stage
diagnostic on a local 4-core development host (Intel Xeon @ 2.10 GHz,
unisolated); cost is counted in solver conflicts, which do not depend on the
machine. No leaderboard, `benchmarks/` or validation-gate row: this is
not an index-calculus improvement and claims none. Protocol, frozen before
data: [PROTOCOL.md](PROTOCOL.md) (commit 8a85584fd, amendment 1 recorded
before any result was seen). Run report and PDF:
[reports/run-20261027T0644Z/](reports/run-20261027T0644Z/).

## The question

The SAT/SMT survey ([RESEARCH_PDP_SAT_SMT_NAGAO_SURVEY.md](../notes/index-calculus/RESEARCH_PDP_SAT_SMT_NAGAO_SURVEY.md)
§6) dismissed "ML-guided SAT" without measuring it. This run measures
the most any learned component could gain inside a SAT point-decomposition
oracle, before anyone builds a GNN, an RL branching policy or a learned
restart schedule. Setup: symmetrised `S₄`, `m = 3`, native CDCL+XOR
solver, the Trimoska-corpus curves, uniform targets. There are three levers:

- **L1, triage:** score each target and only finish the solves that score
  high. The scores come from the bits of `x_r`, all their pairwise
  products, or solver telemetry after 64 or 512 conflicts.
- **L2, cutoff:** stop every solve at a fixed conflict budget.
- **L3, branching order:** static priorities, including one learned from
  VSIDS activity on training refutations.

## Result

All 4,200 labels agree with exhaustive sorted-triple search. Every SAT model
was also checked as a decomposition.

| family | n | l | targets | SAT | SAT conflicts median [IQR] | UNSAT conflicts median [IQR] | UNSAT min–max |
|:--|:--|:--|:--|:--|:--|:--|:--|
| n15l5 | 15 | 5 | 3000 | 576 | 2678 [1388, 4273] | 5781 [5738, 5819] | 5546–5941 |
| n17l6 | 17 | 6 | 1200 | 336 | 20229 [10324, 34124] | 45649 [45539, 45781] | 44924–46297 |

1. **The search is an enumerator, as the survey found.** A refutation costs
   0.963 (n15l5) and 0.998 (n17l6) conflicts per sorted factor-base triple.
   Every refutation lands within 4.1 % of its family median. The time to a
   witness is spread across the whole refutation length, with a median of
   0.44–0.46 of it (`conflict_distribution.svg`).
2. **The prize is large.** A perfect UNSAT predictor, charged its telemetry
   conflicts, would raise relations per conflict by **5.2–9.0×**. This is the
   `ceiling` column below.
3. **Nothing reaches it.** Test AUC sits within the noise of 0.5 for every
   feature group, on both families and both splits. The permuted-label
   control gives 0.45–0.54. The positive control (the label flipped with
   probability 0.25) reaches 0.72–0.79, so the learner itself works. The one
   CI that excludes 0.5, n15l5 telemetry@512 at 0.556 [0.518, 0.599],
   includes the "decided within 512 conflicts" flag. Its gain is 0.99×.
   Train-fitted gains of up to 4.7× fall to about 1× on held-out data.
   Telemetry trained at n = 15 scores 0.50 and 0.52 at n = 17.

| family | split | features | dims | test AUC [95% CI] | permuted-label AUC | test gain [95% CI] | train gain | exact-label ceiling |
|:--|:--|:--|:--|:--|:--|:--|:--|:--|
| n15l5 | even_odd | F-x | 15 | 0.515 [0.478, 0.552] | 0.510 | 0.79 [0.37, 1.31] | 1.99 | 9.01 |
| n15l5 | even_odd | F-xx | 120 | 0.503 [0.466, 0.539] | 0.495 | 1.11 [0.62, 1.77] | 3.49 | 9.01 |
| n15l5 | even_odd | F-tel64 | 11 | 0.514 [0.476, 0.551] | 0.492 | 1.08 [0.85, 1.35] | 1.27 | 8.28 |
| n15l5 | even_odd | F-tel512 | 11 | 0.556 [0.518, 0.599] | 0.508 | 0.99 [0.85, 1.13] | 1.14 | 5.27 |
| n15l5 | aa_mod4 | F-x | 15 | 0.526 [0.491, 0.566] | 0.537 | 0.98 [0.55, 1.43] | 1.79 | 8.83 |
| n15l5 | aa_mod4 | F-xx | 120 | 0.485 [0.450, 0.523] | 0.540 | 1.39 [0.91, 1.96] | 3.95 | 8.83 |
| n15l5 | aa_mod4 | F-tel64 | 11 | 0.531 [0.494, 0.567] | 0.534 | 1.17 [0.92, 1.44] | 1.30 | 8.12 |
| n15l5 | aa_mod4 | F-tel512 | 11 | 0.540 [0.501, 0.575] | 0.478 | 0.91 [0.73, 1.09] | 1.25 | 5.21 |
| n17l6 | even_odd | F-x | 17 | 0.496 [0.444, 0.547] | 0.517 | 0.65 [0.19, 1.26] | 1.69 | 6.69 |
| n17l6 | even_odd | F-xx | 153 | 0.461 [0.408, 0.514] | 0.471 | 0.89 [0.29, 1.68] | 4.67 | 6.69 |
| n17l6 | even_odd | F-tel64 | 11 | 0.508 [0.460, 0.561] | 0.502 | 1.19 [0.33, 2.56] | 1.39 | 6.64 |
| n17l6 | even_odd | F-tel512 | 11 | 0.513 [0.462, 0.569] | 0.499 | 1.30 [0.69, 2.05] | 1.59 | 6.29 |
| n17l6 | aa_mod4 | F-x | 17 | 0.488 [0.437, 0.541] | 0.480 | 0.63 [0.14, 1.32] | 2.68 | 6.08 |
| n17l6 | aa_mod4 | F-xx | 153 | 0.484 [0.435, 0.536] | 0.462 | 1.27 [0.57, 2.08] | 4.04 | 6.08 |
| n17l6 | aa_mod4 | F-tel64 | 11 | 0.486 [0.438, 0.541] | 0.449 | 0.78 [0.23, 1.57] | 1.68 | 6.03 |
| n17l6 | aa_mod4 | F-tel512 | 11 | 0.505 [0.457, 0.557] | 0.486 | 0.94 [0.70, 1.19] | 1.41 | 5.75 |

4. **A cutoff policy (L2) gains nothing.** The train-optimal fixed cutoff
   gives 0.91–1.05× on held-out data. A witness found early is no more
   likely than one found late, so a cutoff trades relations for conflicts at
   par.

| family | split | train-optimal cutoff (conflicts) | train gain | test gain [95% CI] | positive-control AUC | positive-control gain |
|:--|:--|:--|:--|:--|:--|:--|
| n15l5 | even_odd | 1922 | 1.25 | 0.93 [0.79, 1.09] | 0.747 [0.717, 0.777] | 2.11 |
| n15l5 | aa_mod4 | 1915 | 1.12 | 1.05 [0.91, 1.19] | 0.785 [0.756, 0.813] | 2.81 |
| n17l6 | even_odd | 9022 | 1.12 | 0.91 [0.64, 1.21] | 0.717 [0.675, 0.769] | 2.23 |
| n17l6 | aa_mod4 | 3264 | 1.33 | 0.98 [0.50, 1.55] | 0.725 [0.682, 0.772] | 2.74 |

5. **Branching order (L3): default VSIDS is already at the floor.** Putting
   the x-bits first reproduces the default trajectory exactly. Putting
   `X₁, X₂` first is 1.12–1.22× worse. The order learned from VSIDS
   activity is ≥ 10× worse on every held-out target (all capped, so the
   ratio is a lower bound): 10 of its 12 n17l6 variables, and 8 of 10 at
   n15l5, are e-variables, and branching on those is the a3 arm. The
   variables that take part in conflicts are not the ones worth deciding on.

| family | arm | targets | capped | median conflicts | sum / a0 | conflicts per sorted triple | conflicts per pair |
|:--|:--|:--|:--|:--|:--|:--|:--|
| n15l5 | a0_default | 60 | 0 | 5759 | 1.000 | 0.963 | 5.629 |
| n15l5 | a1_x_all | 60 | 0 | 5759 | 1.000 | 0.963 | 5.629 |
| n15l5 | a2_x1x2 | 60 | 0 | 6466 | 1.120 | 1.079 | 6.304 |
| n15l5 | a3_e_vars | 60 | 60 | ≥ 57810 | ≥ 10.030 | ≥ 9.661 | ≥ 56.455 |
| n15l5 | a4_learned_activity | 60 | 60 | ≥ 57810 | ≥ 10.030 | ≥ 9.661 | ≥ 56.455 |
| n17l6 | a0_default | 30 | 0 | 45600 | 1.000 | 0.998 | 11.145 |
| n17l6 | a1_x_all | 30 | 0 | 45600 | 1.000 | 0.998 | 11.145 |
| n17l6 | a2_x1x2 | 30 | 0 | 55806 | 1.220 | 1.217 | 13.599 |
| n17l6 | a3_e_vars | 30 | 30 | ≥ 456490 | ≥ 10.000 | ≥ 9.976 | ≥ 111.448 |
| n17l6 | a4_learned_activity | 30 | 30 | ≥ 456490 | ≥ 10.000 | ≥ 9.976 | ≥ 111.448 |

| features | trained on | scored on | AUC [95% CI] |
|:--|:--|:--|:--|
| F-tel64 | n15l5 (all 3000) | n17l6 (all 1200) | 0.496 [0.458, 0.531] |
| F-tel512 | n15l5 (all 3000) | n17l6 (all 1200) | 0.520 [0.482, 0.557] |

## Verdict against the frozen rules

- **Fires** (some lever ≥ 1.5× with CI lower bound > 1.2× on both
  families): **no**. The largest held-out CI lower bound is 0.92.
- **Closes** (every gain CI includes ≤ 1.2× and every AUC CI includes
  ≤ 0.6): **yes**.
- **Predictions.** P1 (x-bit AUC in [0.45, 0.60]): held, 0.461–0.526. P2
  (telemetry AUC ≤ 0.65): held, ≤ 0.556. P3 (L2 gain ≤ 1.3×): held,
  0.91–1.05. **P4 (every L3 arm within 0.7–1.4× of a0): failed.** The
  e-variable and learned-activity arms are ≥ 10×.

The mechanism a learned component would need is a cheap function of the
target, or of the first few hundred conflicts, that predicts membership in
the 3-fold sumset of the factor base. At this scale none of the
representations tried here has one. The exact affine screen of
[ecc2k130_pdp_features_20260924](../ecc2k130_pdp_features_20260924/) is the
only certified signal known to the repository, and it is `m = 2`
structure. Even a perfect classifier is capped by the product law
(`relations × targets × oracle ≈ m·2^n`): it removes a constant from the oracle box (measured here at 5–9×)
and cannot change the exponent. This
row closes "ML-guided SAT" for the symmetrised-`S₄` SAT oracle, with
measurements behind the survey's earlier dismissal.

## Side result: native solver soundness bug (fixed in this PR)

The first launch stopped at its own label cross-check. Calling
`Solver::solve` again after a conflict-budget `Unknown` resumed from a
trail with an unanalysed conflict. The resumed solve answered wrongly in
both directions: on n15l5, x_r = 0x1fc0 came back SAT with a model that is
not a decomposition, and x_r = 0x7bc9 came back UNSAT although it
decomposes. The solver now restarts from level 0 when it is entered, and
keeps its learnt clauses. The regression test
`resume_after_budget_stop_is_sound` fails without the fix and passes with
it. An audit of every `solve()` caller found no published artefact that
re-solves after `Unknown` without `reset_search()`.

## What this cannot show

- **Scope.** Only these two toy families (`n ≤ 17`, `ℓ ≤ 6`) and one
  solver. Learners are linear and quadratic logistic models. A deep model
  could in principle find what these miss, but the positive control and the
  5–9× ceiling bound what it would be competing for.
- **Not covered.** Nothing here speaks to `n = 131`, ECC2K-130 or rho.
  Gap to the matched rho: not measured here; this is a stage diagnostic.
- **Host.** The run was not on Iso Lab (the container has no AWS identity).
  Conflict counts are deterministic given the seed and commit; wall times
  in the rows are diagnostic only.

## Reproduce

    cargo run --release --example pdp_ml_sat_ceiling -- /tmp/pdp-ml-rows          # labels + analysis, about 12 min on 4 cores + L3
    python3 tools/artifacts/artifacts.py pull research/pdp_ml_sat_ceiling_20261027   # hydrate rows (artifact store, hash-checked)
    cargo run --release --example pdp_ml_sat_ceiling -- /tmp/pdp-ml-an --from-rows research/pdp_ml_sat_ceiling_20261027/run1

Rows: `run1/rows_*.jsonl`, held in the artifact store (`run1/.artifacts.json`; labelled at commit 7dddd8099). Analysis:
`run1/analysis/` (commit d673109e7). SHA-256 values are in the run report.
