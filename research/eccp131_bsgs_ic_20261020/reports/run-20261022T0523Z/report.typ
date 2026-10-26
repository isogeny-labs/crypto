#set page(paper: "a4", margin: (x: 16mm, y: 14mm))
#set text(font: "Libertinus Serif", size: 8.5pt)
#set par(justify: true)

= ECCp-131 checked three-point relation diagnostic

#text(size: 7.5pt)[Run `run-20261022T0523Z` · 2026-10-22 UTC · exact Certicom ECCp-131 · observations only]

The frozen protocol compared the existing two-point small-x relation oracle with a checked three-point extension. Both arms used the published curve, generator and target, B=128 base abscissae, 32 natural setup candidates, 16 target candidates, seeds 131 and 137, and one Rayon thread. The factor-base digest was identical in all four processes. The three-point extension subtracts each signed base point, probes the complete pair table, and verifies every returned EC identity. No cubic table is held.

#figure(image("arity_cost.svg", width: 100%), caption: [Measured cold process and exact-probe intervals, seconds. Two paired seeds per arity. Source: `raw/manifest.json`; rendering: `plot.py`.])

#table(
  columns: (auto, auto, auto, auto, auto, auto),
  align: (right, right, right, right, right, right),
  inset: 3pt,
  [Seed], [Arity], [Planted], [Natural rows], [Rank], [Cold wall s],
  [131], [2], [8/8], [0], [0], [0.11920],
  [131], [3], [8/8], [0], [0], [0.17503],
  [137], [2], [8/8], [0], [0], [0.11962],
  [137], [3], [8/8], [0], [0], [0.16049],
)

All four processes completed within the 300-second watchdog. Setup and target ranks were zero; target-log determination remained false. Median probe intervals were 0.06336 and 0.12744 seconds for arity 2 and 3; median cold walls were 0.11941 and 0.16776 seconds. These L0 observations include host contention and do not measure a complete solution. The native `ecbench` prime instance uses 64-bit fields and is a distinct control, not an exact ECCp-131 cost record.

The exact-curve release test found a checked three-point relation outside the B16 two-point set. Independent PARI/GP 2.17.3 confirmed that property: 256 distinct pair-sum abscissae and a three-point abscissa outside them (`pari-attempt2.stdout.txt`). A small-prime regression confirmed three-point natural rows feed rank and target reconstruction. The first GP script attempt failed to parse multiline loops, despite process exit zero; its raw stderr remains in this report. All three targeted Rust release tests passed. The corrected PARI run had no GP error marker.

At B128, the ordered signed triple list covers at most `8B³=2^24` group points, bounding a uniform nonzero candidate's hit probability by about `2^-106.1875`. At the CLI's B8192 cap the bound is about `2^-88.1875`; `B+1` independent rows would optimistically need about `2^101.188` candidates in expectation. The unshifted published point is fixed, so this uniform model does not assign it a probability. These are method-specific support bounds. The JSON fixes seeds but omits a per-candidate miss transcript, so independent candidate-by-candidate replay is not asserted. The exact natural-relation, full-rank, published-target scalar and complete cold-cost criteria remain open in `problems/eccp131-full-parameter-index-calculus.md`.

#text(size: 7pt)[Raw records and provenance: `raw/manifest.json`, `raw/*.stdout.json`, `raw/*.stderr.txt`, `host-before.json`, `build.meta.json`, `tests.meta.json`, `raw-artifacts.sha256`; source: commit `9a8a1055e0`, binary SHA-256 `2351733ada46…`. Full commands, per-run probe times, memory and limits: `REPORT.md`.]
