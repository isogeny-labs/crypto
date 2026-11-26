# cpu profile: Exact-half-trace incumbent, fixed uniform ECC2K query

- status: **ok**
- command: `research/trace_zero_halftrace_elimination_20261123/bin/elimination one research/trace_zero_projected_20261115/inputs/scale-n131-d12-uniform.json elimination`
- binary: `/workspace/crypto/research/trace_zero_halftrace_elimination_20261123/bin/elimination` sha256 `504a9c6a6ae668a0…`
- commit: `ad5f91c4b439` (0 dirty tracked files)
- host: INTEL(R) XEON(R) PLATINUM 8573C, 1 logical CPUs, kernel 6.18.57, PMU absent, perf_event_paranoid 2
- valgrind: valgrind-3.24.0
- isolation: none; wall 0.586 s (profiled run: not a timing measurement)
- started: 2026-11-23T23:03:34Z

## Hotspots (callgrind, self Ir)

Total: 34987635 Ir (deterministic, simulated; not cycles).

| # | self % | Ir | function | object |
|---|---:|---:|---|---|
| 1 | 11.09 | 3880904 | `full_trace_zero_backend::cryptanalysis::trace_zero_linearized::decompose_trace_zero_linearized` | elimination |
| 2 | 5.36 | 1873656 | `full_trace_zero_backend::binary_ecc::f2m::mul_words_hw` | elimination |
| 3 | 4.72 | 1651088 | `<full_trace_zero_backend::cryptanalysis::trace_zero_word_field::WordField>::reduce` | elimination |
| 4 | 4.14 | 1449271 | `free` | libc.so.6 |
| 5 | 3.33 | 1163720 | `0x00000000000a0fe0` | libc.so.6 |
| 6 | 2.6 | 909665 | `malloc` | libc.so.6 |
| 7 | 2.5 | 873240 | `<alloc::string::String as core::iter::traits::collect::FromIterator<char>>::from_iter::<core::iter::adapters::filter::Filter<core::str::iter::Chars, <full_trace_zero_backend::binary_ecc::f2m::F2mElement>::from_hex::{closure#0}>>` | null |
| 8 | 2.32 | 813010 | `full_trace_zero_backend::cryptanalysis::trace_zero_linearized::decompose_trace_zero_linearized` | null |
| 9 | 1.94 | 679798 | `<num_bigint::biguint::BigUint as num_traits::Num>::from_str_radix` | elimination |
| 10 | 1.91 | 666885 | `full_trace_zero_backend::binary_ecc::f2m::trace_tables` | elimination |
| 11 | 1.87 | 654012 | `full_trace_zero_backend::binary_ecc::f2m::mul_words_hw` | null |
| 12 | 1.84 | 644595 | `full_trace_zero_backend::cryptanalysis::trace_zero_linearized::decompose_trace_zero_linearized` | null |
| 13 | 1.56 | 544950 | `full_trace_zero_backend::cryptanalysis::trace_zero_linearized::decompose_trace_zero_linearized` | null |
| 14 | 1.44 | 504613 | `<full_trace_zero_backend::cryptanalysis::trace_zero_word_field::WordField>::residuals` | elimination |
| 15 | 1.31 | 459576 | `full_trace_zero_backend::binary_ecc::f2m::mul_words_into` | elimination |
| 16 | 1.31 | 459551 | `core::str::converts::from_utf8` | null |
| 17 | 1.27 | 445955 | `full_trace_zero_backend::cryptanalysis::trace_zero_linearized::decompose_trace_zero_linearized` | null |
| 18 | 1.25 | 435689 | `<full_trace_zero_backend::cryptanalysis::trace_zero_word_field::WordField>::admit` | elimination |
| 19 | 1.2 | 418380 | `0x00000000000a02e0` | libc.so.6 |
| 20 | 1.17 | 408936 | `0x0000000000162880` | libc.so.6 |
| 21 | 1.11 | 388872 | `full_trace_zero_backend::binary_ecc::f2m::mul_words_hw` | null |
| 22 | 1.09 | 382144 | `realloc` | libc.so.6 |
| 23 | 1.05 | 368976 | `0x000000000009ebd0` | libc.so.6 |
| 24 | 1.03 | 359498 | `<alloc::string::String as core::iter::traits::collect::FromIterator<char>>::from_iter::<core::iter::adapters::filter::Filter<core::str::iter::Chars, <full_trace_zero_backend::binary_ecc::f2m::F2mElement>::from_hex::{closure#0}>>` | null |
| 25 | 1.02 | 356218 | `0x00000000000a20c0` | libc.so.6 |

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
