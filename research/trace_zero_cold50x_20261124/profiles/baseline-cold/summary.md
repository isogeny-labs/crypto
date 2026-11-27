# cpu profile: Frozen21.36M baseline, another50x descriptive query profile

- status: **ok**
- command: `research/trace_zero_cold50_20261123/candidate3/bin/elimination one research/trace_zero_projected_20261115/inputs/scale-n131-d12-uniform.json elimination`
- binary: `/workspace/crypto/research/trace_zero_cold50_20261123/candidate3/bin/elimination` sha256 `fe0d7ea116db2254…`
- commit: `78aebf3da16e` (0 dirty tracked files)
- host: INTEL(R) XEON(R) PLATINUM 8573C, 1 logical CPUs, kernel 6.18.57, PMU absent, perf_event_paranoid 2
- valgrind: valgrind-3.24.0
- isolation: none; wall 0.486 s (profiled run: not a timing measurement)
- started: 2026-11-24T02:13:28Z

## Hotspots (callgrind, self Ir)

Total: 21360464 Ir (deterministic, simulated; not cycles).

| # | self % | Ir | function | object |
|---|---:|---:|---|---|
| 1 | 18.06 | 3857648 | `full_trace_zero_backend::cryptanalysis::trace_zero_linearized::decompose_trace_zero_linearized` | elimination |
| 2 | 4.86 | 1037100 | `full_trace_zero_backend::binary_ecc::f2m::mul_mod131_hw` | elimination |
| 3 | 3.58 | 764697 | `0x00000000000a0fe0` | libc.so.6 |
| 4 | 3.32 | 710124 | `<full_trace_zero_backend::binary_ecc::f2m::F2mElement>::from_hex` | elimination |
| 5 | 3.15 | 671885 | `<full_trace_zero_backend::binary_ecc::f2m::TraceTables>::new` | elimination |
| 6 | 2.52 | 537889 | `<full_trace_zero_backend::cryptanalysis::trace_zero_word_field::WordField>::square` | elimination |
| 7 | 2.41 | 514643 | `full_trace_zero_backend::cryptanalysis::trace_zero_linearized::decompose_trace_zero_linearized` | null |
| 8 | 2.27 | 483980 | `<full_trace_zero_backend::cryptanalysis::trace_zero_word_field::WordField>::mul` | elimination |
| 9 | 2.18 | 466695 | `full_trace_zero_backend::binary_ecc::f2m::mul_mod131_hw` | null |
| 10 | 2.06 | 441075 | `full_trace_zero_backend::cryptanalysis::trace_zero_linearized::decompose_trace_zero_linearized` | null |
| 11 | 1.94 | 414481 | `full_trace_zero_backend::cryptanalysis::trace_zero_linearized::decompose_trace_zero_linearized` | null |
| 12 | 1.83 | 389903 | `free` | libc.so.6 |
| 13 | 1.8 | 384434 | `0x0000000000162880` | libc.so.6 |
| 14 | 1.68 | 357923 | `<full_trace_zero_backend::cryptanalysis::trace_zero_word_field::WordField>::admit` | elimination |
| 15 | 1.65 | 351890 | `<full_trace_zero_backend::cryptanalysis::trace_zero_word_field::WordField>::square` | null |
| 16 | 1.59 | 340219 | `0x0000000000163280` | libc.so.6 |
| 17 | 1.58 | 336819 | `full_trace_zero_backend::cryptanalysis::trace_zero_linearized::decompose_trace_zero_linearized` | null |
| 18 | 1.4 | 298975 | `core::str::converts::from_utf8` | null |
| 19 | 1.32 | 282149 | `0x000000000009ebd0` | libc.so.6 |
| 20 | 1.28 | 273182 | `0x00000000000a02e0` | libc.so.6 |
| 21 | 1.23 | 263500 | `<full_trace_zero_backend::cryptanalysis::trace_zero_word_field::WordField>::residuals` | elimination |
| 22 | 1.23 | 262695 | `full_trace_zero_backend::cryptanalysis::trace_zero_linearized::decompose_trace_zero_linearized` | null |
| 23 | 1.22 | 261549 | `malloc` | libc.so.6 |
| 24 | 1.14 | 244557 | `full_trace_zero_backend::cryptanalysis::trace_zero_linearized::decompose_trace_zero_linearized` | null |
| 25 | 1.06 | 226121 | `<serde_json::read::SliceRead>::skip_to_escape` | elimination |

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
