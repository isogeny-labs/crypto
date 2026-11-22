# Edwards w-coordinate CSIDH-512 comparison

## Question and source

Can Edwards projective `w = d x² y²` arithmetic reduce the cost of a CSIDH-512 odd-degree successor in this repository? This checks the formulas in [Moriya, Onuki and Takagi (2023)](https://pure-oai.bham.ac.uk/ws/portalfiles/portal/208667805/MoriyaT2023Finite.pdf), especially equations (12), (13), (15) and Corollary 2, building on [Kim, Yoon, Park and Hong (2019)](https://iacr.org/archive/asiacrypt2019/119210173/119210173.pdf). The authors report favorable results for their optimized full Edwards implementations; the present code is an explicit bridge from this repository's Montgomery kernel sampler and is a different implementation.

## Protocol

Run `cargo run --release --example model_step_w_csidh512` from `isogeny_algos`. The code uses the actual CSIDH-512 prime and fixed seeds. For each degree 3, 5, 7, 31 and 587, it samples a rational positive-direction kernel on the base curve. It also repeats exact successor checks on the nonzero curve produced by a degree-3 step. A failure to match the existing Montgomery coefficient aborts before timing. For the timed known-kernel comparison, both arms receive the same Montgomery projective kernel point. The Montgomery arm generates half-kernel multiples and computes its codomain; the Edwards arm converts the point to `w`, generates the multiples with `w` doubling and differential addition, then computes the codomain and converts it to the canonical Montgomery coefficient. The separate hybrid arm receives precomputed Montgomery multiples and converts each one to `w`; its timing has a different input boundary.

The implementation selects the ordinary Edwards normalization using the square character of `A-2`, as specified by Corollary 2. It uses the projective `w` formulas directly over `Fp`, avoiding an extension-field point representation. It is research code, with variable-time square-root checks and no constant-time claim.

## Observations

Three independent release invocations on the local host, each taking the median of 21 calls, produced these known-kernel-point-to-codomain times. Values are nanoseconds, shown as the observed range across invocations. These are local wall times, with occasional outliers; no pinned clock or isolation was used.

| Degree | Montgomery | Edwards `w` native |
| ---: | ---: | ---: |
| 3 | 4,042 | 59,542–62,708 |
| 5 | 4,459–4,500 | 60,166–68,834 |
| 7 | 4,916–4,959 | 60,500–129,417 |
| 31 | 9,500–9,583 | 66,333–66,792 |
| 587 | 107,833–108,917 | 192,834–615,834 |

The exact successor matched for all five degrees on both checked curves. This implementation does not improve this repository's known-kernel successor cost. The result does not evaluate the paper's native sampling, Elligator analogue, optimized coefficient state, or square-root Vélu method. Those are the next relevant candidates for a full CSIDH implementation comparison. No security claim follows from an operation cost comparison.
