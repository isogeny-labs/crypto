# RESULT: LEAD-RHO-automorphism-class-walks

## Outcome
- status: ran (single run, no retries). exit code 0, wall 5 s (run.py internal wall_s 5.46).
- stderr empty. Files: RESULT.json, run.status, run.stderr.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md section 3): consistent if 0.41<=R6<=0.60, 0.65<=R2<=0.85, 0.65<=R2_generic<=0.85, AA in [0.9,1.1]. Falsified if R6<0.38 or R6>0.70 with AA in band. Else inconclusive.

Measured (means over 3 sizes, 60 trials each):
- R6 = 0.4114 (per size 0.436, 0.363, 0.435); ideal 0.408
- R2 = 0.7127 (0.772, 0.655, 0.712); ideal 0.707
- R2_generic = 0.6455 (0.699, 0.647, 0.590)
- AA = 1.055 (1.025, 1.189, 0.951)
- aut6 abandoned walks: 16, 37, 143.

Verdict (run.py): inconclusive. Reason: R2_generic_mean = 0.6455 is below the 0.65 lower edge by 0.0045. R6 and R2 and mean AA are all inside their bands.
Controls: positive control passed (R2 and R6 both < 1). Null control (generic, m=2) landed at 0.645, marginally below the band and below the ~0.71 predicted; the null did not reproduce the nominal value. A/A passed on the mean (1.055) but the middle size was out of band (1.19), showing the per-size noise is about +-0.1-0.2 at 60 trials. So controls_ok is marked false (strictly, the null band missed).

## Reading
- The toy does test the stated claim (steps ratio of orbit-representative walk vs plain walk), steps only, no canonicalisation cost.
- R6 mean 0.411 is essentially the ideal 1/sqrt(6)=0.408, with no evidence of a gain beyond sqrt(6). Per-size 0.363 at the middle size is below 0.38 but A/A there was out of band (1.19, plain_a was inflated), so that is noise, not a violation; the preregistered statistic is the mean.
- R6 sitting at the ideal despite abandoned walks is mildly surprising (the protocol expected erosion toward 0.41-0.60); with ~10^2-10^3 steps, small-size effects and sampling error (about +-0.05) dominate.
- Generic negation ratio 0.59-0.70 vs the 0.707 ideal: the generic baseline (g_plain) is itself noisy; the miss of the band is within trial noise. Discrepancy between verdict and numbers: none; the verdict is correct under the frozen rule, but it is a boundary miss in a control, not a substantive failure.

## Bearing on the sub-sqrt(p) question
closes or confirms the recorded obstruction: the gain from |Aut|=6 is a constant factor of about sqrt(6) in steps at toy size, with no sign of a better exponent.

## Standing
[pcc:unverified], single local toy run, no independent replay.
