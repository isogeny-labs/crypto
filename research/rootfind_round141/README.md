# Round 141: specialized binary modular inverse for polynomial normalization

2026-11-23. Implementation: [PR #5054](https://github.com/isogeny-labs/crypto/pull/5054). **All results are [pcc:unverified]** until independent PCC signing; the local CPU measurements are C++ reproductions, not native production Rust timings.

## Optimization

In `src/cryptanalysis/ecbench/isogeny.rs`, the new `Fp::inv_monic_lead` uses variable-time binary inversion for public prime-field polynomial leading coefficients greater than 3. `Fp::make_monic` invokes it. Existing special inverses for 2 and 3, `Fp::inv`, point mapping, polynomial GCD and all other consumers remain unchanged.

The method maintains canonical modular coefficients and uses `floor(x/2) + floor(p/2) + 1` for odd modular-halving, without risking `x+p` overflow. As before, one inversion is charged to `FieldTally.invs`. The method is **not suitable for secret-dependent inputs**, so do not reuse it in point mapping.

## Candidate search and controls

- Full replacement of Fermat inverse with classical extended Euclid: isolated inverse faster but 61-bit generic cubic and monic-normalization regressions; rejected.
- Full replacement of Fermat inverse with binary inverse: large isolated gains but 61-bit quartic and isogeny regressions, particularly under Clang; rejected.
- Binary inverse only in `make_monic`: kept, as the speedup is concentrated on normalization and full nonmonic quadratic roots while preserving the old inverse for other arithmetic.

These are classical arithmetic implementation methods, **not newly discovered root-finding algorithms**. Related references: Menezes, van Oorschot and Vanstone, *Handbook of Applied Cryptography* chapter 14, Algorithm 14.61 (<https://cacr.uwaterloo.ca/hac/about/chap14.pdf>); the extended-Euclid inverse description at <https://cp-algorithms.com/algebra/module-inverse.html>. Prior repository experiments: `research/rootfind_round137` and `research/rootfind_round140`. The research-KB MCP and repository wrapper were unavailable in this runtime.

## Local C++ reproduction benchmark

Linux Intel Xeon Platinum 8573C, GCC 14.2 and Clang 17 with `-O3 -march=native`, pinned CPU. `perf_event_open(PERF_TYPE_SOFTWARE, PERF_COUNT_SW_TASK_CLOCK)` succeeded, while hardware cycle/cache counters and the standalone `perf` executable were unavailable. Each compiler/mode uses 40 deterministic inputs, four seeds, 11 trials per seed with the first two discarded. The numbers below are medians of 36 retained trials per mode. Code is a C++ arithmetic reproduction inherited from the archived Round 140 suite.

| Prime field | GCC nonmonic quadratic before → after (ns) | GCC speedup | Clang speedup | GCC monic-normalization speedup |
|---|---:|---:|---:|---:|
| 65,521 | 330.17 → 250.11 | 1.320× | 1.390× | 2.091× |
| 1,000,000,007 | 406.42 → 231.28 | 1.757× | 1.684× | 2.756× |
| 4,294,967,291 | 533.15 → 301.69 | 1.767× | 1.698× | 3.316× |
| 2^61−1 | 560.94 → 375.48 | 1.494× | 1.296× | 1.342× |

Generic cubic/quartic/isogeny controls were mixed; the 61-bit Clang generic cubic and 3-isogeny controls measured 0.925× and 0.927×, respectively. Those workloads typically do not invoke the new path, so this does **not** establish a general slowdown or speedup. Repeat source-exact native Rust trials before any merged performance claim.


## Source-exact production Rust A/B (two passing workflow executions)

Rust 1.100.0 (`rustc -C opt-level=3 -C target-cpu=native`), exact `src/cryptanalysis/ecbench/isogeny.rs` from repository `main` versus the PR's file. The test shim exposes `PrimeCurve`/`PrimePoint` and appends **identical** benchmark/oracle helpers to both modules. Each operation has 32 deterministic inputs and 13 alternating A/B trials, two warmups discarded. The numbers are two independent successful GitHub Actions runs, [run 45240296178](https://github.com/isogeny-labs/crypto/actions/runs/45240296178) and [run 45240305320](https://github.com/isogeny-labs/crypto/actions/runs/45240305320). Both passed the native independent correctness oracles and checksums. [pcc:unverified]

| Complete Rust workload | Prime | Run A before → after (ns) | Run A speedup | Run B speedup |
|---|---:|---:|---:|---:|
| Normalize nonmonic polynomial | 65,521 | 83.83 → 39.51 | 2.126× | 2.111× |
| Normalize nonmonic polynomial | 1,000,000,007 | 160.69 → 69.34 | 2.320× | 2.188× |
| Normalize nonmonic polynomial | 4,294,967,291 | 199.49 → 73.07 | 2.733× | 2.574× |
| Normalize nonmonic polynomial | 2^61−1 | 217.99 → 139.29 | 1.573× | 1.718× |
| Nonmonic random quadratic roots | 65,521 | 210.87 → 152.78 | 1.384× | 1.328× |
| Nonmonic random quadratic roots | 1,000,000,007 | 294.63 → 197.78 | 1.499× | 1.511× |
| Nonmonic random quadratic roots | 4,294,967,291 | 352.80 → 218.25 | 1.598× | 1.599× |
| Nonmonic random quadratic roots | 2^61−1 | 386.95 → 300.05 | 1.281× | 1.259× |

Generic cubic/quartic and complete 2-/3-isogeny controls were mostly near parity, but several individual trials were noisy (0.793× to 1.114×). Do **not** interpret these runs as a reliable broad downstream improvement or as a complete ECC2K index-calculus throughput measurement. The isolated inverse control using the **unchanged** `Fp::inv` was near parity, as expected.

An additional native oracle commit exercises 16,384 deterministic full-width nonconstant coefficient inverses against Fermat exponentiation; its separate workflow result must be checked before final acceptance.

## Correctness and next gate

The inherited C++ reproductions passed 140,000 random nonzero prime-field inversion identities per build, small-prime independent brute-force root oracles, deterministic full-query checksum equality, and Clang ASan/UBSan for the unrestricted binary candidate. The production PR adds two Rust tests: randomized comparison of the specialized inverse against Fermat exponentiation and brute-force nonmonic quadratic roots.

`research/rootfind_round141/bench_helper.rs`, `source_exact_main.rs`, and `.github/workflows/rootfind_round141.yml` perform original-source native Rust A/B benchmarking and independent small-field correctness checks. The two earlier source-exact runs passed, while the expanded 16,384-case native oracle and full repository CI still require inspection. The native unit-test module also has not been executed by the narrow test shim. Full-repository tests and ECC2K relation-finding throughput are **not** covered by the local experiment.

Raw perf CSV, C++ reproduction, sanitizer logs and independent summary are included in the run's `rootfind-round141-20261123.zip` conversation artifact, rather than committed in this branch. If artifact permanence is needed, upload these as a GitHub Actions artifact or committed evidence in a follow-up, with hashes and an independent validator.
