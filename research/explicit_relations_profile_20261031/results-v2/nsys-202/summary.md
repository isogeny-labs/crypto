# nsys profile: unlabelled

- status: **ok**
- command: `bash /work/crypto/research/explicit_relations_profile_20261031/replay.sh /work/crypto/gpu/explicit-relations/build/explicit-relations-affine /work/crypto/research/explicit_relations_profile_20261031/results-v1/202.commands`
- binary: `/usr/bin/bash` sha256 `bc5945feb8bd2620…`
- commit: `b50e30d47458` (0 dirty tracked files)
- host: AMD EPYC 75F3 32-Core Processor, 128 logical CPUs, kernel 6.8.0-138-generic, PMU present, perf_event_paranoid 4
- GPU 0: NVIDIA GeForce RTX 4090, cc 8.9, driver 595.91.07, max SM clock 3105 MHz, power limit 450.00 W
- nsys: NVIDIA Nsight Systems version 2025.6.3.541-256337736014v0
- isolation: none; wall 4.752 s (profiled run: not a timing measurement)
- started: 2026-10-31T07:48:32Z

## Nsight Systems

Kernel coverage of the kernel span: **98.7%** (128 launches, span 1394.495285 ms). Gaps are time the GPU had no kernel running.

| kernel | time % | total (ms) | instances | median (µs) |
|---|---:|---:|---:|---:|
| `explicit_relations::search_right_persistent(explicit_relatio` | 50.5 | 695.057237 | 8 | 86765.3775 |
| `void cub::_V_300200_SM_890::detail::merge_sort::DeviceMergeS` | 19.8 | 272.906718 | 19 | 14373.226 |
| `void explicit_relations::build_negative_filter<explicit_rela` | 12.4 | 170.366597 | 1 | 170366.597 |
| `explicit_relations::build_left(explicit_relations::Curve, ex` | 6.5 | 89.376123 | 1 | 89376.123 |
| `explicit_relations::build_right_cache(explicit_relations::Cu` | 5.4 | 74.034887 | 1 | 74034.887 |
| `void cub::_V_300200_SM_890::detail::merge_sort::DeviceMergeS` | 2.5 | 34.720462 | 19 | 1869.17 |
| `void cub::_V_300200_SM_890::detail::merge_sort::DeviceMergeS` | 1.2 | 16.227548 | 1 | 16227.548 |
| `void explicit_relations::build_prefix<explicit_relations::De` | 0.6 | 7.879852 | 1 | 7879.852 |
| `void cub::_V_300200_SM_890::detail::for_each::static_kernel<` | 0.5 | 7.406567 | 1 | 7406.567 |
| `void cub::_V_300200_SM_890::detail::for_each::static_kernel<` | 0.5 | 6.85181 | 1 | 6851.81 |
| `void cub::_V_300200_SM_890::detail::for_each::static_kernel<` | 0.1 | 1.138827 | 2 | 569.4135 |
| `explicit_relations::hash_base_wide(explicit_relations::Curve` | 0.0 | 0.150051 | 12 | 12.48 |
| `void cub::_V_300200_SM_890::detail::for_each::static_kernel<` | 0.0 | 0.129217 | 1 | 129.217 |
| `void cub::_V_300200_SM_890::detail::merge_sort::DeviceMergeS` | 0.0 | 0.082659 | 12 | 6.848 |
| `explicit_relations::build_partial(explicit_relations::Curve,` | 0.0 | 0.030624 | 2 | 15.312 |
| `void cub::_V_300200_SM_890::detail::for_each::static_kernel<` | 0.0 | 0.01456 | 14 | 0.96 |
| `void cub::_V_300200_SM_890::detail::select::DeviceSelectSwee` | 0.0 | 0.014273 | 12 | 1.184 |
| `explicit_relations::resolve_persistent_winner(explicit_relat` | 0.0 | 0.011584 | 8 | 1.472 |
| `void cub::_V_300200_SM_890::detail::scan::DeviceCompactInitK` | 0.0 | 0.009696 | 12 | 0.8 |

| CUDA API | time % | calls | total (ms) |
|---|---:|---:|---:|
| `cudaMemcpy` | 45.8 | 20 | 695.210625 |
| `cudaDeviceSynchronize` | 43.8 | 4 | 666.077854 |
| `cudaFree` | 8.7 | 46 | 132.753777 |
| `cudaStreamSynchronize` | 1.0 | 56 | 15.685713 |
| `cudaMalloc` | 0.5 | 45 | 7.361237 |
| `cudaLaunchKernel` | 0.1 | 77 | 1.071786 |
| `cudaLaunchKernelExC_v11060` | 0.0 | 51 | 0.30125 |
| `cudaMemGetInfo` | 0.0 | 14 | 0.227101 |
| `cudaMemcpyAsync` | 0.0 | 13 | 0.163752 |
| `cudaMemsetAsync` | 0.0 | 17 | 0.126702 |

## Files

- `api_sum.csv`
- `command.stderr`
- `command.stdout`
- `gpu_trace.csv`
- `kern_sum.csv`
- `kernels.svg`
- `manifest.json`
- `memtime_sum.csv`
- `nvtx_sum.csv`
- `profile.nsys-rep`
- `profile.sqlite`
- `summary.json`

Raw profiler output (perf.data, *.ncu-rep, *.nsys-rep, callgrind.out) is large: push it with `crypto store push` (AGENTS.md section 13) rather than committing it.
