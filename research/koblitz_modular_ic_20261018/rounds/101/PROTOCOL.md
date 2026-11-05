# R101: actual degree-112 source order over x=1

Bind the R51 triangular source by SHA-256. Parse every top-level term of its
`G51` and `f51` assignments. Require the exact additive-lambda exponent
grid: one `l^16` term and seven y-coefficients each at l-degrees 8, 4, 1,
and 0. All 28 rational coefficients have power-of-x denominators; evaluate
their numerator parity at x=1. Require `f51(1,y)=y^7+y^3`. Export the exact
specialized equations and check in Singular that the lambda derivative is
the l-linear coefficient. Factor the l-polynomials above y=0 and y=1,
check both products and changed-product controls. Fresh replay must bind
the donor again and reproduce the exported bytes.

The output describes the fiber of the displayed monogenic source order.
It is not the normalization, nor does the number of factors equal the
number of normalized branches without a local integral closure. One
Singular process, seed101, one thread, 2 GiB cap, 120-second timeout.
Freeze this source and protocol in Git before execution; retain failures.
No timing/statistical comparison. Use pinned local CAS because the default
EKS image lacks it.

Literature: R51 donor, R63 degree-112 source, R97 base fiber, R100 norm
factorization; [Stacks branches lemma](https://stacks.math.columbia.edu/tag/0C1S)
states branch counts belong to the normalization, and
[Singular manual](https://www.singular.uni-kl.de/index.php/singular.pdf)
documents polynomial factorization. The live manual PDF fetch timed out;
the local pinned CAS is validated by exact product checks.

Amendment 02: attempt 01 stopped in Python preparation because the scanner
expected a `*` monomial suffix after every rational coefficient, while the
constant term has an empty suffix. The corrected parser accepts the empty
suffix only as the `(l^0,y^0)` term. See `FAILURE-01.md`.
