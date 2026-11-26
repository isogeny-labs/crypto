# cpu profile: Candidate1 residual costs before candidate2

- status: **ok**
- command: `research/trace_zero_cold50_20261123/bin/elimination one research/trace_zero_projected_20261115/inputs/scale-n131-d12-uniform.json elimination`
- binary: `/workspace/crypto/research/trace_zero_cold50_20261123/bin/elimination` sha256 `c89f88ed68ff4db6…`
- commit: `e64469e6aea1` (0 dirty tracked files)
- host: INTEL(R) XEON(R) PLATINUM 8573C, 1 logical CPUs, kernel 6.18.57, PMU absent, perf_event_paranoid 2
- valgrind: valgrind-3.24.0
- isolation: none; wall 0.505 s (profiled run: not a timing measurement)
- started: 2026-11-23T23:27:50Z

## Hotspots (callgrind, self Ir)

Total: 23781983 Ir (deterministic, simulated; not cycles).

| # | self % | Ir | function | object |
|---|---:|---:|---|---|
| 1 | 16.12 | 3834328 | `full_trace_zero_backend::cryptanalysis::trace_zero_linearized::decompose_trace_zero_linearized` | elimination |
| 2 | 6.94 | 1651088 | `<full_trace_zero_backend::cryptanalysis::trace_zero_word_field::WordField>::reduce` | elimination |
| 3 | 3.42 | 813010 | `full_trace_zero_backend::cryptanalysis::trace_zero_linearized::decompose_trace_zero_linearized` | null |
| 4 | 3.22 | 765091 | `0x00000000000a0fe0` | libc.so.6 |
| 5 | 2.99 | 710124 | `<full_trace_zero_backend::binary_ecc::f2m::F2mElement>::from_hex` | elimination |
| 6 | 2.8 | 666885 | `full_trace_zero_backend::binary_ecc::f2m::trace_tables` | elimination |
| 7 | 2.69 | 640609 | `full_trace_zero_backend::cryptanalysis::trace_zero_linearized::decompose_trace_zero_linearized` | null |
| 8 | 2.45 | 583308 | `full_trace_zero_backend::binary_ecc::f2m::mul_words_hw` | null |
| 9 | 2.21 | 525390 | `full_trace_zero_backend::cryptanalysis::trace_zero_linearized::decompose_trace_zero_linearized` | null |
| 10 | 2.12 | 504613 | `<full_trace_zero_backend::cryptanalysis::trace_zero_word_field::WordField>::residuals` | elimination |
| 11 | 1.93 | 459576 | `full_trace_zero_backend::binary_ecc::f2m::mul_words_into` | elimination |
| 12 | 1.83 | 435689 | `<full_trace_zero_backend::cryptanalysis::trace_zero_word_field::WordField>::admit` | elimination |
| 13 | 1.81 | 430502 | `full_trace_zero_backend::cryptanalysis::trace_zero_linearized::decompose_trace_zero_linearized` | null |
| 14 | 1.75 | 416361 | `free` | libc.so.6 |
| 15 | 1.64 | 388872 | `full_trace_zero_backend::binary_ecc::f2m::mul_words_hw` | elimination |
| 16 | 1.63 | 388453 | `0x0000000000162880` | libc.so.6 |
| 17 | 1.46 | 346863 | `<full_trace_zero_backend::cryptanalysis::trace_zero_word_field::WordField>::square` | null |
| 18 | 1.42 | 336819 | `full_trace_zero_backend::cryptanalysis::trace_zero_linearized::decompose_trace_zero_linearized` | null |
| 19 | 1.37 | 326280 | `0x0000000000163280` | libc.so.6 |
| 20 | 1.26 | 298826 | `core::str::converts::from_utf8` | null |
| 21 | 1.2 | 285275 | `full_trace_zero_backend::cryptanalysis::trace_zero_linearized::decompose_trace_zero_linearized` | null |
| 22 | 1.19 | 282129 | `0x000000000009ebd0` | libc.so.6 |
| 23 | 1.17 | 278318 | `malloc` | libc.so.6 |
| 24 | 1.15 | 273135 | `0x00000000000a02e0` | libc.so.6 |
| 25 | 1.05 | 249197 | `full_trace_zero_backend::cryptanalysis::trace_zero_linearized::decompose_trace_zero_linearized` | null |

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
