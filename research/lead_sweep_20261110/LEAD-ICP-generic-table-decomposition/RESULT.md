# RESULT: LEAD-ICP-generic-table-decomposition

## Outcome
- status: ran (single run, no retries). exit code 0, wall 12 s (run.status: exit=0 wall=12). incomplete_runs = 0.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): consistent_with_prediction if IC slope CI lower bound > 0.55 and all controls pass; falsified if IC CI upper bound < 0.55 with controls passing; otherwise inconclusive. Controls: rho CI contains 0.5; free-table gap > 0.08; A/A |diff| < 0.1.

Measured (OLS slope of log2 cost on log2 N, 7 sizes x 5 reps, 300-resample bootstrap):
- IC: 0.7259, CI95 [0.6564, 0.8097] (lower bound > 0.55)
- IC A/A: 0.7555, CI [0.7175, 0.797]; |diff| = 0.0296 (< 0.1, pass)
- Rho (null): 0.5008, CI [0.3373, 0.6704], contains 0.5 (pass)
- Free-table positive control: 0.4651, CI [0.4246, 0.5119]; gap to IC = 0.2608 (> 0.08, pass)
Verdict: consistent_with_prediction. All three controls passed.

## Reading
- The toy tests what the protocol says: a generic arity-2 pair-table relation phase at N in 2^12..2^18, linear algebra excluded. IC exponent about 0.73 is above rho's about 0.50, near the heuristic 2/3 (B^2 + N/B), a bit higher, plausibly from log N factors and the restart charge.
- run.py's verdict field matches the numbers; I found no discrepancy.
- Caveats: the rho CI is wide ([0.34, 0.67]), so the null control is weak. Both IC arms exceed the 0.55 threshold and agree with each other. Seven sizes over a small range bias the slope. The free-table arm at about 0.47 shows that the table build dominates the excess exponent. Nothing was run beyond arity 2, and the 2^18 limit stands.
- Nothing surprising.

## Bearing on the sub-sqrt(p) question
closes or confirms the recorded obstruction: a generic pair table gives relation-phase exponent about 0.73, above rho's 0.5, which matches the recorded closure (measured mitm 0.60-0.75) at toy sizes only.

## Standing
[pcc:unverified], single local toy run, no independent replay.
