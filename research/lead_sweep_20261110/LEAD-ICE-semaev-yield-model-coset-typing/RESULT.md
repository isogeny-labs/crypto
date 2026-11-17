# RESULT: LEAD-ICE-semaev-yield-model-coset-typing

## Outcome
- status: ran. wall 7 s, exit code 0 (run.status: exit=0 wall=7). run.stderr empty.
- Single run of the frozen run.py, no retries.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): |R_x_untyped - nm| <= band and |R_x_typed - nm| <= band, with band = max(3*ns, 0.02); positive control (AP base) must sit below nm by more than the band; A/A within band. Verdict if all hold: consistent_with_prediction.

Measured (R = covered / (N(1-exp(-T/N))), pooled over 4 sizes x 10 reps):
- null: nm = 0.9498, ns = 0.0236, band = 0.0708
- x_untyped 0.9571 (diff +0.0073), x_typed 0.9577 (diff +0.0079): both inside band
- null_untyped 0.9499, null_typed 0.9497, aa_untyped 0.9532 (|aa - null_untyped| = 0.0033 <= band)
- pos_ap 0.0134 (nm - R_pos = 0.936 >> band)
- Controls: positive_detected = true, aa_ok = true, null defined. Controls passed.
- Verdict: consistent_with_prediction.

## Reading
- run.py's verdict field matches the numbers; no discrepancy.
- The x arms sit about 0.007 above the null mean at every size (about 0.3 sd of the pooled null sd), a small consistent positive offset but well inside the band. Per size (12..18) the x-vs-null gaps are +0.017, +0.009, +0.006, -0.004, shrinking with size. The smoke run's 1.005 vs 0.91 gap did not appear at full size.
- Band (0.071) is wide relative to the 0.007 offset, so the test could not detect an effect smaller than about 7 percent of coverage.
- The toy tests only m=3 coverage conservation on prime-field toys with Legendre(x) as a labelling. Not the binary trace-parity confinement, not Groebner solver degree, not the real IDEA-20260918-c05e71 design. Typed bases have larger B (about 1.35x) because T is matched to >= N, so the m! trade shows only as base size.
- Coverage is a proxy for relation yield; no cost, rho or GAE pricing.

## Bearing on the sub-sqrt(p) question
closes or confirms the recorded obstruction: at toy scale typing on a prime field only changes tuple count (constant factor trade), with no non-conserved yield structure seen. It does not test the binary lead itself.

## Standing
[pcc:unverified], single local toy run, no independent replay.
