#set page(paper: "a4", margin: 1.5cm)
#set text(size: 9pt)
#show raw: set text(size: 8pt)
#set table(stroke: 0.4pt + luma(180), inset: 3.5pt)
#show table: set text(size: 7.5pt)

= cpu profile: Current main arithmetic, qualified numeric S3 baseline, n131 uniform discovery

- status: *ok*
- command: #raw("research/trace_zero_rowmap_20261117/native/target/release/trace-zero-rowmap one research/trace_zero_projected_20261115/inputs/scale-n131-d12-uniform.json baseline")
- binary: #raw("/workspace/crypto-trace/research/trace_zero_rowmap_20261117/native/target/release/trace-zero-rowmap") sha256 #raw("7380fefb6583d434…")
- commit: #raw("2043bee6aee1") (3 dirty tracked files)
- host: AMD EPYC 9V74 80-Core Processor, 1 logical CPUs, kernel 6.18.57, PMU absent, perf\_event\_paranoid 2
- valgrind: valgrind-3.24.0
- isolation: none; wall 0.602 s (profiled run: not a timing measurement)
- started: 2026-11-17T03:49:06Z

== Hotspots (callgrind, self Ir)

Total: 52069788 Ir (deterministic, simulated; not cycles).

#table(columns: (auto,auto,auto,1fr),
 [*Rank*], [*self %*], [*Ir*], [*function*],
[1], [14.51], [7,557,664], [#raw("trace_zero_rowmap::baseline::decompose_trace_zero_linearized")],
[2], [5.46], [2,842,971], [#raw("trace_zero_rowmap::baseline::decompose_trace_zero_linearized")],
[3], [5.35], [2,787,092], [#raw("<backend::binary_ecc::f2m::F2mElement>::square_k_times::{closure\n#0}")],
[4], [4.64], [2,415,558], [#raw("backend::binary_ecc::f2m::fold_words_clmul_hw")],
[5], [3.95], [2,055,936], [#raw("trace_zero_rowmap::baseline::decompose_trace_zero_linearized")],
[6], [3.30], [1,719,900], [#raw("backend::binary_ecc::f2m::mul_words_hw")],
[7], [2.78], [1,449,106], [#raw("free")],
[8], [2.56], [1,335,478], [#raw("<backend::cryptanalysis::trace_zero_word_field::WordField>::redu\nce")],
[9], [2.23], [1,162,386], [#raw("0x00000000000a0fe0")],
[10], [1.75], [909,644], [#raw("malloc")],
[11], [1.68], [873,240], [#raw("<alloc::string::String as core::iter::traits::collect::FromItera\ntor<char>>::from_iter::<core::iter::adapters::filter::Filter<cor\ne::str::iter::Chars,\n<backend::binary_ecc::f2m::F2mElement>::from_hex::{closure#0}>>")],
[12], [1.57], [815,813], [#raw("backend::binary_ecc::f2m::reduce_words")],
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

