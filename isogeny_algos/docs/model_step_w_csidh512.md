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

The exact successor matched for all five degrees on both checked curves. These initial timings were for an affine coefficient bridge. They are retained as the baseline for the projective-state optimization below.

## Projective-state optimization

The first bridge divided `A-2` by `A+2`, divided the isogeny product, and computed square characters while recovering each codomain. The revised arm retains the ordinary Edwards coefficient as `D:C` and computes

`D' = D^ell * (product (Wi+Zi))^8`, `C' = C^ell * (product 2Zi)^8`.

It performs only the final division needed to expose the canonical Montgomery coefficient. It retains the Edwards normalization branch across odd-degree steps. Preparing that state from an arbitrary Montgomery coefficient requires a square-character check. The `known_kernel_point_to_codomain` projective benchmark starts after preparation; a separate `edwards_w_projective_cold` arm charges that check. The four-step action starts from the fixed public CSIDH base coefficient, whose Edwards state is known algebraically, so it constructs that state without a square-character check. The code checks exact canonical coefficient equality against Montgomery on the base and a nonzero successor curve for all five degrees, and checks a four-step action under 32 deterministic seeds.

Seven further release invocations, each using the median of 21 calls, gave the following projective-state **known-kernel-point-to-codomain** medians in nanoseconds. Ranges include local timing outliers. The same kernel point was supplied to both arms; Montgomery generates its half-kernel multiples within the timed region.

| Degree | Montgomery range | Projective Edwards `w` range | Typical relation |
| ---: | ---: | ---: | --- |
| 3 | 4,042–4,125 | 2,958–3,000 | `w` about 27% faster |
| 5 | 4,500–4,542 | 3,542–3,583 | `w` about 21% faster |
| 7 | 4,875–5,375 | 4,083–4,166 | `w` about 16% faster |
| 31 | 9,458–9,708 | 10,041–21,208 | Montgomery usually faster |
| 587 | 107,708–155,250 | 139,458–209,833 | Montgomery usually faster |

The cold known-kernel arm, which prepares the Edwards state from the Montgomery coefficient for each call, took about 29–36 µs at degrees 3–31 in its ordinary runs, versus about 4–10 µs for Montgomery; several runs had much higher outliers. Thus the per-step component gain depends on retaining the model state. The fixed CSIDH base makes that retention natural for a scheme action, while an attack beginning from an arbitrary input coefficient must charge its preparation.

The four-step positive action (`3,5,7,31`) sampled kernels separately at each successor. In five further local invocations with the fixed base state, medians ranged from 1.88–2.21 ms for Montgomery and 1.88–2.43 ms for projective `w`, with changes in which arm was faster. Earlier runs with an unnecessary initial square-character check were noisier still. This host-level experiment does not establish an action-level gain. The component gain is measured for small-degree known-kernel successors on an already prepared state. A full native Edwards action would also need point sampling and kernel extraction in Edwards coordinates; the present action uses the same Montgomery sampler in both arms. The paper's Elligator analogue and square-root Vélu method remain untested here.
