#set page(paper: "a4", margin: 1.5cm)
#set text(size: 9pt)
#show raw: set text(size: 8pt)
#set table(stroke: 0.4pt + luma(180), inset: 3.5pt)
#show table: set text(size: 7.5pt)

= cpu profile: Projected lazy cold n131 uniform, unpartitioned discovery profile

- status: *ok*
- command: #raw("/workspace/scratch/trace-projected-baseline one research/trace_zero_projected_20261115/inputs/scale-n131-d12-uniform.json projected-lazy")
- binary: #raw("/workspace/scratch/trace-projected-baseline") sha256 #raw("768193f99099dfa6…")
- commit: #raw("6cd3d7017370") (1 dirty tracked files)
- host: AMD EPYC 9V74 80-Core Processor, 1 logical CPUs, kernel 6.18.57, PMU absent, perf\_event\_paranoid 2
- valgrind: valgrind-3.24.0
- isolation: none; wall 0.729 s (profiled run: not a timing measurement)
- started: 2026-11-15T23:25:41Z

== Hotspots (callgrind, self Ir)

Total: 81844664 Ir (deterministic, simulated; not cycles).

#table(columns: 5,
  [*\#*],  [*self %*],  [*Ir*],  [*function*],  [*object*],
  [1],  [9.86],  [8069255],  [#raw("full_trace_zero_backend::cryptanalysis::trace_zero_linearized::decompose_trace_zero_linearized")],  [trace-projected-baseline],
  [2],  [7.83],  [6409964],  [#raw("full_trace_zero_backend::binary_ecc::f2m::fold_words_clmul_hw")],  [trace-projected-baseline],
  [3],  [4.74],  [3877931],  [#raw("<full_trace_zero_backend::binary_ecc::f2m::F2mElement>::square_k_times::{closure#0}")],  [trace-projected-baseline],
  [4],  [3.67],  [3001369],  [#raw("full_trace_zero_backend::cryptanalysis::trace_zero_linearized::decompose_trace_zero_linearized")],  [null],
  [5],  [2.82],  [2305041],  [#raw("free")],  [libc.so.6],
  [6],  [2.8],  [2288808],  [#raw("<full_trace_zero_backend::binary_ecc::f2m::F2mElement>::add")],  [trace-projected-baseline],
  [7],  [2.75],  [2249863],  [#raw("full_trace_zero_backend::binary_ecc::f2m::reduce_words")],  [trace-projected-baseline],
  [8],  [2.49],  [2035845],  [#raw("full_trace_zero_backend::binary_ecc::f2m::mul_words_hw")],  [trace-projected-baseline],
  [9],  [2.45],  [2005310],  [#raw("full_trace_zero_backend::binary_ecc::f2m::reduce_words")],  [null],
  [10],  [2.27],  [1857624],  [#raw("full_trace_zero_backend::cryptanalysis::trace_zero_linearized::decompose_trace_zero_linearized")],  [null],
  [11],  [2.2],  [1803177],  [#raw("<full_trace_zero_backend::binary_ecc::f2m::F2mElement>::mul")],  [trace-projected-baseline],
  [12],  [2.2],  [1801612],  [#raw("0x00000000000a0fe0")],  [libc.so.6],
  [13],  [1.79],  [1467268],  [#raw("malloc")],  [libc.so.6],
  [14],  [1.41],  [1153844],  [#raw("0x0000000000162880")],  [libc.so.6],
  [15],  [1.34],  [1100460],  [#raw("<full_trace_zero_backend::binary_ecc::f2m::F2mElement>::mask_in_place")],  [trace-projected-baseline],
  [16],  [1.21],  [993496],  [#raw("<full_trace_zero_backend::binary_ecc::f2m::F2mElement>::half_trace")],  [null],
  [17],  [1.17],  [955567],  [#raw("full_trace_zero_backend::cryptanalysis::trace_zero_linearized::decompose_trace_zero_linearized")],  [null],
  [18],  [1.14],  [930609],  [#raw("<full_trace_zero_backend::binary_ecc::f2m::F2mElement>::half_trace")],  [trace-projected-baseline],
  [19],  [1.07],  [873240],  [#raw("<alloc::string::String as core::iter::traits::collect::FromIterator<char>>::from_iter::<core::iter::adapters::filter::Filter<core::str::iter::Chars, <full_trace_zero_backend::binary_ecc::f2m::F2mElement>::from_hex::{closure#0}>>")],  [null],
  [20],  [1.02],  [834529],  [#raw("full_trace_zero_backend::cryptanalysis::trace_zero_linearized::decompose_trace_zero_linearized")],  [null],
  [21],  [1],  [815633],  [#raw("<full_trace_zero_backend::binary_ecc::f2m::F2mElement>::square_k_times")],  [trace-projected-baseline],
  [22],  [0.99],  [811250],  [#raw("full_trace_zero_backend::cryptanalysis::trace_zero_linearized::decompose_trace_zero_linearized")],  [null],
  [23],  [0.89],  [731147],  [#raw("<full_trace_zero_backend::binary_ecc::f2m::F2mElement>::add")],  [null],
  [24],  [0.85],  [697680],  [#raw("<std::hash::random::DefaultHasher as core::hash::Hasher>::write")],  [null],
  [25],  [0.85],  [692139],  [#raw("<full_trace_zero_backend::binary_ecc::f2m::F2mElement>::square::{closure#0}")],  [trace-projected-baseline],
)

== Files

- #raw("annotate-inclusive.txt")
- #raw("annotate.txt")
- #raw("callgrind.out")
- #raw("command.stderr")
- #raw("command.stdout")
- #raw("hotspots.json")
- #raw("hotspots.svg")
- #raw("manifest.json")

Raw profiler output (perf.data, \*.ncu-rep, \*.nsys-rep, callgrind.out) is large: push it with #raw("crypto store push") (AGENTS.md section 13) rather than committing it.

== Graphs

#figure(image("hotspots.svg", width: 100%))

