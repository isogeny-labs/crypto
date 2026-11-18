# Round 39 prediction (written before any data of the round; every number is [pcc:unverified])

Written 2026-11-17, before `ic_autolab freeze --round 39`. Round 38 closed the cross-size transfer; the only generic arm with positive evidence at a size where it is not yet the incumbent is the two-seed target walk `tw2l-x20-j16-n2` at k = 28.

## Question

Is `tw2l-x20-j16-n2` below the k = 28 incumbent `wwalkGg-x09-sr-n2` at w = 0, and by how much? Three looks so far, all against the same incumbent: the SNC-panel multi-curve measurement (480 pairs) gave 0.909; round 36 (64 pairs) gave 0.958 (null); round 37 (256 pairs) gave 0.967 [0.927, 1.008] (null, band `sel < inc`). The round-37 interval touches 1 with half-width 0.04.

## Design

One size (k = 28), one challenger, the round-37 protocol except the sample sizes: train 64 targets (was 16, to cut the in-sample selection noise that sank round 38's train screen), holdout 128 targets x 8 measured rounds = 1024 pairs (was 256), fresh seeds disjoint from every exposure. Arms: `rho-neg`, `inc`, `sel`, `inc-aa`, `rho-neg-aa`. Frozen rule unchanged: improvement [pcc:unverified] needs the paired sel / inc interval below 1 at w = 0 and the band order sel < inc at w = 0, 0.1 and 1.

## Prediction

- Pooled over the three looks the point estimate is about 0.94 to 0.96. Predicted holdout ratio at w = 0: 0.95, 95% interval half-width about 0.02 (the round-37 half-width 0.04 scaled by sqrt(256 / 1024)). Predicted class: **improvement**, with probability about 0.6 [pcc:unverified].
- w = 0.1: 0.95 to 0.97. w = 1: 0.97 to 1.0, close to the band edge; this is where the band condition can fail even if w = 0 passes.
- Floor check: the incumbent sits at S 1.88, the K = 3 floor is 1.662 plus set-up (`OBSTRUCTION.md`); a ratio below 0.88 at w = 0 would put the candidate below the floor and trigger an accounting audit, not a ratchet.

## Optional-stopping disclosure

This is the fourth look at the same arm against the same incumbent. The three earlier looks are listed above and are not pooled into the decision: the round's own 1024 fresh pairs decide, under the rule frozen before data. The arm was chosen from earlier data, but the holdout seeds were not exposed. If the result is null, the arm is closed at k = 28 and no further larger holdout of it is run.

## Outcomes

- Improvement [pcc:unverified]: `tw2l-x20-j16-n2` becomes the k = 28 incumbent (the k = 32 one already is). The cost floor reading is unchanged: a few percent from jump chaining and lazy certification, not a new mechanism.
- Null or regression: the round-37 k = 28 null is confirmed at four times the power; the generic track at k = 28 is closed.
