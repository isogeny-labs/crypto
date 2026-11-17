# RESULT: LEAD-EXO-ideas-corpus-algebraic-transforms

## Outcome
- Status: ran. Exit code 0. Wall 2 s (run.status: exit=0 wall=2). run.stderr empty.
- One run only, no retries, PROTOCOL.md and run.py untouched.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md section 3): max r over all negative-candidate maps and curves < 0.01; positive controls (dbl, tpl, neg) exactly 1.0; null (seeded random function) < 0.01; A/A |rA - rB| <= 0.02. Falsified if any negative candidate has r >= 0.05 with valid controls.

Measured (12 curves, p from 2^8..2^13 range, 3000 pairs per cell):
- max negative-candidate rate: 0.008 (< 0.01)
- null random max: 0.00667 (< 0.01)
- A/A max diff: 0.00367 (<= 0.02)
- controls min: dbl 1.0, tpl 1.0, neg 1.0
- Legendre(y) rates: 0.481 to 0.505 (all within 0.05 of 0.5, none 1.0); reported separately, cannot falsify.

Verdict (run.py): consistent_with_prediction. Controls passed: null yes, positive yes, A/A yes.

## Reading
The toy tests the sharpened claim: generic coordinate maps (x or y into Z_p or F_p^*, x*y+x^2, Legendre y) are not homomorphisms on E(F_p). The numbers agree with run.py's verdict field; no discrepancy. Max negative rate 0.008 is close to the 0.01 threshold but matches the null's noise level (0.0067), so it reads as chance coincidences on small groups, not structure. Primes reach 2^13, a bit beyond the protocol text (k=8..11, "up to 2^14 in full run"); this is within the stated scope. The result is near-tautological, as PROTOCOL section 6 says. It does not evaluate the 15 registry rows or any constructed transform (isogeny, cover, Weil descent), nor any asymptotic cost.

## Bearing on the sub-sqrt(p) question
none: it only confirms that an unconstructed generic map carries no homomorphism, which restates the recorded obstruction (constructors missing) without testing any actual constructor.

## Standing
[pcc:unverified], single local toy run, no independent replay.
