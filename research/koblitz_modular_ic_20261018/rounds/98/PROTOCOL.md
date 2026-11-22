# R98: frozen degree-112 lambda denominator audit

Bind to the accepted R51/R63 triangular source file by SHA-256. Extract exactly
the `poly G51=...;` assignment. Scan **every** division slash in that assignment
and accept only denominators syntactically equal to a power of `x`. Report all
exponents and their maximum. Plant `/(x+1)` in place of one denominator and
require the same scanner to reject it. Run the frozen checker twice in separate
processes and compare byte-identical JSON. One attempt; retain failures.

This is an exact source audit, with no timing or statistical claim. Its result
locates finite poles of the displayed monogenic equation, not normalized
branch points. The subsequent discriminant-norm calculation must still compute
`Res_y(f51, Res_l(G51, dG51/dl))`; local normalization remains separate.

Amendment 02: attempt 01 failed its negative control because the planted
mutation was outside `G51` in the one-line donor. The next attempt confines
the mutation to the extracted assignment. See `FAILURE-01.md`.

Sources checked: R51 `marked23-triangular51.sing`, R63 `CM.md`, R97 `CM.md`,
and [Stacks, curves and function fields](https://stacks.math.columbia.edu/tag/0BXX)
for the distinction between the function field and its normal projective model.
The [Singular manual](https://www.singular.uni-kl.de/index.php/singular.pdf)
was checked for the subsequent resultant construction; the live PDF fetch
timed out, so this round makes no new claim about CAS behavior.
