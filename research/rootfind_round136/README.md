# Rootfinding round 136: Cipolla, Jacobi, and measured tradeoffs

Date: 2026-11-23. Base: https://github.com/isogeny-labs/crypto/pull/5008
Implementation: https://github.com/isogeny-labs/crypto/pull/5010

## Changes
1. Select Cipolla square roots for primes where v2(p-1) >= 20. Implement the Fp2 multiplication, squaring and exponentiation with verified roots; keep the old Tonelli/Atkin paths otherwise.
2. Pre-reject nonsquare discriminants with binary Jacobi for direct quadratic root queries over p = 3 mod 4 with p <= 2^32-1. The recursive splitter does not precheck known-splitting factors; the 61-bit p=3 mod4 path is unchanged.
3. Rejected fixed-window-3 and identity-multiply elimination in modular exponentiation due to inconsistent benchmark gains. Also rejected Cipolla for low 2-adicity (e.g., 97 and 257), where it regressed.

## Production Rust A/B, independently replicated

Source-exact optimized Rust 1.100.0, 13 alternating trial pairs, median nanoseconds/query and median paired speedup. Independent Euler and complete-roots correctness oracles passed twice.
https://github.com/isogeny-labs/crypto/actions/runs/45186540727

| Prime | Complete operation | Run 1 before -> after ns | Run 1 speedup | Rerun speedup |
|---:|---|---:|---:|---:|
| 167772161 | quadratic roots | 807.50 -> 463.29 | 1.746x | 1.456x |
| 998244353 | quadratic roots | 738.05 -> 539.82 | 1.382x | 1.175x |
| 2013265921 | quadratic roots | 953.50 -> 514.25 | 1.854x | 1.707x |
| 167772161 | residue square root | 799.51 -> 428.91 | 1.866x | 1.452x |
| 998244353 | residue square root | 772.26 -> 519.00 | 1.492x | 1.253x |
| 2013265921 | residue square root | 913.80 -> 495.36 | 1.838x | 1.665x |
| 65519 | nonsquare quadratic | 105.36 -> 41.33 | 2.569x | 1.686x |
| 1000000007 | nonsquare quadratic | 177.32 -> 60.28 | 2.942x | 2.759x |
| 4294967291 | nonsquare quadratic | 224.24 -> 61.57 | 3.618x | 3.440x |
| 1000000007 | random quadratic | 244.36 -> 186.61 | 1.395x | 1.363x |
| 4294967291 | random quadratic | 252.76 -> 206.70 | 1.232x | 1.244x |

Negative control: 32-bit positive-only quadratic root queries regressed 8-15% from Jacobi overhead. Complete 3-isogeny discovery remained near parity, ~0.96-1.05x in native runs. No ECC2K relations/sec gain is claimed.

## Local Linux perf task-clock (NOT Rust)

Two independently built C++ algorithmic reproductions, GCC and Clang optimized, CPU pinned, 13 alternating A/B trials; median paired speedup after two warmup pairs.
The local Linux perf_event_open software task-clock counter was available. Hardware counters and the perf executable were not.

| Field / complete operation | GCC speedup | Clang speedup |
|---|---:|---:|
| 167772161 constructed quadratic | 1.498x | 1.470x |
| 998244353 constructed quadratic | 1.197x | 1.180x |
| 2013265921 constructed quadratic | 1.765x | 1.709x |
| 167772161 random quadratic | 1.293x | 1.341x |
| 998244353 random quadratic | 1.216x | 1.162x |
| 2013265921 random quadratic | 1.518x | 1.509x |
| 1000000007 nonsquare quadratic | 3.268x | 3.205x |
| 4294967291 nonsquare quadratic | 2.864x | 3.563x |

Small-prime exhaustive and large-prime randomized differential comparisons passed. GCC ASan/UBSan passed on both reproduction harnesses.

Status: draft and stacked on #5008. Broad repository CI and characteristic-two ECC2K relation throughput remain outstanding.
