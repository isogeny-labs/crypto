# RESULT: LEAD-ICP-residual-partial-relations

## Outcome
- status: ran; exit code 0; wall 3 s (run.status: exit=0 wall=3). run.stderr empty. One run only, no edits to PROTOCOL.md or run.py.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): every arm has point slope in [0.45, 0.55]. Falsified if any large-prime arm (A_slp_B*, T_table) has bootstrap CI upper < 0.45 while controls pass.

Measured slope of log2(mean ops) vs log2 N (n=12..22, 200 reps, 400 bootstraps):
- P_B0 birthday null: 0.497 [0.485, 0.509]  (covers 0.5: pass)
- A_slp_B2: 0.494 [0.481, 0.509]
- A_slp_B8: 0.475 [0.461, 0.486]
- A_slp_B128: 0.349 [0.342, 0.358]  (CI upper < 0.45)
- T_table_B8_l_sqrtN: 0.502 [0.490, 0.514]
- Positive control (synthetic 3*N^0.4): 0.378 [0.352, 0.401], detected: pass
- A/A halves of A_slp_B2: 0.497 vs 0.491, |diff| 0.0056 (informational)

Controls passed (null, positive, A/A). run.py verdict: "falsified" (arms_below_0.45 = [A_slp_B128]).

## Reading
- The literal preregistered criterion fires, on the B=128 arm only. B=2, B=8 and the table arm sit at or near 0.5.
- The numbers do not show an exponent reduction. The B=128 slope is a finite-size artifact. B is fixed at 128 while sqrt(N) runs from 64 to 2048, so at small n each trial costs 128 ops, the mean cost is floored near B, and the log-log curve is flattened. Cost per N ~ sqrt(N*B) is expected asymptotically. The mean cost curve for B128 is 260 -> 2872 (about 11x over 2^10 in N); the local slope rises toward 0.5 at large n (last steps: 1367 -> 2872 over 4x in N, slope about 0.54). This is my reading from the cost table, not a separate test.
- In absolute cost B=128 is never below the birthday null at any size (260 vs 87 at n=12, 2872 vs 2563 at n=22). B=2 and B=8 are also costlier than B=0. So crediting residuals gave no operation saving in this model, only a worse constant.
- The protocol's falsifier (CI upper < 0.45 on any arm) did not account for the fixed-B regime at toy sizes. The verdict field is mechanically correct but should not be read as "the follow-up is worth building".
- Not tested: real Semaev decomposition, linear algebra, a double-large-prime or graph-cycle solver (only the table variant).

## Bearing on the sub-sqrt(p) question
none / constant factor only: the single-large-prime arms cost the same or more than plain birthday; the sole sub-0.45 slope comes from a pre-asymptotic floor at B=128, not a lower exponent.

## Standing
[pcc:unverified], single local toy run, no independent replay.
