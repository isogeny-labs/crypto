#set page(width: 8.5in, height: 11in, margin: 0.7in)
#set text(font: "Liberation Sans", size: 10pt)
#let green = rgb("176b5d")
= Weighted sextic: exact ordinary Z = 1 quotient

*Scope.* Pinned Lean proves, over every field and for arbitrary four
coefficients, an algebra equivalence
`(K[X,Y,Z]/(F))/(class(Z)-1) ≃ K[x,y]/(y²-f(x))`.
The kernel of the previously constructed specialization is exactly
`(class(Z)-1)`. The proof uses both algebra-map round trips.

#image("weighted-proj-kernel.svg", width: 100%)

*Compiler receipt.* Lean 4.34.1, Mathlib
`d13f23b723b8a846827a245b89c10fc7d3f11612`; exit 0. Four audited
theorems use only `propext`, `Classical.choice`, and `Quot.sound`. Source
SHA-256: `264de1ec42c341dab220ccddffe116a5c66fbc680b86f93ad5bd2f77b641a53c`.
Log SHA-256: `4f0420d7d71c1e40a3075c9374fd7d60c8b6a6c2785d4a2c1ce8ca63fa007dc6`.

*Limit.* This is an ordinary quotient result. A weighted grading, the
localized degree-zero ring of the Proj chart, infinity-chart identification,
smooth projective completion, and Pic⁰ transfer remain open. No operation
count or cost ratio changes. The full replay command and standardized rows
are in `WEIGHTED_PROJ_KERNEL.md` and `weighted-proj-kernel.csv`.
