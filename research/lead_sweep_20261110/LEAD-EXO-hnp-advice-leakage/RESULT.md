# RESULT: LEAD-EXO-hnp-advice-leakage

## Outcome
- status: ran. wall 7 s (run.status), exit code 0. run.stderr empty.
- One run of run.py, unmodified, seed 20261110, 6 sizes (12..22 bits), 25 trials each.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md section 3): Cheon slope <= 0.35 and rho slope >= 0.42 and BSGS-null slope >= 0.42, zero recovery failures and wrong answers; A/A median ratio in [0.7, 1.4]. consistent_with_prediction if all hold, falsified if Cheon slope > 0.35.

Measured:
- Cheon fit slope 0.2516 (theory 0.25). Medians 25, 35, 49, 69, 106, 138 scalar mults.
- rho fit slope 1.0063 (theory 0.5). Medians 411 .. 460707.
- BSGS-null slope 0.5039 (theory 0.5). Medians 98 .. 3426.
- recovery_failures 0, wrong_answers 0.
- A/A rho vs rho, median ratio 1.0148 (per-size 0.87..1.16), inside [0.7, 1.4].
- Cheon/rho at 1 op per scalar mult: 0.061 (12 bits) down to 0.0003 (22 bits). At log2(n) ops per scalar mult: 0.74 down to 0.0067. Cheon is cheaper than rho at every size under both pricings; the crossover is not decided on.

Verdict: consistent_with_prediction (run.py field). Controls: positive (recovery verified against the secret, 0 failures) passed; null BSGS slope 0.504 passed; A/A passed.

## Reading
- The toy does test the narrow claim: given d | n-1, d about sqrt(n), and leaked [a^d]G, Cheon scales as about n^(1/4) and no-advice BSGS as n^(1/2). The leak is assumed, not tested, as the protocol states.
- Discrepancy: the rho slope is 1.006, twice the theoretical 0.5. The preregistered threshold (>= 0.42) is passed, but a generic rho should show about 0.5. At 22 bits rho used about 460k steps against sqrt(n) about 2.2k, roughly 200x the expected count. The likely cause is the additive Z_n walk with a partition on x mod 16 (and the Floyd/restart scheme) being a poor random map. Possibly it behaves like a near-linear walk. I did not investigate (no edits or reruns allowed).
- Consequence: the rho reference is a weak, probably mis-specified baseline. The Cheon/rho ratios are overstated and should not be quoted as a matched-rho gap. The BSGS null is the sound comparator (slope 0.504).
- The Cheon exponent is a property of the choice of d in the toy (d chosen near sqrt(n)), so the 0.25 slope is by construction. It confirms known theory.
- A/A passing only shows the rho noise floor is small. It does not validate rho as a faithful generic rho.

## Bearing on the sub-sqrt(p) question
closes or confirms the recorded obstruction: it re-confirms that the 0.25 vs 0.5 gap exists only with the leaked auxiliary points [a^i]G. Nothing is shown for a leak-free ECDLP.

## Standing
[pcc:unverified], single local toy run, no independent replay.
