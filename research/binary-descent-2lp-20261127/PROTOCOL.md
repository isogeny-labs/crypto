# Protocol: double-large-prime Weil descent (arm D2) against matched rho

Study `b2lp-20261127/v1`. Seed policy `isogeny-labs/experiment-seeds/v1`.
Status `[pcc:unverified]`, L0, one shared 4-vCPU / 15 GB cloud container.
**Single target only** (owner instruction): every solve is one cold solve of
one public `Q`; no multi-target amortisation anywhere.

Builds on crypto PR #5261 (`research/binary-weil-descent-20261126/`, arm D,
study `bwd-20261126/v1`), stacked on #5253
(`research/binary-composite-rootfind-20261126/`, study `bcr-20261126/v1`).

## Part A — pilot rule (frozen before the pilot is run)

The pilot only chooses the small-base constant θ of each D2 arm and the rho
lane count of the new rungs. It uses held-out instances only:

- θ pilot: curves `d2pilot-curve|n|ci`, targets `d2pilot-target|n|ci|ti`,
  relation stream `d2pilot-stream|n|ci|ti`; n ∈ {30, 36, 42}, ci ∈ {0, 1},
  ti ∈ {0, 1} (12 cases); grids
  - β = 2/3: θ ∈ {0.5, 1, 2, 4, 8, 16};
  - β = 0.528: θ ∈ {1, 2, 4, 8, 16, 32};
  with the small base `fs = min(F − 1, ⌈θ·F^β⌉)` (F classes including T).
  Rule: for each β, θ* = argmin over the grid of
  `Σ_rungs log( gmean_cases Meq(s=1,u=1) )`; ties to the smaller θ.
- rho lanes for the new rungs n ∈ {51, 54, 57, 60}: held-out
  `pilot-curve|n|ci` (ci = 0, 1), `pilot-target`, `pilot-rho|n|ci|ti|W`,
  8 targets per curve, W ∈ {16, 32, 64, 128, 256}; W* = argmin of mean
  Meq(s=1,u=1), the #5253 rule with a grid widened past #5261's edge W = 64.

Pilot outputs: `data/pilot_theta.jsonl`, `data/pilot_rho_ext2.jsonl`. They
are not evaluation data.

Part B (arms, predictions, analysis) is written and frozen after the pilot
and before any evaluation data.
