#set page(paper: "a4", margin: 1.5cm)
#set text(size: 9pt)
#show raw: set text(size: 8pt)
#set table(stroke: 0.4pt + luma(180), inset: 3.5pt)
#show table: set text(size: 7.5pt)

= Historical discovery: retained optimized executable

- status: *ok*
- command: #raw("/workspace/scratch/trace-rowmap-baseline one research/trace_zero_projected_20261115/inputs/scale-n131-d12-uniform.json kernel-lazy")
- binary: #raw("/workspace/scratch/trace-rowmap-baseline") sha256 #raw("bda296f1f344fbba…")
- commit: #raw("2043bee6aee1") (1 dirty tracked files)
- host: AMD EPYC 9V74 80-Core Processor, 1 logical CPUs, kernel 6.18.57, PMU absent, perf\_event\_paranoid 2
- valgrind: valgrind-3.24.0
- isolation: none; wall 0.622 s (profiled run: not a timing measurement)
- started: 2026-11-17T03:40:13Z

*Build provenance:* retained bda296f1 executable from the preceding merged study. The displayed working-tree commit is profiler context, not its build source. A second profile uses fresh current arithmetic; this historical profile is discovery only [pcc:unverified].

== Hotspots (callgrind, self Ir)

Total: 51775125 Ir (deterministic, simulated; not cycles).

#table(columns: (auto,auto,auto,1fr),
 [*Rank*], [*self %*], [*Ir*], [*function*],
[1], [14.72], [7,620,550], [#raw("backend::cryptanalysis::trace_zero_linearized::decompose_trace_z\nero_linearized")],
[2], [5.54], [2,868,605], [#raw("backend::cryptanalysis::trace_zero_linearized::decompose_trace_z\nero_linearized")],
[3], [5.18], [2,684,144], [#raw("<backend::binary_ecc::f2m::F2mElement>::square_k_times::{closure\n#0}")],
[4], [4.67], [2,415,558], [#raw("backend::binary_ecc::f2m::fold_words_clmul_hw")],
[5], [4.00], [2,072,131], [#raw("backend::cryptanalysis::trace_zero_linearized::decompose_trace_z\nero_linearized")],
[6], [3.32], [1,719,900], [#raw("backend::binary_ecc::f2m::mul_words_hw")],
[7], [2.80], [1,449,109], [#raw("free")],
[8], [2.58], [1,335,478], [#raw("<backend::cryptanalysis::trace_zero_word_field::WordField>::redu\nce")],
[9], [2.25], [1,163,016], [#raw("0x00000000000a0fe0")],
[10], [1.76], [909,658], [#raw("malloc")],
[11], [1.69], [873,240], [#raw("<alloc::string::String as core::iter::traits::collect::FromItera\ntor<char>>::from_iter::<core::iter::adapters::filter::Filter<cor\ne::str::iter::Chars,\n<backend::binary_ecc::f2m::F2mElement>::from_hex::{closure#0}>>")],
[12], [1.58], [815,813], [#raw("backend::binary_ecc::f2m::reduce_words")],
)

Top12 recorded entries; backend:: abbreviates full_trace_zero_backend::. Complete function/file/DSO attribution remains in hotspots.json. Discovery attribution is not a comparative timing result [pcc:unverified].

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

