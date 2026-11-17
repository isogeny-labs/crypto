# cpu profile: Compact workspace n131 uniform unpartitioned discovery

- status: **ok**
- command: `/workspace/scratch/trace-tuned-baseline one research/trace_zero_projected_20261115/inputs/scale-n131-d12-uniform.json tuned-lazy`
- binary: `/workspace/scratch/trace-tuned-baseline` sha256 `0033b96d9acdb62a…`
- commit: `fd6f5a9b0d3b` (1 dirty tracked files)
- host: AMD EPYC 9V74 80-Core Processor, 1 logical CPUs, kernel 6.18.57, PMU absent, perf_event_paranoid 2
- valgrind: valgrind-3.24.0
- isolation: none; wall 0.534 s (profiled run: not a timing measurement)
- started: 2026-11-16T00:14:50Z

## Hotspots (callgrind, self Ir)

Total: 59701801 Ir (deterministic, simulated; not cycles).

| # | self % | Ir | function | object |
|---|---:|---:|---|---|
| 1 | 12.63 | 7541609 | `full_trace_zero_backend::cryptanalysis::trace_zero_linearized::decompose_trace_zero_linearized` | trace-tuned-baseline |
| 2 | 10.61 | 6336186 | `full_trace_zero_backend::binary_ecc::f2m::fold_words_clmul_hw` | trace-tuned-baseline |
| 3 | 4.82 | 2877133 | `full_trace_zero_backend::cryptanalysis::trace_zero_linearized::decompose_trace_zero_linearized` | null |
| 4 | 4.5 | 2684144 | `<full_trace_zero_backend::binary_ecc::f2m::F2mElement>::square_k_times::{closure#0}` | trace-tuned-baseline |
| 5 | 3.69 | 2204001 | `full_trace_zero_backend::binary_ecc::f2m::reduce_words` | trace-tuned-baseline |
| 6 | 3.52 | 2098799 | `full_trace_zero_backend::cryptanalysis::trace_zero_linearized::decompose_trace_zero_linearized` | null |
| 7 | 3.29 | 1964433 | `full_trace_zero_backend::binary_ecc::f2m::reduce_words` | null |
| 8 | 3.23 | 1931160 | `full_trace_zero_backend::binary_ecc::f2m::mul_words_hw` | trace-tuned-baseline |
| 9 | 2.44 | 1455166 | `free` | libc.so.6 |
| 10 | 1.97 | 1176855 | `0x00000000000a0fe0` | libc.so.6 |
| 11 | 1.53 | 914227 | `malloc` | libc.so.6 |
| 12 | 1.46 | 873240 | `<alloc::string::String as core::iter::traits::collect::FromIterator<char>>::from_iter::<core::iter::adapters::filter::Filter<core::str::iter::Chars, <full_trace_zero_backend::binary_ecc::f2m::F2mElement>::from_hex::{closure#0}>>` | null |
| 13 | 1.41 | 840213 | `<full_trace_zero_backend::cryptanalysis::trace_zero_word_field::WordField>::square` | null |
| 14 | 1.36 | 813010 | `full_trace_zero_backend::cryptanalysis::trace_zero_linearized::decompose_trace_zero_linearized` | null |
| 15 | 1.3 | 775502 | `<full_trace_zero_backend::binary_ecc::f2m::F2mElement>::square_k_times` | trace-tuned-baseline |
| 16 | 1.27 | 760973 | `full_trace_zero_backend::cryptanalysis::trace_zero_linearized::decompose_trace_zero_linearized` | null |
| 17 | 1.19 | 711926 | `full_trace_zero_backend::cryptanalysis::trace_zero_linearized::decompose_trace_zero_linearized` | null |
| 18 | 1.14 | 679798 | `<num_bigint::biguint::BigUint as num_traits::Num>::from_str_radix` | trace-tuned-baseline |
| 19 | 1 | 595586 | `0x0000000000162880` | libc.so.6 |
| 20 | 0.99 | 591673 | `full_trace_zero_backend::cryptanalysis::trace_zero_linearized::decompose_trace_zero_linearized` | null |
| 21 | 0.88 | 525390 | `full_trace_zero_backend::cryptanalysis::trace_zero_linearized::decompose_trace_zero_linearized` | null |
| 22 | 0.86 | 511434 | `<full_trace_zero_backend::cryptanalysis::trace_zero_word_field::WordField>::square` | trace-tuned-baseline |
| 23 | 0.84 | 499785 | `<full_trace_zero_backend::cryptanalysis::trace_zero_word_field::WordField>::residuals` | trace-tuned-baseline |
| 24 | 0.8 | 478192 | `full_trace_zero_backend::binary_ecc::f2m::mul_words_into` | trace-tuned-baseline |
| 25 | 0.77 | 459475 | `core::str::converts::from_utf8` | null |

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
