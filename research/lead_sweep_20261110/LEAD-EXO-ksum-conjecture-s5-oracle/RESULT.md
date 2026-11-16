# RESULT: LEAD-EXO-ksum-conjecture-s5-oracle

## Outcome
status: ran. wall 7 s (run.status; run.py reports 7.26 s). exit code 0. stderr empty. One run, no retries.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md section 3): k=5 slope in [0.55, 0.80] (analytic 2/3); k=6 slope >= 0.5. Falsified if k=5 < 0.55 or k=6 < 0.5 with valid controls.
Measured (log2 mean min-over-B cost vs log2 N, bits 12..22, 3 seeds):
- k=4: 0.687 (analytic 2/3)
- k=5: 0.663 (analytic 2/3), inside [0.55, 0.80]
- k=6: 0.611 (analytic 3/5), >= 0.5
- k=5 A/A (seeds 111-113): 0.668; |diff| = 0.0048 < 0.08, pass
- k=5 planted (FB in [-N^0.25, N^0.25]): 0.168 < 0.663 - 0.10, pass
- k=3: slope null, no points (see Reading)
Verdict: consistent_with_prediction. Controls_pass = true (A/A and positive control; the k=3 null ordering check is not met, see below).

## Reading
- The toy tests only the MITM exponent ladder on a uniform random FB; the measured slopes match the analytic ladder to within about 0.03. The planted arm shows the instrument detects a sub-generic cost (slope 0.17).
- Discrepancy: the k=3 arm returned no points. Presumably every cell exceeded the 4e5-op cap or found fewer than B relations and was discarded. I did not investigate this and did not modify the code. The protocol's "k=3 slope ~1" ordering check is therefore untested. The pass criteria do not depend on it, and run.py's verdict field does not mention it.
- The k=4 slope (0.687) is slightly above k=5 (0.663) and k=6 is 0.611. The ordering k4 > k5 > k6 holds, but k4 and k5 are analytically equal (2/3), so the 0.02 gap is finite-size noise.
- Slopes sit at the analytic values with only 6 sizes and a coarse B grid. They are finite-size fits, not asymptotic exponents.
- Nothing surprising otherwise. Per section 6, this does not cover k-SUM lower bounds, Wagner k-tree, real S_5 geometry, or priced cost (no 5a band).

## Bearing on the sub-sqrt(p) question
closes or confirms the recorded obstruction: it confirms that generic MITM on an unstructured oracle gives exponent about 0.6-0.7, above rho's 1/2, which is the recorded conditional obstruction (not a proof of a lower bound).

## Standing
[pcc:unverified], single local toy run, no independent replay.
