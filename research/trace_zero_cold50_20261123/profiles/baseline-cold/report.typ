#set page(paper: "a4", margin: 1.5cm)
#set text(size: 9pt)
#show raw: set text(size: 8pt)
#set table(stroke: 0.4pt + luma(180), inset: 3.5pt)
#show table: set text(size: 7.5pt)

= cpu profile: Exact-half-trace incumbent, fixed uniform ECC2K query

- status: *ok*
- command: #raw("research/trace_zero_halftrace_elimination_20261123/bin/elimination one research/trace_zero_projected_20261115/inputs/scale-n131-d12-uniform.json elimination")
- binary: #raw("/workspace/crypto/research/trace_zero_halftrace_elimination_20261123/bin/elimination") sha256 #raw("504a9c6a6ae668a0…")
- commit: #raw("ad5f91c4b439") (0 dirty tracked files)
- host: INTEL(R) XEON(R) PLATINUM 8573C, 1 logical CPUs, kernel 6.18.57, PMU absent, perf\_event\_paranoid 2
- valgrind: valgrind-3.24.0
- isolation: none; wall 0.586 s (profiled run: not a timing measurement)
- started: 2026-11-23T23:03:34Z

== Hotspots (callgrind, self Ir)

Total: 34987635 Ir (deterministic, simulated; not cycles).

#table(columns: 5,
  [*\#*],  [*self %*],  [*Ir*],  [*function*],  [*object*],
  [1],  [11.09],  [3880904],  [#raw("full_trace_zero_backend::cryptanalysis::trace_zero_linearized::decompose_trace_zero_linearized")],  [elimination],
  [2],  [5.36],  [1873656],  [#raw("full_trace_zero_backend::binary_ecc::f2m::mul_words_hw")],  [elimination],
  [3],  [4.72],  [1651088],  [#raw("<full_trace_zero_backend::cryptanalysis::trace_zero_word_field::WordField>::reduce")],  [elimination],
  [4],  [4.14],  [1449271],  [#raw("free")],  [libc.so.6],
  [5],  [3.33],  [1163720],  [#raw("0x00000000000a0fe0")],  [libc.so.6],
  [6],  [2.6],  [909665],  [#raw("malloc")],  [libc.so.6],
  [7],  [2.5],  [873240],  [#raw("<alloc::string::String as core::iter::traits::collect::FromIterator<char>>::from_iter::<core::iter::adapters::filter::Filter<core::str::iter::Chars, <full_trace_zero_backend::binary_ecc::f2m::F2mElement>::from_hex::{closure#0}>>")],  [null],
  [8],  [2.32],  [813010],  [#raw("full_trace_zero_backend::cryptanalysis::trace_zero_linearized::decompose_trace_zero_linearized")],  [null],
  [9],  [1.94],  [679798],  [#raw("<num_bigint::biguint::BigUint as num_traits::Num>::from_str_radix")],  [elimination],
  [10],  [1.91],  [666885],  [#raw("full_trace_zero_backend::binary_ecc::f2m::trace_tables")],  [elimination],
  [11],  [1.87],  [654012],  [#raw("full_trace_zero_backend::binary_ecc::f2m::mul_words_hw")],  [null],
  [12],  [1.84],  [644595],  [#raw("full_trace_zero_backend::cryptanalysis::trace_zero_linearized::decompose_trace_zero_linearized")],  [null],
  [13],  [1.56],  [544950],  [#raw("full_trace_zero_backend::cryptanalysis::trace_zero_linearized::decompose_trace_zero_linearized")],  [null],
  [14],  [1.44],  [504613],  [#raw("<full_trace_zero_backend::cryptanalysis::trace_zero_word_field::WordField>::residuals")],  [elimination],
  [15],  [1.31],  [459576],  [#raw("full_trace_zero_backend::binary_ecc::f2m::mul_words_into")],  [elimination],
  [16],  [1.31],  [459551],  [#raw("core::str::converts::from_utf8")],  [null],
  [17],  [1.27],  [445955],  [#raw("full_trace_zero_backend::cryptanalysis::trace_zero_linearized::decompose_trace_zero_linearized")],  [null],
  [18],  [1.25],  [435689],  [#raw("<full_trace_zero_backend::cryptanalysis::trace_zero_word_field::WordField>::admit")],  [elimination],
  [19],  [1.2],  [418380],  [#raw("0x00000000000a02e0")],  [libc.so.6],
  [20],  [1.17],  [408936],  [#raw("0x0000000000162880")],  [libc.so.6],
  [21],  [1.11],  [388872],  [#raw("full_trace_zero_backend::binary_ecc::f2m::mul_words_hw")],  [null],
  [22],  [1.09],  [382144],  [#raw("realloc")],  [libc.so.6],
  [23],  [1.05],  [368976],  [#raw("0x000000000009ebd0")],  [libc.so.6],
  [24],  [1.03],  [359498],  [#raw("<alloc::string::String as core::iter::traits::collect::FromIterator<char>>::from_iter::<core::iter::adapters::filter::Filter<core::str::iter::Chars, <full_trace_zero_backend::binary_ecc::f2m::F2mElement>::from_hex::{closure#0}>>")],  [null],
  [25],  [1.02],  [356218],  [#raw("0x00000000000a20c0")],  [libc.so.6],
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

