# Rational-family continuation

Latest continuation: PRIMITIVE-HANDOFF.md (normalized complete solver).

## Goal
Establish and verify a mathematical >=10x improvement for hyperelliptic index
calculus for genus2+. The objective is active; algebra/rank gates are progress.

## Lead / hypothesis
Higher-pole rational functions admit signed subset/Walsh batches with more
anchors per relation and residual degree<=g. This may reduce anchor dimension.
Primary interpolation prior art and derivation: RATIONAL-PROTOCOL.md.
It is not a mathematical novelty assertion.

## Spec
RATIONAL-PROTOCOL.md and RATIONAL-RANK-PROTOCOL.md. Preserve their frozen caps,
selected curves and raw outputs. No performance experiment has been registered
for this construction; freeze a complete-cost protocol before one runs.

## State
Five algebra tests and17heldout-example tests pass. Finite algebra panel:
9408signed curve cases,1108complete Jacobian relation checks, plus2401arbitrary
assignments and explicit degeneracy controls. Rank screen36arms,145268visited,
5360principal checks;23rank attainments,12exhaustions and one tuple cap.
Independent rank(R)-rank(R_off) matches all36forest ranks. Reproduction script
rebuilds the library and reproduces verified counts byte-for-byte. Nine
producer-evaluated identity certificates, no proof pointer/independent issuer.
Report: RATIONAL-REPORT.pdf, native Typst+Rust SVG source, both pages inspected.

## Next steps
Evaluate cold construction and residual-filter cost before integrating timings.
Genus2H8d2attainsrank7after705visits; d0H8exhausts at0and d0H12works at714visits.
For g3/g4 smaller anchors increase visits substantially. Increasing pole degree
alone does not remove the g-2 evaluation dependency needed to force all but2
zeros onto anchors. Seek a structured way to meet that constraint or change
recombination; do not infer throughput/rank from sign capacity. Avoid another
equivalent arithmetic rewrite. Charge setup, every constructed output including
unused batch tails, normalization, orientation, target handling and final check.

## Runs in flight
None. Scientific count/algebra runs are terminal. Fresh PR CI may run after push;
poll the actual current head, do not rerun the terminal count panel on that basis.

## Stop and success conditions
Full objective requires >=10x complete cold same-target pipeline evidence at
genus2,3,4 and a justified mathematical mechanism/novelty assessment, not only
these finite identities/ranks. Keep exhausted/capped arms and any runtime losses.
Do not merge without the applicable owner workflow authorization.

The shared .resume/RESUME.md has an older prepared-screen note and is held by
another task. This handoff, RATIONAL-SESSION.md and HELDOUT-STATUS.md supersede it
for this study. Do not edit the shared file to refresh this work's state.
