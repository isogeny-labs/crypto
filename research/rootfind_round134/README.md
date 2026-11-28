# Root-finding Round 134: faster Tonelli-Shanks over prime fields (2026-11-23)

Production file: `src/cryptanalysis/ecbench/isogeny.rs`. Draft stacked on [PR #5002](https://github.com/isogeny-labs/crypto/pull/5002).

## Candidates evaluated and retained

1. **Jacobi nonresidue selection**: for **prime** p, Jacobi(z,p) = Legendre(z,p). Binary reciprocity selects a Tonelli-Shanks auxiliary nonresidue without repeated Fermat exponentiations.
2. **Shared exponentiation**: with p−1=q·2^s (q odd), calculate h=a^((q−1)/2); then t=h²·a=a^q and r=h·a=a^((q+1)/2). Saves one full field exponentiation.
3. **Jacobi input rejection**: determine Jacobi(a,p) before Tonelli setup. Reject nonsquares with integer arithmetic, rather than a Fermat exponentiation.

**Rejected candidate:** Completely deleting the initial residuosity test made a first source-exact Rust benchmark faster on residues but **0.28–0.72× on nonsquares** (up to 3.6× slower). It was replaced by Jacobi input rejection. The final candidate is the three changes above.

## Local Linux perf measurements

CPU: Intel Xeon Platinum 8573C. GCC and Clang `-O3 -march=native`, CPU pinned; **Linux perf_event_open software task-clock available**. Two independent seeds/compiler, 11 alternating A/B pairs each, first two pairs excluded as warmup; median of nine within-pair ratios. Values below are C++ algorithmic reproductions, **not Rust**; hardware cycles/cache counters were unavailable.

| Complete C++ operation | Example GCC before → after | Paired speedup across four executions |
| --- | ---: | ---: |
| Quadratic, p=65,521 | 1,688 → 321 ns | **4.64–5.34×** |
| Quadratic, p=998,244,353 | 2,074 → 1,404 ns | **1.38–1.61×** |
| Quadratic, p=1,000,000,009 | 2,763 → 499 ns | **5.20–5.96×** |
| Quadratic, p=2,013,265,921 | 2,967 → 999 ns | **2.62–3.09×** |
| Nonsquare square-root query, p=1,000,000,009 | 178 → 18.4 ns | **9.70–10.75×** |
| Constructed cubic, p=1,000,000,009 | 11,264 → 8,344 ns | **1.30–1.36×** |
| Constructed quartic, p=1,000,000,009 | 20,498 → 17,209 ns | **1.16–1.26×** |
| Random ψ3, p=65,521 | 3,681 → 3,970 ns | **0.97–1.04×** |

Small control workloads, including repeated roots and p=1,000,000,007 quadratics, were near parity or sometimes regressed. The best combined local speedups are localized and are not whole-workload guarantees. Earlier ablations show Jacobi selection alone and shared exponentiation also improved constructed quadratics.

## Source-exact optimized Rust run

[GitHub Actions native A/B](https://github.com/isogeny-labs/crypto/actions/runs/45166204719) **passed** (rustc 1.100.0, optimized, independent Euler-criterion oracles, 13 alternating A/B trials). Actual production module compiled twice, with baseline from Round133. Medians and paired ratios:

| Complete native Rust operation | Baseline → candidate ns | Paired speedup |
| --- | ---: | ---: |
| Distinct quadratic, p=65,521 | 1,242 → 301 | **4.14×** |
| Distinct quadratic, p=998,244,353 | 1,113 → 719 | **1.55×** |
| Distinct quadratic, p=1,000,000,009 | 2,145 → 441 | **4.87×** |
| Distinct quadratic, p=2,013,265,921 | 2,346 → 909 | **2.58×** |
| Nonsquare direct root, p=1,000,000,009 | 151 → 35.5 | **4.25×** |
| Cubic, p=1,000,000,009 | 11,731 → 10,196 | **1.15×** |
| Complete 3-isogeny, p=1,000,000,009 | 7,125 → 7,046 | **1.01×**, near parity |

p=1,000,000,007 complete quadratic control was **0.99×**, slightly slower. No overall isogeny or ECC2K throughput speedup is claimed.

## Correctness and gates

C++ exhaustive small-prime quadratic enumeration (p=3,5,7,11,13,19,23,31,101), randomized differential polynomial arithmetic, and GCC ASan/UBSan passed for the final candidate. Production Rust source-exact oracle verified roots and nonsquares across p=5,17,97,257,65,521,998,244,353,1,000,000,009,2,013,265,921. The independent tests assume input modulus is an odd prime.

**Still draft** until an independent Rust replication and broad repository CI, and a suitable non-constructed/range-of-input benchmark, pass. These improvements target a prime-field isogeny polynomial root solver; no ECC2K-130 relation-finding throughput comparison has yet been performed.
