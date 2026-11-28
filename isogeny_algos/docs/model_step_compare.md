# Matched curve-model isogeny comparison

## Question and scope

Does changing the representation of one elliptic curve reduce the cost of an odd-degree isogeny step, including the information a path search needs to identify its successor?

Run `cargo run --release --example model_step_compare` from `isogeny_algos/`. The example fixes the prime `p = 1073746799`, the supersingular Montgomery curve `y² = x³ + x`, and one deterministically selected kernel generator and evaluation point for each degree 3, 5, and 7. Its Weierstrass, Montgomery x-only, and Edwards arms use the **same curve and kernel**, rather than selecting a different curve for each model. The normalized Edwards equation is `x² + y² = 1 - x²y²`; conversion from Montgomery uses a square root of 2 in this field.

The example measures three tasks:

1. `successor_j`: given a kernel generator, construct the isogeny and produce the codomain `j`. This is the isogeny portion of a path-search successor calculation.
2. `step_and_eval_x`: also evaluate one point, returning one coordinate in the chosen model.
3. `full_successor_from_curve`: start with the curve, find an order-`ell` point by random sampling and cofactor multiplication, then produce the codomain `j`.

Every arm checks that its kernel has order `ell`, that the codomain `j` agrees, and that an evaluated point lies on its codomain. The Edwards x-only evaluation is checked against its full-point formula. All timing rows are generated after these checks. The deterministic point source is repeated for each full-successor arm; the common point search is charged to each arm. The two Edwards rows distinguish a preconverted kernel from conversion charged to the step.

## First measured result

On an Apple M4 Pro, seven consecutive release-binary runs gave the following medians of each run's `successor_j` median (nanoseconds):

| Degree | Montgomery x-only | Edwards, preconverted | Edwards, conversion charged |
| ---: | ---: | ---: | ---: |
| 3 | 126.5 | 194.8 | 197.8 |
| 5 | 219.1 | 297.2 | 330.2 |
| 7 | 276.4 | 365.0 | 418.8 |

For this exact kernel-to-`j` task, the Edwards implementation did not beat the local Montgomery implementation at any tested degree. This is a component result about these formulas and this 31-bit field; it does not contradict Edwards CSIDH results measured with other formulas, degrees, and complete implementations.

The complete `full_successor_from_curve` timings were dominated by point search and varied substantially between runs. For degree 7, the seven run medians ranged from 5.14 to 9.20 microseconds for Montgomery and 5.34 to 10.20 microseconds for Edwards with conversion. The overlap does not support a full-successor speed claim in either direction. This also shows why replacing only the kernel-to-`j` formula cannot be assumed to move attack wall time: the search and cofactor work must be included. A real path-search comparison would additionally charge graph traversal and storage.

Verification before rebasing: `cargo run --release --example model_step_compare` completed with every row marked `verified=true`. On the rebased checkout based on `a80ec41ae5f`, `cargo test --release --lib` passed 53 tests, with one existing ignored test. The primary numbers above can be reproduced by running the example seven times and taking the median of each CSV row's `median_ns` value.

## Interpretation limits

These are single-step measurements over a 31-bit field on one host. They do not measure a complete CSIDH action, a meet-in-the-middle search, memory traffic, endomorphism work, or a deployed parameter set. `successor_j` omits kernel discovery; `full_successor_from_curve` includes a deliberately simple common discovery method. The Weierstrass arm is a correctness reference, while Montgomery x-only is the relevant local arithmetic baseline. The models use different output coordinates, so `step_and_eval_x` compares the work to produce an image coordinate, with correctness checked on each codomain.

The timing harness reports the median of 21 batches of 1,000 operations for the first two tasks and 21 batches of 100 for the full successor. These tiny operations are sensitive to host contention and code layout. A speed claim requires repeated, paired measurements on a quiet host and a comparison to an external optimized implementation of the same computation. A result here can establish whether a proposed model deserves that larger experiment; it cannot establish a cryptanalytic complexity improvement.
