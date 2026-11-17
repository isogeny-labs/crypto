#set page(paper: "a4", margin: 1.5cm)
#set text(size: 9pt)
#show raw: set text(size: 8pt)
#set table(stroke: 0.4pt + luma(180), inset: 3.5pt)
#show table: set text(size: 7.5pt)

= cpu profile: Merged index-calculus projected S3 kernel n131 uniform discovery

- status: *ok*
- command: #raw("/workspace/scratch/trace-rowmap-baseline one research/trace_zero_projected_20261115/inputs/scale-n131-d12-uniform.json kernel-lazy")
- binary: #raw("/workspace/scratch/trace-rowmap-baseline") sha256 #raw("bda296f1f344fbba…")
- commit: #raw("2043bee6aee1") (1 dirty tracked files)
- host: AMD EPYC 9V74 80-Core Processor, 1 logical CPUs, kernel 6.18.57, PMU absent, perf\_event\_paranoid 2
- valgrind: valgrind-3.24.0
- isolation: none; wall 0.622 s (profiled run: not a timing measurement)
- started: 2026-11-17T03:40:13Z

== Hotspots (callgrind, self Ir)

Total: 51775125 Ir (deterministic, simulated; not cycles).

#table(columns: 5,
  [*\#*],  [*self %*],  [*Ir*],  [*function*],  [*object*],
  [1],  [14.72],  [7620550],  [#raw("full_trace_zero_backend::cryptanalysis::trace_zero_linearized::decompose_trace_zero_linearized")],  [trace-rowmap-baseline],
  [2],  [5.54],  [2868605],  [#raw("full_trace_zero_backend::cryptanalysis::trace_zero_linearized::decompose_trace_zero_linearized")],  [null],
  [3],  [5.18],  [2684144],  [#raw("<full_trace_zero_backend::binary_ecc::f2m::F2mElement>::square_k_times::{closure#0}")],  [trace-rowmap-baseline],
  [4],  [4.67],  [2415558],  [#raw("full_trace_zero_backend::binary_ecc::f2m::fold_words_clmul_hw")],  [trace-rowmap-baseline],
  [5],  [4],  [2072131],  [#raw("full_trace_zero_backend::cryptanalysis::trace_zero_linearized::decompose_trace_zero_linearized")],  [null],
  [6],  [3.32],  [1719900],  [#raw("full_trace_zero_backend::binary_ecc::f2m::mul_words_hw")],  [trace-rowmap-baseline],
  [7],  [2.8],  [1449109],  [#raw("free")],  [libc.so.6],
  [8],  [2.58],  [1335478],  [#raw("<full_trace_zero_backend::cryptanalysis::trace_zero_word_field::WordField>::reduce")],  [trace-rowmap-baseline],
  [9],  [2.25],  [1163016],  [#raw("0x00000000000a0fe0")],  [libc.so.6],
  [10],  [1.76],  [909658],  [#raw("malloc")],  [libc.so.6],
  [11],  [1.69],  [873240],  [#raw("<alloc::string::String as core::iter::traits::collect::FromIterator<char>>::from_iter::<core::iter::adapters::filter::Filter<core::str::iter::Chars, <full_trace_zero_backend::binary_ecc::f2m::F2mElement>::from_hex::{closure#0}>>")],  [null],
  [12],  [1.58],  [815813],  [#raw("full_trace_zero_backend::binary_ecc::f2m::reduce_words")],  [trace-rowmap-baseline],
  [13],  [1.57],  [813010],  [#raw("full_trace_zero_backend::cryptanalysis::trace_zero_linearized::decompose_trace_zero_linearized")],  [null],
  [14],  [1.52],  [789183],  [#raw("full_trace_zero_backend::cryptanalysis::trace_zero_linearized::decompose_trace_zero_linearized")],  [null],
  [15],  [1.5],  [775502],  [#raw("<full_trace_zero_backend::binary_ecc::f2m::F2mElement>::square_k_times")],  [trace-rowmap-baseline],
  [16],  [1.47],  [758919],  [#raw("full_trace_zero_backend::cryptanalysis::trace_zero_linearized::decompose_trace_zero_linearized")],  [null],
  [17],  [1.4],  [727135],  [#raw("full_trace_zero_backend::binary_ecc::f2m::reduce_words")],  [null],
  [18],  [1.31],  [679798],  [#raw("<num_bigint::biguint::BigUint as num_traits::Num>::from_str_radix")],  [trace-rowmap-baseline],
  [19],  [1.09],  [563350],  [#raw("0x0000000000162880")],  [libc.so.6],
  [20],  [1.06],  [546768],  [#raw("full_trace_zero_backend::cryptanalysis::trace_zero_linearized::decompose_trace_zero_linearized")],  [null],
  [21],  [1.01],  [525390],  [#raw("full_trace_zero_backend::cryptanalysis::trace_zero_linearized::decompose_trace_zero_linearized")],  [null],
  [22],  [0.97],  [504613],  [#raw("<full_trace_zero_backend::cryptanalysis::trace_zero_word_field::WordField>::residuals")],  [trace-rowmap-baseline],
  [23],  [0.89],  [459573],  [#raw("core::str::converts::from_utf8")],  [null],
  [24],  [0.86],  [444290],  [#raw("0x0000000000163280")],  [libc.so.6],
  [25],  [0.84],  [435689],  [#raw("<full_trace_zero_backend::cryptanalysis::trace_zero_word_field::WordField>::admit")],  [trace-rowmap-baseline],
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

