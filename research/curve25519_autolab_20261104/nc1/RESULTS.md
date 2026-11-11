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

## D3 results (run after the amendment; BSGS-class boundary result, outside the IC ratchet) [pcc:unverified]

Commands: the three `d3` lines in `run.sh`, exactly as written (budgets 1000x4, 300x4, 50x4 evaluations; hill-climb seed 1;
randomised-b seeds 8 / 4 / 2). Train on siblings minus confirm r values; E below is on the five held-out confirm r values.
Raw: `raw/d3-k{20,24,28}.txt` (the TRAIN lines hold the designs). c chosen by best train score is c = 8 at every k.

| k | c | train |Sigma| frac at sqrt(r) | held-out E | E + set-up/sqrt(r) | RB per point E | RB per class E | residual flag |
|--:|--:|--:|--:|--:|--:|--:|--:|
| 20 | 2 | 0.505 | 1.0362 | 1.1384 | 0.8881 | 1.1041 | |
| 20 | 3 | 0.541 | 0.9954 | 1.1563 | 0.8880 | 0.9954 | |
| 20 | 4 | 0.582 | 0.9784 | 1.1514 | 0.8881 | 1.0027 | |
| 20 | 8 | 0.605 | 0.9270 | 1.2658 | 0.8880 | 0.9357 | |
| 24 | 2 | 0.501 | 0.9742 | 1.0017 | 0.8867 | 1.0713 | |
| 24 | 3 | 0.549 | 1.0141 | 1.0548 | 0.8867 | 1.0249 | |
| 24 | 4 | 0.572 | 0.9540 | 1.0049 | 0.8866 | 0.9820 | |
| 24 | 8 | 0.595 | 0.9358 | 1.0194 | 0.8867 | 0.9386 | |
| 28 | 2 | 0.500 | 1.0828 | 1.0898 | 0.8864 | 1.0665 | |
| 28 | 3 | 0.539 | 0.9943 | 1.0050 | 0.8863 | 1.0062 | |
| 28 | 4 | 0.547 | 0.9837 | 0.9975 | 0.8863 | 0.9880 | |
| 28 | 8 | 0.582 | 0.9593 | 0.9796 | 0.8863 | 0.9439 | residual 1.8e-4: not ranked |

Controls: D0 0.9211 / 0.9187 / 0.9179, D1 1.0021 / 1.0005 / 1.0001 (E + set-up 1.0374 / 1.0114 / 1.0034) at k = 20 / 24 / 28.
Randomised-b per point returns every design to the Poisson value 0.886 to 0.888, as required. Per-class randomisation
(a few full-size b values) leaves E close to the designed value, so the designs' E sits above 0.886 for structural reasons (wasted same-class pairs), not because of small b values.

Reading: no D3 design is below D0 by 5% held-out (best 0.927 against 0.921 at k = 20; D3 is above D0, and no D3 value is
below the i.i.d. 0.887). Versus the D1 layout, the lowest E + set-up is 0.9975 at k = 28 (c = 4) against 1.0034, and 1.0017 at
k = 24 (c = 2) against 1.0114: differences below 1%, inside the spread across r (sd up to 0.06), so no separation from D1.
The k = 32 gate (held-out E below 0.886 at k = 20 and 28) fails; k = 32 was not run.

Implied S / rho-neg at k = 20 (ASSUMED conversion, not measured), c = 8 headline at w = 0 / 0.1 / 1: 0.932 / 0.932 / 0.930;
c = 4: 0.853 / 0.861 / 0.904; D1 control: 0.774 / 0.789 / 0.867 (all in `raw/d3-k20.txt` CONVERSION lines).
