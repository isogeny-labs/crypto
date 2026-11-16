# Degree-five cover search on remaining trace -14 source orbits

Frozen 16 November 2026 before counted runs. The twenty recorded trace -14
normal-form source equations over `K=F3[a]/(a^5+2a+1)` form four five-element
Frobenius orbits, checked in the prior `TraceMinus14Orbits.lean`. Degree
five has been excluded for source mask 202 on twelve recorded genus-five
carriers, and for source mask 11 on the fixed carrier mask 9735. The
remaining source-orbit representatives are masks 38 and 80; both have
order 258 and an order-43 subgroup.

`screen.sage` adapts the frozen complete degree-five denominator screen to
these two source masks and the twelve exact carrier masks. It enumerates
`1`, `Q_i`, `(x-c)²`, `Q_iQ_j`, `Q_i(x-c)²`, and `H²`, using each carrier's
actual rational branch factor. It includes all root assignments, all
rank-deficient quadratic-square fallbacks, and the exact identity
`f C²=B(A³+A²B+bB³)`. Expected logical coverage is 14,446,377
assignments per cell. First run `(bmask,fmask)=(38,9735)` and `(80,9735)`;
if both complete soundly, continue the remaining carriers.

A hit requires independent identity replay, subgroup transfer into the
small-field Jacobian, and complete same-target GHS/rho costs. The source
class and isogeny-assisted mechanism are already represented by published
work, including [Iijima–Momose–Chao 2015](https://eprint.iacr.org/2015/805.pdf),
[Howe 2022](https://arxiv.org/pdf/2110.04221), and
[Joux–Vitse 2012](https://www.iacr.org/archive/eurocrypt2012/72370010/72370010.pdf).
No publication priority is inferred from a local exact-parameter search.
