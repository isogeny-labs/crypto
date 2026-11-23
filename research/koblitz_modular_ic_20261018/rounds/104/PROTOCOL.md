# R104: leading x=1 residue of the cleared N51 coordinate

R102 found exactly 22 standard-basis coefficients with x+1 order -9.
Bind its full result JSON and R101's exact x=1 `f,G` specialization by
SHA-256. The coefficient of `(x+1)^0` in `(x+1)^9(-n)` is the sum of
their 22 basis monomials: after removing equal x+1 powers, each nonzero
F2 numerator and denominator has residue 1. Generate this `H104`, check
all monomial exponents are below 16 and 7, and check in pinned Singular
that it is nonzero and already reduced modulo `(f101,G101)`. Reject one
removed term. Evaluate `H104` at y=0 and y=1, then reduce modulo the
irreducible quadratic `l^2+l+1` to obtain its residues at R101's two
source-order maximal ideals. Freshly replay and compare exact exports.

The monic equations and x-power-only G51 denominators make the local
y/lambda order free of rank 112 over F2[x] localized at x+1, with basis
`l^i y^j`, `i<16,j<7`. Thus the nonzero `H104` certifies that exponent 9
is minimal for membership of n in this **specific source order**, not
merely a coefficientwise bound. The normalization can contain new integral
elements, so no normalized function pole valuation follows.

One local pinned Singular process, seed104, one thread, 2 GiB virtual
memory, 120-second timeout. Commit source and protocol before data;
retain failures. The default EKS image lacks this pinned CAS.

Sources: R51 donor, R63 standard monomials, R98 denominators, R101 fiber,
R102 orders; [Stacks monic free-algebra example](https://stacks.math.columbia.edu/tag/00FD)
and [Stacks finite integral extensions](https://stacks.math.columbia.edu/tag/00GH).
