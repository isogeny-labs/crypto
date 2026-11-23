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

The four-step positive action (`3,5,7,31`) sampled kernels separately at each successor. In five further local invocations with the fixed base state, medians ranged from 1.88–2.21 ms for Montgomery and 1.88–2.43 ms for projective `w`, with changes in which arm was faster. [pcc:unverified] Earlier runs with an unnecessary initial square-character check were noisier still. This host-level experiment does not establish an action-level gain. The component gain is measured for small-degree known-kernel successors on an already prepared state. At this earlier stage, the action used the same Montgomery sampler in both arms; the later native action below adds Edwards sampling and kernel extraction. The paper's Elligator analogue and square-root Vélu method remain untested here.

## Native w sampling and complete actions (2026-11-22)

The example now includes the positive and negative eigenspaces from Moriya–Onuki–Takagi Algorithm 3. It samples a nonexceptional square `w`, doubles it with projective `D:C` arithmetic, tests the square character of the resulting `W/Z`, and selects `w(2P)` for the negative direction or its reciprocal for the positive direction. A native `w`-ladder clears `(p+1)/(4 ell)`; failed kernels are resampled. The resulting `w` kernel feeds the projective codomain formula directly. This action does not invoke the Montgomery point sampler, ladder, or kernel conversion. The only Montgomery representation retained is the canonical output coefficient used for equality and as the comparison boundary.

The correctness gate compares each positive and negative step at degrees 3, 5, 7, 31 and 587 with `Csidh::step`. Complete actions are compared against `Csidh::action_fast` on three exponent vectors: mixed signs at indices 0, 1, 2 and 9; mixed signs at indices 0 and 73 (degrees 3 and 587); and alternating signs at **all 74** CSIDH-512 primes. Every complete action is checked with three native random seeds against the same canonical output from the tuned Montgomery action. The benchmark uses fixed seeds and includes native state initialization, sampling, cofactor clearing, kernel arithmetic, codomain updates and final canonical coefficients. It times `action_fast` from the same base curve and exponent vector.

In 11 independent release-process invocations on the shared host, each reporting the median of five within-process timings, the complete-action medians were:

| Exponent vector | Tuned Montgomery `action_fast` | Native Edwards `w` | Paired Montgomery / `w` median | `w` wins |
| --- | ---: | ---: | ---: | ---: |
| Four mixed signs | 1.54 ms | 5.22 ms | 0.29 | 0/11 |
| Degrees 3 and 587, mixed signs | 2.11 ms | 2.80 ms | 0.67 | 1/11 |
| All 74, alternating signs | 15.82 ms | 95.40 ms | 0.17 | 0/11 |

For the all-74 vector, paired Montgomery / `w` ratios ranged 0.146–0.208 across these invocations. The native path is slower under this protocol. The result is a comparison of **this sequential native action** with the repository's **batched, tree-strategy Montgomery action**; it does not establish that all Edwards `w` strategies lose. Both arms use variable-time research code and local wall time under substantial host load.

The example also reports complete isolated positive and negative steps for degrees 3, 5, 7, 31 and 587. In the same 11 invocations, per-degree paired ratios had broad overlap with one, driven by variable sampling attempts and host load. There was no stable full-step crossover to assign to a degree. The exact per-degree medians and ratios can be reproduced from the example's CSV output; known-kernel arithmetic above still shows the prepared-state small-degree component gain. The next performance test is an Edwards action that batches primes under one sampled point and pushes `w` points through each isogeny, matching `action_fast`'s amortization. The high-degree square-root Vélu arm remains separate.

## Batched native action and tree (2026-11-22 follow-up)

The sequential numbers above were collected before deferring the final Montgomery conversion. The current native action carries `D:C` throughout and converts to Montgomery form once at the output. Two further native arms reuse each sampled `w` point: a direct batch loop and a tree that isolates kernels by product ladders, pushes stacked points with the published projective `w` isogeny evaluation, and plans splits with the same cost recurrence used by `action_fast`. A point missing an `ell` component leaves that exponent pending for the next sample. All arms start cold from the fixed public CSIDH-512 base curve; the initial Edwards state is constructed algebraically, and final canonical conversion is timed. The tree is a comparable batching strategy, while its arithmetic, sampling and implementation remain distinct from Montgomery's.

The exact canonical output matched `action_fast` for the three exponent vectors above with three native seeds each. In the isolated-step sweep, each of 33 seeds per sign and degree matched the corresponding Montgomery step. Timing order alternated between arms by seed. This sweep supersedes any apparent fixed-seed degree crossover: Montgomery's median complete-step time was lower at every checked degree in both directions.

Fifteen independent release-process invocations of the **current** code, each taking the median of five complete-action calls, gave these medians. This first sample's process-level CSV was not archived; treat it as a screening result, with the archived replication below as the inspectable measurement:

| Exponent vector | Montgomery `action_fast` | Native sequential | Native batch loop | Native planned tree |
| --- | ---: | ---: | ---: | ---: |
| Four mixed signs | 0.644 ms | 1.641 ms | 0.803 ms | 0.781 ms |
| Degrees 3 and 587, mixed signs | 0.841 ms | 0.948 ms | 1.117 ms | 0.938 ms |
| All 74, alternating signs | 6.790 ms | 40.390 ms | 19.134 ms | 9.020 ms |

For the all-74 action, the paired Montgomery / native-tree ratio had median 0.724, and the tree was faster in 3 of 15 invocations; its observed 7.843–45.874 ms range includes host outliers. [pcc:unverified] The native tree narrows the earlier gap but does not meet the preregistered 5% complete-action speedup criterion. Its square-root Vélu analogue is not implemented, so the result is specific to these formulas and this implementation. The per-degree median step times (Montgomery / native `w`, milliseconds) across the 33-seed sweep were: positive 3 (0.365 / 0.494), 5 (0.383 / 0.612), 7 (0.627 / 0.796), 31 (0.317 / 0.429), 587 (0.424 / 0.565); negative 3 (0.315 / 0.468), 5 (0.328 / 0.443), 7 (0.310 / 0.443), 31 (0.328 / 0.420), 587 (0.395 / 0.524). There is no observed complete-step degree crossover in this sweep. The earlier prepared-kernel component gain applies to a different timing boundary.

### Raw-output replication under variable host load

The 15-process replication's reported rows are preserved in [`model_step_w_csidh512_runs_20261122.csv`](model_step_w_csidh512_runs_20261122.csv): 32 rows per process, covering `complete_action` and `seed_sweep_step_*`. The executable emits 84 rows per process; the unrelated earlier component rows were excluded from this dataset. The executable was built with `cargo run --release --example model_step_w_csidh512` at source head `a7fb0764dc394c6ff545608fafd2a23fff9361df` plus this PR's uncommitted changes, then invoked 15 more times as `isogeny_algos/target/release/examples/model_step_w_csidh512`. The host was an Apple M4 Pro, Darwin 27.2.0 arm64, with rustc 1.93.1. Each process performs exact output assertions before the timing rows; `exact_output_checked=true` records that in-process check, not an independent PCC verdict. Each complete-action row is a median of five calls. This replication was collected later on the same shared host with much greater scheduling variance. It yielded these across-process medians (milliseconds):

| Exponent vector | Montgomery `action_fast` | Native sequential | Native batch loop | Native planned tree | Tree wins / 15 | Median paired Montgomery / tree |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| Four mixed signs | 1.753 | 3.765 | 1.142 | 1.627 | 6 | 0.863 |
| Degrees 3 and 587, mixed signs | 0.867 | 1.478 | 2.244 | 1.441 | 6 | 0.855 |
| All 74, alternating signs | 36.519 | 193.645 | 83.175 | 39.577 | 7 | 0.937 |

The all-74 Montgomery and native-tree process medians ranged 5.950–95.654 ms and 9.202–98.306 ms respectively. [pcc:unverified] A paired process-ratio median bootstrap (20,000 resamples of the 15 process pairs, Python `random.Random(20261122)`, percentile interval) gives a 95% interval of 0.461–1.940 for Montgomery time divided by native-tree time. That interval spans equality and a 5% native gain, so this replication alone cannot establish a performance ordering. Both 15-process samples fail to establish the planned 5% action-level gain with uncertainty excluding zero. In the raw replication, the 33-seed step sweep in each process again had a lower across-process median for Montgomery at every checked degree and sign. Individual step medians can be recomputed by filtering the CSV for `seed_sweep_step_positive` or `seed_sweep_step_negative` and taking the median across processes. This result is a measured null for this implementation and host, not a general lower bound on Edwards arithmetic.
