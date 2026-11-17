# cpu profile: Optimized v1 cold n131 d12 uniform, discovery only

- status: **ok**
- command: `/workspace/scratch/trace-fast-baseline-v1 one research/trace_zero_linearized_20261115/workload.json scale-n131-d12-uniform candidate`
- binary: `/workspace/scratch/trace-fast-baseline-v1` sha256 `15b6156c1e95c922…`
- commit: `a14a654255c7` (0 dirty tracked files)
- host: AMD EPYC 9V74 80-Core Processor, 1 logical CPUs, kernel 6.18.57, PMU absent, perf_event_paranoid 2
- valgrind: valgrind-3.24.0
- isolation: none; wall 2.321 s (profiled run: not a timing measurement)
- started: 2026-11-15T22:34:46Z

## Hotspots (callgrind, self Ir)

Total: 282128592 Ir (deterministic, simulated; not cycles).

| # | self % | Ir | function | object |
|---|---:|---:|---|---|
| 1 | 7.69 | 21686318 | `0x00000000000a0fe0` | libc.so.6 |
| 2 | 6.66 | 18794660 | `0x0000000000163280` | libc.so.6 |
| 3 | 5.33 | 15029305 | `full_trace_zero_backend::binary_ecc::f2m::fold_words_clmul_hw` | trace-fast-baseline-v1 |
| 4 | 4.25 | 11989178 | `free` | libc.so.6 |
| 5 | 4.16 | 11732070 | `full_trace_zero_backend::binary_ecc::f2m::mul_words_hw` | trace-fast-baseline-v1 |
| 6 | 4.02 | 11353123 | `full_trace_zero_backend::cryptanalysis::trace_zero_linearized::decompose_trace_zero_linearized` | trace-fast-baseline-v1 |
| 7 | 3.49 | 9832592 | `<full_trace_zero_backend::binary_ecc::f2m::F2mElement>::mul` | trace-fast-baseline-v1 |
| 8 | 3.04 | 8572995 | `<serde_json::value::Value as serde_core::de::Deserialize>::deserialize::<&mut serde_json::de::Deserializer<serde_json::read::SliceRead>>'2` | null |
| 9 | 2.81 | 7937299 | `malloc` | libc.so.6 |
| 10 | 2.58 | 7290377 | `0x00000000000a02e0` | libc.so.6 |
| 11 | 2.43 | 6864031 | `full_trace_zero_backend::binary_ecc::f2m::reduce_words` | trace-fast-baseline-v1 |
| 12 | 2.29 | 6458320 | `0x000000000009ebd0` | libc.so.6 |
| 13 | 2.17 | 6117938 | `full_trace_zero_backend::binary_ecc::f2m::reduce_words` | null |
| 14 | 1.85 | 5210925 | `<full_trace_zero_backend::binary_ecc::f2m::F2mElement>::add` | trace-fast-baseline-v1 |
| 15 | 1.71 | 4824886 | `full_trace_zero_backend::binary_ecc::operation_tally::tick_n` | trace-fast-baseline-v1 |
| 16 | 1.66 | 4684384 | `0x0000000000162880` | libc.so.6 |
| 17 | 1.46 | 4109700 | `<full_trace_zero_backend::binary_ecc::f2m::F2mElement>::mask_in_place` | trace-fast-baseline-v1 |
| 18 | 1.39 | 3931479 | `<serde_json::value::Value as serde_core::de::Deserialize>::deserialize::<&mut serde_json::de::Deserializer<serde_json::read::SliceRead>>'2` | null |
| 19 | 1.39 | 3910690 | `<full_trace_zero_backend::binary_ecc::f2m::F2mElement>::mul::{closure#0}` | trace-fast-baseline-v1 |
| 20 | 1.37 | 3877931 | `<full_trace_zero_backend::binary_ecc::f2m::F2mElement>::square_k_times::{closure#0}` | trace-fast-baseline-v1 |
| 21 | 1.19 | 3356093 | `full_trace_zero_backend::cryptanalysis::trace_zero_linearized::decompose_trace_zero_linearized` | null |
| 22 | 1.13 | 3183488 | `<std::hash::random::DefaultHasher as core::hash::Hasher>::write` | null |
| 23 | 1.12 | 3158965 | `0x000000000009eaf0` | libc.so.6 |
| 24 | 1.08 | 3059428 | `core::str::converts::from_utf8` | null |
| 25 | 1.03 | 2905084 | `full_trace_zero_backend::binary_ecc::f2m::mul_words_into` | trace-fast-baseline-v1 |

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
