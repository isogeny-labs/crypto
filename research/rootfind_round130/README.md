# Round 130: In-place owned polynomial remainder

Baseline: Round 129 production file at `d0bece58dcad94b83e7b122cc79bd83936373672`.

The polynomial multiply stage already constructs `Vec<u64> t`. It passed `&t` to `rem`, which immediately cloned the vector. Added `rem_owned(Vec<u64>, &[u64])` to execute the existing reduction algorithm on its caller-owned buffer; `rem` remains a public-internal adapter to handle borrowed slices. `mulmod` now hands ownership to `rem_owned` and avoids a copy and heap allocation each modular polynomial multiplication. The result and tally are identical.

## Candidate screening (local C++ reproduction, **not Rust**)
- Candidate A: Extended-Euclid modular inverse. Local complete-root gains mostly small and mixed, including a regression on 1e9+7 monic cubics. Not included.
- Candidate B: 64-bit reciprocal modular reduction (32-bit primes). Mixed: some 1.1-1.3x gains, but severe regression on a 65,521 psi3 workload. Not included.
- Candidate C: Owned in-place polynomial remainder. Local C++ initial screen showed gains across most monic/scaled cubic and quartic workloads with isolated regressions in a psi3 case. Included provisionally, pending native Rust validation.

Local tests run on AMD EPYC 9V74 with CPU pinning and Linux `perf_event_open` software task-clock; hardware counters and the `perf` executable were unavailable. The C++ benchmark follows production arithmetic and is not source-exact. Native Rust reproducibility uses `.github/workflows/rootfind_round130.yml`, which compiles both actual Rust modules with identical tests, constant input generation outside timed loops, checksums, and alternating 13-trial A/B queries on 4 prime fields and 8 workloads.

## Acceptance gate
Leave this stacked PR draft until native source-exact benchmark and independent oracle pass. Evaluate a broad basket of 2/3-isogeny, psi3, known-root and random polynomial cases. If representative workloads regress, revert or further narrow. Main-repository CI and ECC2K relation-finding throughput remain separate outstanding gates.

## Measured results, 23 November 2026

### Production Rust: source-exact A/B, PASS twice

[Initial native run](https://github.com/isogeny-labs/crypto/actions/runs/45132429331), then a successful **rerun of the same job**. Rust 1.100.0, 13 alternating A/B timing trials per workload; 32 workloads covering 4 prime fields and 8 query types. Both executions passed the independent oracle for the production Rust baseline and optimized implementation; 32/32 workload speedups > 1 in each execution. Median paired-ratio speedups are reported below.

| Complete query | p | Run 1, before→after (ns) | Run 1 | Replication |
|---|---:|---:|---:|---:|
| find_isogeny(2) | 65,521 | 3,024.39 → 2,674.62 | 1.175× | 1.186× |
| find_isogeny(3) | 65,521 | 4,512.19 → 4,274.74 | 1.129× | 1.123× |
| find_isogeny(2) | 1,000,000,007 | 4,805.31 → 4,085.68 | 1.154× | 1.103× |
| find_isogeny(3) | 1,000,000,007 | 7,327.36 → 6,329.98 | 1.145× | 1.086× |
| find_isogeny(3) | 4,294,967,291 | 9,904.98 → 9,172.89 | 1.078× | 1.090× |
| find_isogeny(3) | 2^61−1 | 17,464.52 → 16,306.98 | 1.069× | 1.088× |
| Monic cubic roots | 1,000,000,007 | 14,586.59 → 12,699.51 | 1.158× | 1.141× |
| Monic quartic roots | 2^61−1 | 63,209.85 → 56,284.00 | 1.121× | 1.108× |
| Random quartic roots | 1,000,000,007 | 7,955.33 → 7,310.40 | 1.048× | 1.080× |

Run-to-run absolute timings changed, so interpret **paired within-run ratios** rather than mixing timestamps across runs. A source-exact Rust end-to-end ECC2K benchmark was not run.

### Local Linux perf_event_open: PASS

AMD EPYC 9V74, CPU pinned, GCC 14.2 and Clang 17, `-O3 -march=native`, software task-clock counters; 13 alternating A/B trials per case, 8 full-query repetitions per trial. Two independent data seeds/compiler = **4 runs × 22 workload/field cases = 88 comparisons**. All 88 paired-median speedups were >1, with representative ranges:

| Workload | p | Four local run speedups |
|---|---:|---|
| Monic cubic | 1,000,000,007 | 1.173–1.245× |
| Nonmonic cubic | 4,294,967,291 | 1.172–1.206× |
| Nonmonic quartic | 65,521 | 1.114–1.153× |
| Isogeny ψ3 quartic | 4,294,967,291 | 1.081–1.152× |
| Monic cubic | 2^61−1 | 1.119–1.203× |

These are C++ line-for-line arithmetic reproductions, **not** Rust or direct isogeny production timing. Independent C++ equivalence and GCC ASan/UBSan passed. Task-clock is process CPU-time accounting; no hardware instruction/cycle/cache measurements. Full raw CSVs and source are in the per-run artifact ZIP.

### Decision

The owned-polynomial-remainder optimization is supported by two native Rust executions and four local perf-counter reproductions, with no regression observed in the sampled cases. **Remain draft and stacked on #4981** until broad repository CI and representative downstream index-calculus relation-finding/transfer acceptance are completed; do not claim an ECC2K relations/sec speedup.
