# RESULT: LEAD-RHO-lhw-modular-filter-price

## Outcome
- status: ran; exit code 0; wall 3 s (run.status: exit=0 wall=3). run.stderr empty.
- Single run of run.py, unmodified, no retries.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): pass_fraction >= 0.75 -> consistent_with_prediction;
< 0.25 -> falsified; otherwise inconclusive. Any failed control forces inconclusive.

Measured: pass_fraction = 0.4074 (22 of 54 cells), so the middle band: inconclusive.
Controls: positive (w=1, M=255, m=24: P=31.875, expected 31.875) ok;
negative/null (unrestricted 40-bit; special 1.00518 vs generic 1.00576) ok;
A/A (median ratio of generic halves 1.00046) ok. All controls passed.
run.py verdict: "inconclusive".

Breakdown from rows (my tally):
- Mersenne cells: pass at w=4 (all s>=8, 9/9), at w=6 for s>=10 (6/6, s=8 fails), 0 at w=8.
- Fermat cells: pass only at w=4 for s>=10 (except s=12,m=24) and w=6 for s=12, m>=32.
- Effect shrinks quickly with w; at w=8 the special P is 1.00-1.2, near generic.

## Reading
- The toy computes exactly what the protocol states (exact DP residue collision price), so it tests
  residue non-uniformity, not an ECDLP algorithm.
- The verdict field matches the numbers (0.407 lies in [0.25, 0.75)). No discrepancy.
- Substantive observation: non-uniformity is real and large at low weight (Mersenne P up to ~5.9 at
  s=12, w=4 vs generic max ~1.3-2.4), but the claim as preregistered (>=75% of cells) fails because
  the threshold of 1.2x the max over 12 generic moduli is not met at w=8 or for most Fermat cells.
  Generic moduli themselves show P>1 at small w (max up to 2.4), so the max-of-12 baseline is strict.
- Direction (cheaper vs costlier filtering) is not decided; toy sizes are far from cryptographic.

## Bearing on the sub-sqrt(p) question
none: this only measures residue collision price at toy sizes with no group-operation cost or matched-rho
comparison; the weight-dependent non-uniformity is a constant-factor-type effect at best and untested here.

## Standing
[pcc:unverified], single local toy run, no independent replay.
