#set page(paper: "a4", margin: 1.5cm)
#set text(size: 9pt)
#show raw: set text(size: 8pt)
#set table(stroke: 0.4pt + luma(180), inset: 3.5pt)
#show table: set text(size: 7.5pt)

= cpu profile: Identical baseline A/A

- status: *ok*
- command: #raw("research/trace_zero_projection_setup_20261123/native/target/release/trace-zero-projection-setup one research/trace_zero_projected_20261115/inputs/scale-n131-d12-uniform.json adjoint")
- binary: #raw("/workspace/crypto/research/trace_zero_projection_setup_20261123/native/target/release/trace-zero-projection-setup") sha256 #raw("7a1ffd3115e52049…")
- commit: #raw("065176db5621") (0 dirty tracked files)
- host: INTEL(R) XEON(R) PLATINUM 8573C, 1 logical CPUs, kernel 6.18.57, PMU absent, perf\_event\_paranoid 2
- valgrind: valgrind-3.24.0
- isolation: none; wall 0.587 s (profiled run: not a timing measurement)
- started: 2026-11-23T18:53:10Z

== Hotspots (callgrind, self Ir)

Total: 44038405 Ir (deterministic, simulated; not cycles).

#table(columns: 5,
  [*\#*],  [*self %*],  [*Ir*],  [*function*],  [*object*],
  [1],  [8.71],  [3834328],  [#raw("full_trace_zero_backend::cryptanalysis::trace_zero_linearized::decompose_trace_zero_linearized")],  [trace-zero-projection-setup],
  [2],  [6.02],  [2649828],  [#raw("<full_trace_zero_backend::binary_ecc::f2m::F2mElement>::square_k_times::{closure#0}")],  [trace-zero-projection-setup],
  [3],  [5.49],  [2415558],  [#raw("full_trace_zero_backend::binary_ecc::f2m::fold_words_clmul_hw")],  [trace-zero-projection-setup],
  [4],  [4.25],  [1873656],  [#raw("full_trace_zero_backend::binary_ecc::f2m::mul_words_hw")],  [trace-zero-projection-setup],
  [5],  [3.75],  [1651088],  [#raw("<full_trace_zero_backend::cryptanalysis::trace_zero_word_field::WordField>::reduce")],  [trace-zero-projection-setup],
  [6],  [3.29],  [1449219],  [#raw("free")],  [libc.so.6],
  [7],  [2.64],  [1162999],  [#raw("0x00000000000a0fe0")],  [libc.so.6],
  [8],  [2.07],  [909660],  [#raw("malloc")],  [libc.so.6],
  [9],  [1.98],  [873240],  [#raw("<alloc::string::String as core::iter::traits::collect::FromIterator<char>>::from_iter::<core::iter::adapters::filter::Filter<core::str::iter::Chars, <full_trace_zero_backend::binary_ecc::f2m::F2mElement>::from_hex::{closure#0}>>")],  [null],
  [10],  [1.85],  [815813],  [#raw("full_trace_zero_backend::binary_ecc::f2m::reduce_words")],  [trace-zero-projection-setup],
  [11],  [1.85],  [813010],  [#raw("full_trace_zero_backend::cryptanalysis::trace_zero_linearized::decompose_trace_zero_linearized")],  [null],
  [12],  [1.65],  [727135],  [#raw("full_trace_zero_backend::binary_ecc::f2m::reduce_words")],  [null],
  [13],  [1.54],  [679798],  [#raw("<num_bigint::biguint::BigUint as num_traits::Num>::from_str_radix")],  [trace-zero-projection-setup],
  [14],  [1.51],  [664716],  [#raw("<full_trace_zero_backend::binary_ecc::f2m::F2mElement>::square_k_times")],  [trace-zero-projection-setup],
  [15],  [1.49],  [654012],  [#raw("full_trace_zero_backend::binary_ecc::f2m::mul_words_hw")],  [null],
  [16],  [1.45],  [640609],  [#raw("full_trace_zero_backend::cryptanalysis::trace_zero_linearized::decompose_trace_zero_linearized")],  [null],
  [17],  [1.31],  [576576],  [#raw("0x0000000000162880")],  [libc.so.6],
  [18],  [1.19],  [525390],  [#raw("full_trace_zero_backend::cryptanalysis::trace_zero_linearized::decompose_trace_zero_linearized")],  [null],
  [19],  [1.15],  [504613],  [#raw("<full_trace_zero_backend::cryptanalysis::trace_zero_word_field::WordField>::residuals")],  [trace-zero-projection-setup],
  [20],  [1.04],  [459616],  [#raw("core::str::converts::from_utf8")],  [null],
  [21],  [1.04],  [459576],  [#raw("full_trace_zero_backend::binary_ecc::f2m::mul_words_into")],  [trace-zero-projection-setup],
  [22],  [0.99],  [435689],  [#raw("<full_trace_zero_backend::cryptanalysis::trace_zero_word_field::WordField>::admit")],  [trace-zero-projection-setup],
  [23],  [0.98],  [430502],  [#raw("full_trace_zero_backend::cryptanalysis::trace_zero_linearized::decompose_trace_zero_linearized")],  [null],
  [24],  [0.95],  [418252],  [#raw("0x00000000000a02e0")],  [libc.so.6],
  [25],  [0.89],  [393930],  [#raw("<full_trace_zero_backend::binary_ecc::f2m::F2mElement>::mask_in_place")],  [trace-zero-projection-setup],
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

