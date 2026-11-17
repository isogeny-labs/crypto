# Review qualification: retained v3 NIST controls

Reviewed head: `03e4a7d7609e5dc3d406d58bf44f606410f9a78a`. This dated note corrects the interpretation of the preserved original report and note. It does not change the original protocol, rows, summaries, figures or PDF. [Original hashes](original-artifacts.json); [native statistical replay](replay.rs); [replay output](replay.json); [printable qualification](qualification.pdf).

## What the retained rows establish

The independent Rust replay checked all 9,015 JSONL rows, the sample identities/counts, all 15 reported metric summaries (targets, ranks, tied empirical p values, medians, available quantiles, dependent-pair values and convergence differences), and the recorded accuracy and parallelogram gates. All checks passed. The three protocol file hashes match the recorded hashes. This is a replay of stored numerical rows; it does not independently recompute the factoring or rational-height arithmetic.

E1b contains **7,200 control values in total**, consisting of 800 sampled Hasse-interval primes per curve, each yielding three values, on three curves. The original statement “2,400 controls” is correct per curve, not for the full study. There are also nine target rows and 150 planted positive controls. M1 was unchanged in all 7,200 stored control comparisons after the specified ECM supplement, and all 150 stored positive controls passed their numerical detection rule.

| Curve | Control values | Unsplit composite entries | Pair controls | Stored float zeros | Absolute cosine ≤ 1e-12 |
|:--|--:|--:|--:|--:|--:|
| P-192 | 2,400 | 1,301 | 500 | 82 | 296 |
| P-224 | 2,400 | 1,475 | 500 | 101 | 328 |
| P-256 | 2,400 | 1,592 | 500 | 148 | 306 |

## Factoring scope

The original conclusion that the verdict does not depend on the factoring budget is too broad. The retained comparison establishes agreement between rho with a 2^18 iteration budget and that same instrument supplemented by ECM at B1 = 2,000 and 20 curves per remaining composite. **4,368 composite entries remain unsplit.** The rows provide neither complete factors nor certificates excluding every factor below 2^32 from those residuals. Successful positive controls support this instrument on those controls; they do not certify every residual. M1 is the returned small-prime content under the recorded budgets. The target comparisons retain their recorded finite-budget interpretation and do not certify the corresponding ranks under complete factorization.

## Numerical height scope

The original phrase “canonical pairing is exactly 0 for 59–66%” is not established by the finite numerical evidence. The counts 296, 328 and 306 are reproduced with the explicit descriptive threshold **absolute cosine ≤ 1e-12**. Only 82, 101 and 148 stored floating-point values, respectively, are literally zero. Even those numerical zeros are not certificates of an exactly zero canonical pairing. This threshold is a review diagnostic, not a new preregistered decision rule.

The producer evaluates a finite sequence at five doublings in floating-point logarithms. Its dependent-pair checks and 50 parallelogram checks per curve pass the frozen thresholds; the maximum absolute parallelogram defects are approximately 0.001354, 0.001354 and 0.002708 nats. These controls support the stated instrument checks, not a rigorous error bound on every random pair or an exact logarithmic identity. The target values and empirical ranks remain reproduced as stored numerical observations. The rank implementation includes ties conservatively; the zero-valued P-224 target is not assigned a spurious minimum p value.

## Custody and validation boundary

The report discloses a first attempt lost on container restart and a retained rerun of approximately 12,342 seconds. This review does not reconstruct the lost attempt or certify complete accounting across both attempts. The preserved raw rows do not include all factorizations, residual composite values, rational points, or height sequences needed for an independent mathematical recomputation. No new research run was launched. All 18 pages of the original PDF were rendered and visually inspected during review.

From the repository root, repeat the statistical replay with:

```sh
cargo run --locked --manifest-path research/nist_cyclotomic_controls_20261011/review-20261019/Cargo.toml -- research/nist_cyclotomic_controls_20261011
```

Pass the original experiment directory as the sole argument. It fails on a sample-count, identity, summary or recorded gate mismatch. The checked-in JSON is the output from the retained bytes bound by `original-artifacts.json`.
