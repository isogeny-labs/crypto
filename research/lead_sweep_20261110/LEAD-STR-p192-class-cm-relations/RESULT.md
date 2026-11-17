# RESULT: LEAD-STR-p192-class-cm-relations

## Outcome
- status: ran; exit code 0; wall 2 s (run.status: exit=0 wall=2). run.stderr empty.
- One run only, no retries, run.py and PROTOCOL.md untouched.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): fraction = 1.0 in the CM arm and the A/A arm, all lambda roots of x^2+x+1 mod N, wrong-scalar (lambda^2) control matches < 5% (expected 0%). Any mismatch falsifies.

Measured:
- CM arm (seed 1001): 12 curves, 9408/9408 checks pass (frac 1.0), all_roots true, negative control 0/96 false matches.
- A/A arm (seed 2002): 12 curves, 9408/9408 pass (frac 1.0), all_roots true, negative control 0/96.
- Orders N range 6229..119179 (prime, N = 1 mod 3).
- Verdict (run.py): consistent_with_prediction. Controls passed: null (lambda^2) 0%, positive (lambda found by exhaustive search is a root), A/A 1.0.

## Reading
- The toy checks only that (a + c*zeta)(Q) equals the scalar [(a + c*lambda) mod N]Q on j=0 curves of prime order. That is a theorem-level fact (End of a prime-order cyclic group acts by scalars), so the pass is expected.
- Minor accounting oddity: tot = 9408 = 96 points x 98 elements while smooth_elems = 9312 = 96 x 97, so the per-point element count (97 vs 98) differs by one. This does not change the 100% pass, but the exact element set was not independently audited.
- The negative control (wrong eigenvalue) never matched, so the instrument can reject a wrong scalar. The positive control uses lambda recovered by DLP search, which is circular for the scalar claim but fine as an instrument check.
- The toy uses j=0, class number 1, p < 2^17. Nothing about P-192, its class group, or the completeness gap.

## Bearing on the sub-sqrt(p) question
closes or confirms the recorded obstruction: smooth CM relations on prime-order j=0 groups are known scalar integer relations and give no information about d (confirms only the toy-level obstruction; no cost priced).

## Standing
[pcc:unverified], single local toy run, no independent replay.
