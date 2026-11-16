# R105: first x=1 Hasse jets of the marked-23 triangular source

Bind the exact R51 `marked23-triangular51.sing` donor by SHA-256. Expand
`f51(x,y)` completely at `x=1+t` and `y=z` or `y=1+z`, recording the
smallest t-order at every z-degree. Independently expand every one of the
28 rational G51 coefficients through t-degree 4 using integer generalized
binomial coefficients, with denominators powers of x (units at t=0).
Check the k=0 jets against the accepted R101 equations, and record the
q-adic orders of each lambda jet at y=0 and y=1, where
`q=l^2+l+1`. Include a changed-donor-hash rejection.

This is exact source-order tangent/Newton data, not a normalization or
branch-count certificate. Its purpose is to select the next local closure
calculation and avoid guessing tangent directions from the nonreduced fiber.
One deterministic source-bound Python process, no cloud resources, no timing
comparison. Commit source/protocol before data; retain failed attempts.

Sources: R51 donor, R101 fiber, R104 socle; [Stacks normalization of one
dimensional schemes](https://stacks.math.columbia.edu/tag/0C44),
[Stacks branches lemma](https://stacks.math.columbia.edu/tag/0C1S), and
the installed Singular `normal.lib` positive-characteristic `normalP` routine.
