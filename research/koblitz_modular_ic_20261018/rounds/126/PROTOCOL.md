# R126: tame-seven Hensel lift and N51 evaluation

Exact local computation, with no timing or comparative claim. For each of the
four x=0 base places and both F4 residue roots, take the terminal G51
polynomial from R119–R122. Substitute `s=t^7` and `W=t^q(1+U)` with q fixed
by the R125 seven-edge. The coefficient of `U` is a unit, so lift the unique
root `U in t F4[[t]]` through t-degree 255. Reconstruct the N51 polynomial
from the SHA-pinned R124 coefficients and the R125 substitutions, verify every
row's SHA-256 against R125, and substitute the lifted root. Report an exact
normalized N51 order only when a first nonzero coefficient occurs below both
G51 and N51 input precision cutoffs. Otherwise report a certified lower bound.

Controls: all source hashes, terminal support checks, zero G51 residual,
F4 conjugacy, byte-identical replay, changed-input rejection, independent
coefficient-by-coefficient root replay, and finite Lean checks for any claimed
first coefficient. This is local exact arithmetic with zero cloud cost.

Prior work: R119–R125 protocols and results. The simple-root lifting step uses
[Stacks, Henselian local rings](https://stacks.math.columbia.edu/tag/04GE);
the ramified edge setup follows [Stoll, p-adic analysis notes](https://www.mathe2.uni-bayreuth.de/stoll/teaching/pAdicAnalysis-WS2015/Skript-pAdicAnalysis-pub-print.pdf).
