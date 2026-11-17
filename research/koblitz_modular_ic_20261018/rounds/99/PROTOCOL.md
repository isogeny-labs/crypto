# R99: actual degree-112 lambda resultant norm, frozen before execution

From the accepted R51/R63 `marked23-triangular51.sing` donor, compute the
nonzero lambda derivative, `Res_l(G51, dG51/dl)`, and its norm
`Res_y(f51, Res_l(...))` in F2(x). Check the degree-7/16 monic source
conditions, elimination of `n,l,y`, and nonzero numerator/denominator.
Retain the exact rational norm as a source-bound output. This attempt does
not yet factor the numerator; factoring and local normalization follow.

One process, seed99, one thread, 6 GiB virtual-memory cap, 300-second timeout.
The donor SHA-256 must equal the R98 recorded hash. Commit this source and
protocol before execution. Accept only exit0, exact pass marker, no native
`?` errors, and a nonempty output. Retain a timeout, OOM, or CAS failure as
an outcome; no source changes within the attempt. The default EKS image
lacks the pinned Singular CAS, so this is the existing pinned local-CAS
exception pending a digest-pinned image.

The claim is exact algebra, with no timing or statistical comparison.
R98 establishes `D(x)` as the coefficient-regular finite open. R97 gives
the degree-7 base-order discriminant support. A nonzero resultant norm
would identify additional finite non-etale fibers of the displayed
degree-16 monogenic order on `D(x)`; it would not prove normalized
ramification indices. Source references: R51 donor, R63 `CM.md`, R97
`CM.md`, R98 `CM.md`, [Singular manual](https://www.singular.uni-kl.de/index.php/singular.pdf)
(`resultant`, `numerator`, `denominator`), and [Stacks 53.2](https://stacks.math.columbia.edu/tag/0BXX)
(normal projective models of curve function fields).
