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
