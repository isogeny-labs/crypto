# Root finding Round 134 (2026-11-23)

Stacked on [Round 133 PR #5002](https://github.com/isogeny-labs/crypto/pull/5002). Production source: `src/cryptanalysis/ecbench/isogeny.rs`.

## Three improvements evaluated and implemented together

1. **Binary Jacobi nonresidue search:** for odd prime p, Jacobi(z,p)=Legendre(z,p). Avoid one Fermat modular power for every tested Tonelli-Shanks nonresidue candidate.
2. **Shared Tonelli-Shanks powers:** with p-1=q*2^s, q odd, compute h=a^((q-1)/2), t=h²a=a^q, r=ha=a^((q+1)/2). Avoid one of two separate modular exponentiations.
3. **Omit redundant Euler precheck:** when a is nonsquare, t=a^q has order exactly 2^s, so the i<m search returns None. The parent roots() verifies the returned root.

The C++ reproductions benchmarked Jacobi alone, Jacobi plus exponent fusion, and all three; only the combined optimization is committed in production Rust.

## Local Linux perf: **C++ reproduction, not Rust**

Intel Xeon Platinum 8573C; GCC/Clang -O3 -march=native; CPU pinned; Linux perf_event_open software task-clock counters **available**. 4 independent runs (two seeds/compiler), 11 alternating A/B pairs/run, first 2 warmups, median of remaining 9 within-pair ratios. 12 preconstructed inputs repeated 40 times per trial.

| Complete root query | Example GCC before → after (ns) | Paired speedup across four runs |
| --- | ---: | ---: |
| Constructed quadratic p=65,521 | 1,454 → 304 | **4.78–5.69×** |
| Constructed quadratic p=998,244,353 | 1,888 → 895 | **1.66–2.09×** |
| Constructed quadratic p=1,000,000,009 | 3,198 → 555 | **5.50–5.86×** |
| Constructed quadratic p=2,013,265,921 | 2,747 → 848 | **3.17–3.43×** |
| Constructed cubic p=1,000,000,009 | 11,031 → 8,052 | **1.27–1.40×** |
| Constructed quartic p=1,000,000,009 | 21,188 → 17,086 | **1.22–1.25×** |
| Random ψ3 p=65,521 | 4,141 → 4,225 | 0.99–1.09× |
| Repeated quadratic p=65,521 | 38.6 → 37.3 | 0.95–1.02× |

Jacobi alone improved constructed quadratic queries 3.14–4.19× at 65,521 or 1,000,000,009 (field/run dependent); Jacobi + fused powers yielded 3.76–4.24×. Hardware PMU cycles/cache were unavailable, so results use software task-clock; the raw local CSVs are provided separately.

## Validation

Local exhaustive monic quadratics over p=3,5,7,11,13,19,23,31,101 and large-field differential tests passed for baseline and all candidate variants. GCC ASan/UBSan passed. Native Rust source-exact A/B workflow in `.github/workflows/rootfind_round134.yml` compiles baseline Round133 and this branch, runs independent square-root oracles (including p=998244353 and p=2013265921), and collects 13 alternating paired trials on complete roots, direct sqrt and isogeny controls. **Do not infer native Rust gain from C++ figures until that workflow passes and its timings are inspected.**

## Gate

Keep draft until native source-exact correctness/performance and broad repo CI pass. Some controls are near parity/regressive; no claim for complete 3-isogeny discovery or ECC2K relation-finding throughput.
