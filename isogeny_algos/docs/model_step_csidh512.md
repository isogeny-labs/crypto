# CSIDH-512 curve-model transfer check

## Question

Can a complete positive-direction isogeny step on the CSIDH-512 field use twisted Edwards formulas and return the *same Montgomery coefficient* as the existing x-only formulas? If so, does that substitution reduce the cost of a sparse group action or of successor generation inside a bounded path search?

Run `cargo run --release --example model_step_csidh512` from `isogeny_algos/`. The example uses the repository's CSIDH-512 prime and field implementation. Both arms start from the same Montgomery coefficient and draw the same candidate x-coordinates from a fixed seed. Point selection, cofactor multiplication, kernel construction, and the image-curve coefficient are charged to a full step. The Edwards arm also charges conversion of the kernel point; its codomain is converted back to a Montgomery coefficient for the next edge and for the path-search hash key.

The example checks five degrees (3, 5, 7, 31, and 587) on the base curve, then takes a complete three-step positive action at degrees 3, 5, and 7. A bounded two-prime frontier search builds four nodes from each of two endpoints, hashes the Montgomery coefficients, and checks for a collision. This exercises the same successor interface as a graph search, while keeping the number of nodes fixed and small. The target is constructed independently before timing.

The comparison has important scope limits. It uses the affine Edwards formulas in `kernel/models.rs`, which invert during kernel additions; it does not implement the faster Edwards w-coordinate formulas described in the literature. It handles positive directions, so it does not replace the full signed CSIDH group action. The bounded frontier is a performance diagnostic at the full field size, not an estimate of a practical attack's total cost or success probability. Times include the same field arithmetic and candidate-point generator in both arms; they exclude process startup and target construction.

## Results

On an Apple M4 Pro, the release example passed every coefficient and frontier equality check. Each row below is the median of 21 consecutive process runs. The ratio is the median of 21 paired Edwards/Montgomery run ratios, so it may differ slightly from the quotient of the displayed medians. Times are microseconds.

| Degree | Known kernel: Montgomery | Known kernel: Edwards | Paired ratio | Full step: Montgomery | Full step: Edwards | Paired ratio |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 3 | 4.13 | 35.00 | 8.49 | 300.13 | 319.04 | 1.08 |
| 5 | 4.58 | 38.25 | 8.34 | 289.71 | 314.29 | 1.10 |
| 7 | 4.92 | 41.71 | 8.47 | 339.75 | 373.04 | 1.09 |
| 31 | 9.63 | 95.38 | 9.92 | 310.25 | 404.54 | 1.31 |
| 587 | 109.33 | 2127.38 | 19.31 | 507.88 | 2509.29 | 4.93 |

The Edwards known-kernel successor was slower in all 21 runs at each degree. At degree 587, the complete positive step was also slower in all 21 runs. For small degrees, candidate-point selection and cofactor multiplication dominate, so the much larger kernel-construction ratio shrinks in a full step. The full-step paired ratios varied considerably; for degree 3 they ranged from 0.48 to 2.37. Those rows are directionally informative, not precise speed estimates.

The complete three-step positive action at degrees 3, 5, and 7 had median times of 927 microseconds (Montgomery) and 1034 microseconds (Edwards); the median paired Edwards/Montgomery ratio was 1.12, with Edwards slower in 16 of 21 runs. The bounded two-prime path search had medians of 2856 and 3066 microseconds respectively, a median paired ratio of 1.08 with Edwards slower in 17 of 21 runs. The path-search ratio ranged from 0.81 to 1.61, so a small wall-time difference is unresolved under this host noise. There is no observed speedup to transfer to this attack computation from the implemented Edwards formulas.

Verification: `cargo run --release --example model_step_csidh512` completed with every row marked `verified=true`. The example asserts equality with `Csidh::step`, equality of Montgomery and Edwards coefficients at five degrees, equality of the three-step action, equality of both path-search frontiers, and an actual frontier collision. The branch's `cargo test --release --lib` passed 53 tests, with one existing ignored test, before this example was added.
