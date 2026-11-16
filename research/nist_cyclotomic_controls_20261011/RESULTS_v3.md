# Controlled re-test: results (v3)

v3, `AMENDMENT_2.md`: E1b and E4r only. Seed base 20261011. Protocol file hashes are recorded in `results_v3.json`. Every verdict below is the decision rule applied mechanically.

## E1b — M1 with ECM-completed factoring (rho 2^18, then ECM B1 = 2000 × 20 curves per composite)

Null: the first N = 800 of v2's Hasse-interval primes per curve (same seeds). Family: 9 targets × M1, Bonferroni α = 0.05/9 = 0.0056. Power check: min attainable p = 0.0025, passes.

| curve | value | M1 target (rho → ECM) | control median [2.5%, 97.5%] | rank / N | p (two-sided) | verdict | controls whose M1 changed under ECM | unsplit composites (controls) |
|:--|:--|--:|:--|--:|--:|:--|--:|--:|
| P-192 | n − 1 | 17.5 → 17.5 | 27.8 [2.6, 87.7] | 256/800 | 0.6417 | within control distribution | 0/800 | 457 |
| P-192 | n + 1 | 7.9 → 7.9 | 30.2 [2.0, 84.0] | 100/800 | 0.2522 | within control distribution | 0/800 | 419 |
| P-192 | twist | 4.5 → 4.5 | 24.4 [0.0, 77.5] | 112/800 | 0.2896 | within control distribution | 0/800 | 425 |
| P-224 | n − 1 | 29.0 → 29.0 | 27.5 [1.0, 95.3] | 420/800 | 0.9513 | within control distribution | 0/800 | 494 |
| P-224 | n + 1 | 8.1 → 8.1 | 27.6 [2.0, 83.1] | 95/800 | 0.2397 | within control distribution | 0/800 | 493 |
| P-224 | twist | 59.0 → 59.0 | 28.7 [0.0, 81.8] | 712/800 | 0.2222 | within control distribution | 0/800 | 488 |
| P-256 | n − 1 | 125.0 → 125.0 | 28.7 [2.6, 88.8] | 796/800 | 0.0125 | within control distribution | 0/800 | 526 |
| P-256 | n + 1 | 41.6 → 41.6 | 29.9 [2.0, 83.7] | 543/800 | 0.6442 | within control distribution | 0/800 | 533 |
| P-256 | twist | 15.1 → 15.1 | 27.6 [0.0, 84.6] | 214/800 | 0.5368 | within control distribution | 0/800 | 533 |

Positive control (`q32 · q100 · r`, q32 a random prime in [2^31, 2^32)); pass = M1 equals log2(q32) exactly:

| curve | detection |
|:--|:--|
| P-192 | 50/50 planted 32-bit primes recovered exactly |
| P-224 | 50/50 planted 32-bit primes recovered exactly |
| P-256 | 50/50 planted 32-bit primes recovered exactly |

## E4r — two-point-lift pairing from rational-model heights (k = 5; k = 4 for the accuracy check)

Null: the same N = 500 random pairs per curve as v2 (same seeds). Bonferroni α = 0.05/6 = 0.0083. Power check: min attainable p = 0.0040, passes. Accuracy gate (pre-registered): on the dependent pairs, abs(|cos| − 1) ≤ 1e-4 and abs(det/(ĥĥ)) ≤ 1e-4 at k = 5 on every curve, else no orthogonality verdict.

| curve | metric | target | random pairs median [2.5%, 97.5%] (N) | rank | p | verdict | dependent pairs ([2]G̃; [3]G̃) | median abs(k5 − k4) over controls |
|:--|:--|--:|:--|--:|--:|:--|:--|--:|
*P-192: parallelogram defect over the first 50 control pairs: max abs = 1.35e-3 nats.*

| P-192 | |cos| | 5.888e-4 | 1.428e-16 [0.000e0, 8.157e-3] (500) | 316/500 | 0.739 | within control distribution | 1.000e0; 1.000e0 | 0.00e0 |
| P-192 | det/(ĥĥ) | 1.000e0 | 1.000e0 [9.999e-1, 1.000e0] (500) | 184/500 | 0.739 | within control distribution | 0.000e0; -3.752e-16 | 0.00e0 |
*P-224: parallelogram defect over the first 50 control pairs: max abs = 1.35e-3 nats.*

| P-224 | |cos| | 0.000e0 | 1.223e-16 [0.000e0, 6.437e-3] (500) | 0/500 | 0.407 | within control distribution | 1.000e0; 1.000e0 | 0.00e0 |
| P-224 | det/(ĥĥ) | 1.000e0 | 1.000e0 [1.000e0, 1.000e0] (500) | 172/500 | 1.000 | within control distribution | 0.000e0; -5.521e-16 | 0.00e0 |
*P-256: parallelogram defect over the first 50 control pairs: max abs = 2.71e-3 nats.*

| P-256 | |cos| | 4.359e-4 | 1.078e-16 [0.000e0, 5.788e-3] (500) | 313/500 | 0.750 | within control distribution | 1.000e0; 1.000e0 | 0.00e0 |
| P-256 | det/(ĥĥ) | 1.000e0 | 1.000e0 [1.000e0, 1.000e0] (500) | 187/500 | 0.750 | within control distribution | 5.798e-16; -6.185e-16 | 0.00e0 |

Accuracy gate (dependent pairs): **passed**. Parallelogram gate (max abs defect ≤ 1e-2 nats on the first 50 control pairs per curve): **passed**.


---
Run time 12342 s. Raw per-sample rows: `rows_v3.jsonl`.
