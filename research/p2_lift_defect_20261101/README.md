# H110 — canonical p²-lift defect cocycle

Status: **preregistered; implementation pending** (2026-11-01).

H110 tests a mechanism distinct from the prior character, theta, cover, and
division-polynomial experiments: whether the nonhomomorphic defect of a
deterministic Hensel section into `E(Z/p²Z)` is a uniformly low-degree
polynomial in the unknown scalar.

The section is explicitly **not** treated as a group homomorphism. Reduction,
kernel membership, the first formal coordinate, coordinate dependence, and
the unique torsion-lift correction are separate obligations.

- [Frozen protocol](PROTOCOL.md)
- [Transfer/cost assessment](assessment.json)
- [Editable typed correspondence](figures/h110-p2-lift-defect-flow.svg)
- [Rendered preregistration report](h110-p2-lift-defect-preregistration.pdf)
- Paired reference PR: to be linked after creation.

Frozen workload: seven curves, four generator relabelings, every nonzero target
(9,096 records), 32 randomized-section/permutation controls, coordinate-
isomorphic controls, and torsion-lift zero controls.

No correctness, novelty, ECDLP algorithm, or subexponential runtime claim is
made by this preregistration.
