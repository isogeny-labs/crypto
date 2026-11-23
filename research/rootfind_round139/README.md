# Root finding round 139: even quartics and impossible 3-kernels

Date: **2026-11-23**. Production draft [PR #5021](https://github.com/isogeny-labs/crypto/pull/5021), stacked on [PR #5014](https://github.com/isogeny-labs/crypto/pull/5014). Source: `src/cryptanalysis/ecbench/isogeny.rs`.

## Changes retained and their proofs

1. **Even quartic reduction:** When `f(x)=x⁴+b x²+c` is monic, solve the quadratic in `y=x²`, then solve `x²=y` for each rational `y`. Ascending sort and dedup preserve existing behavior for repeated roots. Covers binomial quartics and `ψ₃` with `b=0`. Does **not** affect generic quartics with nonzero odd coefficients.
2. **No rational 3-kernel filter:** For nonsingular `E: y²=x³+ax` over prime `p>3` and `a!=0`, `ψ₃/3=x⁴+2ax²-a²/3`. Its discriminant in `x²` is `16a²/3`. If `(3/p)=-1` (equivalently, `p mod 12 ∈ {5,7}`), the discriminant is nonsquare; consequently there is no rational 3-division abscissa and `find_isogeny(E,3)` must return None. This skips root extraction entirely.

No changes to generic polynomial arithmetic, the binary-field ECC2K solver, or exact `FieldTally` semantics other than eliminating operations no longer performed.

## Native production Rust before/after (first successful CI execution)

[Source-exact Rust benchmark and independent oracle](https://github.com/isogeny-labs/crypto/actions/runs/45218812982). Rust 1.100.0, `-C opt-level=3 -C target-cpu=native`, 32 predetermined cases per workload, 13 alternating A/B pairs with two discarded warmups. Times are median ns per complete query; speedups are medians of paired ratios. Baseline is the actual Rust file at round138.

| Operation | Prime | Before ns | After ns | Paired speedup |
|---|---:|---:|---:|---:|
| Even quartic, four roots | 65,521 | 6,196.8 | 980.3 | **6.306×** |
| Even quartic, four roots | 1,000,000,007 | 6,800.9 | 803.3 | **8.469×** |
| Even quartic, four roots | 2⁶¹−1 | 19,965.8 | 682.3 | **29.266×** |
| Random even quartic | 1,000,000,007 | 3,591.7 | 240.3 | **14.911×** |
| Random even quartic | 2⁶¹−1 | 12,062.8 | 460.8 | **26.180×** |
| Nonmonic even quartic | 2⁶¹−1 | 15,814.2 | 1,332.8 | **16.433×** |
| `ψ₃` roots, `b=0` | 1,000,000,009 | 6,071.4 | 994.3 | **6.141×** |
| `ψ₃` roots, `b=0` | 2⁶¹−1 | 9,966.3 | 515.6 | **19.200×** |
| Complete 3-isogeny, `b=0` | 1,000,000,007 | 3,816.8 | 856.5 | **4.453×** |
| Complete 3-isogeny, `b=0` | 2⁶¹−1 | 10,042.5 | 10.1 | **983.133×** |

The extreme last row is a **negative kernel-existence query** (no kernel can exist in this family over that field), not an isogeny construction win. Source-exact Rust independent small-field exhaustive root oracles and checksum equality across all 40 field/workload comparisons passed. Generic `ψ₃` and random quartic controls were at **1.006–1.044×** in the first CI execution; they are essentially unchanged. A rerun was requested to check repeatability.

## Local Linux perf, separate C++ reproduction

CPU-pinned AMD EPYC 9V74, GCC 14.2 and Clang 17 with `-O3`, two seeds per compiler. 11 alternating A/B trials (two warmups discarded). All four executions used Linux `perf_event_open` software task-clock counters successfully; hardware counters and the standalone `perf` binary were unavailable.

| Workload | Four-run local speedup range |
|---|---:|
| Even quartic with 4 roots, 65,521 | 4.96–5.28× |
| Even quartic with 4 roots, 1,000,000,007 | 6.14–6.98× |
| Even quartic with 4 roots, 2⁶¹−1 | 15.57–19.21× |
| Random even quartic, 1,000,000,007 | 7.11–10.38× |
| Random even quartic, 2⁶¹−1 | 18.47–28.50× |
| `ψ₃` roots, `b=0`, 1,000,000,009 | 4.94–6.69× |
| Complete 3-isogeny, `b=0`, 1,000,000,007 | 3.21–3.85× |
| Complete 3-isogeny, `b=0`, 2⁶¹−1 | 1,162–1,484× |

Local C++ differential and exhaustive brute-force oracles: **8,000 checks across 18 prime fields**. Clang ASan/UBSan and leak checking passed. Benchmark implementation and raw A/B CSVs available as a separate downloadable conversation artifact. Do not conflate these local C++ timings with native production Rust.

## Status and remaining gates

- Source-exact Rust compiler and first benchmark/oracle execution passed; second execution has been requested.
- Draft stacked PR; broad repository CI and interactions with other transfer accounting are not yet confirmed.
- No ECC2K binary-field index-calculus relations/sec improvement is claimed.
