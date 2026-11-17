# Conditional uniform collection recurrence

Nine new named theorems in [UniformCollection.lean](UniformCollection.lean).
The unmodified scoped repository gate passes **16 declarations** (nine named,
seven generated), zero failing/extra trusted. Only `propext`, `Classical.choice`
and `Quot.sound` occur. Lean4.34.1, Mathlib
`d13f23b723b8a846827a245b89c10fc7d3f11612`.

`RankSystem` includes positive query count, mass conservation, strictly positive
progress and decreasing height. Improving successors form a **list with query
multiplicity**. `Solves` explicitly assumes the terminal/first-step equations.
The results prove their rearrangement and uniqueness, scaling, conditional
degree-request factorization, the one-rank `E=1/p` special case and a sufficient
lower-degree/no-larger-call comparison.

The actual binding to the native536-state census, iid stopping-time semantics,
absorption/integrability and a first-step probability theorem remain open.
Larger initial mean rank is insufficient to assert fewer collection calls.
The native solver/entire experiment is not claimed universally formalized.

The original handwritten `.lean.txt` draft remains untouched. The first actual
elaboration failure and its source are in the raw validation archive; the final
successful module is not inferred from that failed attempt. Historical208 named /
269 audited declarations were not all rerun; this is a separate fresh16-declaration
scoped gate. The17 prior polynomial certificates stay unchanged; this module
introduces conditional recurrence proofs, not a new curve-polynomial claim.

[Run report](../reports/uniform-20261029/README.md) ·
[gate/raw restoration](../validation/uniform-20261029/README.md).
