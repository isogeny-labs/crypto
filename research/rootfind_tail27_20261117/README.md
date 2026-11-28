# Rootfinder CPU tail length follow-up (2026-11-17 PT)

## Scope and code changes

- Source: `gpu/rootfind/rootfind.cuh::rf_decompose_thread`, CPU only, `CHUNK=32`.
- Before: use active `cnt` only if `cnt <= 24`; otherwise invert a full padded batch of 32.
- After: use active `cnt` if `cnt <= 27`; otherwise retain full-length behavior. This eliminates `3 * (32-cnt)` Montgomery multiplications for counts 25, 26, 27 (21, 18, or 15 operations).
- GPU path is *unchanged*, and other chunk widths retain their prior 3/4 threshold.
- New `gpu/rootfind/bench_tail27.cpp` uses repository `fp64.cuh` and mirrored `rf_decompose_thread` prefix/unwind logic. Verifies every length 1 through 32 and tests 2,048 randomized nonzero batches per length.

## Local reproduction and measurements (not actual checked-out headers)

The local sandbox cannot resolve GitHub DNS, so the local benchmark reproduces the current header's Montgomery multiplication and signed Euclid inverse in an isolated C++17 program. Only CI will compile the actual modified header. AMD / GPU throughput is not implied.

Host: Intel Xeon Platinum 8370C VM, Linux 6.18.57, Debian 13, CPU 0 affinity, `-O2 -std=c++17`. Thirteen alternating before/after paired runs, 2,048 batches per arm, each repeated 16 times; each arm used the same seeded inputs. Medians are per inversion core. The local `perf_event_open(PERF_TYPE_SOFTWARE, PERF_COUNT_SW_TASK_CLOCK)` measured CPU task-clock directly. `PERF_TYPE_HARDWARE/PERF_COUNT_HW_CPU_CYCLES` returned ENOENT; `perf` executable is not installed.

| Count / 32 | GCC before task ns | GCC after task ns | GCC speedup | Clang before task ns | Clang after task ns | Clang speedup |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 24 (control) | 484.39 | 490.89 | 0.987x | 481.26 | 465.75 | 1.033x |
| 25 | 560.30 | 489.49 | **1.145x** | 540.45 | 478.72 | **1.129x** |
| 26 | 552.82 | 489.34 | **1.130x** | 540.70 | 490.80 | **1.102x** |
| 27 | 565.92 | 508.10 | **1.114x** | 541.11 | 494.00 | **1.095x** |
| 28 (control) | 563.67 | 559.46 | 1.008x | 564.80 | 529.04 | 1.068x |
| 30 (control) | 552.91 | 557.54 | 0.992x | 532.91 | 535.11 | 0.996x |
| 31 (control) | 544.68 | 549.44 | 0.991x | 538.09 | 540.62 | 0.995x |
| 32 (control) | 545.62 | 550.23 | 0.992x | 542.22 | 538.12 | 1.008x |

Controls vary due to codegen/layout and VM scheduling, not to a different arithmetic path. Each inversion output matched the baseline and multiplied by its input to Montgomery one. The artifact is a **microbenchmark**, not a complete `rf64_decompose` performance claim.

## Other candidates screened, not retained

- `rf_pair_offset(ai,n)` can be rearranged from `2*(ai*n-ai*(ai-1)/2)` to `ai*(2*n-ai+1)` for `0 <= ai <= n <= 2^32-1`. Validated on 32,768 randomized pairs and boundary values, but measured results were inconsistent (one GCC process 0.954x, another 1.117x, Clang 1.134x). The rewrite is **not committed** until performance is reproducible at the actual call site.
- Extending the tail threshold beyond 27 was screened. Some isolated cases improved, but others regressed or fluctuated. Only 25–27 had consistently favorable GCC/Clang paired measurements and were retained.

## CI / next validation gate

```bash
make -C gpu/rootfind test
make -C gpu/rootfind bench-tail27
# On a PMU-capable host:
for arm in tail25-before tail25-after tail26-before tail26-after \
           tail27-before tail27-after tail28-before tail28-after \
           full32-before full32-after; do
  perf stat -r 5 -e cycles,instructions,branches,branch-misses,cache-misses -- \
    taskset -c 0 gpu/rootfind/bench_tail27 --perf-$arm
done
```

Require targeted compilation, original header tests, GPU checks and full-decomposition throughput before merging or claiming a rootfinder pipeline speedup. GitHub-hosted runners can install `perf` but commonly deny hardware PMU with `perf_event_paranoid=4`; if so, these measurements remain software task-clock / wall time only.
