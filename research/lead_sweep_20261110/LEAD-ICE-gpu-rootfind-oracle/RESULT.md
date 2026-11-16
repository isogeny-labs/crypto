# RESULT: LEAD-ICE-gpu-rootfind-oracle

## Outcome
- Status: ran. Wall 2 s. Exit code 0. run.stderr empty.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): mismatches = 0 and |z| < 4. Falsified if any mismatch, any planted decomposition missed, or |z| >= 4. Inconclusive if only a control or A/A fails.

Measured (p=65519, N=65715, |FB| points=1030, T=600):
- oracle vs direct mismatches: 0
- yield per target: observed 16.207 vs expected 16.144; z_yield = 0.270
- A/A split-half: z_AA = -2.150 (|z|<4, pass)
- positive control: found [60, 60] (100%)
- negative control (b+1): 30900 mismatches (> 0, pass)

Verdict: consistent_with_prediction. All controls passed.

## Reading
The toy checks the algebraic contract of the S3 oracle (roots equal x(R -/+ P1)) and the yield model |FB|^2/N on a 16-bit prime curve, in Python. The run.py verdict agrees with the numbers. z_AA of -2.15 is within the band but not tiny; no other oddity. The toy says nothing about the CUDA kernel, GPU throughput, the 300-1000x estimate, fp256 arithmetic, m>2 decompositions or cost against matched rho (PROTOCOL s6). The mismatch count being exactly 0 is expected for exact algebra; the negative control shows the comparison can detect errors.

## Bearing on the sub-sqrt(p) question
none: it confirms only the textbook oracle algebra and the |FB|^2/N yield at toy size; no scaling or cost information.

## Standing
[pcc:unverified], single local toy run, no independent replay.
