# R127: degree-7 marked-23 base fiber

Exact local-algebra calculation. Bind the tracked R51 `f23(x,y)` source by
SHA-256, set `P7(x)=x^7+x^6+x^3+x+1`, and work in
`F128=F2[a]/P7(a)`. Verify P7 is irreducible over F2, then specialize the
actual monic degree-7 base polynomial at `x=a`. Compute its derivative,
gcd, squarefree decomposition, distinct-degree factor products, and all F128
rational roots with multiplicity. Check that the factors multiply back to
the source-specialized polynomial and that the degree count remains seven.

This identifies base source-order branches and residue degrees at the degree-7
discriminant point. It does not promote a factor multiplicity to a normalized
ramification index, and does not claim N51 valuations or a full degree-112
fiber. Independent field arithmetic, changed-source rejection, byte replay,
and finite Lean checks of any displayed product/degree identity are controls.
One local exact process; no cloud cost or comparative measurement. Source and
protocol must be pushed before calculating result data.

Prior work: R51 actual marked-23 base donor, R97 base discriminant, R100
lambda-discriminant support, R126 x=0 local divisor. The normalized local
ring/DVR distinction follows [Stacks 0B8Y](https://stacks.math.columbia.edu/tag/0B8Y);
simple-root lifting uses [Stacks 04GE](https://stacks.math.columbia.edu/tag/04GE).
