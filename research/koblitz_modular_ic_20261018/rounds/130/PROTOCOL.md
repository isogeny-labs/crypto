# R130: full degree-7 lambda fiber over the normalized base places

Exact source-bound finite-field calculation. The input `source-data/triangular-f-g.sing`
is the verbatim `f51` and `G51` assignment excerpt from the R51 native donor,
whose full SHA-256 is recorded. Pin the excerpt, the R105 parser, and the
R127/R129 result files. Specialize all 29 actual G51 terms at `x=a` and
at the three R129 base residue roots: `y=123`, `y=90` in F128 and `y=z`
in `F128[z]/Q4(z)`, where Q4 is R127's irreducible quartic. Verify each
degree-16 polynomial is monic and has nonzero lambda derivative. Compute
the exact distinct-degree factor products via repeated finite-field
Frobenius/gcd; multiply all products back to the specialized G51 and
account for every one of the 16 lambda degrees in each base residue field.

If all three fibers are squarefree, simple-factor Hensel lifting gives
unramified lambda extensions above these completed base places. Record the
resulting F2 residue degrees and inherited ramification, checking total
fiber degree 112*7=784. This is a fiber census, not N51 valuation or
subgroup transfer. Require independent arithmetic/product checks, byte
replay, changed-input rejection and Lean finite degree arithmetic.
Local deterministic exact computation, zero cloud spend. Publish source,
input excerpt and protocol before result data.

Prior work: R51 donor, R100 lambda discriminant support, R105 parser,
R127/R129 base fiber. [Stacks Henselian factor lifting](https://stacks.math.columbia.edu/tag/04GE)
and [étale maps](https://stacks.math.columbia.edu/tag/00U0) supply the
simple-factor interpretation.
