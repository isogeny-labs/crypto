#set page(paper: "a4", margin: 1.5cm)
#set text(size: 9pt)
#show raw: set text(size: 8pt)
#set table(stroke: 0.4pt + luma(180), inset: 3.5pt)
#show table: set text(size: 7.5pt)

= nsys profile: unlabelled

- status: *ok*
- command: #raw("bash /work/crypto/research/explicit_relations_profile_20261031/replay.sh /work/crypto/gpu/explicit-relations/build/explicit-relations-affine /work/crypto/research/explicit_relations_profile_20261031/results-v1/202.commands")
- binary: #raw("/usr/bin/bash") sha256 #raw("bc5945feb8bd2620…")
- commit: #raw("b50e30d47458") (0 dirty tracked files)
- host: AMD EPYC 75F3 32-Core Processor, 128 logical CPUs, kernel 6.8.0-138-generic, PMU present, perf\_event\_paranoid 4
- GPU 0: NVIDIA GeForce RTX 4090, cc 8.9, driver 595.91.07, max SM clock 3105 MHz, power limit 450.00 W
- nsys: NVIDIA Nsight Systems version 2025.6.3.541-256337736014v0
- isolation: none; wall 4.752 s (profiled run: not a timing measurement)
- started: 2026-10-31T07:48:32Z

== Nsight Systems

Kernel coverage of the kernel span: *98.7%* (128 launches, span 1394.495285 ms). Gaps are time the GPU had no kernel running.

#table(columns: 5,
  [*kernel*],  [*time %*],  [*total (ms)*],  [*instances*],  [*median (µs)*],
  [#raw("explicit_relations::search_right_persistent(explicit_relatio")],  [50.5],  [695.057237],  [8],  [86765.3775],
  [#raw("void cub::_V_300200_SM_890::detail::merge_sort::DeviceMergeS")],  [19.8],  [272.906718],  [19],  [14373.226],
  [#raw("void explicit_relations::build_negative_filter<explicit_rela")],  [12.4],  [170.366597],  [1],  [170366.597],
  [#raw("explicit_relations::build_left(explicit_relations::Curve, ex")],  [6.5],  [89.376123],  [1],  [89376.123],
  [#raw("explicit_relations::build_right_cache(explicit_relations::Cu")],  [5.4],  [74.034887],  [1],  [74034.887],
  [#raw("void cub::_V_300200_SM_890::detail::merge_sort::DeviceMergeS")],  [2.5],  [34.720462],  [19],  [1869.17],
  [#raw("void cub::_V_300200_SM_890::detail::merge_sort::DeviceMergeS")],  [1.2],  [16.227548],  [1],  [16227.548],
  [#raw("void explicit_relations::build_prefix<explicit_relations::De")],  [0.6],  [7.879852],  [1],  [7879.852],
  [#raw("void cub::_V_300200_SM_890::detail::for_each::static_kernel<")],  [0.5],  [7.406567],  [1],  [7406.567],
  [#raw("void cub::_V_300200_SM_890::detail::for_each::static_kernel<")],  [0.5],  [6.85181],  [1],  [6851.81],
  [#raw("void cub::_V_300200_SM_890::detail::for_each::static_kernel<")],  [0.1],  [1.138827],  [2],  [569.4135],
  [#raw("explicit_relations::hash_base_wide(explicit_relations::Curve")],  [0.0],  [0.150051],  [12],  [12.48],
  [#raw("void cub::_V_300200_SM_890::detail::for_each::static_kernel<")],  [0.0],  [0.129217],  [1],  [129.217],
  [#raw("void cub::_V_300200_SM_890::detail::merge_sort::DeviceMergeS")],  [0.0],  [0.082659],  [12],  [6.848],
  [#raw("explicit_relations::build_partial(explicit_relations::Curve,")],  [0.0],  [0.030624],  [2],  [15.312],
  [#raw("void cub::_V_300200_SM_890::detail::for_each::static_kernel<")],  [0.0],  [0.01456],  [14],  [0.96],
  [#raw("void cub::_V_300200_SM_890::detail::select::DeviceSelectSwee")],  [0.0],  [0.014273],  [12],  [1.184],
  [#raw("explicit_relations::resolve_persistent_winner(explicit_relat")],  [0.0],  [0.011584],  [8],  [1.472],
  [#raw("void cub::_V_300200_SM_890::detail::scan::DeviceCompactInitK")],  [0.0],  [0.009696],  [12],  [0.8],
)

#table(columns: 4,
  [*CUDA API*],  [*time %*],  [*calls*],  [*total (ms)*],
  [#raw("cudaMemcpy")],  [45.8],  [20],  [695.210625],
  [#raw("cudaDeviceSynchronize")],  [43.8],  [4],  [666.077854],
  [#raw("cudaFree")],  [8.7],  [46],  [132.753777],
  [#raw("cudaStreamSynchronize")],  [1.0],  [56],  [15.685713],
  [#raw("cudaMalloc")],  [0.5],  [45],  [7.361237],
  [#raw("cudaLaunchKernel")],  [0.1],  [77],  [1.071786],
  [#raw("cudaLaunchKernelExC_v11060")],  [0.0],  [51],  [0.30125],
  [#raw("cudaMemGetInfo")],  [0.0],  [14],  [0.227101],
  [#raw("cudaMemcpyAsync")],  [0.0],  [13],  [0.163752],
  [#raw("cudaMemsetAsync")],  [0.0],  [17],  [0.126702],
)

== Files

- #raw("api_sum.csv")
- #raw("command.stderr")
- #raw("command.stdout")
- #raw("gpu_trace.csv")
- #raw("kern_sum.csv")
- #raw("kernels.svg")
- #raw("manifest.json")
- #raw("memtime_sum.csv")
- #raw("nvtx_sum.csv")
- #raw("profile.nsys-rep")
- #raw("profile.sqlite")
- #raw("summary.json")

Raw profiler output (perf.data, \*.ncu-rep, \*.nsys-rep, callgrind.out) is large: push it with #raw("crypto store push") (AGENTS.md section 13) rather than committing it.

== Graphs

#figure(image("kernels.svg", width: 100%))

