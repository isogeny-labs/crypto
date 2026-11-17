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

#table(columns: 5,
  [*\#*],  [*self %*],  [*Ir*],  [*function*],  [*object*],
  [1],  [14.51],  [7557664],  [#raw("trace_zero_rowmap::baseline::decompose_trace_zero_linearized")],  [trace-zero-rowmap],
  [2],  [5.46],  [2842971],  [#raw("trace_zero_rowmap::baseline::decompose_trace_zero_linearized")],  [null],
  [3],  [5.35],  [2787092],  [#raw("<full_trace_zero_backend::binary_ecc::f2m::F2mElement>::square_k_times::{closure#0}")],  [trace-zero-rowmap],
  [4],  [4.64],  [2415558],  [#raw("full_trace_zero_backend::binary_ecc::f2m::fold_words_clmul_hw")],  [trace-zero-rowmap],
  [5],  [3.95],  [2055936],  [#raw("trace_zero_rowmap::baseline::decompose_trace_zero_linearized")],  [null],
  [6],  [3.3],  [1719900],  [#raw("full_trace_zero_backend::binary_ecc::f2m::mul_words_hw")],  [trace-zero-rowmap],
  [7],  [2.78],  [1449106],  [#raw("free")],  [libc.so.6],
  [8],  [2.56],  [1335478],  [#raw("<full_trace_zero_backend::cryptanalysis::trace_zero_word_field::WordField>::reduce")],  [trace-zero-rowmap],
  [9],  [2.23],  [1162386],  [#raw("0x00000000000a0fe0")],  [libc.so.6],
  [10],  [1.75],  [909644],  [#raw("malloc")],  [libc.so.6],
  [11],  [1.68],  [873240],  [#raw("<alloc::string::String as core::iter::traits::collect::FromIterator<char>>::from_iter::<core::iter::adapters::filter::Filter<core::str::iter::Chars, <full_trace_zero_backend::binary_ecc::f2m::F2mElement>::from_hex::{closure#0}>>")],  [null],
  [12],  [1.57],  [815813],  [#raw("full_trace_zero_backend::binary_ecc::f2m::reduce_words")],  [trace-zero-rowmap],
  [13],  [1.53],  [797058],  [#raw("trace_zero_rowmap::baseline::decompose_trace_zero_linearized")],  [null],
  [14],  [1.52],  [792546],  [#raw("<full_trace_zero_backend::binary_ecc::f2m::F2mElement>::square_k_times")],  [trace-zero-rowmap],
  [15],  [1.43],  [746177],  [#raw("trace_zero_rowmap::baseline::decompose_trace_zero_linearized")],  [null],
  [16],  [1.4],  [727135],  [#raw("full_trace_zero_backend::binary_ecc::f2m::reduce_words")],  [null],
  [17],  [1.39],  [723779],  [#raw("trace_zero_rowmap::baseline::decompose_trace_zero_linearized")],  [null],
  [18],  [1.31],  [679798],  [#raw("<num_bigint::biguint::BigUint as num_traits::Num>::from_str_radix")],  [trace-zero-rowmap],
  [19],  [1.15],  [600591],  [#raw("trace_zero_rowmap::baseline::decompose_trace_zero_linearized")],  [null],
  [20],  [1.08],  [563253],  [#raw("0x0000000000162880")],  [libc.so.6],
  [21],  [1.01],  [525390],  [#raw("trace_zero_rowmap::baseline::decompose_trace_zero_linearized")],  [null],
  [22],  [0.97],  [504613],  [#raw("<full_trace_zero_backend::cryptanalysis::trace_zero_word_field::WordField>::residuals")],  [trace-zero-rowmap],
  [23],  [0.88],  [459499],  [#raw("core::str::converts::from_utf8")],  [null],
  [24],  [0.84],  [435689],  [#raw("<full_trace_zero_backend::cryptanalysis::trace_zero_word_field::WordField>::admit")],  [trace-zero-rowmap],
  [25],  [0.83],  [430227],  [#raw("0x0000000000163280")],  [libc.so.6],
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

