#set page(paper: "us-letter", margin: (x: 0.82in, y: 0.72in))
#set text(font: "Helvetica Neue", size: 10pt)
#set heading(numbering: none)

= K13 relation scan: paired SIMD route

_Before-data source review, 2026-11-15 PST. No scientific timing or outcome row._

The fixed Koblitz E0 study over the binary field of degree 13 uses a
33-point signed base: one self-negative point, then 16 adjacent sign pairs.
The candidate keeps the first point separate and scans four complete
eight-point pair blocks. This lets an x86 host try the library's existing
paired SIMD kernel; a declined call takes the existing lazy fallback.

#figure(
  image("simd-pair-scan.svg", width: 100%),
  caption: [Source routes and validation status; no measured performance values.],
)

== Correctness boundary

A one-time layout check requires reciprocal sign indices, equal abscissae
and finite points. The paired kernel returns interleaved abscissae and
slopes; the adapter uses them as ordered lazy sums before the same table
lookups. Exceptional sums, unavailable SIMD and other layouts use the
previous path. The first witness and completed-block addition charge
remain part of the fixed correctness gate.

== Checks and next action

The ARM release parity test passed on 263 K13 probes, including exceptional
inputs. An x86 cross-target build checked the SIMD-specific code. The
AVX-512 route has not executed on this revision, so its runtime parity and
whole-process wall cost remain open. Amendment 6 and a new source receipt
must precede any L2 study job. The monthly cost gate must clear first.

Internal sources: `src/bin/ecbench/pdp3_koblitz.rs`,
`src/cryptanalysis/koblitz_fast.rs`, `AMENDMENT-6.md`, and the exact
provenance and inference limits in `SIMD_PAIR_SCAN_REVIEW_20261115.md`.
