# Root-finding optimization Round 131 (2026-11-23)

**Base:** Round130 commit `20bb59ed9ec414542fc890cc684ba22eee1f2268`. **Production file:** `src/cryptanalysis/ecbench/isogeny.rs`.

## Retained implementation

For a monic quadratic root factor `g=x²+bx+c` over `F_p`, when `p ≡ 3 (mod 4)`, avoid an additional polynomial modular exponentiation and GCD during Cantor–Zassenhaus splitting. Compute `D=b²-4c`, the square root `s=D^((p+1)/4)`, verify `s²=D`, and output `(-b±s)/2`. Divide by two with a conditional add of `p` and a shift, avoiding a field inverse. Nonresidues and other primes use the previous generic path. No external API changes.

## Local Linux perf comparison (C++ reproductions, NOT Rust)

AMD EPYC 9V74, CPU 0, software `PERF_COUNT_SW_TASK_CLOCK` using `perf_event_open`. GCC 14.2 and Clang 17, `-O3 -march=native`. Two RNG seeds each, 13 alternating A/B trials per case. Full source-backed Round130 reproduction baseline variant `2532` versus quadratic shortcut variant `6628`; median paired timing ratios (four runs total).

| Complete root query | Prime | GCC 2 runs | Clang 2 runs |
|---|---:|---:|---:|
| Two distinct quadratic roots | 1,000,000,007 | 2.70–2.71x | 2.70–2.97x |
| Two distinct quadratic roots | 4,294,967,291 | 3.30–3.43x | 3.21–3.23x |
| Two distinct quadratic roots | 2^61−1 | 3.37–3.40x | 2.99–3.10x |
| Three known cubic roots | 1,000,000,007 | 1.63–1.65x | 1.64–1.67x |
| Realistic ψ₃ quartic | 1,000,000,007 | 1.24–1.25x | 1.24–1.26x |
| Realistic ψ₃ quartic | 4,294,967,291 | 1.22–1.23x | 1.25–1.26x |
| Realistic ψ₃ quartic | 2^61−1 | 1.01–1.04x | 0.99–1.04x |
| Quadratic control p ≡ 1 mod 4 | 65,521 | ~1.00x | 1.00–1.03x |

Local hardware instruction/cycle counters and the standalone perf executable were unavailable. The software task-clock was available. Detailed local raw trial CSVs, reproducible C++ source, ASan/UBSan logs and full per-case summary are in the accompanying run artifact.

**Rejected candidates:** Moving owned GCD dividends into remainder reduction (near parity or noisy results: 0.99–1.04x); combining it with the quadratic path (no repeatable incremental gains). Neither is included in the production patch.

## Correctness and native verification

GCC ASan/UBSan passed. Local C++ differential checks passed; exhaustive quadratic root enumeration over small primes `3,5,7,11,13,19,23,31,101` tested **12,425** monic quadratics against brute force, in both variants. Production Rust has an exhaustive quadratic regression test.

**Source-exact native Rust build and A/B workflow passed:** https://github.com/isogeny-labs/crypto/actions/runs/45140201140. The workflow builds the actual baseline and optimized production module as separate Rust modules, verifies independent known-factor and exhaustive quadratic oracles, checks A/B checksums, and runs 13 alternating timings across four prime fields. Native numeric results are available in the workflow's logs, not copied here, and must not be inferred from C++ figures.

**Acceptance:** Keep as draft, stacked on #4994 (itself stacked on #4981 and #4971). Full repo CI and ECC2K index-calculus relations/sec remain unmeasured. This patch optimizes the prime-field isogeny root solver only.
