# RESULT: LEAD-EXO-quantum-ecdlp-resources

## Outcome
- Status: ran. Exit code 0. Wall 2 s (run.status: exit=0 wall=2). run.stderr empty.
- Single run of run.py, no retries, PROTOCOL.md and run.py untouched.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): statistic < 0.35, zero bound violations, zero gcd errors, |A - A/A| <= 0.01. Falsified if statistic >= 0.35, any bound exceeded or any gcd wrong.

Measured:
- Mean swap fraction (arm A, seed 1, N=20000 per size, n=12..22): 0.27464
- A/A arm (seed 2): 0.27442; |diff| = 0.00022 (<= 0.01)
- Per-size fraction falls monotonically 0.2853 (n=12) to 0.2674 (n=22)
- Bound violations + gcd errors: 0 (divsteps max 30/35/39/45/50/53 vs BY bound 37/43/49/55/61/66)
- Mean divsteps 22.2 (n=12) to 43.1 (n=22); binary GCD mean 15.8 to 30.0 steps

Verdict: consistent_with_prediction.
Controls: positive (gcd == math.gcd, bound check) passed; null (binary GCD cswap/step = 1.0 at all sizes) passed; A/A passed.

## Reading
- The toy tests the narrow claim: the fraction of divstep iterations with delta>0 and g odd (swap events) is about 0.27, well below binary GCD's 1.0 by construction.
- Divsteps use about 1.4x the iterations of the binary GCD here (e.g. 43.1 vs 30.0 at n=22), so total swap events are not obviously lower per inversion in the toy: roughly 0.27*43 = 11.5 swaps vs 30 cswaps. Still lower, but the null is a straw-man baseline.
- run.py verdict field agrees with the numbers; no discrepancy found.
- Fraction trends slightly downward with n; extrapolation to n=256 is not tested.
- Not tested: reversibility, constant-time form (a data-dependent swap fraction does not translate to a cheaper fixed circuit, since the circuit must handle the worst case each step), Toffoli/qubit/depth counts.

## Bearing on the sub-sqrt(p) question
none: this is a quantum-circuit resource heuristic about a modular-inverse subroutine; it advances no classical or sub-sqrt(p) ECDLP algorithm.

## Standing
[pcc:unverified], single local toy run, no independent replay.
