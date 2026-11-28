# Root finding, round 138: sparse factors and known torsion kernels

Date: **2026-11-23**. Draft PR [#5014](https://github.com/isogeny-labs/crypto/pull/5014), stacked on [#5011](https://github.com/isogeny-labs/crypto/pull/5011). Source: `src/cryptanalysis/ecbench/isogeny.rs`.

## Changes retained

1. When a degree ≥3 polynomial has zero constant term, strip the entire `x^k` factor, solve the smaller quotient, restore the **distinct** root zero, then sort/deduplicate. This removes expensive Frobenius/GCD work, especially for repeated-zero factors.
2. For monic binomial cubics `x^3+c` in prime fields with `p ≡ 2 (mod 3)`, compute `(-c)^((2p-1)/3)` using scalar field exponentiation; the cube map is bijective because `gcd(3,p-1)=1`.
3. Short-circuit 2-isogeny kernel discovery on nonsingular `b=0` curves (`ψ₂=x(x²+a)`), and 3-isogeny discovery on nonsingular `a=0` curves (`ψ₃=3x(x³+4b)`): the **smallest** root is zero. Guard `p>3` and nonsingularity. This avoids root-solving entirely, not just shortens it.

The arbitrary-curve path still uses the original algorithm. These are **special-family** improvements, not a generic polynomial-factorization breakthrough.

## Source-exact native Rust A/B (two passing runs)

[Run 1](https://github.com/isogeny-labs/crypto/actions/runs/45206689602) and its independent successful rerun (same run, second attempt). Rust 1.100.0, optimized `-C opt-level=3 -C target-cpu=native`, real production Rust source from round 137 versus round 138. Thirteen alternating A/B trial pairs, first two discarded; 32 deterministic inputs per workload; medians of paired speedups.

| p | Complete operation | Run 1 before → after (ns) | Speedup, run 1 / independent rerun |
|---|---|---:|---:|
| 65,521 | 2-isogeny, `b=0` | 2,882.7 → 29.8 | **96.6× / 78.9×** |
| 65,521 | 3-isogeny, `a=0` | 2,772.8 → 34.1 | **81.3× / 66.7×** |
| 1,000,000,007 | 2-isogeny, `b=0` | 4,122.9 → 41.9 | **98.5× / 99.6×** |
| 1,000,000,007 | 3-isogeny, `a=0` | 2,827.3 → 43.6 | **64.9× / 54.7×** |
| 1,000,000,007 | `ψ₃` roots, `a=0` | 2,697.9 → 442.0 | **6.10× / 5.29×** |
| 1,000,000,007 | Binomial cubic roots | 2,001.9 → 197.0 | **10.11× / 8.35×** |
| 1,000,000,007 | Quartic with `x²` factor | 8,246.0 → 238.4 | **34.66× / 31.61×** |
| 1,000,000,007 | General quartic with root zero | 7,165.6 → 4,894.4 | **1.46× / 1.52×** |
| 4,294,967,291 | 3-isogeny, `a=0` | 3,707.5 → 42.9 | **86.5× / 66.2×** |
| `2^61-1` | 3-isogeny, `a=0` | 12,970.8 → 41.5 | **313.0× / 305.3×** |

The `p=65,521` binomial cubic is correctly **not** specialized (`p ≡1 mod3`), and is approximately at parity. General-curve isogeny benchmarks were near parity in the first native run; the independent rerun included some regressions (~5–14% on selected 65,521 control cases). Do not generalize the large special-case wins to arbitrary curves.

## Local Linux perf, **C++ algorithmic reproduction**

CPU-pinned AMD EPYC 9V74, GCC 14.2 and Clang 17 (`-O3 -march=native`), two deterministic input seeds per compiler, 11 alternating before/after pairs, first two discarded, using Linux `perf_event_open` `PERF_COUNT_SW_TASK_CLOCK`. The standalone `perf` executable is absent; this records **software task-clock counters**, not hardware cycles or cache misses.

Across all four compiler/seed runs for `p=1,000,000,007`: retained combined optimization measured **53.4–106.1×** for `a=0` 3-isogenies, **84.0–157.6×** for `b=0` 2-isogenies, **5.58–6.77×** for `ψ₃` roots with `a=0`, **7.76–10.22×** for binomial cubics, **27.4–35.6×** for quartics with an `x²` factor, and **1.36–1.49×** for arbitrary quartics with a zero root. Normal `ψ₃` and random quartic controls were mostly around parity, sometimes regressive.

## Validation

* Two source-exact CI runs passed independent small-field exhaustive root enumeration (5, 7, 11, 13, 17, 19, 23, 31, 101), large-prime cube-root verification, and checksum comparisons of full isogeny results.
* Local C++ differential oracles passed on 15 primes, including constructed roots, quartics, and generic and special isogeny queries. GCC ASan/UBSan passed.
* Skipped-first-multiplication in modular exponentiation was also screened locally, but complete-query results were mixed, so it is **not** retained.
* The native production Rust and local C++ timings are distinct experiments, not interchangeable.

## Remaining gates

Keep this stacked PR draft pending broader repository CI and any behavior-sensitive accounting checks. In particular, the fast paths change `FieldTally` (accurately reflecting that less arithmetic was performed). No characteristic-two ECC2K relation-finding throughput improvement is claimed; that is a separate code path.
