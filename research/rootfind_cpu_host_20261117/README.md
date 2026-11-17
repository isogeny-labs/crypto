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

## Follow-up: P-256 host root-finding arithmetic (same date)

This follow-up uses **P-256 only**, independently of the Fp64 improvements.
`gpu/rootfind/fp256x64.cuh` now uses a public, fixed radix-16 exponent
on the **host** and removes the same three batch identity multiplications.
The original device exponentiation is preserved until actual GPU counters
and occupancy can be measured. The P-256 exponent `p-2` has 64 hex digits:
31 zeros, 31 F's, one 1 and one D. The chain precomputes
`a^3, a^12, a^13, a^15` with 3 squarings and 3 multiplies; the loop
performs 252 squarings and 32 multiplies, totaling **255 squarings
and 35 multiplies** instead of **256 squarings and 128 multiplies**.
The batch algorithm now needs `3(n-1)` products rather than `3n`.
It is an ordinary addition-chain/comb optimization, not a new algorithm.

A local, isolated C++17 reproduction of the existing P-256 limb multiply,
the original exponentiation, and the new radix-16 code was built with
`g++ -O2 -std=c++17` and pinned to CPU 0 of an AMD EPYC 9V74 VM.
The corpus contains 8,192 deterministic public inputs, including Montgomery
one. The before/after outputs were bitwise identical for every inversion;
the result multiplied by the input is Montgomery one, and all results in
batches 0/1/2/4/16/32/64 agreed (n=0 changes to an explicit no-op).
The benchmark ran nine repeats per arm and used the median per arm.
Across three separately launched pinned processes:

| Experiment | Old ns/op across runs | New ns/op across runs | Per-run speedup |
|---|---|---|---|
| P-256 inverse | 27,295 / 28,328 / 25,481 | 20,716 / 20,199 / 20,389 | 1.318 / 1.403 / 1.250× |
| P-256 batch of 32 | 34,515 / 34,788 / 32,131 | 28,412 / 27,959 / 24,507 | 1.215 / 1.244 / 1.311× |

The median speedups were **1.318× per host P-256 inverse** and **1.244×
per 32-element host batch** over these three runs; timings show appreciable
run-to-run variance. A 128-input sanitizer reproduction passed with
`-fsanitize=undefined -fno-sanitize-recover=all`; that run's timings are
not performance evidence. The branch also adds the actual header-based
`gpu/rootfind/bench_host_inv256.cpp` comparison target and P-256 independent
bitwise inversion checks in `test_cpu.cpp`.

**Not yet verified:** local git clone/DNS access is unavailable, so the
measurements above are from a faithful, stand-alone reproduction, *not*
the actual checked-out `fp256x64.cuh` or an end-to-end rootfinder.
The repository test and benchmark targets have been added but not run on
the repository checkout. Direct `perf_event_open` with valid hardware
cycle-counter attributes returned **EACCES (permission denied)** and the
`perf` executable is absent. Thus neither the Fp64 nor P-256 results are
`perf` PMU observations. The two changes are host-only and do not establish
GPU speedups or full IC-pipeline improvements.

Next gate:
```sh
make -C gpu/rootfind test
make -C gpu/rootfind bench-host-inv
make -C gpu/rootfind bench-host-inv256
perf stat -r 5 -e cycles,instructions,branches,branch-misses,cache-misses \\
  -- gpu/rootfind/bench_host_inv256
```
Use a PMU-accessible host with the checked-out PR, compare the same
input stream, and follow with the fixed-target complete `rf256_decompose`
call. Keep the PR draft until that validation passes.

### Scope / repository hygiene
The follow-up also restored `docs/session-logs/2026-11-17.md`
to its exact base Git blob (SHA-1
`3573026a2c8b768a75bfaa718d67c0c7e69df37f`), correcting an
unrelated, accidental deletion of approximately 1.71 million lines
that had leaked into this PR. All rootfinder measurement detail lives
in this scoped report instead.

## Perf-arm isolation (prepared; not executed on this restricted VM)

The two host benchmarks expose `--perf-inv-before`,
`--perf-inv-after`, `--perf-batch32-before` and
`--perf-batch32-after`. Each mode uses the *same deterministic
corpus* but executes only the named arm, so instruction and cycle
counts are not contaminated by the alternative algorithm's benchmark.
Run the normal no-argument mode first for bitwise correctness checks.

Example for a host with `perf` and authorized PMU counters:

```sh
CPU=0  # choose a CPU actually allowed by the host cpuset
for bin in bench_host_inv bench_host_inv256; do
  for arm in inv-before inv-after batch32-before batch32-after; do
    perf stat -r 5 -e cycles,instructions,branches,branch-misses,cache-misses \\
      -- taskset -c "$CPU" "gpu/rootfind/$bin" "--perf-$arm"
  done
done
```

Compare counts for each *same*-field, same-kernel before/after pair.
The perf modes include deterministic corpus generation and standard
benchmark-loop overhead; report this fact, and also use
`perf record -g` to identify actual hotspots before interpreting
whole-call impact. No `perf` counter numbers are claimed for this run.

An additional `clang++ -O2 -std=c++17` build of the same local
reproduction produced **1.297×** for P-256 host inversion
(20,851 → 16,083 ns) and **1.259×** for batch-32
(24,669 → 19,597 ns), with identical public-input correctness checks.
Compiler sensitivity reinforces that the measurements are indicative,
not deployment-level numbers.


## Follow-up: eliminate identity multiplies in the root-finding oracle (2026-11-17)

Root-finding itself uses two additional Montgomery batch-inversion loops
in `gpu/rootfind/rootfind.cuh`, separate from `fp64_batch_inv`:

* `rf_decompose_thread<F,T,CHUNK>`: seed the prefix accumulator from
  `d[0]`, start the prefix at 1, stop the unwind at 1, and assign
  `d[0]=inv_acc`. Saves **3 field multiplications per full thread chunk**
  for every supported `CHUNK` (8, 16, 32, 64). Preserves the inverse of
  each nonzero (or one-substituted) denominator exactly.
* `rf_pair_run`: seed `pre[0]=d[0]`, omit the final unused unwind
  multiplication, and skip the unreachable `bi==ai` branch for `k>=1`.
  Saves **2 field multiplications per nonempty pair-table run**, with
  unchanged denominator-zero fallback. `m>=1` follows from the
  existing `b0>=n` early return.

All changes operate on field elements in the same Montgomery form and
require no change to relation ordering or lookup tables. They affect the
host and GPU compilations, **but the performance evidence below is CPU
only**; verify GPU occupancy and end-to-end oracle throughput before
adopting a device speedup claim.

### Local paired measurements (isolated reproduction, not PMU counters)

AMD EPYC 9V74 VM, CPU 0 pinned, C++17 `-O2`, deterministic public
8192-row input corpus, median of nine timing passes inside each
process, **15 independent processes**. Every process checked exact
bitwise agreement over all 8192 rows, `pair` lengths 1..16, and
`oracle` chunk sizes 16/32. The `oracle` core omits table lookups,
point arithmetic and relation output; the `pair` core omits its
point-coordinate computation. These are not full-oracle measurements.

| Isolated core | Old median ns/op | New median ns/op | Median of 15 paired speedups | Paired speedup range |
|---|---:|---:|---:|---:|
| Oracle CHUNK=16 | 253.01 | 244.06 | 1.019x | 0.915-1.105x |
| Oracle CHUNK=32 | 389.31 | 379.55 | 1.035x | 0.973-1.650x |
| Pair m=16 | 258.36 | 247.87 | 1.040x | 0.985-1.236x |
| Pair m=1 | 124.38 | 119.15 | 1.033x | 0.993-1.225x |

Some individual runs regress, and the wide ranges make **any single-run
speedup unreliable**. The median paired gains are small (~2-4%).
An on-repository benchmark `make -C gpu/rootfind bench-batch-core`
now compiles with the real `fp64.cuh` and performs paired bitwise
verification before measuring these particular inversion loops.

### Actual repository test and perf status

GitHub Actions [rootfind host perf validation run 44180765115](https://github.com/isogeny-labs/crypto/actions/runs/44180765115)
on the post-loop-change revision successfully installed
`perf version 7.0.14`, compiled `test_cpu.cpp` against the real
headers, and printed `ALL PASS`. The hosted VM set
`kernel.perf_event_paranoid=4`, and the PMU probe failed; the
workflow explicitly reported `PMU unavailable` instead of
fabricating counter values. The local Debian 13 container also denies
software task-clock and hardware cycles/instructions via
`perf_event_open` (`EACCES`) and cannot resolve the package mirrors.

The newly added `bench-batch-core` target and extended workflow were
committed **after** that passing run and must pass their own CI build
before the PR can be treated as fully verified. Even with a passing
unit suite, GPU tests and full `rf64_decompose` benchmarking remain
required; the PR must stay draft until those gates are satisfied.
