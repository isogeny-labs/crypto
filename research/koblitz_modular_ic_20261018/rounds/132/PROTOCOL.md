# R132: base fibers at the degree-11 and degree-22 lambda supports

Exact finite-field calculation. `source-data/bad-factors.txt` is a byte-bound
extraction of R100's verified irreducible degree-11 and degree-22 factors.
Pin that file, the tracked R51 base equation, and the R100/R127 parsers.
For each factor `P_d(x)` set `a=x mod P_d` in `F_(2^d)` and specialize the
actual monic degree-7 `f23(x,y)`. Verify the factor irreducibility, the
specialized derivative gcd, all distinct-degree factor products, and total
degree seven. A squarefree fiber gives normalized base places with residue
degrees d times the irreducible y-factor degrees and ramification one.

This is the complete base fiber census at those two closed x-points. The
degree-16 lambda extension is known to have a discriminant support there,
so no lambda-place or N51 valuation follows from the base factorization.
Independent source evaluation/finite-field factor checks, changed-input
rejection, byte replay and Lean finite degree arithmetic are controls.
Local exact process, zero cloud spend; push source/protocol before data.

Prior work: R51 source, R97 base discriminant, R100 lambda norm factors,
R130/R131 degree-7 fiber. [Stacks Henselian factor lifting](https://stacks.math.columbia.edu/tag/04GE)
governs the squarefree base-factor interpretation.
