# RESULT: LEAD-RHO-cheaper-step-primitives

## Outcome
- Status: ran (single run, no retries). Exit code 0, wall 0 s (run.status: exit=0 wall=0). run.stderr empty.

## Statistic vs preregistered prediction
- Prediction (PROTOCOL section 3): min over primes, W in {16,64,256,1024}, I in {10,20,40} M, S in {0.8,1.0} M of tag/plain cost per step >= 1.0 and controls pass = consistent; < 0.95 with controls passing = falsified; between = inconclusive.
- Measured: min_ratio = 1.253 (W=16, I=40 M, S=0.8 M; same at all four primes). Ratios rise to about 1.83 at W=1024. Tag cost is about 11.0 (10.4 at S=0.8) M per step; plain is 8.3 down to 5.8 M per step.
- Controls: positive lockstep rate 1.0 (>= 0.9, pass); raw-X negative control merge rate 0.0 (<= half, pass); fraction of points whose X mod r changes under rescaling 0.943 (> 0.8, pass); A/A ratio 1.0 (within 1%, pass). controls_ok = true.
- Verdict: consistent_with_prediction.

## Reading
- The toy tests only the cost-model half plus the "raw Jacobian tag is not point-valued" half. It does not measure ns/step, and no collision search is run.
- run.py verdict matches the numbers. Note the tag arm is favoured by the model (free tag, 8M+3S), and still loses at W >= 16.
- Out-of-statistic observation: at W=8 and I=40 M, S=0.8 M the ratio is 0.963, so the tag arm edges ahead only for very few walks per inversion and the most expensive inversion. This is outside the preregistered W >= 16 range and does not change the verdict. Even there it is not below 0.95.
- The primes actually used were 2063, 4099, 8219, 16411 (next p = 3 mod 4 at or above the nominal values). Counts are deterministic, so the four primes give identical ratios up to rounding.
- Point halving on binary curves is not tested.

## Bearing on the sub-sqrt(p) question
closes or confirms the recorded obstruction: under the counted-op model an inversion-free step is not cheaper than batched affine at W >= 16, and the raw tag is not point-valued; this is at most a constant-factor matter anyway, never a sub-sqrt(p) one.

## Standing
[pcc:unverified], single local toy run, no independent replay.
