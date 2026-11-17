# Fp64 CPU root-finder optimization: initial local measurement

Date: 2026-11-17. Scope: **host/emulation only** for the 64-bit prime-field
root-finding oracle, using public synthetic inputs. The CUDA inversion algorithm
is deliberately unchanged; no CUDA, ECDLP or end-to-end performance claim.

## Changes

1. `gpu/rootfind/fp64.cuh::fp64_inv`: on CPU only, replace the
   `p-2` binary-exponentiation inversion with signed extended Euclid on the
   canonical residue, then return the Montgomery representation. The GPU
   `__CUDA_ARCH__` branch retains the original uniform Fermat exponent.
   The CPU path is variable-time and **must never be used on secrets**.
   This choice follows the existing repo's differentiated host/device design,
   rather than attempting to deploy a secret-data modular inverse.
2. `fp64_batch_inv`: omit the first prefix product by Montgomery one and
   the final two unwind products by one. Cost: `3n-3` multiplications for
   `n>=1` rather than `3n`, plus the same single inversion; `n=0` is a
   no-op. The batch result is bit-identical.
3. Reproducibility: `gpu/rootfind/bench_host_inv.cpp` freezes the original
   implementations alongside the new ones; `make -C gpu/rootfind bench-host-inv`.
   `test_cpu.cpp` adds independent Fermat-reference checks, empty batch, and
   56/62-bit primes.

### Prior art

- Internal: `RESEARCH_GPU_ROOTFIND_DECOMPOSITION.md` §§1–2 (batched
  inverses, host/device split), `RESEARCH_SEMAEV_ROOTFIND_VS_GROEBNER.md`,
  `gpu/rootfind/fp64.cuh` baseline. No novel mathematical inversion claimed.
- Bernstein–Yang, *Fast constant-time gcd computation and modular inversion*,
  TCHES 2019, https://gcd.cr.yp.to/papers.html#safegcd.
  This implementation **does not implement safegcd**; that paper illustrates
  why a constant-time variant is required if inputs might be secret.

## Local measurements (preliminary, not full repository throughput)

An isolated C++17 harness reproducing the exact pre/post `Fp64` operations
was compiled with `g++ -O2 -std=c++17` and pinned with `taskset` to one
CPU of an AMD EPYC 9V74 VM, Linux 6.18.57, 4 CPU-equivalent cgroup limit.
One 65,536-element public random nonzero Montgomery input stream per field,
median of nine runs; smaller times are better. Each candidate was checked
against the old algorithm on the full 65,536-element stream, and batch
sizes `1,2,16,32,64` gave identical outputs. Additional 10,000 random
plus edge inputs across nine primes passed under UBSan (including
`p=9223372036854775783` to check the signed bound).

| Prime field | Inversion old ns | EEA ns | Inversion speedup | Batch32 old ns | Batch32 combined ns | Batch32 speedup |
|---|---:|---:|---:|---:|---:|---:|
| 40-bit | 213.12 | 96.42 | 2.21× | 634.19 | 337.65 | 1.88× |
| 56-bit | 297.07 | 131.64 | 2.26× | 589.19 | 362.25 | 1.63× |
| 61-bit | 316.62 | 133.28 | 2.38× | 656.13 | 376.29 | 1.74× |
| 62-bit | 310.63 | 133.64 | 2.32× | 644.34 | 370.37 | 1.74× |

Noise is appreciable for small batch configurations; the above are
single-host medians, not confidence intervals. The independent bench added
in this PR uses the repo header directly and seven timed trials. Run it
on a repository checkout to obtain authoritative header-level numbers.

Explored but **rejected**: left-to-right Fermat, 3- and 4-bit sliding-window
Fermat, and binary almost-inverse style Euclid. They either regressed on
this CPU (e.g., LTR ~0.6–0.8× baseline; binary ~0.7× baseline) or failed
to give repeatable gains. There is no claim that they lose on a GPU or
on other platforms.

## Perf / validation limitations

- `perf` executable is **not installed** in the local runtime
  (`perf: command not found`). A direct `perf_event_open` on a
  per-thread user-space hardware-cycles counter returned `ENOENT`
  under this VM; PMU counters were unavailable. Therefore the numbers
  above are **steady-clock wall ns**, **not perf stat cycles/instructions,
  retired events, or cache misses**. This does not satisfy the requested
  `perf`-counter validation.
- Network/DNS to GitHub is unavailable in this container. The local harness
  used a manually mirrored copy of the field functions; the changed files
  were committed through the GitHub connector. `make test`,
  `make bench-host-inv` against the actual checkout,
  `cargo test --release --lib`, CUDA validation, and end-to-end throughput
  are **not claimed as executed** here.
- The GitHub PR checks visible at creation did not run the targeted C++
  root-finder unit suite. Keep the PR draft until it passes targeted
  compilation, full `test_cpu`, and perf measurements on a PMU-enabled host.

## Next measured gate

```sh
make -C gpu/rootfind test
make -C gpu/rootfind bench-host-inv
perf stat -r 5 -e cycles,instructions,branches,branch-misses,cache-misses \
  -- gpu/rootfind/bench_host_inv
cargo test --release --lib gpu_rootfind
cargo test --release --lib
```

Then compare the full `rf64_decompose` operation on equal public target
streams, preserving `backend=emulate`, lookup type, chunk and thread count,
and retain the change only if correctness and throughput both hold.
