# P2 lever F: partial degree-2 base (interpolating D and E) - pilot, negative

Run 2026-11-17. Binary g16e (sha256 prefix `1c0cd167beb04816`) = `fffa1b1243` +
`../P2_instrument.patch`, which adds arm `F` to the P1 patch. All numbers are
`[pcc:unverified]`. This is a pilot, not a frozen study: 1 curve per cell, 2 seeds.

## Arm

- **F (`F_deg2_part`).** The degree-1 base of arm E, plus a hash-selected
  fraction phi (`GL_D2FRAC`) of degree-2 places as base columns. All other
  places are large primes, at most two per relation.
- **Limits.** phi = 0 is E. phi = 1 is D with large primes allowed.
- **Pairing.** F is paired with the P1 runs of c, D, E and R on the same curve and
  seed.
- **Control.** E is rerun by the new binary. 30 of 30 reruns reproduce P1's E
  field for field.

## Result (`analyze_p2.py`)

| g | p | phi | base cols | F / rho | c / F | min(D,E) / F |
|---|---|---|---|---|---|---|
| 6 | 101 | 0.03 / 0.1 / 0.3 | 130 / 320 / 822 | 0.075 / 0.083 / 0.090 | 2.8 / 2.5 / 2.3 | 0.84 / 0.76 / 0.70 |
| 6 | 211 | 0.03 / 0.1 / 0.3 | 428 / 1205 / 3442 | 0.025 / 0.031 / 0.039 | 0.71 / 0.58 / 0.46 | 0.63 / 0.51 / 0.40 |
| 7 | 53 | 0.03 / 0.1 / 0.3 | 51 / 99 / 245 | 0.078 / 0.074 / 0.066 | 9.1 / 9.5 / 10.7 | 0.85 / 0.90 / 1.01 |
| 8 | 31 | 0.03 / 0.1 / 0.3 | 27 / 50 / 94 | 0.136 / 0.107 / 0.079 | 46 / 59 / 79 | 0.49 / 0.62 / 0.84 |
| 10 | 11 | 0.03 / 0.1 / 0.3 | 12 / 14 / 19 | 2.64 / 1.45 / 0.89 | >=48 / >=87 / >=141 | 0.19 / 0.35 / 0.57 |

All solves are certified.

## Reading

- F's cost is monotone in phi. It moves toward whichever endpoint is better:
  toward E at g = 6, and toward D at g >= 8.
- In 14 of 15 cells F is worse than the better endpoint. The exception is a tie
  (1.01) at g=7 p=53 phi=0.3.
- So there is no interior optimum on these cells. A random subset of degree-2
  places in the base adds columns, and with them relations needed, but makes no
  single relation more likely. Unlike a degree-1 smoothness bound, the selection
  has no "small" end to keep.
- **Verdict: killed at pilot level** for hash-selected subsets.
- **Not tested:** a structured subset, such as degree-2 places whose quadratic
  has a small coefficient, chosen to match a sieve. That is not a different
  probability model and is not expected to change the picture.
