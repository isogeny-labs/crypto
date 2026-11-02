# nsys profile: unlabelled

- status: **ok**
- command: `bash /work/crypto/research/explicit_relations_profile_20261031/replay.sh /work/crypto/gpu/explicit-relations/build/explicit-relations-affine /work/crypto/research/explicit_relations_profile_20261031/results-v1/101.commands`
- binary: `/usr/bin/bash` sha256 `bc5945feb8bd2620…`
- commit: `b50e30d47458` (0 dirty tracked files)
- host: AMD EPYC 75F3 32-Core Processor, 128 logical CPUs, kernel 6.8.0-138-generic, PMU present, perf_event_paranoid 4
- GPU 0: NVIDIA GeForce RTX 4090, cc 8.9, driver 595.91.07, max SM clock 3105 MHz, power limit 450.00 W
- nsys: NVIDIA Nsight Systems version 2025.6.3.541-256337736014v0
- isolation: none; wall 4.817 s (profiled run: not a timing measurement)
- started: 2026-10-31T07:48:21Z

## Nsight Systems

Kernel coverage of the kernel span: **99.1%** (126 launches, span 1762.704102 ms). Gaps are time the GPU had no kernel running.

| kernel | time % | total (ms) | instances | median (µs) |
|---|---:|---:|---:|---:|
| `explicit_relations::search_right_persistent(explicit_relatio` | 76.6 | 1338.874452 | 8 | 167338.435 |
| `explicit_relations::build_right_cache(explicit_relations::Cu` | 7.3 | 128.110264 | 1 | 128110.264 |
| `void cub::_V_300200_SM_890::detail::merge_sort::DeviceMergeS` | 7.0 | 121.708157 | 18 | 6762.558 |
| `void explicit_relations::build_negative_filter<explicit_rela` | 4.1 | 72.367704 | 1 | 72367.704 |
| `explicit_relations::build_left(explicit_relations::Curve, ex` | 2.4 | 42.51511 | 1 | 42515.11 |
| `void cub::_V_300200_SM_890::detail::merge_sort::DeviceMergeS` | 0.9 | 15.56987 | 18 | 902.0725 |
| `void cub::_V_300200_SM_890::detail::for_each::static_kernel<` | 0.8 | 13.107736 | 1 | 13107.736 |
| `void cub::_V_300200_SM_890::detail::merge_sort::DeviceMergeS` | 0.4 | 7.844488 | 1 | 7844.488 |
| `void cub::_V_300200_SM_890::detail::for_each::static_kernel<` | 0.2 | 3.224925 | 1 | 3224.925 |
| `void explicit_relations::build_prefix<explicit_relations::De` | 0.2 | 2.991196 | 1 | 2991.196 |
| `void cub::_V_300200_SM_890::detail::for_each::static_kernel<` | 0.0 | 0.25405 | 2 | 127.025 |
| `explicit_relations::hash_base_wide(explicit_relations::Curve` | 0.0 | 0.149506 | 12 | 12.448 |
| `void cub::_V_300200_SM_890::detail::merge_sort::DeviceMergeS` | 0.0 | 0.083297 | 12 | 6.912 |
| `explicit_relations::build_partial(explicit_relations::Curve,` | 0.0 | 0.032352 | 2 | 16.176 |
| `void cub::_V_300200_SM_890::detail::for_each::static_kernel<` | 0.0 | 0.018688 | 1 | 18.688 |
| `void cub::_V_300200_SM_890::detail::for_each::static_kernel<` | 0.0 | 0.015074 | 14 | 0.96 |
| `void cub::_V_300200_SM_890::detail::select::DeviceSelectSwee` | 0.0 | 0.014208 | 12 | 1.184 |
| `explicit_relations::resolve_persistent_winner(explicit_relat` | 0.0 | 0.011616 | 8 | 1.472 |
| `void cub::_V_300200_SM_890::detail::scan::DeviceCompactInitK` | 0.0 | 0.009728 | 12 | 0.8 |

| CUDA API | time % | calls | total (ms) |
|---|---:|---:|---:|
| `cudaMemcpy` | 70.5 | 20 | 1339.120926 |
| `cudaDeviceSynchronize` | 20.6 | 4 | 390.995926 |
| `cudaFree` | 7.6 | 46 | 144.812105 |
| `cudaStreamSynchronize` | 0.9 | 56 | 16.782542 |
| `cudaMalloc` | 0.3 | 45 | 6.444719 |
| `cudaLaunchKernel` | 0.1 | 77 | 1.154114 |
| `cudaLaunchKernelExC_v11060` | 0.0 | 49 | 0.322051 |
| `cudaMemGetInfo` | 0.0 | 14 | 0.243664 |
| `cudaMemcpyAsync` | 0.0 | 13 | 0.17526 |
| `cudaMemsetAsync` | 0.0 | 17 | 0.155522 |

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
