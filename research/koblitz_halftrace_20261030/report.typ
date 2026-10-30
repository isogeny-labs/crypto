#set page(paper: "a4", margin: 16mm)
#set text(size: 9.5pt, font: "DejaVu Sans")
#set heading(numbering: none)
= Native Koblitz cached lifting
2026-10-30 · Engineering implementation candidate · Generic decomposition track

#block(fill: rgb("fff0dc"), inset: 10pt, radius: 4pt)[
  *Measurement pending.* The native baseline was built before source edits.
  The mandatory S3 checkpoint upload failed because this session has no approved
  uploader. No scientific EKS job was launched. No native speed ratio is claimed.
]

#image("pipeline.svg", width: 100%)

== Implementation
Lazy, curve-local byte tables evaluate the existing odd-degree halftrace. The
explicit factor-base parameter `halftrace_tables=1` preserves the first root,
ordered points, signed orbit maps and existing relation/LA implementation.
Default recipes retain the original point-lifting path. Initialized immutable
tables are shared by clones. No field-wide table or new Python path is introduced.

== Verification and accounting
Native checks compare exhaustive small-field roots and sampled wider roots to
the generic field definition, test scope restoration and field separation, and
compare ordered point lifts including non-Koblitz coefficients. An end-to-end
fixture compares two- and three-summand known-answer solves; a second test
exercises the public ecbench pipeline and its point hash and counters. Both
tests pass, and strict library Clippy passes. Full-suite completion is tracked
separately under `development/`; development elapsed times are not benchmarks.

The frozen comparison has eight independent one-target workloads, the same
base and seed in both arms, strong-rho and IC A/A controls, six alternating
measured rounds and one warmup. EKS Spot L2 admission and independent scalar
replay are mandatory. Cache construction belongs in the cold setup cost.

== Limits and sources
The degree-31 invariant subspace has ord_31(2)=5; it does not discharge the
degree-83 confidence gate or establish transfer to ECC2K-130. Existing coarse
Artin–Schreier solve counters alone do not price the cache delta. External
arithmetic comparisons have not been measured. The scoreboard and leaderboard
remain unchanged because no admitted performance result exists.

Baseline source: `3b34d504bc13ce668e86d8e68cdf57e3fbaba89a`.
The primary binary point-recovery contract is
#link("https://www.secg.org/sec1-v2.pdf")[SEC 1 v2.0, section 2.3.4]. Existing
`src/binary_ecc/f2m.rs` supplies the native trace/halftrace precedent.
Implementation paths: `src/cryptanalysis/koblitz_fast.rs`,
`src/cryptanalysis/koblitz_index_calculus.rs`, and
`src/cryptanalysis/ic_framework/plugins.rs`.
The study's `README.md`, `PROTOCOL.md`, `spec.json` and `BASELINE.txt` retain
the detailed contract and evidence links. The exact access gap is recorded in
`problems/native-koblitz-measurement-access.md`.
