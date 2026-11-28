# Root-finding optimization Round 133 (2026-11-23)

Stacked on [Round 132 PR #4999](https://github.com/isogeny-labs/crypto/pull/4999). Production file: `src/cryptanalysis/ecbench/isogeny.rs`. [Round 133 PR #5002](https://github.com/isogeny-labs/crypto/pull/5002).

## Retained optimizations

1. **Tonelli-Shanks for p≡1 mod4:** For every monic quadratic input or monic degree-two factor, compute the discriminant, check its Legendre symbol, extract the square root with Tonelli-Shanks, and return the distinct roots. This bypasses the initial polynomial Frobenius power, polynomial GCD and, for split factors, repeated equal-degree splitting. The existing p≡3 mod4 fast path is unchanged for nonzero discriminants.
2. **Zero discriminant:** Return `-b/2` directly as the unique root when `b²-4c=0` for odd primes. No modular exponentiation is needed, including in the existing p≡3 mod4 path.
3. **Rejected / not retained:** The standalone zero-discriminant fast-path variant cannot improve general nonrepeated quadratics; full C++ performance was around 0.95–1.03×, with some near-parity regressions. More generic higher-degree arithmetic changes are not retained in this round.

## Local Linux profiling (C++ algorithmic reproduction)

AMD EPYC 9V74. GCC 14.2 and Clang 17, `-O3 -march=native`, single-process CPU-pinned trials. Four benchmark executions (two RNG seeds per compiler), 11 alternating A/B pairs per variant, first two warmup pairs excluded, median **paired** ratio over the remaining 9 pairs. Linux `perf_event_open` **software task-clock** is available; hardware PMU counters and `perf` CLI were not available. Values below are **not production Rust timings**.

Baseline = Round 132 algorithmic reproduction; candidate = Tonelli-Shanks + zero-discriminant fast path.

| Complete operation | Prime field | Before ns (example run) | After ns | Paired speedup range, 4 runs |
| --- | --- | ---: | ---: | ---: |
| Quadratic roots, distinct | 65,521 | 4,893 | 1,002 | **3.80–4.87×** |
| Quadratic roots, distinct | 1,000,000,009 | 6,901 | 1,826 | **3.01–3.74×** |
| Quadratic roots, repeated | 65,521 | 1,421 | 32.9 | **39.25–47.27×** |
| Quadratic roots, repeated | 1,000,000,009 | 2,398 | 28.5 | **57.99–82.59×** |
| Cubic roots (constructed) | 65,521 | 7,286 | 5,605 | **1.25–1.29×** |
| Cubic roots (constructed) | 1,000,000,009 | 12,417 | 8,825 | **1.35–1.52×** |
| Quartic roots (constructed) | 1,000,000,009 | 18,687 | 14,485 | **1.23–1.29×** |
| 3-isogeny ψ₃ root polynomial (random a,b) | 1,000,000,009 | 6,362 | 5,737 | **1.00–1.12×** |
| Repeated quadratic | 1,000,000,007 | 163 | 28.2 | **5.26–5.77×** |
| Repeated quadratic | 2⁶¹−1 | 173 | 29.3 | **5.89–6.16×** |

p≡3 mod4 distinct quadratics and random quartic, random cubic and ψ₃ controls were mostly near parity; some regressions below 1.00×. Improvements on p≡1 mod4 cubic/quartic are consistent with eliminating expensive quadratic-factor splitting. Nonresidue quadratics were included in independent correctness tests but not specifically timed as a separate workload.

## Correctness and native Rust gate

- C++ exhaustive root oracles: all monic quadratics over p=3,5,7,11,13,19,23,31,101 (12,425 polynomial inputs); both reference and candidate agree with brute-force roots.
- Additional C++ differential randomized field arithmetic, polynomial multiplication/powers, constructed, repeated-root and random cubic/quartic tests across p=101, 65,521, 1,000,000,009, 1,000,000,007, 4,294,967,291, 2⁶¹−1: PASS.
- GCC AddressSanitizer and UndefinedBehaviorSanitizer: PASS.
- Rust source-exact A/B workflow (`.github/workflows/rootfind_round133.yml`) compiles the actual Round 132 Rust baseline versus the Round 133 production Rust candidate, independently checks roots, and executes 13 alternating trials per operation including p=1,000,000,009. **The local runtime lacks rustc/cargo. Two GitHub Actions source-exact native Rust A/B executions passed, including independent correctness oracles.**

## Reproduce local C++ measurements

The companion Round133 local artifact contains `round133_bench.cpp`, `round133_seed2.cpp`, `summary133.csv`, 4 raw trial CSVs, and sanitizer logs. Compile with `g++ -O3 -march=native -std=c++20` or equivalent Clang settings, run `--verify`, then collect timed CSV. Candidate bitmask 88548 is Tonelli-Shanks alone, 154084 is repeated-root handling alone, 219620 is combined. Use 23012 as the Round132 baseline. The output mode is `taskclock` when `perf_event_open` is available; this is **software task-clock, not CPU cycles**.

## Production Rust A/B (two successful independent executions)

[Run 1](https://github.com/isogeny-labs/crypto/actions/runs/45156668313) and [run 2](https://github.com/isogeny-labs/crypto/actions/runs/45156813457). Each compiles the actual Round132 and Round133 production Rust source files with `rustc -C opt-level=3 -C target-cpu=native`; independent oracles pass on both, and 13 alternating paired trials produce the following **complete roots() query** results. Times are median nanoseconds per query. Paired speedup uses medians of within-trial ratios.

| Complete operation | Prime | Native run 1 before → after | Run 1 speedup | Independent run 2 speedup |
| --- | --- | ---: | ---: | ---: |
| Distinct quadratic roots | 65,521 | 3,418 → 1,008 ns | **3.384×** | **4.203×** |
| Distinct quadratic roots | 1,000,000,009 | 6,259 → 1,860 ns | **3.349×** | **3.912×** |
| Repeated quadratic root | 65,521 | 1,106 → 29.8 ns | **37.552×** | **32.749×** |
| Repeated quadratic root | 1,000,000,009 | 1,737 → 31.5 ns | **54.912×** | **72.636×** |
| Cubic known roots | 65,521 | 7,356 → 5,138 ns | **1.473×** | **1.314×** |
| Cubic known roots | 1,000,000,009 | 14,078 → 10,811 ns | **1.321×** | **1.303×** |
| Quartic known roots | 1,000,000,009 | 23,132 → 18,883 ns | **1.227×** | **1.237×** |
| Repeated quadratic root | 1,000,000,007 | 145 → 30.1 ns | **4.803×** | **4.584×** |
| Repeated quadratic root | 2⁶¹−1 | 174 → 29.8 ns | **5.857×** | **5.527×** |

At p=1,000,000,009, complete `find_isogeny(3)` showed **1.012×** in the first run but **0.991×** in the second; at p=65,521 it showed **1.099–1.126×**. Thus the overall isogeny speedup is workload- and field-dependent. Some previously fast nonrepeated p≡3 mod4 quadratics regressed approximately 1–6%, and random/quartic workloads did not universally improve. No overall ECC index-calculus gain is claimed.

## Acceptance

Do not merge until native Rust source-exact benchmark correctness and performance have been inspected and full-repo CI passes. These are prime-field isogeny root optimizations, **not an ECC2K-130 index-calculus throughput improvement**. Avoid aggregating unrelated per-field microbenchmark speedups into end-to-end claims.
