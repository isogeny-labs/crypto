# RESULT: LEAD-ICP-coincidence-harvest-structured-base

## Outcome
- Status: ran to completion. Wall 12 s, exit code 0. run.stderr empty. Single run, no retries.

## Statistic vs preregistered prediction
- Prediction (PROTOCOL.md section 3): all |kappa_rel-1| <= 0.1 and |slope| < 0.03 -> consistent_with_prediction. Falsified only if mean slope > 0.05 and mean/SE > 3.
- Measured kappa_window by rung (bits 12..17): 1.0026, 1.0025, 0.9979, 0.9971, 0.9989, 0.9992. Max |kappa-1| = 0.0029.
- Slope d ln kappa / d ln N: -0.00118, SE 0.00038 (6 replicate series). Slightly negative, far below the 0.05 falsification threshold and inside +-0.03.
- Controls: null (random base denominator) fine. A/A ratio 0.99998..1.0058 on every rung (band 0.85-1.15), pass. Positive control log-GAP kappa 10.6, 15.1, 26.2, 30.0, 42.6, 60.3 (all >= 3), pass. controls_ok = true.
- Verdict (run.py): consistent_with_prediction.

## Reading
- The toy tests the stated window-base claim at 12-17 bits: the window base gives no excess pair-sum x-key coincidences over a random base, and no growth with N. The log-GAP positive control shows the detector sees planted structure, and its kappa grows with N, as expected for an additive-progression base.
- No discrepancy between run.py's verdict and the numbers. The slope mean/SE is about -3, but the sign is negative (kappa slightly falling) and the magnitude is 0.001, so it is not growth. A tiny negative drift is within what finite-size effects could give, and is not interpreted.
- Caveats from PROTOCOL section 6 stand: base is a window (x-interval), not the FAM-1 best log-GAP line; no target preselection; coincidence count is a proxy for relation yield; sizes are toy.

## Bearing on the sub-sqrt(p) question
closes_or_confirms_obstruction: the window base shows kappa_rel = 1 with zero growth, confirming the recorded conservation obstruction at toy sizes (only for the window base, not the untested log-GAP line or preselection).

## Standing
[pcc:unverified], single local toy run, no independent replay.
