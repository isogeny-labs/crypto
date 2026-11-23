# cpu profile: Cold full-query baseline, n131 d12 uniform; discovery profile only

- status: **ok**
- command: `/workspace/scratch/trace-baseline-profile one research/trace_zero_linearized_20261115/workload.json scale-n131-d12-uniform baseline`
- binary: `/workspace/scratch/trace-baseline-profile` sha256 `32b951744bb13211…`
- commit: `a6505f782c40` (1 dirty tracked files)
- host: AMD EPYC 9V74 80-Core Processor, 1 logical CPUs, kernel 6.18.57, PMU absent, perf_event_paranoid 2
- valgrind: valgrind-3.24.0
- isolation: none; wall 3.094 s (profiled run: not a timing measurement)
- started: 2026-11-15T20:55:32Z

## Hotspots (callgrind, self Ir)

Total: 612766243 Ir (deterministic, simulated; not cycles).

| # | self % | Ir | function | object |
|---|---:|---:|---|---|
| 1 | 9.27 | 56782427 | `full_trace_zero_backend::binary_ecc::f2m::fold_words_clmul_hw` | trace-baseline-profile |
| 2 | 6.8 | 41695721 | `0x00000000000a0fe0` | libc.so.6 |
| 3 | 6.6 | 40449501 | `<full_trace_zero_backend::binary_ecc::f2m::F2mElement>::square_k_times::{closure#0}` | trace-baseline-profile |
| 4 | 4.44 | 27192935 | `<num_bigint::biguint::BigUint as core::ops::bit::BitXorAssign<&num_bigint::biguint::BigUint>>::bitxor_assign` | null |
| 5 | 4.2 | 25756651 | `free` | libc.so.6 |
| 6 | 3.63 | 22216743 | `<num_bigint::biguint::BigUint as core::ops::bit::BitXorAssign<&num_bigint::biguint::BigUint>>::bitxor_assign` | null |
| 7 | 3.6 | 22049367 | `full_trace_zero_backend::binary_ecc::f2m::reduce_words` | trace-baseline-profile |
| 8 | 3.21 | 19652694 | `full_trace_zero_backend::binary_ecc::f2m::reduce_words` | null |
| 9 | 2.78 | 17026134 | `malloc` | libc.so.6 |
| 10 | 2.42 | 14811162 | `<num_bigint::biguint::BigUint as core::ops::bit::BitXorAssign<&num_bigint::biguint::BigUint>>::bitxor_assign` | trace-baseline-profile |
| 11 | 2.26 | 13826197 | `<full_trace_zero_backend::binary_ecc::f2m::F2mElement>::square::{closure#0}` | trace-baseline-profile |
| 12 | 2.08 | 12725475 | `full_trace_zero_backend::binary_ecc::f2m::mul_words_hw` | trace-baseline-profile |
| 13 | 2.05 | 12542419 | `0x00000000000a02e0` | libc.so.6 |
| 14 | 1.93 | 11813409 | `0x000000000009ebd0` | libc.so.6 |
| 15 | 1.84 | 11271135 | `<full_trace_zero_backend::binary_ecc::f2m::F2mElement>::mul` | trace-baseline-profile |
| 16 | 1.47 | 8989125 | `trace_zero_fast::baseline::decompose_trace_zero_linearized` | trace-baseline-profile |
| 17 | 1.4 | 8572995 | `<serde_json::value::Value as serde_core::de::Deserialize>::deserialize::<&mut serde_json::de::Deserializer<serde_json::read::SliceRead>>'2` | null |
| 18 | 1.28 | 7828158 | `trace_zero_fast::baseline::decompose_trace_zero_linearized` | null |
| 19 | 1.26 | 7728776 | `0x0000000000162880` | libc.so.6 |
| 20 | 1.21 | 7385422 | `<full_trace_zero_backend::binary_ecc::f2m::F2mElement>::square` | trace-baseline-profile |
| 21 | 1.14 | 7007280 | `<full_trace_zero_backend::binary_ecc::f2m::F2mElement>::mask_in_place` | trace-baseline-profile |
| 22 | 0.95 | 5814863 | `0x000000000009eaf0` | libc.so.6 |
| 23 | 0.94 | 5743296 | `<full_trace_zero_backend::binary_ecc::f2m::F2mElement>::add` | trace-baseline-profile |
| 24 | 0.75 | 4605464 | `trace_zero_fast::baseline::decompose_trace_zero_linearized` | null |
| 25 | 0.7 | 4304948 | `trace_zero_fast::baseline::decompose_trace_zero_linearized` | null |

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
