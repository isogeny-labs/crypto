# optlevel in Rust: the port from Python, with its evidence (2026-11-17)

`tools/optlevel/` was a Python CLI (`optlevel.py`, `score.py`, `plot.py`,
`test_optlevel.py`) around C kernels. It is now a Rust crate with the same
subcommands, flags and file formats; the four `.py` files are deleted. This
page records what was measured to justify that, on the one host available.

Every number below is a producer measurement on one shared cloud VM, tier
`T-timing`, and is an unverified assertion `[pcc:unverified]`. Nothing here is
a speed claim about research code, a benchmark row or a leaderboard row, and no
`bench.record/v1` was opened: the runs compare a measuring tool with its
predecessor and calibration kernels with each other. No cloud resource was
used and no pod was contacted.

## 1. Result

| question | answer |
|:--|:--|
| What is Rust now? | The whole tool: command line, host detection, run and pin logic, `perf`/`ncu`/`nsys`/`nvidia-smi` ingestion, scoring, schema validation, SVG, and the CPU calibration and control kernels (`src/kernels.rs`, inside the binary). |
| What is not Rust? | `kernels/gpu_calib.cu` (CUDA, built with `nvcc` at first use, as before). `kernels/optk.c`, kept as the reference the Rust kernels were ported from and built only by `calibrate --c-kernels`. `run.sh` (ten lines: build if stale, exec). |
| Do the committed records still hold? | Yes. All 17 committed records validate. Re-scoring each of the 15 committed measurements that have their files reproduces the committed record exactly (every value; 29 attainments bit for bit); the two committed roofline SVGs and the two `controls.md` tables come out byte for byte (`tests/golden.rs`). |
| Python against Rust on live controls? | Same control binaries, same calibration, interleaved: single-thread attainments differ by 0.01% to 5.7% (median 1.3%), all inside the A/A noise floor of either tool (4% to 42% on this host), and no larger than each tool's difference from itself between rounds (0.3% to 7.1%). Roofs, intensity, binding, vector share, flags, grades and limiting factors are identical. |
| C against Rust kernels? | Rust is higher in seven of nine classes (1.03 to 1.82 times on the ceiling) and level, inside the spread, in two (`fma_f64` 0.99 and 1.03; `modmuladd_p62` 0.99 and 0.97). Bandwidth is level (0.97 to 1.10). |
| Tool overhead? | With `kernel_seconds` in the kernel's line the rate is `ops / kernel_seconds`: the tool adds nothing. Otherwise the tool's wall clock per run of `/bin/true` is 0.9 to 1.0 ms unpinned and 1.1 to 1.5 ms pinned, against 0.7 to 0.8 ms for a bare `posix_spawn` + `wait` in C and 2.5 to 2.7 ms pinned in the Python tool (minima). |
| Tests | 64 (`cargo test --release`), against 33 before. |

The most consequential finding is not about the language. The C kernels read
the clock after every pass over an 8–16 KiB array, a pass of 55 to 100 ns for
the fast loops. On this host that read costs the in-cache SIMD classes 29% to
47% of their rate.
Every ceiling the Python tool measured for those classes was low by 1.4 to 1.8
times, and the positive control could not show it, because the control was the
same kernel. Section 3 has the numbers.

## 2. Host and conditions

| | |
|:--|:--|
| CPU | Intel Xeon (Sapphire Rapids class) @ 2.10 GHz nominal, KVM guest, 4 vCPUs, SMT off; the host of `research/optlevel_20261117/` (fingerprint `44113cdb2d87e834`, computed identically by both tools) |
| PMU, GPU | none (`T-timing`; `gpu` subcommands exit 3) |
| compilers | rustc 1.97.0, gcc 13.3.0 |
| binaries | `optlevel` SHA-256 `4fe440ae…` (rebuilt once from the committed source: same hash); Python and C inputs in `inputs.sha256` |
| load | nothing else was started. 1-minute load average 0.3 to 2.2 during the runs (the runs themselves: four pinned copies part of the time); CPU used by everything else, from `/proc/stat` per run: 0.01 to 0.05 cores; steal 0.014 to 0.016 cores |
| noise | large. The effective clock moved between 2.08 and 2.88 GHz across calibrations (`CLOCK_UNSTABLE` in three of four), and single-thread A/A noise floors of in-cache kernels were 4% to 42%. Every table gives spreads; maxima are the tool's ceiling rule, medians are beside them |

## 3. C against Rust calibration kernels

Method: `optlevel calibrate --c-kernels` runs, inside each repeat, every
variant of a class once, single-thread, pinned to the same CPU: the three C
builds of `kernels/optk.c` (`c-native`, `c-w512`, `c-scalar`, the flags of the
Python tool) and the three Rust variants (`avx512`, `avx2`, `scalar`). So the
arms are interleaved by construction. Two independent runs: run 1 with 7
repeats of 0.3 s (`ab/ab.calibration.json`), run 2 with 3 repeats of 0.5 s
(`equiv/rs.calibration.json`). Rates in G operations/s; spread = (max − min) /
median over the repeats of that variant.

Run 1 (7 repeats; load average 1.6 before, 2.2 after; other CPU 0.05 cores):

| class | best C build | C max | C median | C spread | best Rust variant | Rust max | Rust median | Rust spread | Rust/C max | Rust/C median |
|:--|:--|--:|--:|--:|:--|--:|--:|--:|--:|--:|
| `muladd_u32` | muladd_u32@c-w512 | 17.5 | 16.4 | 25% | muladd_u32@avx512 | 26.2 | 25.5 | 15% | 1.50 | 1.55 |
| `muladd_u64` | muladd_u64@c-w512 | 7.47 | 7.18 | 11% | muladd_u64@avx512 | 10.3 | 10.0 | 21% | 1.38 | 1.40 |
| `mulwide_u64` | mulwide_u64@c-scalar | 1.79 | 1.75 | 20% | mulwide_u64@scalar | 1.88 | 1.83 | 49% | 1.05 | 1.05 |
| `modmuladd_p31` | modmuladd_p31@c-w512 | 2.11 | 2.05 | 18% | modmuladd_p31@avx2 | 3.84 | 3.73 | 7% | 1.82 | 1.83 |
| `modmuladd_p62` | modmuladd_p62@c-native | 0.695 | 0.655 | 20% | modmuladd_p62@scalar | 0.685 | 0.628 | 30% | 0.99 | 0.96 |
| `xor_u64` | xor_u64@c-w512 | 9.69 | 9.43 | 19% | xor_u64@avx512 | 17.6 | 16.7 | 14% | 1.81 | 1.77 |
| `popcnt_u64` | popcnt_u64@c-w512 | 8.82 | 8.74 | 29% | popcnt_u64@avx512 | 12.9 | 12.3 | 14% | 1.46 | 1.40 |
| `fma_f64` | fma_f64_reg@c-native | 43.0 | 42.1 | 7% | fma_f64_reg@avx512 | 42.7 | 41.3 | 11% | 0.99 | 0.98 |
| `ptr_hop` | chase@c-scalar | 0.543 | 0.523 | 8% | chase@avx512 | 0.572 | 0.561 | 6% | 1.05 | 1.07 |
| bandwidth, 1 core, GB/s | xor_u64@c-native | 19.6 | 19.2 | 6% | xor_u64 | 19.0 | 18.6 | 5% | 0.97 | 0.97 |
| bandwidth, 2 cores | xor_u64@c-native | 38.3 | 37.0 | 21% | xor_u64 | 38.6 | 38.1 | 4% | 1.01 | 1.03 |
| bandwidth, 4 cores | xor_u64@c-native | 73.4 | 72.7 | 25% | xor_u64 | 75.2 | 74.1 | 19% | 1.02 | 1.02 |

Run 2 (3 repeats; load average 1.7 before, 2.0 after; other CPU 0.01 cores):

| class | best C build | C max | C median | C spread | best Rust variant | Rust max | Rust median | Rust spread | Rust/C max | Rust/C median |
|:--|:--|--:|--:|--:|:--|--:|--:|--:|--:|--:|
| `muladd_u32` | muladd_u32@c-w512 | 15.4 | 13.1 | 33% | muladd_u32@avx512 | 21.9 | 20.8 | 23% | 1.42 | 1.59 |
| `muladd_u64` | muladd_u64@c-w512 | 7.44 | 7.18 | 30% | muladd_u64@avx512 | 10.2 | 9.72 | 44% | 1.37 | 1.35 |
| `mulwide_u64` | mulwide_u64@c-native | 1.76 | 1.68 | 18% | mulwide_u64@scalar | 1.81 | 1.80 | 15% | 1.03 | 1.07 |
| `modmuladd_p31` | modmuladd_p31@c-w512 | 2.09 | 2.08 | 0% | modmuladd_p31@avx2 | 3.75 | 3.02 | 41% | 1.80 | 1.45 |
| `modmuladd_p62` | modmuladd_p62@c-w512 | 0.713 | 0.675 | 38% | modmuladd_p62@scalar | 0.692 | 0.659 | 12% | 0.97 | 0.98 |
| `xor_u64` | xor_u64@c-w512 | 9.34 | 9.24 | 3% | xor_u64@avx512 | 17.0 | 17.0 | 3% | 1.82 | 1.84 |
| `popcnt_u64` | popcnt_u64@c-w512 | 8.44 | 8.11 | 29% | popcnt_u64@avx512 | 11.4 | 11.2 | 6% | 1.35 | 1.39 |
| `fma_f64` | fma_f64_reg@c-w512 | 41.3 | 34.2 | 21% | fma_f64_reg@avx512 | 42.6 | 40.3 | 15% | 1.03 | 1.18 |
| `ptr_hop` | chase@c-native | 0.536 | 0.526 | 6% | chase@avx2 | 0.571 | 0.567 | 4% | 1.06 | 1.08 |
| bandwidth, 1 core, GB/s | xor_u64@c-native | 18.6 | 18.1 | 9% | xor_u64 | 19.3 | 18.8 | 3% | 1.04 | 1.03 |
| bandwidth, 2 cores | xor_u64@c-native | 36.6 | 34.1 | 11% | xor_u64 | 37.7 | 37.3 | 1% | 1.03 | 1.09 |
| bandwidth, 4 cores | xor_u64@c-native | 65.2 | 61.5 | 60% | xor_u64 | 71.7 | 70.9 | 38% | 1.10 | 1.15 |

**Where the difference comes from.** `ab_kernels.sh` ran every C build and
every Rust variant, plus each Rust variant with `--timer-every-pass` (the C
kernels' protocol), 5 interleaved rounds of 0.3 s pinned to one CPU
(`ab/incache.tsv`, load average 1.1 to 2.2). Maxima over rounds, G ops/s:

| kernel | C best | Rust, clock read every pass | Rust, clock read about every 50 µs | Rust every-pass / C | batched / every-pass |
|:--|--:|--:|--:|--:|--:|
| `muladd_u32` | 17.5 (c-w512) | 16.9 (avx512) | 26.3 (avx512) | 0.97 | 1.55 |
| `muladd_u64` | 7.13 (c-w512) | 6.75 (avx512) | 10.1 (avx512) | 0.95 | 1.50 |
| `modmuladd_p31` | 2.11 (c-w512) | 3.51 (avx2) | 3.75 (avx2) | 1.67 | 1.07 |
| `xor_u64` | 9.58 (c-w512) | 10.6 (avx512) | 16.6 (avx512) | 1.11 | 1.56 |
| `popcnt_u64` | 8.86 (c-w512) | 9.03 (avx512) | 12.7 (avx512) | 1.02 | 1.40 |
| `fma_f64` (array loop) | 10.9 (c-w512) | 9.87 (avx512) | 18.4 (avx512) | 0.90 | 1.87 |
| `fma_f64_reg` | 42.7 (c-w512) | 42.4 (avx512) | 42.5 (avx512) | 0.99 | 1.00 |
| `mulwide_u64` | 1.80 (c-w512) | 1.75 (scalar) | 1.79 (scalar) | 0.97 | 1.02 |
| `modmuladd_p62` | 0.701 (c-native) | 0.670 (scalar) | 0.680 (scalar) | 0.96 | 1.01 |
| `chase` (in L1) | 0.557 (c-native) | 0.552 (avx512) | 0.578 (avx512) | 0.99 | 1.05 |
| `horner_p31` (control) | 0.159 (c-scalar) | 0.152 (scalar) | 0.152 (avx512) | 0.96 | 1.00 |
| `clock` | 2.87 (c-scalar) | 2.89 (avx2) | 2.91 (scalar) | 1.01 | 1.01 |

Readings:

- With the same timing protocol the Rust kernels are within about 10% of the
  C ones in either direction, except `modmuladd_p31`, where the Rust loop is
  1.67 times faster (LLVM's AVX2 code for the Montgomery row operation, with
  the reductions written as `min(r, r − p)`, against gcc's AVX-512 code).
- Reading the clock once per 50 µs instead of once per pass raises the fast
  in-cache loops by 1.40 to 1.87 times and leaves the slow ones (1 to 4 ns per
  operation, or a 25 µs pass) where they were. That is the per-pass
  `clock_gettime` and the loop around it: 33 to 48 ns per pass in this guest,
  from the two Rust columns, against a pass of 55 to 100 ns. The C kernels
  have this cost in every ceiling they reported.
- The Rust arrays are page-aligned like the C ones (`posix_memalign(4096)`).
  A first Rust version used ordinary `Vec` allocations and lost 15% on
  `fma_f64` and `xor_u64` to cache-line-straddling 64-byte loads; aligning
  fixed it.

**Decision per class.** The ceiling is the best of the builds, so a slower
kernel would silently raise every attainment.

| class | kept | why |
|:--|:--|:--|
| `muladd_u32`, `muladd_u64`, `modmuladd_p31`, `xor_u64`, `popcnt_u64` | Rust | 1.35 to 1.82 times the C ceiling in both runs |
| `mulwide_u64`, `ptr_hop` | Rust | 1.03 to 1.06 times in both runs; the generated loops are the same instructions as gcc's |
| `fma_f64` | Rust | 0.99 and 1.03: level. Both are 16 register-resident `zmm` accumulators (explicit `_mm512_fmadd_pd` in Rust; autovectorized Rust reached only 2 G/s and was replaced) |
| `modmuladd_p62` | Rust, **unresolved at this noise** | 0.99 and 0.97 on maxima, inside spreads of 12% to 38%; a separate six-round run (not kept as a file) gave maxima of 0.701 G/s for this kernel, 0.702 with the reduction spelled as subtract-and-select, and 0.698 and 0.695 for C `native` and `w512`. The loops are the same 3 multiplies per element. A deficit of up to 3% cannot be excluded on this host |
| bandwidth (triad, xor, popcount streams) | Rust | 0.97 to 1.10: level |
| scalar ceilings (diagnosis only) | Rust | at or above C (0.99 to 1.11) except `popcnt_u64@scalar`: 1.87 against 2.28 G/s, the same seven instructions, cause not found. It lowers no ceiling; it raises that class's reported SIMD headroom by 22% |

`kernels/optk.c` stays in the tree and `calibrate --c-kernels` adds its three
builds to the best-of, so on any host the ceiling can be made at least the C
one at the cost of a C compiler and about a minute. That is the remedy for the
unresolved class and for CPU families nobody has compared yet (this A/B is one
Sapphire Rapids guest; no AMD, no AVX2-only and no ARM host was available).

**Consequence for records already committed.** A calibration taken with the
Python tool has `muladd_u32`, `muladd_u64`, `modmuladd_p31`, `xor_u64` and
`popcnt_u64` ceilings that are 1.4 to 1.8 times too low. On this host the
vectorized control (the C `modmuladd_p31` loop) scores 0.83 to 0.89 against
the Python calibration (section 4) and would score about 0.5 against the Rust
one; the Rust `modmuladd_p31` control scores 0.92 and 0.98 against the Rust
calibration (`controls/controls.md`). Attainments of in-cache SIMD kernels are
not comparable across the two calibrations. `fma_f64`, `modmuladd_p62`,
`mulwide_u64`, `ptr_hop` and every memory-bound record are unaffected (ratios
0.97 to 1.10). The two real-kernel records in `research/optlevel_20261117/`
are `fma_f64` records and keep their meaning.

Controls with the Rust tool and Rust kernels (`controls/`, 5 A/B pairs of 1 s,
calibration flags `VIRTUALIZED`, other CPU 0.01 cores):

| control | A single [low, high] | A full | score | grade | A/A noise | limiting factor |
|:--|:--|--:|--:|:--|--:|:--|
| positive-simd-incache | 0.918 [0.853, 1.057] | 0.989 | 0.953 | A | 0.198 | none above threshold |
| scalar-incache | 0.170 [0.157, 0.187] | 0.177 | 0.173 | D | 0.151 | core bound / not vectorized (static vector share 0.00; SIMD ceiling 5.7x scalar) |
| serial-dependency | 0.041 [0.041, 0.043] | 0.043 | 0.042 | E | 0.024 | core bound / not vectorized |
| stream-daxpy | 1.002 [0.878, 1.028] | 0.974 | 0.988 | A | 0.138 | at the memory-bandwidth roof |
| pointer-chase | 0.021 [0.020, 0.022] | 0.022 | 0.022 | E | 0.083 | memory latency bound (latency attainment 1.00) |
| aa-repeat | 0.981 [0.940, 1.063] | 1.014 | 0.997 | A | 0.104 | none above threshold |

They separate as `research/optlevel_20261117/` section 3 expects. The scalar
and serial controls are lower than there (0.17 and 0.04 against 0.32 and 0.07)
because the ceiling they are divided by is 1.8 times higher.
`controls/controls-roofline.svg` is the roofline; no PNG or PDF was rendered
(no SVG renderer on this host).

## 4. Python against Rust on the controls

Run before the Python was deleted (`equivalence.sh`, output in `equiv/`). To
make the tool the only variable, both tools measured the **same** control
binaries (the C kernels the Python tool had built) and both scored against the
**same** calibration (the Python tool's, `equiv/py.calibration.json`). Five
controls, two rounds; in each round and control the two tools ran back to
back, Python first in round 1 and Rust first in round 2; 5 A/B pairs of 1 s
and 5 runs of 4 pinned copies each. Load average before each measurement is in
`equiv/run.log` (1.6 to 2.2, the runs' own); CPU used by everything else was
0.01 to 0.02 cores in every record.

| control | round | A single: Python | Rust | Rust/Python − 1 | A full: Python | Rust | Rust/Python − 1 | A/A noise: Python | Rust |
|:--|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| positive-simd-incache | 1 | 0.8918 | 0.8635 | −3.2% | 0.9707 | 0.9458 | −2.6% | 0.310 | 0.245 |
| positive-simd-incache | 2 | 0.8283 | 0.8394 | +1.3% | 0.8916 | 0.9374 | +5.1% | 0.349 | 0.395 |
| scalar-incache | 1 | 0.3030 | 0.3037 | +0.2% | 0.2767 | 0.3040 | +9.9% | 0.422 | 0.406 |
| scalar-incache | 2 | 0.2912 | 0.2911 | −0.01% | 0.2787 | 0.2942 | +5.6% | 0.341 | 0.317 |
| serial-dependency | 1 | 0.0728 | 0.0712 | −2.3% | 0.0787 | 0.0774 | −1.6% | 0.042 | 0.074 |
| serial-dependency | 2 | 0.0731 | 0.0740 | +1.3% | 0.0796 | 0.0793 | −0.4% | 0.040 | 0.041 |
| stream-daxpy | 1 | 0.9741 | 1.0297 | +5.7% | 0.9805 | 0.9898 | +0.9% | 0.188 | 0.141 |
| stream-daxpy | 2 | 0.9939 | 1.0062 | +1.2% | 1.0215 | 1.0334 | +1.2% | 0.099 | 0.212 |
| pointer-chase | 1 | 0.0211 | 0.0212 | +0.6% | 0.0227 | 0.0230 | +1.2% | 0.161 | 0.169 |
| pointer-chase | 2 | 0.0213 | 0.0210 | −1.7% | 0.0233 | 0.0229 | −1.8% | 0.114 | 0.166 |

- All 20 differences are inside the larger of the two A/A noise floors
  (`within_noise` in `equiv/summary.tsv`). The floors are wide on this host,
  so the sharper comparison is with each tool against itself: between rounds
  the Python tool's single-thread attainment moved by 0.3% to 7.1% and the
  Rust tool's by 1.0% to 4.1%. The tool-to-tool differences (0.01% to 5.7%,
  median 1.3%) are the same size, with both signs.
- Everything that does not depend on a timed run is identical between the two
  tools' records in every control: host fingerprint, roofline block (roofs,
  bound, binding, bandwidth), intensity block, static vector share, flags,
  letter grades and the limiting-factor sentence (compared with numbers
  masked). The record key sets are identical; the Rust `host` block adds
  `affinity_cpus`, `cpu_quota_cores`, `cpu_quota_source`, `usable_cpus` and
  `usable_cpus_basis`.
- Calibration path: the Python tool driving its C builds against the Rust
  tool driving its own compile of the same C source and flags
  (`equiv/calibrate_c_builds_py_vs_rs.tsv`): per-class maxima differ by −8.9%
  to +7.9% (median +1.0%), inside the spreads of section 3; bandwidth 18.5,
  37.2, 68.4 GB/s (Python) against 19.3, 37.7, 71.8 (Rust) at 1, 2 and 4
  cores.
- Offline, with no timing in the way, the two tools agree exactly: the golden
  tests of section 1.

## 5. Overhead of the tools themselves

`overhead.sh` (output `overhead/overhead.tsv`; Python and Rust interleaved) and
`overhead/spawn_baseline.c` (a bare C `posix_spawn` or `fork`/`exec` of
`/bin/true` with `waitpid`, 200 runs; `overhead/spawn_baseline.txt`).

| quantity | Python tool | Rust tool | baseline |
|:--|--:|--:|--:|
| `validate` of one record, wall, median of 20 | 64.5 ms | 4.4 ms (includes the shell's own fork and two `date` calls) | |
| tool's `wall_s` per run of `/bin/true`, pinned, median (min) of 10, three rounds | 3.1 (2.5 to 2.7) ms | 1.3 to 1.7 (1.1 to 1.5) ms | |
| the same, `--no-pin` | not measured | 1.0 to 1.4 (0.9 to 1.0) ms | bare `posix_spawn` + `wait`: 0.96 to 1.19 (0.73 to 0.83) ms; `fork` + `exec`: 1.03 to 1.10 (0.86 to 0.90) ms |
| 50 ms kernel: `wall_s − kernel_seconds` per run, median | 3.8 ms | 1.8 ms | (contains the kernel's own start-up, allocation and warm-up pass) |
| 50 ms kernel: `startup_share` | 0.071 | 0.033 | |
| 50 ms kernel: measured rate, median of three medians [range] | 1.58 [1.54, 1.75] G/s | 1.71 [1.26, 1.72] G/s | |
| `measure --repeat 5` of that kernel (10 runs, 500.0 ms of kernel time): total wall | 634 ms | 537 ms | 500 ms |

- **A kernel that reports `kernel_seconds` sees no tool overhead**: its rate
  is `ops / kernel_seconds`, measured inside the kernel. The two tools' rates
  for the 50 ms kernel overlap.
- A kernel that reports no `kernel_seconds` is charged the tool's wall clock
  (`WALL_INCLUDES_STARTUP`). There the Rust tool adds about 0.1 to 0.2 ms over
  a bare `posix_spawn` unpinned and 0.3 to 0.8 ms pinned (the pin needs a
  `fork` and moves the child to another CPU); the Python tool adds about 1.8
  ms. A graded record needs a timed region of at least 0.5 s, of which 0.6 ms
  is 0.1%.
- Everything outside the kernels' timed regions, per measured run (tool
  start, host probe, bookkeeping, and the kernel's own start-up): 3.7 ms with
  the Rust tool against 13.4 ms with the Python tool.

## 6. The three findings of the pod runs, fixed

**6.1 CPU count.** Before: `os.cpu_count()` and the affinity mask only; on the
GPU pod "all 48 cores" for a container allotted 6 vCPUs
(`research/hky_pod_runs_20261117/RESULTS.md`). Now the tool reads cgroup v2
`cpu.max` and cgroup v1 `cpu.cfs_quota_us / cpu.cfs_period_us` at the
process's cgroup and its ancestors as well, and records the affinity count,
the quota with its source, and the resulting `usable_cpus`; `calibrate`,
`controls` and the "all N cores" text use `usable_cpus`.

Shown here on a real cgroup (`cgroup/`): a v1 cpu cgroup with a quota of two
CPUs was created on this host, both tools ran `probe` inside it, and it was
removed.

| | Python tool | Rust tool, same cgroup | Rust tool, no quota |
|:--|:--|:--|:--|
| `logical_cpus` | 4 | 4 | 4 |
| `allowed_cpus` | 4 | 4 | 4 |
| `cpu_quota_cores` | (no such field) | 2.0, "cgroup v1 cpu.cfs_quota_us 200000 / cpu.cfs_period_us 100000 (sys/fs/cgroup/cpu/optlevel-rs-test)" | null |
| CPUs calibrate would scale to | 4 | 2 | 4 |
| `fingerprint` | `44113cdb2d87e834` | `44113cdb2d87e834` | `44113cdb2d87e834` |

The pod's own layout could not be reproduced (its `cgroup_cpu_max` was
recorded as null, so it was cgroup v1 or a v2 group below the namespace root).
`tests/fixes.rs` covers v2 at the root (`600000 100000` with 48 CPUs in the
mask gives 6), v2 with the tightest ancestor, v1, v1 with a host path the
container cannot see, no quota, and `OPTLEVEL_CPUS=N` for a container whose
allotment shows in neither interface. Whether the Runpod container exposes its
6-vCPU quota through either interface is **not known**; if it does not,
`OPTLEVEL_CPUS=6` is the remedy and `probe` shows which case applies.

**6.2 GPU memory roof.** Before: `gpu calibrate` put the triad only into the
bandwidth figure, `gpu measure` ignored the kernel's own `bytes_declared`, and
the pod run had to relabel the triad `fma_f64` with `--bytes-per-op 24` by
hand. Now a GPU calibration has a `triad` class (compute roof = the `fma_f64`
ceiling, memory roof = bandwidth / declared bytes), `gpu measure` takes the
declared bytes from the kernel's `optlevel.ops/v1` lines as `measure` does on
CPU (`--bytes-per-op` overrides), and a `triad` measurement scored against an
old calibration uses the `fma_f64` ceiling with the flag
`CEILING_CLASS_ALIAS_FMA_F64`. Evidence here, without a GPU: the committed pod
measurement `gpu3_triad.measure.json`, with its class set back to `triad`,
scores to the same attainment and roofline as the committed hand-relabelled
record (`gpu_triad/`: the Rust tool prints attainment 1.000 on the memory
roof, exit 0; the Python tool ends in an uncaught `KeyError: "calibration has
no ceiling for operation class 'triad'; classes: ['fma_f64', 'muladd_u32',
'muladd_u64', 'popcnt_u64', 'xor_u64']"`, exit 1). `tests/fixes.rs`:
`triad_scores_against_a_calibration_written_before_the_class_existed`,
`gpu_calibration_has_a_triad_class_and_score_uses_declared_bytes`,
`gpu_path_end_to_end_on_stub_tools`.

**6.3 `gpu probe`.** Before: it printed the device list and three tool paths;
`tools/profile.sh ncu` then exited 2 for want of `gawk` and, with `gawk`, was
refused by the driver (`results/ncu_muladd_u32.log`). Now `gpu probe` (and
`probe`) names missing helper programs on stderr and in the JSON, runs the
control kernel once under `ncu`, and on `ERR_NVGPUCTRPERM` reports "counters
withheld by the driver" with `gpu_tier: T-gpu-timing`. On this host, which has
no `gawk`: `optlevel: helper tool gawk is missing (tools/profile.sh needs it
for ncu and nsys)`. The classifier is tested on the text `ncu` printed on the
pod (`results/gpu3_probe.json`), and the subcommand end to end against stub
`nvidia-smi`, `nvcc` and `ncu` programs
(`gpu_probe_reports_missing_helpers_and_withheld_counters`).

## 7. What was wrong in the Python tool

Found while porting; each is fixed in the Rust tool unless it says otherwise.

1. **Ceilings low by 1.4 to 1.8 times for in-cache SIMD classes**: the C
   kernels read the clock after every pass (section 3). The controls could not
   detect it because the positive control was the same kernel.
2. The three pod findings of section 6.
3. `controls --rescore` disassembled the measured binaries again, so it failed
   once the compiler cache was gone. It now reuses the recorded vector share.
4. With `--scale copies:k`, the copies' pipes were drained one after another;
   a copy that wrote more than a pipe buffer would block until the copies
   before it had exited, inside the timed region. Output now goes to files.
5. Child CPU time came from `os.times()`, which counts in 10 ms ticks, so
   `cpu_s`, `cpu_utilisation` and `other_cpu_cores` of a short run were coarse.
   It is `getrusage` now (microseconds).
6. `gpu calibrate` rebuilt `gpu_calib.cu` on every call; it is cached by
   source hash.
7. `gpu measure` did not flag a command that exited non-zero
   (`COMMAND_FAILED` on CPU only), and an operation class missing from the
   calibration ended `score` in a Python traceback instead of a message.
8. Not fixed, recorded: the timing tier's limiting-factor sentence for a
   kernel whose counter covers a small part of the timed region is still
   "core bound" (`research/optlevel_20261117/README.md` section 4); the Rust
   scalar `popcnt_u64` kernel is 18% below the C one (section 3).

The scoring arithmetic itself had no error that the port found: 33 of 33
ported tests pass unchanged in meaning, and the golden tests reproduce every
committed record.

## 8. Not tested here

| what | why | what covers it instead |
|:--|:--|:--|
| `T-pmu` path (`perf stat` groups, IPC, misses, `llc-miss` traffic, top-down) | no PMU, no `perf` on this host | parsers and tier logic on synthetic `perf` output (`perf_csv`, `pmu_summary`, `topdown_normalise`, `pmu_tier_uses_measured_traffic_and_topdown`, `perf_probe_classification`); never run on a PMU host, in either language |
| GPU path on a GPU (`gpu probe`, `gpu calibrate`, `gpu measure`, `nvcc` build, `ncu` check) | no GPU, no `nvcc` | stub tools end to end; the committed pod files for ingestion and scoring. `gpu probe`'s `ncu --launch-count 1` invocation has never met a real `ncu` |
| `gpu_calib.cu` after its comment-only edit | no `nvcc` | the code lines are unchanged from the version that ran on the pod |
| other CPUs (AMD, AVX2-only, ARM) | one host | `--c-kernels` on first use there; the `base` variants compile for any target but ran nowhere |
| cgroup v2 on a live system, and the pod's layout | this host is cgroup v1 | fixture trees |
| the edited pod job scripts | no pod was contacted | `bash -n`, and the `jq` filters against committed result files |
| a quiet host | shared VM | spreads are stated; maxima and medians are both given |

## 9. Commands

```sh
export CARGO_TARGET_DIR=$PWD/tools/optlevel/target
cargo build --release --manifest-path tools/optlevel/Cargo.toml
cargo test  --release --manifest-path tools/optlevel/Cargo.toml          # 64 tests
B=$PWD/tools/optlevel/target/release/optlevel; R=research/optlevel_rs_20261117

# section 4, then 3, 5 and the controls (about 27 minutes; the Python tool must still exist: commit 833a35d21e)
$R/equivalence.sh tools/optlevel/optlevel.py $B $R/equiv 2 5 1.0
$R/equivalence_summary.sh $R/equiv > $R/equiv/summary.tsv
OPTLEVEL_CACHE=$R/equiv/cache $B calibrate --c-kernels --repeat 7 --seconds 0.3 --out $R/ab/ab.calibration.json
$R/ab_from_calibration.sh $R/ab/ab.calibration.json > $R/ab/ab.calibration.summary.tsv
$R/ab_from_calibration.sh $R/equiv/rs.calibration.json > $R/equiv/rs.calibration.ab.tsv
$R/ab_kernels.sh $B <dir with optk-native, optk-w512, optk-scalar> $R/ab/incache.tsv 5 0.3
$R/ab_summary.sh $R/ab/incache.tsv > $R/ab/incache.summary.tsv
$R/overhead.sh tools/optlevel/optlevel.py $B <optk-native> research/optlevel_20261117/controls-run2/stream-daxpy.record.json $R/overhead/overhead.tsv 20
gcc -O2 $R/overhead/spawn_baseline.c -o spawn && ./spawn 0 && ./spawn 1
$B controls --out $R/controls --repeat 5

# section 6.1 (root; removes the cgroup again)
G=/sys/fs/cgroup/cpu/optlevel-rs-test; mkdir $G; echo 100000 > $G/cpu.cfs_period_us; echo 200000 > $G/cpu.cfs_quota_us
sh -c "echo \$\$ > $G/tasks; exec $B probe" > $R/cgroup/rust_probe_quota2.json; rmdir $G
```

The `equiv/cache` directory (the C builds both tools measured) was not
committed; `inputs.sha256` has the hashes of the Rust binary, the two Python
files and `optk.c` that the runs used. `ab_kernels.sh` first ran with an
earlier build of the kernels; those rows are not kept. The numbers in section
3 are all from the committed binary.
