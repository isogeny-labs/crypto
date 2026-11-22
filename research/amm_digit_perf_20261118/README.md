# AMM cube-root 3-adic digit optimization (2026-11-18)

Code: `src/cryptanalysis/prime_radical_root.rs`, PR #4706.

## Changes

1. Precompute `c_inv = c^(3^s - 1)` once per `Cube` context, for s>=2.
2. For s=1, cube membership implies h=1, so `a^e3` is already a root.
3. For s=2, the Sylow log is a multiple of three, making the correction one of `1,c_inv,c_inv^2`.
4. For s>=3, update `c^(-l)` and `c^(-l/3)` incrementally as 3-adic digits are recovered, rather than repeatedly exponentiating `c`.
5. Replace exponentiation to `3^k` with exactly k cubings, each using two group multiplies.

The operations are variable-time and are appropriate only for public polynomial-root inputs.

## Local reproduction

GCC -O2, AMD EPYC 9V74, CPU pinned with sched_setaffinity; 4,096 inputs for each workload; 13 alternating before/after pairs with median time per operation. Software task-clock via Linux `perf_event_open` is available, but hardware cycles fail with ENOENT; no perf CLI or Rust toolchain is installed in the local sandbox. These timings benchmark **standalone C++17 mirrors of the AMM arithmetic**, not the Rust source or end-to-end root solving. Context setup is excluded.

| Prime | s=v3(p-1) | before ns | after ns | wall speedup | software task-clock speedup |
|---|---:|---:|---:|---:|---:|
| 163 | 4 | 196.017 | 132.858 | **1.475x** | 1.474x |
| 487 | 5 | 336.590 | 189.345 | **1.778x** | 1.775x |
| 17,497 | 7 | 669.154 | 360.246 | **1.857x** | 1.857x |
| 1,000,000,009 | 2 | 340.082 | 289.834 | **1.173x** | 1.174x |
| 2^61-1 | 2 | 414.056 | 362.027 | **1.144x** | 1.144x |

Clang -O2 repeated the 13-pair tests and showed a positive median for each tested cube workload, with 1.029x-1.936x observed. Across 14 prime moduli 27,367 exact baseline/optimized comparisons passed, including non-cubes; all recovered roots were checked by cubing. UBSan passed with `-fsanitize=undefined -fno-sanitize-recover=all`.

## Actual Rust CI validation

`.github/workflows/radical-root-arithmetic.yml` builds an isolated test crate that imports the actual production Rust module. Newly added `amm_digit_loop_matches_previous_reference` compares Fp and Fp2 outcomes against the *immediately previous* algorithm, and `bench_amm_digit_loop` produces paired actual-Rust timings. Neither an end-to-end speedup nor a GPU speedup is claimed.

## Follow-up

- Inspect real Rust CI logs, especially for Fp2 and big prime values.
- Benchmark `Cube::new` to quantify the one-time precomputation cost, and compare full cubic/quartic throughput across factor-base inputs.
- Profile hardware cycles/instructions on a permitted PMU-enabled machine.


## Additional Cardano discriminant optimization

In `RadicalRootCtx::cubic_depressed`, `m=-P/3` is needed after discriminant calculation. Reuse it: `D=(Q/2)^2-m^3` rather than calculating `P^3/27` separately. This eliminates two `Fp::mul` calls on the common cubic path, with no change in the mathematics. `cubic_monic` also changes `2*inv27` from a generic multiplication into one modular doubling.

The arithmetic C++17 reproduction has **196,608** exact matching random pairs across six primes, with UBSan passing. For 13 AB/BA paired measurements, AMD EPYC 9V74, GCC -O2, the following are local **C++-mirror arithmetic** results (not compiled Rust):

| p | before ns | after ns | wall speedup | software task-clock speedup |
|---|---:|---:|---:|---:|
| 487 | 17.520 | 15.127 | **1.158x** | 1.158x |
| 17,497 | 17.352 | 14.747 | **1.177x** | 1.187x |
| 1,000,000,009 | 17.695 | 15.214 | **1.163x** | 1.163x |
| 2^61-1 | 13.637 | 9.238 | **1.476x** | 1.476x |

Clang -O2 showed **1.103x–1.181x** across all six primes (the compiler-dependent P-256/Mersenne reduction results must not be generalized). The Rust CI `cardano_discriminant_reuse_matches_reference` compares 100,000 pairs over ten primes, and the ignored `bench_cardano_discriminant` performs a new actual-Rust paired comparison. No full cubic-root throughput speedup has been established.
