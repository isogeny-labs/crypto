# RESULT: LEAD-RHO-best-of-candidates-indegree

## Outcome
- status: ran; exit code 0; wall 3 s (run.status: exit=0 wall=3). run.stderr empty.
- Single run of run.py, unmodified; RESULT.json holds the output. smoke=false, 1500 trials/arm/size.

## Statistic vs preregistered prediction
Prediction (PROTOCOL s3): R = mean steps(plain)/mean steps(best-of-2); consistent if CI(R) within (1.05,1.30) and A/A CI contains 1; falsified if upper CI(R) < 1.03; inconclusive otherwise.

Measured:
- N=2^14+1: plain 161.46, best 42.97, R = 3.758, CI [3.582, 3.958]; A/A 0.981 [0.946, 1.022]; plain/randfn 0.9996.
- N=2^18+1: plain 657.78, best 48.89, R = 13.45, CI [12.74, 14.26]; A/A 0.993 [0.960, 1.029]; plain/randfn 1.019.

Verdict (run.py): inconclusive. R lies far above the preregistered window (1.05,1.30), so it is not consistent; it is not falsified either, because the upper CI is not below 1.03. Under the frozen rules this is "otherwise: inconclusive".
Controls: A/A CI contains 1 at both sizes (pass). Positive control plain/randfn is about 1 (0.9996, 1.019), tracking sqrt(pi N/2) (160.4, 641.7) (pass). Null control is the plain arm itself. controls_ok = true.

## Reading
- The instrument is sound (controls pass), but the best-of-2 arm does not measure in-degree concentration at the 1.13 scale. Mean steps for best stays about 43-49 while N grows 16x, so R grows with N (3.8 to 13.5). That is not a constant-factor effect.
- Likely mechanism (my interpretation, not tested): choosing the lower-hash candidate makes each point's successor a hash-biased map, a strongly contracting function with a tiny image, so walks fall into very short cycles and repeat almost immediately. This is a degenerate collapse in the toy, not the mild in-degree concentration that the GPU lead recorded (1.129).
- A repeat of a point in a collapsing map is not a useful collision for rho (no discrete-log information; the toy counts first revisit, not a collision between two independent walks), so the large R is not a speed-up and must not be read as one.
- run.py's verdict field matches the numbers under the frozen rule; no discrepancy. The toy does not test the lead's claim faithfully: the lead's selection preserves a sensible walk on EC points with distinguished points, while this selection by hash order shrinks the reachable set.
- Not measured: net per-step cost (best-of-2 doubles evaluations), batched inversion, GPU W=16-32, distinguished points.

## Bearing on the sub-sqrt(p) question
none: the toy outcome reflects a degenerate hash-biased map rather than the lead's in-degree effect, and the lead is constants-only (Shoup) regardless.

## Standing
[pcc:unverified], single local toy run, no independent replay.
