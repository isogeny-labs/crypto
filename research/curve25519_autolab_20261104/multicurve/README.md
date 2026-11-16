# Multi-curve re-baseline of the ledger incumbents

Claim `PCC-20261105-37a78d`, standing `unverified`. Every earlier comparison in this loop used one curve per size (all 37 holdout specs have exactly one). This run measures the ledger incumbents and rho-neg on five Curve25519-family curves per size.

## Curves

The family rule (`curve25519_family`) takes the largest prime p = 5 mod 8 below 2^k and the smallest valid A. The new `siblings` subcommand applies the same rule to the next primes down, with A at most 4000. Each size uses the ledger curve and four siblings. Specs were frozen before data (`spec-k*.json`, siblings in `siblings-k*.json`).

## Result

12 fresh public targets x 2 rounds per curve. The audit checked 720 of 720 records at each size and reproduced 8 of 8 replays. S is at w = 0, averaged over the five curves.

| k | incumbent | incumbent S | rho-neg S | mean per-curve ratio | per-curve range | SD |
|--:|:--|--:|--:|--:|:--|--:|
| 20 | `wsum2g-b15-n2` | 2.601 | 1.660 | 1.57 | 1.44 to 1.69 | 0.09 |
| 24 | `wwalkGg-x09-sr-n2` | 2.111 | 1.193 | 1.78 | 1.52 to 2.10 | 0.21 |
| 28 | `wwalkGg-x09-sr-n2` | 1.869 | 1.040 | 1.80 | 1.62 to 1.94 | 0.12 |
| 32 | `wwalkG-x06-sr-n2` | 1.930 | 1.013 | 1.91 | 1.69 to 2.13 | 0.16 |

## Reading [pcc:unverified]

- The single-curve ledger ratios (1.76x, 1.94x, 1.81x, 1.82x) are within the curve-to-curve spread. The plateau near 1.8x to 1.9x holds across curves at k = 24 to 32.
- At k = 20 the box incumbent is 1.57x on average, better than the 1.76x of the ledger curve.
- The ratio of one size varies by about plus or minus 0.2 between curves, which is larger than most differences the loop resolved between arms. A paired comparison within a curve stays valid; a ratio to rho quoted from one curve does not carry to others.
- No incumbent S falls below the two-unknown-seed floor of 1.662, so none trips the audit trigger in OBSTRUCTION.md.
- The incumbents' parameters (B or x) were tuned on the ledger curve; their ratios on the sibling curves are held-out.
