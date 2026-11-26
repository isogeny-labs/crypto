# Literature and repository receipt

Checked before candidate implementation on 2026-11-18.

## Knowledge-base receipt

The configured research-KB MCP tools are absent. The sibling
`crypto-autoresearcher` checkout contains `scripts/research-kb` and the lead
atlas only as `VLT1` encrypted blobs in this runtime, so neither the wrapper
nor individual lead records can be read. Receipt: **KB unavailable**. This is
not evidence of novelty or absence.

The user-supplied atlas review already identifies the relevant obstruction
classes: generic k-SUM/table methods, bounded-degree algebra, and the open
non-enumerative `S_5`/relation-oracle lane. This experiment changes the
positive extraction mechanism inside the bounded-degree algebra lane; it does
not evade or disprove the k-SUM obstruction.

## Repository sources read

- `research/prime31_ic_20261115/rr9/RR9_D9_BLACKBOX_RESULT_20261117.md`:
  two exact public negative degree-nine unit certificates; the missing next
  step is sign-free positive extraction.
- `research/prime31_ic_20261115/native/examples/prime31_rr9_public_macaulay.rs`:
  degree-five dense normal-form extraction already exists and is retained as
  a small-degree control. It does not recover the registered positive at the
  degree-nine sparse boundary.
- `research/prime31_ic_20261115/native/examples/prime31_rr9_d9_blackbox_core.rs`:
  the existing matrix-free `M^T D M` and scalar BM implementation is reused.
- `research/postgb_linear_stage_20261026` and the Semaev-oracle replacement
  literature receipt: multiplication-matrix/Krylov extraction after a basis is
  already a known lane. The proposed test instead asks whether the raw sparse
  Macaulay right kernel is one-dimensional and already an evaluation vector.
- PRs #4604, #4623, #4647 and #4662: exact negative certificates, complete
  generic-equivalent prime31 recovery, negation-quotient engineering, and the
  multi-target four-sum comparison. None supplies this public-only positive
  RR9 extraction or a cold one-target rho result.

## Primary methods

- Douglas Wiedemann, “Solving Sparse Linear Equations Over Finite Fields,”
  IEEE Transactions on Information Theory 32(1), 1986,
  DOI `10.1109/TIT.1986.1057137`. Scalar Krylov/Berlekamp--Massey is used only
  to propose an annihilating polynomial; the candidate vector is checked by
  direct multiplication.
- Jean-Guillaume Dumas, Erich Kaltofen, Emmanuel Thome, and Gilles Villard,
  “Linear Time Interactive Certificates for the Minimal Polynomial and the
  Determinant of a Sparse Matrix,” arXiv `1507.01083`. Relevant to the existing
  one-sided sparse rank certificate, not a novelty claim for this experiment.
- Fabrice Rouillier, “Solving Zero-Dimensional Systems Through the Rational
  Univariate Representation,” AAECC 9 (1999), DOI
  `10.1007/s002000050114`. General quotient extraction remains the fallback
  when the kernel has dimension greater than one.
- Petit--Kosters--Messeng (PKC 2016), DOI
  `10.1007/978-3-662-49387-8_1`, and Amadori--Pintore--Sala (FFA 2018), DOI
  `10.1016/j.ffa.2018.01.009`, remain the prime-field index-calculus context.

The elementary extraction identity is scoped narrowly: if the directly checked right
kernel is one-dimensional and contains an affine evaluation vector with
nonzero constant coordinate, every nonzero kernel vector normalizes to that
evaluation vector. No broader asymptotic or novelty claim is attached.
