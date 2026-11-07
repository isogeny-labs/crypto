#set page(paper: "a4", margin: 16mm)
#set text(font: "DejaVu Sans", size: 10pt)
#set par(justify: false)
#text(size: 20pt, weight: "bold")[GPU relation-query cost breakdown]

#text(fill: rgb("555555"))[RTX 4090 · fixed 63-bit inputs · 31 October 2026]

This diagnostic replays the first eight actual queries from each of the two
large affine inputs, preserving the original bases, full right cache, filter,
and index. Five interleaved rounds yield 30 replay cells and 240 queries.
Every response matched the uninstrumented capture byte for byte.

#image("stages.svg", width: 100%)

#table(columns: (2fr, 1fr, 1fr), inset: 5pt,
  [*Instrumented block-cycle share*], [*Seed 101*], [*Seed 202*],
[Cache load + input conversion], [2.72%], [4.72%],
[Affine subtraction], [3.11%], [3.72%],
[Batch normalization (inverse tree)], [32.22%], [36.55%],
[Affine output + conversion], [1.80%], [1.98%],
[Negative filter + index lookup], [43.60%], [31.12%],
[Empty barrier / clock control], [16.54%], [21.91%],
)

*Interpretation.* These are sums of thread-zero block-cycle intervals, including
barriers, scheduling, and compiler fences. They are not GPU wall-time shares.
The empty interval is shown without subtraction. Input conversion includes the
cache load; output conversion includes affine reconstruction; lookup includes
the negative filter. No fraction is applied to the historical oracle seconds.

*Overhead and controls.* CUDA events bracket the original or staged search
kernel only. Setup, host launch gaps, and winner resolution are excluded.
Raw per-round A/A spreads and the staged/stock event ratio are reproduced on
the following page. Shared-host clocks remain L0 diagnostics.

#pagebreak()
#text(size: 17pt, weight: "bold")[Verification and provenance]

#raw("Diagnostic prefix: two 63-bit targets, eight queries each, five rounds, three arms. Block\n    cycles include barriers and scheduling; NOT additive wall time.\nseed=101 queries_per_arm=40 kernel_ms_b=6630.054074999999 kernel_ms_stages=6182.247755\n    kernel_ms_x=6617.166291 staged_over_stock=0.933365277272387\n    AA_relative_spreads=[0.0010747572281673869, 0.0017998176960455638, 0.0018353295840099568,\n    0.0026758852802064277, 0.0023413769175147287]\n  load_convert: block_cycles=226842652649 cycles_per_tile=1845.171\n    share_including_empty=0.027214\n  subtract: block_cycles=259312748053 cycles_per_tile=2109.287 share_including_empty=0.031110\n  normalize: block_cycles=2685962164722 cycles_per_tile=21848.004\n    share_including_empty=0.322234\n  affine_out: block_cycles=149861876447 cycles_per_tile=1218.998\n    share_including_empty=0.017979\n  lookup: block_cycles=3634373127293 cycles_per_tile=29562.516 share_including_empty=0.436015\n  empty: block_cycles=1379074846110 cycles_per_tile=11217.594 share_including_empty=0.165447\nseed=202 queries_per_arm=40 kernel_ms_b=3447.530571 kernel_ms_stages=3229.2192993\n    kernel_ms_x=3448.417175300001 staged_over_stock=0.936555617328344\n    AA_relative_spreads=[0.0002507535540454614, 0.001266927598094058, 0.0001240852263306903,\n    0.00047302482779062826, 0.0003653041988215758]\n  load_convert: block_cycles=198052529242 cycles_per_tile=2849.916\n    share_including_empty=0.047216\n  subtract: block_cycles=155936730996 cycles_per_tile=2243.883 share_including_empty=0.037176\n  normalize: block_cycles=1533109434379 cycles_per_tile=22060.982\n    share_including_empty=0.365498\n  affine_out: block_cycles=82883701483 cycles_per_tile=1192.671 share_including_empty=0.019760\n  lookup: block_cycles=1305443417065 cycles_per_tile=18784.937 share_including_empty=0.311221\n  empty: block_cycles=919155830430 cycles_per_tile=13226.375 share_including_empty=0.219129", block: true)

*Validation.* All 30 cells / 240 queries passed exact reply, table identity,
exit and coverage checks. All 37,584 arithmetic cases passed on host and GPU;
315 differential query cases included 261 hits and 54 exhausted misses.
Memcheck, racecheck and synccheck reported zero errors; 29 Rust tests and
Clippy passed. Native analysis recomputed byte-identically; canonical export
and coverage/file checks reported zero findings. The owned pod was deleted.

*Profiler failure.* All five optional external profiler invocations exited 2
before collection because the image lacked gawk. No Nsight counters or timeline
values were obtained in v1. The separate version-2 protocol repairs prerequisites
and attempts only these external profiles; it does not replace this panel.

*Perturbation.* Staged/stock kernel-event ratios were 0.9334 and 0.9366.
The diagnostic kernel specializes the full-cache path and changes code generation
as well as adding instrumentation: 58 registers / 56-byte stack versus stock
48 registers / 120-byte stack; neither reports spills. These ratios do not isolate
clock overhead or establish a production improvement. The next comparison needs
an uninstrumented full-cache specialization on matched complete workloads.

*Scope.* This is a fixed query-prefix diagnostic, not another complete recovery
panel. Instrumentation changes synchronization and register use. The preserved
ptxas logs and any supported Nsight counters help assess that perturbation.
The source and all data remain reproducible from the linked study artifacts.

#text(size: 8pt)[Source: ffa0da3efaf222bdcc958d45013aa469d5fbecf7
Protocol: 8b77f5a916607e7f7a0d094d194515d55457823759b9114b97808f7c6416bb17
Study: research/explicit_relations_profile_20261031
Raw data: results-v1/export.json and archive-manifest.json]
