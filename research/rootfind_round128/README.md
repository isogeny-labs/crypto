# Root-finding Round 128: polynomial remainder and prime-field arithmetic

Optimized production file: `src/cryptanalysis/ecbench/isogeny.rs`. This is the isogeny-transfer polynomial root finder, not the separate radical-root solver.

## Changes

- Canonical 16-bit and 32-bit modular products use narrow integer divisions, while the 61-bit Mersenne prime uses end-around-carry reduction. Canonical additions/subtractions avoid wide modulo, with the original arithmetic as fallback.
- Polynomial remainder avoids an entire Fermat inversion per *monic* divisor, skips canceled leading terms, and skips zero divisor coefficients.
- Scalar and polynomial power routines omit their final unused square.

## Local Linux profiling (C++ reproductions, not native Rust)

AMD EPYC 9V74; GCC 14.2 and Clang 17; CPU 0 pinned; Linux perf_event_open software task-clock; 11 alternating A/B trials; each compiler independently repeated. Median of paired per-trial ratios.

| Prime | GCC first | GCC repeat | Clang first | Clang repeat |
|:--|--:|--:|--:|--:|
| 65,521 | 1.862x | 1.941x | 1.910x | 1.926x |
| 1,000,000,007 | 1.767x | 1.822x | 1.866x | 1.880x |
| 4,294,967,291 | 1.980x | 1.971x | 2.021x | 1.966x |

These are **complete constructed-cubic root queries**, not only isolated field multiplication. Isolated multiplication was mixed (0.93–1.09x depending on compiler and prime). Faster monic remainder is the dominant individual optimization: GCC 1e9+7 isolated reduction 6.16x, complete constructed-cubic root 1.44x in ablation.

Correctness in local C++ reproduction: 1,030,409 comparisons across five prime fields, including known-root cubic extraction; GCC ASan/UBSan passed. Hardware counters unavailable, but software task-clock succeeded.

## Source-exact Rust validation

`bench_helper.rs` appends A/B benchmark entrypoints to separate copies of baseline and candidate production Rust files. `source_exact_main.rs` compiles both modules into the same standalone executable (with only the `PrimeCurve`/`PrimePoint` types stubbed). The dedicated GitHub Actions workflow compares results and native execution times while avoiding the unrelated malformed `prime_root_trace.rs` baseline (#4959).

Do not merge until source-exact Rust A/B and repository tests pass. No ECC2K relation-throughput claim.

Local reproducibility artifacts are also provided in the Round 128 archive in the automation history.
