# RESULT: LEAD-RHO-dp-multitarget-precomputation

## Outcome
- Status: ran. Exit code 0. Wall 1 s (run.status: exit=0 wall=1). run.stderr empty. One run only, no retries.

## Statistic vs preregistered prediction
- Prediction (PROTOCOL.md section 3): slope_BL in [0.25, 0.42]; falsified if outside the band while controls pass. Null slope in [0.42, 0.58]; 8x table faster at every size; A/A slope difference < 0.08.
- Measured: slope_BL = 0.3278 (in band). Null slope = 0.4958 (in band). A/A diff = 0.0171 (< 0.08). 8x-table slope 0.378; precompute slope 0.758.
- Online means (BL / null / 8x), bits 12..22: 46.5/163.6/17.9; 92.1/290.1/20.5; 87.8/487.1/51.9; 144.9/1066.2/62.6; 269.6/2389.0/123.1; 531.2/4845.7/229.4. Failures 0.
- Tradeoff ratio online*T*D/N per size: 2.72, 3.37, 1.29, 1.77, 1.99, 2.65 (roughly constant, range 1.3 to 3.4).
- Controls: null ok, positive ok, A/A ok.
- Verdict: consistent_with_prediction.

## Reading
- The toy checks only that a Z_N additive-jump walk with T = D = N^(1/3) reproduces the recorded N^(1/3) online scaling, while table-free DP rho gives sqrt(N). It does.
- run.py's verdict field matches the numbers. No discrepancy found.
- Caveats: the 8x-table slope (0.378) is also in the BL band, so slope alone does not separate the arms; the positive control is the per-size online mean. Table entries are tiny (15 to 130), so DP-overhead bias is present. The tradeoff ratio is noisy (a factor of about 2.6 across sizes) and not tightly constant. The A/A check used only two seeds. Precompute slope 0.76 is consistent with N^(2/3) to within the toy's noise.
- The toy has no group-operation costs or batching, so it cannot test a scheme that beats the tradeoff.

## Bearing on the sub-sqrt(p) question
confirms the recorded obstruction: the incumbent Bernstein-Lange tradeoff behaves as recorded in the toy. It gives no sign of anything better and does not reopen the lead.

## Standing
[pcc:unverified], single local toy run, no independent replay.
