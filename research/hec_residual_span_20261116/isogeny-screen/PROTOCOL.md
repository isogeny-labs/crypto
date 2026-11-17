# Exact genus-three rational kernel qualification

Frozen before execution. Inputs are unchanged retained (p,c)=(113,3),(421,2).
Rust std-only; no target search or performance measurement. Once-only execution,
raw stdout/stderr/exit retained without overwrite. Git checkpoint pushed before run.

Enumerate every dimension-three subspace of F2^7/<all-ones> using all independent
triples of its 63 nonzero elements and deduplicate sorted eight-element spans.
Require 1395 subspaces, 135 isotropic under the branch-point Weil form,
and exactly two stable under the seven-cycle. Require both stable subspaces
isotropic and equal to the kernels of T+T2+T4 and T3+T5+T6.
Check integer cyclic product and F2 cubic factorization independently.
For each model require q=1 mod7, (-c)^((p-1)/7)!=1 and no finite root.
With mu7 rational, Kummer theory makes the nonseventh-power test an
irreducibility certificate for X7+c. Failure retains the model and rejects
application of the conditional theorem; no replacement fixtures or rerun.

This qualifies the finite algebra and the hypotheses of DERIVATION.md. It does
not mechanically verify algebraic geometry, construct the isogenies, benchmark
an algorithm, or establish a tenfold gain. The standard polarization argument
remains a written proof obligation subject to independent review.
