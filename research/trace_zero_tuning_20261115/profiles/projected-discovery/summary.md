# cpu profile: Projected lazy cold n131 uniform, unpartitioned discovery profile

- status: **ok**
- command: `/workspace/scratch/trace-projected-baseline one research/trace_zero_projected_20261115/inputs/scale-n131-d12-uniform.json projected-lazy`
- binary: `/workspace/scratch/trace-projected-baseline` sha256 `768193f99099dfa6…`
- commit: `6cd3d7017370` (1 dirty tracked files)
- host: AMD EPYC 9V74 80-Core Processor, 1 logical CPUs, kernel 6.18.57, PMU absent, perf_event_paranoid 2
- valgrind: valgrind-3.24.0
- isolation: none; wall 0.729 s (profiled run: not a timing measurement)
- started: 2026-11-15T23:25:41Z

## Hotspots (callgrind, self Ir)

Total: 81844664 Ir (deterministic, simulated; not cycles).

| # | self % | Ir | function | object |
|---|---:|---:|---|---|
| 1 | 9.86 | 8069255 | `full_trace_zero_backend::cryptanalysis::trace_zero_linearized::decompose_trace_zero_linearized` | trace-projected-baseline |
| 2 | 7.83 | 6409964 | `full_trace_zero_backend::binary_ecc::f2m::fold_words_clmul_hw` | trace-projected-baseline |
| 3 | 4.74 | 3877931 | `<full_trace_zero_backend::binary_ecc::f2m::F2mElement>::square_k_times::{closure#0}` | trace-projected-baseline |
| 4 | 3.67 | 3001369 | `full_trace_zero_backend::cryptanalysis::trace_zero_linearized::decompose_trace_zero_linearized` | null |
| 5 | 2.82 | 2305041 | `free` | libc.so.6 |
| 6 | 2.8 | 2288808 | `<full_trace_zero_backend::binary_ecc::f2m::F2mElement>::add` | trace-projected-baseline |
| 7 | 2.75 | 2249863 | `full_trace_zero_backend::binary_ecc::f2m::reduce_words` | trace-projected-baseline |
| 8 | 2.49 | 2035845 | `full_trace_zero_backend::binary_ecc::f2m::mul_words_hw` | trace-projected-baseline |
| 9 | 2.45 | 2005310 | `full_trace_zero_backend::binary_ecc::f2m::reduce_words` | null |
| 10 | 2.27 | 1857624 | `full_trace_zero_backend::cryptanalysis::trace_zero_linearized::decompose_trace_zero_linearized` | null |
| 11 | 2.2 | 1803177 | `<full_trace_zero_backend::binary_ecc::f2m::F2mElement>::mul` | trace-projected-baseline |
| 12 | 2.2 | 1801612 | `0x00000000000a0fe0` | libc.so.6 |
| 13 | 1.79 | 1467268 | `malloc` | libc.so.6 |
| 14 | 1.41 | 1153844 | `0x0000000000162880` | libc.so.6 |
| 15 | 1.34 | 1100460 | `<full_trace_zero_backend::binary_ecc::f2m::F2mElement>::mask_in_place` | trace-projected-baseline |
| 16 | 1.21 | 993496 | `<full_trace_zero_backend::binary_ecc::f2m::F2mElement>::half_trace` | null |
| 17 | 1.17 | 955567 | `full_trace_zero_backend::cryptanalysis::trace_zero_linearized::decompose_trace_zero_linearized` | null |
| 18 | 1.14 | 930609 | `<full_trace_zero_backend::binary_ecc::f2m::F2mElement>::half_trace` | trace-projected-baseline |
| 19 | 1.07 | 873240 | `<alloc::string::String as core::iter::traits::collect::FromIterator<char>>::from_iter::<core::iter::adapters::filter::Filter<core::str::iter::Chars, <full_trace_zero_backend::binary_ecc::f2m::F2mElement>::from_hex::{closure#0}>>` | null |
| 20 | 1.02 | 834529 | `full_trace_zero_backend::cryptanalysis::trace_zero_linearized::decompose_trace_zero_linearized` | null |
| 21 | 1 | 815633 | `<full_trace_zero_backend::binary_ecc::f2m::F2mElement>::square_k_times` | trace-projected-baseline |
| 22 | 0.99 | 811250 | `full_trace_zero_backend::cryptanalysis::trace_zero_linearized::decompose_trace_zero_linearized` | null |
| 23 | 0.89 | 731147 | `<full_trace_zero_backend::binary_ecc::f2m::F2mElement>::add` | null |
| 24 | 0.85 | 697680 | `<std::hash::random::DefaultHasher as core::hash::Hasher>::write` | null |
| 25 | 0.85 | 692139 | `<full_trace_zero_backend::binary_ecc::f2m::F2mElement>::square::{closure#0}` | trace-projected-baseline |

## Files

- `annotate-inclusive.txt`
- `annotate.txt`
- `callgrind.out`
- `command.stderr`
- `command.stdout`
- `hotspots.json`
- `hotspots.svg`
- `manifest.json`

Raw profiler output (perf.data, *.ncu-rep, *.nsys-rep, callgrind.out) is large: push it with `crypto store push` (AGENTS.md section 13) rather than committing it.
