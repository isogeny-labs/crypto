# Errata (after the validator report; PROTOCOL.md stays frozen)

- **E1 (validator F1).** PROTOCOL §7 P1 gives as its reason "the 9-target Q mean was about 14% above
  `sqrt(pi r / 2)`". That is wrong. Over the 9 targets (3 curves, two with h = 2 and one with h = 1)
  the measured Q mean is 1.0009× the mean of `sqrt(pi r/2)`. The 14% compared it with the h = 2
  curves' value alone. The prediction is scored on its numbers, not on this reason.
- **E2 (validator F2).** The first bench-record metric for arm w read 1.6046 [1.4069, 1.8371]. From
  `analysis.json` it is 1.60530 [1.40698, 1.83734]. `benchrec` refuses to rewrite a recorded metric ("a rerun is a new record"), so the
  record keeps the mistyped value and this erratum is the correction. The other metrics were entered
  rounded to 4 significant figures.
- **E3 (validator F7).** RESULTS.md did not give the per-curve decomposition. It now does: the pooled
  "below plain rho" rests on the one prime-order curve, 2640.
