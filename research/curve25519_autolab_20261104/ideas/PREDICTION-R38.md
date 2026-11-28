# Round 38 prediction (written before any data of the round; every number is [pcc:unverified])

Written 2026-11-17, before `ic_autolab freeze --round 38`. The generic-track candidate space is nearly exhausted: `propose` says "nothing left to propose (every family and neighbour tried)" at k = 28 and k = 32, and returns exactly one untried arm at k = 20 and k = 24, the cross-size transfer of the k = 28 incumbent. This round is therefore a single-hypothesis test, with no exploration slots.

## Question

The k = 28 incumbent `wwalkGg-x09-sr-n2` (`wwalk:x=0.9,g=1,grow=2,seed=run`) has S = 1.883 / 1.926 / 2.309 at w = 0 / 0.1 / 1 (ledger, round 33). The incumbents at k = 20 and k = 24 have S = 2.575 / 2.691 / 3.741 (`wsum2g-b15-n2`) and 2.016 / 2.189 / 3.743 (`tw2l-x20-j32-n2`). The ratio S(w = 1) / S(w = 0) is 1.23 for the transfer arm and 1.86 for `tw2l`: the transfer arm stores far fewer points per relation. Does it transfer, and does it order below the incumbent at every weight, which the decision rule requires?

## Arms

`rho-neg` (matched reference), `inc`, `inc-aa` (A/A), and the candidate `wwalkGg-x09-sr-n2`, at k = 20 and k = 24, under the round's frozen protocol (default target counts: 16 train targets, 32 holdout targets x 2 rounds).

## Predictions, derived from the ledger

- k = 24. The seed=run re-baseline gives the wwalk incumbent S = 2.11 at w = 0 at k = 24, and the w-profile of the k = 28 arm scales it to about 2.16 at w = 0.1 and 2.59 at w = 1. Against `tw2l-x20-j32-n2`: sel / inc about 1.05 at w = 0, 0.99 at w = 0.1, 0.69 at w = 1. Expected class: **null** (the w = 0 interval contains or exceeds 1), with the candidate ahead at w = 1 only: a trade, like rounds 6 and 10.
- k = 20. wwalk arms were regressions or nulls against `wsum2g-b15-n2` at k = 20 in rounds 32 (1.092 [0.956, 1.248], fixed table) and 34 (null 0.938 [0.864, 1.019] at x = 0.3 to 0.5); the fixed table flatters wwalk by 15 to 22 percent (R4-I1). Expected sel / inc at w = 0: 1.1 to 1.3, class **regression** or null; w = 1 ratio 0.8 to 1.0.
- Model. Under the K = 3 floor (`OBSTRUCTION.md`) neither arm can go below S about 1.66 at w = 0 plus set-up; a candidate with S below 1.7 at either size would contradict the floor ratio and be a finding in itself.

## Decision rule and what each outcome means

The frozen rule: improvement [pcc:unverified] needs the paired sel / inc interval below 1 at w = 0 and the band order sel < inc at every weight.
- Improvement at k = 24 or k = 20 (contrary to the prediction): ratchet, and the cross-size transfer is a real lever for the low-insert family.
- Null or regression at w = 0 with sel / inc below 0.8 at w = 1 (the prediction at k = 24): the arm is recorded as the Pareto alternative at w = 1, not ratcheted; the ledger's incumbents at k = 20, 24 and 32 (w = 1 distance 1.5x, 1.8x, 1.8x) are then shown to trail the k = 28 one by their w = 1 table cost, which is the next thing a new family would have to fix.
- Anything else: reported as measured.

## What this does not test

New families (code changes) and the sibling-points mechanism, which was run outside the ledger (`siblings/`, Stages 1 to 5). Counted units only: no wall-time claim; development evidence until the validation gate.
