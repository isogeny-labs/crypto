# Round 40 prediction (written before any data of the round; every number is [pcc:unverified])

Written 2026-11-17, before `ic_autolab freeze --round 40`. Round 39 resolved `tw2l-x20-j16-n2` at k = 28 (sel / inc 0.944 [0.919, 0.970], 1024 pairs). The same arm is null at k = 20 in round 37 (0.986 [0.941, 1.034], 256 pairs); k = 24 and k = 32 already have a `tw2l` incumbent.

## Question

Is `tw2l-x20-j16-n2` below the k = 20 incumbent `wsum2g-b15-n2` at w = 0? Earlier looks: SNC-panel 0.936 (480 multi-curve pairs), round 36 0.998 (64 pairs), round 37 0.986 [0.941, 1.034] (256 pairs). At k = 20, sqrt(r) is about 1000, so set-up costs weigh more than at k = 28, which weakens the gain from dropping the probe phase.

## Design

One size (k = 20), one challenger, the round-39 protocol: train 64 targets, holdout 128 targets x 8 rounds = 1024 pairs, fresh seeds disjoint from every exposure, arms `rho-neg`, `inc`, `sel`, `inc-aa`, `rho-neg-aa`. The frozen rule is unchanged.

## Prediction

- Pooled estimate of the three looks: about 0.96. Predicted holdout ratio at w = 0: 0.965, half-width about 0.025. Probability of class improvement [pcc:unverified] about 0.35: the point estimate sits within about 1.5 half-widths of 1.
- w = 0.1: 0.95 to 0.97. w = 1: 0.85 to 0.90 (round 37 gave 0.866), so the band condition is not the risk here; the w = 0 interval is.
- Floor check: the k = 20 incumbent sits at S 2.5, 50 percent above the K = 3 floor, so there is more room than at k = 28 but also more set-up cost; a ratio below 0.85 at w = 0 would be a surprise, not an accounting finding.

## Optional-stopping disclosure

This is the fourth look at this arm at k = 20. The round's own 1024 fresh pairs decide under the frozen rule; earlier looks are not pooled into the decision. If the result is null, the arm is closed at k = 20.

## Outcomes

- Improvement [pcc:unverified]: `tw2l-x20-j16-n2` becomes the k = 20 incumbent, and a `tw2l` arm is then the incumbent at every size (k = 24 holds `tw2l-x20-j32-n2`).
- Null or regression: the k = 20 null is confirmed at four times the power; set-up cost at small r limits the walk there.
