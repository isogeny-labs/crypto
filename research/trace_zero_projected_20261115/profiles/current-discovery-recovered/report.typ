#set page(paper: "a4", margin: 1.5cm)
#set text(size: 9pt)
#show raw: set text(size: 8pt)
#set table(stroke: 0.4pt + luma(180), inset: 3.5pt)
#show table: set text(size: 7.5pt)

= cpu profile: Optimized v1 cold n131 d12 uniform, discovery only

- status: *ok*
- command: #raw("/workspace/scratch/trace-fast-baseline-v1 one research/trace_zero_linearized_20261115/workload.json scale-n131-d12-uniform candidate")
- binary: #raw("/workspace/scratch/trace-fast-baseline-v1") sha256 #raw("15b6156c1e95c922…")
- commit: #raw("a14a654255c7") (0 dirty tracked files)
- host: AMD EPYC 9V74 80-Core Processor, 1 logical CPUs, kernel 6.18.57, PMU absent, perf\_event\_paranoid 2
- valgrind: valgrind-3.24.0
- isolation: none; wall 2.321 s (profiled run: not a timing measurement)
- started: 2026-11-15T22:34:46Z

== Hotspots (callgrind, self Ir)

Total: 282128592 Ir (deterministic, simulated; not cycles).

#table(columns: (auto, auto, auto, 1fr),
 [*Rank*], [*self %*], [*Ir*], [*function*],
[1], [7.69], [21,686,318], [#raw("0x00000000000a0fe0")],
[2], [6.66], [18,794,660], [#raw("0x0000000000163280")],
[3], [5.33], [15,029,305], [#raw("backend::binary_ecc::f2m::fold_words_clmul_hw")],
[4], [4.25], [11,989,178], [#raw("free")],
[5], [4.16], [11,732,070], [#raw("backend::binary_ecc::f2m::mul_words_hw")],
[6], [4.02], [11,353,123], [#raw("backend::cryptanalysis::trace_zero_linearized::decompose_trace_z\nero_linearized")],
[7], [3.49], [9,832,592], [#raw("<backend::binary_ecc::f2m::F2mElement>::mul")],
[8], [3.04], [8,572,995], [#raw("<serde_json::value::Value as\nserde_core::de::Deserialize>::deserialize::<&mut\nserde_json::de::Deserializer<serde_json::read::SliceRead>>'2")],
[9], [2.81], [7,937,299], [#raw("malloc")],
[10], [2.58], [7,290,377], [#raw("0x00000000000a02e0")],
[11], [2.43], [6,864,031], [#raw("backend::binary_ecc::f2m::reduce_words")],
[12], [2.29], [6,458,320], [#raw("0x000000000009ebd0")],
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

