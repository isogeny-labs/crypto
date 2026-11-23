# RESULT: LEAD-TRF-odd-char-descent-smith-quartic

## Outcome
- Status: ran to completion (single run, no retries). Exit code 0, wall 130 s (run.status: exit=0 wall=130).
- RESULT.json: one JSON object, verdict = "consistent_with_prediction".

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): consistent if C_pooled >= 0.80, |C_A - C_B| <= 0.15, >= 30 smooth hits; falsified if C_pooled < 0.50 with >= 30 hits.
Measured (q=5, window W = {|t|<=11, 5 not | t}, |W|=18):
- n1_pass: A 3496, B 3559 of 20000 each; a1=a2=0: A 508, B 429; non-smooth rejected 90 / 69; smooth hits pooled 778.
- coverage_A = 1.0, coverage_B = 1.0, pooled = 1.0, A/A diff = 0.0.
- Distinct t seen: every integer in [-21,16] plus 18 (39 values), so all 18 window traces covered.
- q=7 (descriptive only): 164 smooth hits, coverage 0.84375 of 32.
Verdict: consistent_with_prediction.
Controls: positive control passed (0 N1 mismatches vs brute force on 150 quartics; s3 not divisible by 3 count = 0 in both arms and q=7). A/A passed (diff 0). Negative/null reference: null_uniform_coverage = 1.0, so it does not discriminate.

## Reading
- The toy tests only the existence side: quartic counts with N1=q+1, N2=q^2+1 reach every admissible t at q=5. With ~390-510 candidates per arm over 18 targets, saturation is unsurprising; the 0.80 threshold was easy to meet.
- The null comparison (1.0) shows the check has little power: a uniform draw with the same hit count would also saturate. The result therefore gives little evidence beyond "a1=a2=0 quartics are common at q=5".
- Observed t values include 5 | t and |t|>11 (up to 18, down to -21), i.e. the prereg window is narrower than the full Weil bound 2q^{3/2}=22; not a discrepancy with the verdict field, just a note that the window is conservative.
- No discrepancy between run.py's verdict and the numbers. Smoothness test is partial (protocol s4/s6).
- Nothing tests Smith's construction from a given E, non-degeneracy, explicit isogeny, or Diem's q^{4/3} cost.
- q=7 coverage 0.84 with fewer hits is consistent with sampling shortfall, not an obstruction.

## Bearing on the sub-sqrt(p) question
none: point-count existence at q=5 says nothing about an efficiently computable isogeny or the descent for E(F_p); the recorded obstruction (applies to F_{p^k}, not E(F_p)) is unchanged.

## Standing
[pcc:unverified], single local toy run, no independent replay.
