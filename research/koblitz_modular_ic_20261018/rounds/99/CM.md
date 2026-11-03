# R99: exact lambda derivative norm on the actual degree-112 source

The frozen one-process Singular attempt `norm99-01` passed: the actual R51
`G51` has a nonzero lambda derivative and resultant, and its norm through the
actual degree-7 `f51` is nonzero in F2(x). The exact output is in the retained
`norm99.sing`; its numerator has degree 1968, 73 terms, all exponents divisible
by 16 and constant term 1. The denominator is `x^2608`. The initial run
exited 0, had no native `?` errors, and printed the exact pass marker. A
separate fresh Singular process produced byte-identical `norm99.sing` and
again passed. Both run directories are in the committed 18 KiB archive with
SHA-256 `7cebf6664d1442401cc8ba40cde0df109fd29a24c663af6d67b0b5e9cb72caa7`.

This is the relative discriminant norm of the displayed monogenic lambda
order on `D(x)`, not normalized ramification. The observed 16-divisibility
suggests a degree-123 Frobenius root polynomial; the next exact calculation
is to verify that power identity and factor the degree-123 root over F2[x].
Combine that factor support with R97's base-order supports and infinity for
local normalization.

Sources: R51 `marked23-triangular51.sing`, R63 `CM.md`, R97 `CM.md`, R98
`CM.md`, [Singular manual](https://www.singular.uni-kl.de/index.php/singular.pdf)
for resultant and rational-number extraction, and
[Stacks 53.2](https://stacks.math.columbia.edu/tag/0BXX) for the
normal-projective-model distinction.
