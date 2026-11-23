# Root-finding optimization, round 137: inversion-free polynomial GCD

Date: **2026-11-23**. Stacked on [round 136, PR #5010](https://github.com/isogeny-labs/crypto/pull/5010).
Production code: `src/cryptanalysis/ecbench/isogeny.rs`.
Source-exact [native Rust validation and A/B run](https://github.com/isogeny-labs/crypto/actions/runs/45196462103).

## Change retained

The prime-field root finder repeatedly computes `gcd(f, x^p-x)` and GCDs during Cantor-Zassenhaus splitting. The old Euclidean loop called an exact field remainder, paying a Fermat inverse for each intermediate **nonmonic** divisor.

A GCD is defined only up to nonzero field scaling. Use *pseudo-remainders* in `gcd()`:

`R <- lc(B) * R - lc(R) * x^shift * B`

This clears a highest-degree coefficient without calculating `inv(lc(B))`. The leading degree necessarily falls over a field. The implementation consumes the current polynomial buffer and short-circuits constant divisors. **Important:** modular exponentiation continues to use exact `rem_owned()`; pseudo-remainders would be incorrect in `powmod()`.

## Native Rust source-exact before/after

Linux GitHub Actions, `rustc 1.100.0`, `-C opt-level=3 -C target-cpu=native`; two modules are compiled from the exact round-136 and round-137 production files. Thirteen alternating A/B trial pairs, first two discarded, remaining 11 paired medians. Nanoseconds per operation (32 cases). Checksums and independent ordinary-Euclid and evaluated-root oracles passed.

| Prime | Operation | Before ns | After ns | Median paired speedup |
|---:|---|---:|---:|---:|
| 65,521 | GCD (degree 4 vs 3) | 369.32 | 61.48 | **6.005×** |
| 1,000,000,007 | GCD | 713.52 | 79.33 | **9.045×** |
| 4,294,967,291 | GCD | 778.71 | 73.36 | **10.509×** |
| 2^61-1 | GCD | 836.57 | 74.63 | **11.276×** |
| 65,521 | 3-isogeny discovery | 3,261.06 | 3,061.39 | **1.057×** |
| 1,000,000,007 | 3-isogeny discovery | 4,685.54 | 4,275.31 | **1.068×** |
| 4,294,967,291 | 3-isogeny discovery | 6,656.54 | 6,068.84 | **1.092×** |
| 2^61-1 | 3-isogeny discovery | 13,153.39 | 12,508.36 | **1.052×** |
| 1,000,000,007 | ψ3 quartic root query | 4,623.99 | 4,122.40 | **1.117×** |
| 4,294,967,291 | ψ3 quartic root query | 6,568.00 | 6,013.03 | **1.089×** |
| 65,521 | known-root quartic | 7,054.69 | 7,114.62 | **0.988×** |

The 3-isogeny and ψ3 improvements were independently replicated in a second native Rust execution, although some high-degree controls remain near parity. Timing includes common query preparation overhead. The GCD microbenchmark is not a forecast for complete-application speedup.

## Local Linux `perf_event_open`: before and after

CPU-pinned **AMD EPYC 9V74**, GCC 14.2 and Clang 17, `-O3 -march=native`, two independent deterministic input seeds per compiler; 11 alternating trial pairs (discard first two). Collect kernel **PERF_COUNT_SW_TASK_CLOCK** via `perf_event_open`, with nanosecond/query medians and paired-ratio medians. The standalone `perf` binary and hardware cycle/cache counters were unavailable. Local code is a **C++ algorithmic reproduction**, not the actual Rust production implementation.

| Workload | Median paired speedup range across 4 compiler/seed runs |
|---|---:|
| Polynomial GCD, p=65,521 | **3.91–5.23×** |
| Polynomial GCD, p=1,000,000,007 | **6.08–8.56×** |
| Polynomial GCD, p=4,294,967,291 | **7.53–10.24×** |
| Polynomial GCD, p=2^61-1 | **7.54–8.51×** |
| Complete ψ3 quartic roots, all four fields | **1.025–1.223×** |
| Complete known-root cubics | **0.975–1.159×** |
| Complete known-root quartics | **0.967–1.088×** |
| Complete random cubics | **1.034–1.155×** |
| Complete random quartics | **0.949–1.155×** |

The local full-query table includes regressions and should not be interpreted as universally favorable. Production `find_isogeny()` is evaluated separately in native Rust.

## Other candidates screened

* **Pseudo-remainders plus zero-term-skipping convolution:** positive in some tests, mixed or regressive in others. Not retained.
* **Move-only Euclidean remainder reuse:** generally near parity with regressions. Not retained.
* **Pseudo-remainders plus move-only variant:** no reliable advantage over pseudo-remainders alone. Not retained.

## Validation and limits

* Independently compared pseudo-GCD to ordinary field Euclid, normalized, across 14 small and large primes; checked constructed common factors and evaluated every reported cubic/ψ3/quartic root.
* Local C++ differential oracle passed (small-field exhaustive quadratics; randomized low-degree and prime-field gcd/root cases); GCC AddressSanitizer and UndefinedBehaviorSanitizer passed.
* Source-exact native Rust A/B compiled and passed its oracle and checksum checks.
* **Independent source-exact native Rust replication passed:** [second execution](https://github.com/isogeny-labs/crypto/actions/runs/45197138340), same 13 alternating trials with independent Euclid and root oracles. GCD speedups: **5.400×, 7.406×, 8.492×, 9.480×** for the four fields above. Complete 3-isogeny speedups: **1.079×, 1.111×, 1.098×, 1.095×**. The two native runs were executed on different GitHub runner instances, so absolute nanosecond timings vary.
* **Not complete:** repository-wide Rust CI, end-to-end ECC2K relation throughput (characteristic two uses separate code), and hardware-counter profiling. Keep this PR draft/stacked until gates are complete.

Local reproduction archive is attached to the automation result; source and raw CSVs are available there.
