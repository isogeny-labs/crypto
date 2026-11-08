# NC-1 Stage 0: slope-coverage controls

BSGS-class boundary result, outside the IC ratchet. Not index calculus. Zero group operations. [pcc:unverified]
Every number below is an unverified assertion produced by `src/bin/nc1_stage0.rs`; no pcc claim has been filed.

## Pre-registered amendment (2026-11-08)

Made by the coordinator after the D0 k = 20 value 0.9211 was seen and before any D3 run: the D0 interval becomes
[0.85, 0.93]. Reason: the i.i.d. control matches 0.887, so the engine is right, and a 16-jump walk shows the same 3 to 4%
excess over i.i.d. in NC-2 Stage 0 (`nc2/RESULTS.md`). Under the amended interval all D0 rows are inside. The
section below records the original stop, which preceded the amendment.

## Status: STOPPED at the D0 control gate (before the amendment)

The task rule: if the D0 random 16-jump walk lands outside [0.85, 0.92], stop and report instead of tuning.
At k = 20 D0 gives E = 0.9211 (40 runs: 5 held-out r values x 8 seeds, sd 0.0008), which is 0.0011 above the
interval edge. The D3 hill climb (train/held-out designs, randomised-b controls) was therefore NOT run, and no D3
number exists. The D3 commands are in `run.sh`, commented out. Nothing was tuned.

Context for the reader, not a justification: D0 is 0.9187 at k = 24 and 0.9179 at k = 28 (inside the interval), and
the i.i.d. control matches the birthday law, so the slope formula is not the cause. The excess is consistent with
short-range repeats in a 16-jump walk (all adjacent pairs share one of 16 difference vectors), a waste term that
shrinks as r grows. That reading is an unverified explanation, not tested here. The D0 k = 20 truncation residual
is 1.14e-4 at T_cap = 3.2 sqrt(r), just above the 1e-4 ranking limit, so that row is a lower bound by a small amount.

## Commands and seeds

`research/curve25519_autolab_20261104/nc1/run.sh` (from the repository root) builds with
`cargo build --release --bin nc1_stage0` and runs `control --k 20 --seeds 8`, `--k 24 --seeds 4`,
`--k 28 --seeds 2`. Seeds are derived in the binary from (r, seed index) with splitmix64. r values: the five confirm
curves per k (`c1/confirm-curves.json`). Raw output: `raw/control-k{20,24,28}.txt` (ROW lines are per run JSON,
SUMMARY lines are means). Unit tests: `cargo test --release --bin nc1_stage0`, 5 passed
(`raw/cargo-test.txt`); they compare the exact formula |Sigma(T)| with a brute force over all d at r = 101 for
random vectors, the BSGS layout, a D3-style design, a D0 walk, b = 0 / equal-b / opposite-b points, with and
without the negation fold. The design table was committed before any run (commit bbb1e900af).

## Results (E in units of sqrt(r); E = sum over T of P(no collision within T points), T_cap = 3.2 sqrt(r), 5.0 without negation)

Set-up uses the cost rule in the source header (1.5 log2 of a coefficient per scalar multiplication; D0 charges 34 full
multiplications for the start and 16 random jumps, which is a property of the model, not of a tuned rho).

| design | k | n runs | E | interval | E + set-up/sqrt(r) | max residual |
|:--|--:|--:|--:|:--|--:|--:|
| i.i.d. points (control) | 20 | 40 | 0.8882 | 0.8862 exact | 0.8882 | 7.6e-5 |
| i.i.d. points | 24 | 20 | 0.8866 | | 0.8866 | 4.3e-5 |
| i.i.d. points | 28 | 10 | 0.8863 | | 0.8863 | 3.7e-5 |
| D0 random 16-jump walk | 20 | 40 | 0.9211 | [0.85, 0.92]: OUTSIDE | 3.3154 | 1.1e-4 |
| D0 | 24 | 20 | 0.9187 | inside | 1.6582 | 8.2e-5 |
| D0 | 28 | 10 | 0.9179 | inside | 1.1380 | 7.4e-5 |
| D1 bsgs.negation layout | 20 | 5 | 1.0021 | [0.95, 1.05]: inside | 1.0374 | 0 |
| D1 | 24 | 5 | 1.0005 | inside | 1.0114 | 0 |
| D1 | 28 | 5 | 1.0001 | inside | 1.0034 | 0 |
| D3 held-out (best) | 20, 24, 28 | not run | not run | | not run | |

### No-negation references (D2 context)

| design (no negation) | k | E | P(first collision by 1.5 sqrt(r)) | published (abstract) |
|:--|--:|--:|--:|--:|
| i.i.d. | 20 / 24 / 28 | 1.2555 / 1.2538 / 1.2535 | 0.6746 / 0.6752 / 0.6753 | 0.6753 |
| BSGS, n = 0.75 sqrt(r) | 20 / 24 / 28 | 1.4182 / 1.4170 / 1.4168 | 0.5621 / 0.5623 / 0.5625 | 0.5625 |
| D2 reconstruction, alpha = 0.75 (best of grid) | 20 / 24 / 28 | 1.4170 / 1.4167 / 1.4167 | 0.5637 / 0.5627 / 0.5626 | 0.71875 |

The D2 reconstruction (body of Bernstein-Lange 2012/294 unread, parameters recalled from the abstract) does not
reach the published 0.71875 and sits at the BSGS value: it is an approximation, and the reference is not reproduced. The
grid alpha in {0.4, 0.5, 0.6, 0.75, 0.9} is in `raw/control-k*.txt`. No ratio against rho-neg is meaningful for
the no-negation rows.

### Implied S / rho-neg at k = 20 (ASSUMED conversion, not measured)

S(w) = E + set-up/sqrt(r) + 30/sqrt(r) + w E, rho-neg S = 1.447 / 1.547 / 2.448 at w = 0 / 0.1 / 1 (record values).

| design | w = 0 | w = 0.1 | w = 1 |
|:--|--:|--:|--:|
| i.i.d. (unrealisable, no set-up) | 0.671 | 0.685 | 0.759 |
| D0 | 2.348 | 2.256 | 1.764 |
| D1 | 0.774 | 0.789 | 0.867 |

The D1 row restates a BSGS-family layout with E about 1.00 and is not a new result. The i.i.d. row is a bound
on what a free random design would give, not an algorithm.

## Approximations and caveats

- The sum is truncated at T_cap; the residual column bounds the neglected survival (not ranked above 1e-4).
- D0 uses an oblivious jump index (seeded PRNG), not an x-dependent walk, as the exact model requires.
- Set-up costs follow the cost rule in the source header; the conversion to S/rho-neg is the record's, applied as assumed.
- No D3 result, so the record's Stage 0 question (a design with E at most 0.84 at k = 20 and 28) is open.
