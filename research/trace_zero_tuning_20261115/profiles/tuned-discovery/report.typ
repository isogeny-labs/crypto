#set page(paper: "a4", margin: 1.5cm)
#set text(size: 9pt)
#show raw: set text(size: 8pt)
#set table(stroke: 0.4pt + luma(180), inset: 3.5pt)
#show table: set text(size: 7.5pt)

= cpu profile: Compact workspace n131 uniform unpartitioned discovery

- status: *ok*
- command: #raw("/workspace/scratch/trace-tuned-baseline one research/trace_zero_projected_20261115/inputs/scale-n131-d12-uniform.json tuned-lazy")
- binary: #raw("/workspace/scratch/trace-tuned-baseline") sha256 #raw("0033b96d9acdb62a…")
- commit: #raw("fd6f5a9b0d3b") (1 dirty tracked files)
- host: AMD EPYC 9V74 80-Core Processor, 1 logical CPUs, kernel 6.18.57, PMU absent, perf\_event\_paranoid 2
- valgrind: valgrind-3.24.0
- isolation: none; wall 0.534 s (profiled run: not a timing measurement)
- started: 2026-11-16T00:14:50Z

== Hotspots (callgrind, self Ir)

Total: 59701801 Ir (deterministic, simulated; not cycles).

#table(columns: (auto, auto, auto, 1fr),
 [*Rank*], [*self %*], [*Ir*], [*function*],
[1], [12.63], [7,541,609], [#raw("backend::cryptanalysis::trace_zero_linearized::decompose_trace_z\nero_linearized")],
[2], [10.61], [6,336,186], [#raw("backend::binary_ecc::f2m::fold_words_clmul_hw")],
[3], [4.82], [2,877,133], [#raw("backend::cryptanalysis::trace_zero_linearized::decompose_trace_z\nero_linearized")],
[4], [4.50], [2,684,144], [#raw("<backend::binary_ecc::f2m::F2mElement>::square_k_times::{closure\n#0}")],
[5], [3.69], [2,204,001], [#raw("backend::binary_ecc::f2m::reduce_words")],
[6], [3.52], [2,098,799], [#raw("backend::cryptanalysis::trace_zero_linearized::decompose_trace_z\nero_linearized")],
[7], [3.29], [1,964,433], [#raw("backend::binary_ecc::f2m::reduce_words")],
[8], [3.23], [1,931,160], [#raw("backend::binary_ecc::f2m::mul_words_hw")],
[9], [2.44], [1,455,166], [#raw("free")],
[10], [1.97], [1,176,855], [#raw("0x00000000000a0fe0")],
[11], [1.53], [914,227], [#raw("malloc")],
[12], [1.46], [873,240], [#raw("<alloc::string::String as core::iter::traits::collect::FromItera\ntor<char>>::from_iter::<core::iter::adapters::filter::Filter<cor\ne::str::iter::Chars,\n<backend::binary_ecc::f2m::F2mElement>::from_hex::{closure#0}>>")],
)

Top12 recorded entries; backend:: abbreviates full_trace_zero_backend::. Complete symbols, file/DSO attribution and all entries remain in hotspots.json. Profile self percentages are discovery attribution, not an end-to-end phase comparison [pcc:unverified].

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

