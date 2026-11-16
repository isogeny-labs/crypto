# k = 20 development screen for the 1.3x target

Claim `PCC-20261107-40672e`, standing `unverified`. Owner request 2026-11-07: aim for 1.3x matched rho at k = 20 and 24 with two or more unknown seeds. At k = 24, 1.3x is S = 1.55, below the two-seed collision floor of 1.662, so it cannot be reached in this family. k = 20 is the size where it is within the floor (target S about 2.02 on the development mean).

## Run

22 arms on the five multi-curve curves (`spec-screen.json`, frozen before data), 12 targets x 2 rounds each. The audit checked 3,960 of 3,960 records and reproduced 8 of 8 replays. Five fresh sibling curves (`confirm-curves.json`) were held back for confirmation and were not used.

## Result (mean S at w = 0, ratio to rho-neg S 1.554) [pcc:unverified]

| arm | S | ratio |
|:--|--:|--:|
| `inc` (box B = 15, stride-generator probes) | 2.636 | 1.70 |
| `ww-j6-x10-g` | 2.677 | 1.72 |
| `ww-j6-x07-g` | 2.739 | 1.77 |
| `ww-j8-x07-g` | 2.791 | 1.80 |
| box B = 12 / 20 / 10 / 8 | 3.034 / 3.115 / 3.316 / 4.685 | 1.96 / 2.01 / 2.15 / 3.03 |
| all walk-probe `ww-*-w` | 4.86 to 5.16 | 3.13 to 3.33 |

## Reading

- The incumbent is at the optimum of its neighbourhood: neighbouring box sizes, jump counts (4, 6, 8, 16), table sizes (x = 0.7, 1.0) and probe types all tie or lose.
- No arm is within 30 percent of the 1.3x target (S 2.02). The two-seed collision floor is 1.662, so 1.3x at k = 20 needs the cost within 22 percent of the floor; the incumbent is 59 percent above it, and no variant in this neighbourhood closes that.
- No confirmation round was run, because no arm was cheaper than the incumbent in sample [pcc:unverified].
- Stride-generator probes cost about 1.8x less than walk probes at k = 20 for every wwalk variant.
