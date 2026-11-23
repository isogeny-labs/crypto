# Root finding optimization Round 132 (2026-11-23)

**Stacked on:** Round 131 PR #4995, commit `dfca7d664ee445e8d9fabbd9b9490209591d968a`. **Production module:** `src/cryptanalysis/ecbench/isogeny.rs`.

## Optimizations retained

1. **Linear polynomials:** Directly return `-c` after monic normalization, rather than computing `x^p mod f`, a polynomial GCD, and a generic splitter.
2. **Quadratics over p ≡ 3 (mod 4):** Compute the discriminant `d=b²-4c`, a checked square root `s=d^((p+1)/4)`, and the two modular halves `(-b±s)/2`. Return no roots for a nonresidue and deduplicate a repeated root. This executes *before* the Frobenius/GCD stage, unlike the previous Round131 `split()` shortcut.
3. **Stack-backed polynomial multiplication/remainder (experiment only, REJECTED):** A <=9 coefficient fixed array removed an intermediate allocation for degree <=4 reductions. Results were compiler/field-sensitive, and a 32-bit field showed regressions. **Not included** in production code.

Both retained fast paths require the same conditions as the old solver, without changing public APIs. In the supported prime fields p <= 2^63-1, conditional modular halving cannot overflow u64.

## Local before/after Linux perf measurements

AMD EPYC 9V74, CPU pinned to CPU 0, native GCC and Clang optimized C++ reproductions of the production algorithm. Four executions (two RNG seeds each compiler); 11 alternating baseline/candidate A/B trials per workload. **Collected Linux `perf_event_open(PERF_TYPE_SOFTWARE, PERF_COUNT_SW_TASK_CLOCK)` software task-clock counters**, reported as nanoseconds per complete operation and *median paired speedup*. The standalone `perf` executable and hardware counters were unavailable.

**These measurements are C++ reproductions, not native Rust production speedups.** Baseline option 6628 models Round131; candidate option 23012 adds both early low-degree fast paths.

| Complete roots() operation | Prime | Before → after (example, ns) | Range of four independent median paired ratios |
|---|---|---|---|
| Linear | 65,521 | 847 → 29.8 | **28.5–35.1×** |
| Linear | 1,000,000,007 | 1,374 → 28.6 | **45.9–52.5×** |
| Linear | 2^61 − 1 | 3,381 → 28.6 | **115.7–140.7×** |
| Split quadratic | 1,000,000,007 | 1,963 → 183 | **9.5–11.3×** |
| Split quadratic | 2^32 − 5 | 2,288 → 218 | **10.3–12.1×** |
| Split quadratic | 2^61 − 1 | 5,481 → 185 | **28.9–32.7×** |
| Cubic, constructed roots | 1,000,000,007 | 6,782 → 7,592 | **0.97–1.04×** |
| ψ₃ quartic, representative random inputs | 1,000,000,007 | 4,617 → 4,644 | **0.93–1.00×** |

The large low-degree ratios are due to bypassing the prior exponentiation/GCD; they are **not** end-to-end isogeny speedups. Cubic and ψ₃ controls did not reliably improve. The optional fixed-array experiment (39396) slowed standalone `mulmod()` on GCC 32-bit primes (approximately 0.86×), and was rejected.

## Native Rust source-exact A/B (two successful executions)

The workflow compiles the **actual Rust baseline and candidate production source files** with Rust 1.100.0 `-C opt-level=3 -C target-cpu=native`, runs the same independent correctness oracles on both, and executes 13 alternating A/B trials per workload. The first run and an independent rerun both passed: [GitHub Actions run 45148126063](https://github.com/isogeny-labs/crypto/actions/runs/45148126063).

| Complete operation | Prime | First run (before → after ns, paired speedup) | Independent rerun speedup |
|---|---|---|---|
| Monic linear | 65,521 | 1,085 → 46.9, **23.34×** | **24.63×** |
| Monic linear | 1,000,000,007 | 1,623 → 46.3, **34.52×** | **37.09×** |
| Monic linear | 2^61−1 | 4,097 → 46.4, **86.77×** | **118.09×** |
| Monic quadratic | 1,000,000,007 | 3,536 → 226, **15.81×** | **18.02×** |
| Monic quadratic | 2^32−5 | 3,902 → 260, **15.00×** | **15.01×** |
| Monic quadratic | 2^61−1 | 7,859 → 219, **36.20×** | **34.95×** |
| Quadratic with repeated root | 1,000,000,007 | 2,960 → 172, **16.52×** | **14.83×** |
| Scaled quadratic | 2^61−1 | 6,813 → 484, **14.09×** | **14.41×** |

The p=65,521 quadratic control (p≡1 mod 4) stayed near parity. Most cubic, quartic, ψ₃ and full isogeny workloads remained approximately at parity, sometimes **up to 2–3% slower**. The native figures demonstrate fast low-degree root extraction, **not** faster complete 3-isogeny discovery or ECC2K index-calculus throughput.

## Correctness validation

- Independent exhaustive quadratic oracles at p=3,5,7,11,13,19,23,31,101 (12,425 polynomials) passed for both baseline and optimized local reproductions.
- Differential randomized modular multiplication and exponentiation, known-factor/repeated-root cubic and quartic roots, and random quartic roots passed for p=101,65,521,1e9+7,2^32−5,2^61−1.
- GCC AddressSanitizer + UndefinedBehaviorSanitizer passed.
- Rust source includes additional exhaustive tests of nonmonic/padded linear, scaled quadratic, repeated and irreducible roots. The source-exact native Rust A/B CI workflow additionally runs independent oracles and 13 alternating trials. Source-exact native Rust build, correctness oracles and repeated A/B numerical timings passed. Broad repository CI and true end-to-end workloads remain required before merge.

## Reproduce

Use the local `rootfind-round132-20261123.zip` run artifact. Its `round132_bench.cpp` uses option 6628 as the baseline; 23012 = early low-degree fast paths; 39396 = stack experiment; 55780 = both. For example:

```sh
g++ -O3 -march=native -std=c++20 round132_bench.cpp -o benchmark_gcc
./benchmark_gcc --verify
./benchmark_gcc > taskclock_trials.csv
python3 summarize132.py
```

**Scope/merge gate:** Keep this PR draft and stacked on #4995. Native source-exact benchmarks, broad repository CI, and true ECC2K index-calculus throughput remain separate acceptance gates. Optimized code is in the prime-field isogeny root-finding helper; no binary-field cryptanalytic improvement is claimed.
