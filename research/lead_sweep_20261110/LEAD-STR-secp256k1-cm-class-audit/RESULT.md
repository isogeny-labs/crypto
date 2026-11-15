# RESULT: LEAD-STR-secp256k1-cm-class-audit

## Outcome
- Status: failed (run.py crashed before any rho arm ran). Wall: 9 s. Exit code: 1.
- RESULT.json is empty (no JSON emitted). run.stderr tail:
  `RuntimeError: no prime-order curve` raised at run.py line 31 in find_curve(p, 0, rng, True),
  called from main() line 154 (first j=0 curve search, a=0).
- One run only, per rules. run.py and PROTOCOL.md were not edited; no retry.

## Statistic vs preregistered prediction
- Prediction (PROTOCOL.md section 3): R6 = mean(steps_plain)/mean(steps_orb6) in [1.84, 2.69]
  with controls passing (positive R2 in [0.99,1.63], generic-curve negation null in [0.99,1.63],
  A/A in [0.8,1.25]). R6 > 2.69 with controls passing would falsify the obstruction.
- Measured: nothing. R6, R2, null ratio and A/A ratio were never computed.
- Verdict: not_run. Controls passed: no (not evaluated).

## Reading
- The toy did not test the lead's claim. The crash is in curve setup: find_curve tried its
  4000 random b values (a=0, a prime p ~ 2^B with p=1 mod 3, p=3 mod 4) and found none with
  prime point count > 100. It failed in 9 s, so the smallest or an early size is the likely
  trigger (not confirmed; the traceback does not print B or p).
- Candidate cause (assumption, not verified): for j=0 there are only 6 distinct group orders
  over F_p up to b-class, so if none of the six is prime for the chosen p, no b can succeed;
  the protocol's prime-selection rule does not check this. A different p would be needed.
- The preregistered smoke test (p ~ 2^10) apparently passed, so this is a size-specific
  setup failure, not a logic failure of the walk code.
- No discrepancy between a verdict field and numbers, since no verdict was produced.

## Bearing on the sub-sqrt(p) question
none. No measurement was obtained; the recorded sqrt 6 obstruction is neither confirmed nor
challenged by this run.

## Standing
[pcc:unverified], single local toy run, no independent replay. Failure is itself the recorded
result; fixing it needs a new preregistered protocol (prime selection ensuring a prime-order
j=0 curve exists), not an edit to this one.
