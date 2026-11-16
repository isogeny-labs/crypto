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

#table(columns: (auto, auto, auto, 1fr),
 [*Rank*], [*self %*], [*Ir*], [*function*],
[1], [9.86], [8,069,255], [#raw("backend::cryptanalysis::trace_zero_linearized::decompose_trace_z\nero_linearized")],
[2], [7.83], [6,409,964], [#raw("backend::binary_ecc::f2m::fold_words_clmul_hw")],
[3], [4.74], [3,877,931], [#raw("<backend::binary_ecc::f2m::F2mElement>::square_k_times::{closure\n#0}")],
[4], [3.67], [3,001,369], [#raw("backend::cryptanalysis::trace_zero_linearized::decompose_trace_z\nero_linearized")],
[5], [2.82], [2,305,041], [#raw("free")],
[6], [2.80], [2,288,808], [#raw("<backend::binary_ecc::f2m::F2mElement>::add")],
[7], [2.75], [2,249,863], [#raw("backend::binary_ecc::f2m::reduce_words")],
[8], [2.49], [2,035,845], [#raw("backend::binary_ecc::f2m::mul_words_hw")],
[9], [2.45], [2,005,310], [#raw("backend::binary_ecc::f2m::reduce_words")],
[10], [2.27], [1,857,624], [#raw("backend::cryptanalysis::trace_zero_linearized::decompose_trace_z\nero_linearized")],
[11], [2.20], [1,803,177], [#raw("<backend::binary_ecc::f2m::F2mElement>::mul")],
[12], [2.20], [1,801,612], [#raw("0x00000000000a0fe0")],
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

