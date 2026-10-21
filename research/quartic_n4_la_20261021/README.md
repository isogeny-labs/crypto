# N4 iteration 2: double large primes (B2) and structured elimination (B1) against the recorded B0 baseline

2026-10-21. Branch `claude/quartic-ic-n4-la-20261021`, stacked on #2227 (which carries #2283). Frozen protocol:
[PROTOCOL.md](PROTOCOL.md) (commit `eab1bdfacd`, not edited). Report: [REPORT.pdf](REPORT.pdf). Records: `results/`
(`grid/new_records.jsonl` for this study, `baseline_B0_and_rho_records.jsonl` copied from
`research/quartic_n4_20261019/results/grid/all_records.jsonl`, the B0 baseline and the matched rho runs of #2283).
Sources are `common.rs`, `ghs.rs` (byte-identical to #2283) and `ic.rs`, `solver.rs`, `n4.rs`, `summ.rs` (new). Native
Rust; Python only lays out the figures.

## Answer in one paragraph

B2 (double large primes) is a **measured improvement over the B0 baseline**: every one of its 57 units verified
(`kG = Q`), and the paired cost ratio B2/B0 is 0.355 [0.324, 0.392] at q = 4051, 0.224 [0.222, 0.227] at 65479 and
0.297 [0.295, 0.299] at 262111, with the interval below 1 at every q >= 4051. Against the matched rho the distance
shrinks to IC/rho = 0.26 [0.23, 0.29] at q = 4051, 0.05 at 65479 and **0.024 (0.02 to 0.03) at q = 1 048 507**
(`ell` about 2.8e17), band-resolved below rho for every q >= 991; the exponent on the primary range q >= 4051 is
**1.045 [1.037, 1.052] against rho's 1.476 [1.438, 1.508]**. B1 (structured elimination) is a small stage gain
(B1/B0 = 0.81 to 0.91 up to q = 65479) and a regression at q = 262111 (1.53); it is recorded as a trade, not adopted. B2 becomes the new
baseline. Scope as before: GHS-weak family only, plain rho, shared VM, no generic or prime-order claim.

## What B2 does

A line through two factor-base points `A, B` meets the quartic in two more rational points `X, Y`, the roots of
`c4 s^2 + (c4 + c3) s - c1` with `c4 = F(D)`, `c1 = grad F(A).D`, `D = B - A` and `c3` from `F(A - D)` (about 85
multiplications and one square root per pair). Both outside the factor base: an edge between two large primes. One
inside: a grounded vertex (its log is `eta` minus the base part). Breadth-first potentials turn every non-tree edge
into a relation among factor-base logs and `eta` (a row of weight about 20 to 27, inferred from the linear-algebra operation counts, not recorded), a random subset of 1.2 n + 30 rows goes
to Lanczos, and the graph gives every reached vertex a log by propagation, so the descent finds its three points
among about 80% of all rational points. The factor base shrinks from `3.0 q^0.6` to `2.0 q^0.5` points (n = 2048 at
q = 1 048 507 against 5349 for B0 at 262 111), so linear algebra and the relation phase are both about `q^1.0`.

## Results (cold single-target cost in M-eq; rho runs as in #2283, 18 to 150 per size, 12 and 18 at the two new sizes)

### B2

| arm | q | units ok/total | rho runs | IC M-eq | rho M-eq | IC/rho [95% boot] w=0 | w=0.1 | w=1 | resolved |
|:--|--:|--:|--:|--:|--:|:--|:--|:--|:--|
| B2 | 223 | 5/5 | 150 | 8.010e5 | 2.081e5 | 3.85 [2.67,5.25] | 3.62 [2.53,4.93] | 2.51 [1.85,3.30] | rho lower |
| B2 | 499 | 5/5 | 150 | 8.226e5 | 5.875e5 | 1.40 [1.08,1.79] | 1.30 [1.01,1.66] | 0.85 [0.68,1.06] | unresolved |
| B2 | 991 | 5/5 | 150 | 1.209e6 | 1.530e6 | 0.79 [0.71,0.88] | 0.74 [0.66,0.83] | 0.52 [0.45,0.62] | IC lower |
| B2 | 2011 | 5/5 | 150 | 1.971e6 | 4.545e6 | 0.43 [0.39,0.49] | 0.41 [0.36,0.45] | 0.28 [0.26,0.31] | IC lower |
| B2 | 4051 | 5/5 | 150 | 3.387e6 | 1.315e7 | 0.26 [0.23,0.29] | 0.24 [0.22,0.27] | 0.18 [0.16,0.19] | IC lower |
| B2 | 8191 | 5/5 | 120 | 6.201e6 | 4.066e7 | 0.15 [0.14,0.17] | 0.14 [0.13,0.16] | 0.11 [0.10,0.12] | IC lower |
| B2 | 16363 | 5/5 | 120 | 1.164e7 | 1.128e8 | 0.10 [0.09,0.11] | 0.10 [0.09,0.11] | 0.07 [0.07,0.08] | IC lower |
| B2 | 32719 | 5/5 | 80 | 2.289e7 | 2.803e8 | 0.08 [0.07,0.09] | 0.08 [0.07,0.09] | 0.06 [0.05,0.07] | IC lower |
| B2 | 65479 | 5/5 | 80 | 4.498e7 | 9.202e8 | 0.05 [0.04,0.06] | 0.05 [0.04,0.05] | 0.04 [0.03,0.04] | IC lower |
| B2 | 131071 | 3/3 | 24 | 8.979e7 | 2.550e9 | 0.04 [0.03,0.04] | 0.03 [0.03,0.04] | 0.03 [0.02,0.03] | IC lower |
| B2 | 262111 | 3/3 | 18 | 2.276e8 | 6.400e9 | 0.04 [0.03,0.05] | 0.03 [0.03,0.04] | 0.02 [0.02,0.03] | IC lower |
| B2 | 524287 | 3/3 | 18 | 4.908e8 | 2.008e10 | 0.02 [0.02,0.03] | 0.02 [0.02,0.03] | 0.02 [0.01,0.02] | IC lower |
| B2 | 1048507 | 3/3 | 12 | 1.087e9 | 4.489e10 | 0.02 [0.02,0.03] | 0.02 [0.02,0.03] | 0.02 [0.01,0.02] | IC lower |
| B2 | 13 | 0.895 [0.878,0.915] | 1.482 [1.462,1.499] | 0.587 [0.558,0.612] | 0.910 [0.895,0.927] | 1.489 [1.469,1.506] |

### Paired against the recorded B0 (cost ratio of means by size, bootstrap over units)

| X / Y | q | X units | Y units | cost ratio X/Y [95% boot] |
|:--|--:|--:|--:|:--|
| B2/B0 | 223 | 5 | 5 | 0.932 [0.630,1.304] |
| B2/B0 | 499 | 5 | 5 | 0.517 [0.385,0.693] |
| B2/B0 | 991 | 5 | 5 | 0.475 [0.422,0.536] |
| B2/B0 | 2011 | 5 | 5 | 0.407 [0.359,0.459] |
| B2/B0 | 4051 | 5 | 5 | 0.355 [0.324,0.392] |
| B2/B0 | 8191 | 5 | 5 | 0.324 [0.313,0.336] |
| B2/B0 | 16363 | 5 | 5 | 0.273 [0.261,0.285] |
| B2/B0 | 32719 | 5 | 5 | 0.248 [0.245,0.252] |
| B2/B0 | 65479 | 5 | 5 | 0.224 [0.222,0.227] |
| B2/B0 | 131071 | 3 | 3 | 0.197 [0.124,0.282] |
| B2/B0 | 262111 | 3 | 3 | 0.297 [0.295,0.299] |
| B1/B0 | 4051 | 5 | 5 | 0.876 [0.820,0.941] |
| B1/B0 | 8191 | 5 | 5 | 0.912 [0.871,0.949] |
| B1/B0 | 16363 | 5 | 5 | 0.813 [0.777,0.845] |
| B1/B0 | 32719 | 5 | 5 | 0.853 [0.837,0.870] |
| B1/B0 | 65479 | 5 | 5 | 0.842 [0.832,0.851] |
| B1/B0 | 131071 | 3 | 3 | 0.875 [0.550,1.254] |
| B1/B0 | 262111 | 3 | 3 | 1.534 [1.523,1.547] |
| B0n/B0 | 4051 | 3 | 5 | 0.983 [0.919,1.049] |
| B0n/B0 | 8191 | 3 | 5 | 1.039 [1.016,1.062] |
| B0n/B0 | 16363 | 3 | 5 | 1.012 [0.965,1.059] |
| B0n/B0 | 32719 | 3 | 5 | 1.027 [1.007,1.046] |
| B0n/B0 | 65479 | 3 | 5 | 1.012 [1.005,1.019] |

### Exponents (primary range q >= 4051, then all sizes)

| arm | sizes | IC exponent [95% boot] (w=0) | rho exponent (w=0) | gap rho-IC [95% boot] | IC (w=1) | rho (w=1) |
|:--|--:|:--|:--|:--|:--|:--|
| B2 | 991 | 5/5 | 150 | 1.209e6 | 1.530e6 | 0.79 [0.71,0.88] | 0.74 [0.66,0.83] | 0.52 [0.45,0.62] | IC lower |
| B0 | 7 | 1.083 [1.040,1.128] | 1.485 [1.437,1.525] | 0.402 [0.335,0.468] | 1.154 [1.114,1.201] | 1.486 [1.437,1.526] |
| B0n | 5 | 1.109 [1.101,1.118] | 1.513 [1.455,1.574] | 0.404 [0.345,0.465] | 1.159 [1.154,1.163] | 1.514 [1.455,1.574] |
| B1 | 7 | 1.167 [1.159,1.174] | 1.488 [1.440,1.528] | 0.322 [0.273,0.363] | 1.178 [1.174,1.182] | 1.489 [1.441,1.529] |
| B2 | 9 | 1.045 [1.037,1.052] | 1.475 [1.437,1.508] | 0.430 [0.391,0.463] | 1.033 [1.028,1.039] | 1.476 [1.438,1.508] |
| B2 | 131071 | 3/3 | 24 | 8.979e7 | 2.550e9 | 0.04 [0.03,0.04] | 0.03 [0.03,0.04] | 0.03 [0.02,0.03] | IC lower |
| B0 | 11 | 0.998 [0.971,1.023] | 1.486 [1.464,1.505] | 0.487 [0.455,0.521] | 1.086 [1.061,1.110] | 1.496 [1.474,1.516] |
| B2 | 13 | 0.895 [0.878,0.915] | 1.482 [1.462,1.499] | 0.587 [0.558,0.612] | 0.910 [0.895,0.927] | 1.489 [1.469,1.506] |

Phase exponents of B2 on the primary range: factor base 0.56, relations 1.01, linear algebra 1.13, descent 0.02.
Phase shares of B2 at q >= 262 111: linear algebra 62 to 67%, relations 32 to 37%, descent and verification under 1%
(`reports/phases_b2.svg`).

### Arms B1 and the control B0n

| arm | q | units ok/total | rho runs | IC M-eq | rho M-eq | IC/rho [95% boot] w=0 | w=0.1 | w=1 | resolved |
|:--|--:|--:|--:|--:|--:|:--|:--|:--|:--|
| B0n | 4051 | 3/3 | 90 | 9.372e6 | 1.360e7 | 0.69 [0.62,0.77] | 0.68 [0.61,0.76] | 0.62 [0.56,0.70] | IC lower |
| B0n | 8191 | 3/3 | 72 | 1.985e7 | 3.935e7 | 0.50 [0.45,0.57] | 0.50 [0.45,0.57] | 0.48 [0.43,0.54] | IC lower |
| B0n | 16363 | 3/3 | 72 | 4.312e7 | 1.154e8 | 0.37 [0.33,0.42] | 0.37 [0.33,0.42] | 0.36 [0.33,0.41] | IC lower |
| B0n | 32719 | 3/3 | 48 | 9.466e7 | 2.949e8 | 0.32 [0.28,0.38] | 0.32 [0.28,0.38] | 0.32 [0.28,0.38] | IC lower |
| B0n | 65479 | 3/3 | 48 | 2.029e8 | 9.547e8 | 0.21 [0.18,0.25] | 0.21 [0.19,0.25] | 0.22 [0.19,0.26] | IC lower |
| B1 | 4051 | 5/5 | 150 | 8.354e6 | 1.315e7 | 0.64 [0.58,0.70] | 0.63 [0.57,0.69] | 0.61 [0.55,0.66] | IC lower |
| B1 | 8191 | 5/5 | 120 | 1.743e7 | 4.065e7 | 0.43 [0.39,0.47] | 0.43 [0.39,0.47] | 0.43 [0.39,0.47] | IC lower |
| B1 | 16363 | 5/5 | 120 | 3.467e7 | 1.128e8 | 0.31 [0.28,0.34] | 0.31 [0.28,0.34] | 0.33 [0.30,0.37] | IC lower |
| B1 | 32719 | 5/5 | 80 | 7.868e7 | 2.802e8 | 0.28 [0.25,0.31] | 0.29 [0.26,0.32] | 0.31 [0.28,0.34] | IC lower |
| B1 | 65479 | 5/5 | 80 | 1.687e8 | 9.204e8 | 0.18 [0.16,0.21] | 0.19 [0.17,0.21] | 0.21 [0.19,0.24] | IC lower |
| B1 | 131071 | 3/3 | 24 | 3.995e8 | 2.543e9 | 0.16 [0.13,0.19] | 0.16 [0.13,0.19] | 0.17 [0.14,0.20] | IC lower |
| B1 | 262111 | 3/3 | 18 | 1.174e9 | 6.402e9 | 0.18 [0.15,0.24] | 0.18 [0.15,0.23] | 0.18 [0.14,0.23] | IC lower |
| B0n | 5 | 1.109 [1.101,1.118] | 1.513 [1.455,1.574] | 0.404 [0.345,0.465] | 1.159 [1.154,1.163] | 1.514 [1.455,1.574] |
| B1 | 7 | 1.167 [1.159,1.174] | 1.488 [1.440,1.528] | 0.322 [0.273,0.363] | 1.178 [1.174,1.182] | 1.489 [1.441,1.529] |

## Predictions and controls

* P1 (all units verify): holds. Six first attempts of B2 failed and were retried (charged): 3 "no full relation
  (eta unanchored)", 1 "too few relations" (q = 223 and 991), and **2 wrong answers at q = 223** (first attempts, seeds
  3 and 4, n = 30 columns, 39 to 40 rows): their hold-out had 0 checkable rows (`holdout_ok = holdout_bad = 0`), so the
  non-unique kernel passed the hold-out vacuously and only the final `kG = Q` check caught it. Next version: require at
  least 5 checkable hold-out rows. No wrong answer at any q >= 499.
* P2 (B2/B0 <= 0.35 for q >= 65479, <= 0.5 at 4051): holds (0.224, 0.197, 0.297; 0.355).
* P3 (B2 exponent in [0.95, 1.15] on q >= 4051): holds, 1.045 (0.895 over all sizes).
* P4 (B1/B0 >= 0.6): holds (0.81 to 0.91), with a regression to 1.53 at q = 262 111.
* P5 (B0n/B0 in [0.9, 1.1]): holds (0.98 to 1.04), the new build did not change B0.
* P6 (B2 band-resolved below rho for q >= 2011): holds, and from q = 991 (unresolved at 499: 1.40 at w = 0, 0.85 at w = 1).
* Controls: up to 80 sampled relations per unit (full, one-prime and **graph-edge**) sum to one class through the canonical `Pic^0`
  group law (`relations_bad = 0`). Of the 57 accepted B2 units, 53 checked 80 relations, two checked 79 and two checked 76; every checked relation matched. ( instance control at q = 524287 and 1048507 (`results/instance_checks_ext.txt`, 4 of 4);
  rho calibration steps/sqrt(ell) 1.29, 1.38 and 1.06 at 262111, 524287 and 1048507 (criterion [1.0, 1.5]), every rho run verified.
* Wall time (practicality note, same shared VM): at q = 1 048 507 B2 takes 14.0 s per solve and rho 241.9 s (ratio 0.058);
  at 262 111 3.2 s against 18.9 s (0.17).

## Limits

GHS-weak family only (`4 | #E0`, `E0/F_{q^3}`); plain rho without a negation map (a negation map could lower rho by up to
sqrt 2, which would raise every IC/rho above by that factor and leave the exponents unchanged); L0 shared 4-core VM; the
rho baseline for q <= 262111 is the recorded runs of #2283 (same code, same instances); the exponent fit extends to
q = 1 048 507 only; B1 at q = 262 111 rests on 3 instances. No generic-curve, prime-order, key-recovery or deployed-curve
claim, no comparison with an external implementation.

## Next improvement

Linear algebra is 62 to 67% of B2 and the rows are heavy (weight about 20 to 27 by the same inference, tree-path vectors): choose cycle rows by weight
under a rank guard instead of at random (the lightest-first selection was rank deficient), shorten the BFS trees (more
grounded vertices, bidirectional), and move the pair stage to batched inversions. Then tune `c` and the row margin, and
require a checkable hold-out.

## Reproduce

```bash
cd research/quartic_n4_la_20261021
rustc --edition 2021 -O n4.rs -o n4
./n4 unit B2 65479 1 2.0 0 1.2 12     # one B2 unit (arm, q, seed, c, rho runs, rows multiplier, maxw)
./run_grid.sh $PWD/n4 results/grid
./n4 summarize results/summary_all.json 0 results/grid/new_records.jsonl results/baseline_B0_and_rho_records.jsonl
python3 reports/plot.py results/summary_all.json reports
```
