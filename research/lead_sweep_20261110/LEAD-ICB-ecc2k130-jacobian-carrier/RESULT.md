# RESULT: LEAD-ICB-ecc2k130-jacobian-carrier

## Outcome
- status: ran; exit code 0; wall 2 s (run.status: exit=0 wall=2). Single run, no retries.

## Statistic vs preregistered prediction
- Prediction (PROTOCOL.md section 3): "all four cells pass. Falsified if any cell fails." Cells: a in {0,1} x n in {11,13}.
- Measured: a=0 passes (places_ok, weil_ok) at n=11 and n=13. a=1 FAILS places_ok at n=11 and n=13 (weil_ok true). So 2 of 4 cells pass.
- The a=1 failure holds at every rung (5..23), always N1=4. Read-only side check (not part of run.py) at n=11: B_2 = (N_2 - N_1)/2 = (2-4)/2 = -1 < 0, so there is no non-negative degree-2 place count.
- Controls: null pass rate 0.0 at n=11,13 (<5%, ok; 0.013 at n=5, 0.001 at n=7); A/A pass rates [0.0, 0.0], difference 0 < 0.01 (ok); positive control genus-2 accepted 5000/5000 per seed (>0, ok). Controls passed.
- Verdict (run.py): falsified. Matches the numbers: prediction fails for the a=1 cells.

## Reading
- The toy tests exactly the stated necessary conditions, deterministically. The failure is arithmetic, not a bug seen in the output: for a=1, N_2 < N_1, which forces a negative count of degree-2 places on any curve whose point counts match A.
- Caveat: that rigorous exclusion is for this count-vector construction (N_k = 2^k+1+t_k for k<n, N_n = 2^n+1-(n-1)t_n) taken as the counts of a curve carrying A. I did not independently re-derive that construction. The Weil-restriction normalisation is the main assumption to check.
- a=0 (the curve with N1=2) passes at all rungs; passing does not show existence, and the null control is vacuous at n>=11 (0.0), so discrimination there is only weakly demonstrated.
- The run does not touch ECC2K-130 at n=131 or any cost.

## Bearing on the sub-sqrt(p) question
closes or confirms the recorded obstruction: for a=1 a cheap point-count obstruction exists at every rung, so Torelli rarity is not the only barrier there; a=0 stays only rarity-limited.

## Standing
[pcc:unverified], single local toy run, no independent replay.
