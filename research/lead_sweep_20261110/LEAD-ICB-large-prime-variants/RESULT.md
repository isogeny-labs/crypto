# RESULT: LEAD-ICB-large-prime-variants

## Outcome
- status: ran; exit code 0; wall 0 s per run.status (integer-second timer; the toy is fast, though a 2^22 pure-Python sieve finishing in under 1 s is slightly surprising and was not investigated, since run.py must not be touched). run.stderr empty.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): R = G_aimed/G_unaimed >= 0.8 and G_aimed > 1.2 -> consistent_with_prediction; R < 0.8 or G_aimed <= 1.2 -> falsified; any control failure -> inconclusive.

Measured (20 seeds per cell, target K+10 = 56 usable):
- Mean raw samples unaimed: plain 625.6, 1LP 425.6, 2LP 404.5.
- Mean raw samples aimed: plain 211.75, 1LP 175.5, 2LP 174.55.
- G_unaimed = 1.5466, G_aimed = 1.2131, R = 0.7844.
- G_aimed > 1.2 holds (barely, 1.2131); R < 0.8 fails (0.7844).
Verdict: falsified (run.py field "falsified"; matches my reading of the numbers).
Controls: null G = 1.0 exactly in both samplers (pass); positive control G_unaimed 1.55 > 1.2 (pass); A/A deviation 0.039 (plain) and 0.022 (2LP unaimed), both < 0.15 (pass). controls_ok = true.

## Reading
- The verdict is consistent with the numbers. The falsification rests on the R criterion alone, and only by a margin of 0.016 below the 0.8 threshold; G_aimed passes its threshold by 0.013. Both are close to their cut-offs, and with 20 seeds the sampling noise (A/A deviations 2-4%) is comparable to those margins, so the result is a marginal falsification, not a decisive one. The smoke run's R=0.74 and this R=0.78 point the same way.
- Direction: aimed sampling (multiples of 30) already makes relations about 3x cheaper in plain mode, and large primes then recover less (1.21 vs 1.55). Mild but real attenuation in this model.
- 1LP gains here (1.47 unaimed, 1.21 aimed) because the toy ignores cofactor factoring cost; it cannot reproduce the recorded real-engine finding that 1LP is worse than plain.
- Limitations per PROTOCOL s6: aimed sampling is a divisibility bias, not the m=3 Table A/C recipe; no binary/Koblitz structure, no Semaev/GTTD, no linear algebra growth, no matched rho.

## Bearing on the sub-sqrt(p) question
none: a marginal, constant-factor attenuation of a relation-collection gain in an integer-smoothness toy says nothing about the exponent, and rho remains far ahead; it does not close the real combined step.

## Standing
[pcc:unverified], single local toy run, no independent replay.
