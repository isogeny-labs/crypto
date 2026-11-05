# R114: wild x=0 edge under a radical substitution

Substitute `x=r^2`, `v=r^3(1+h)` into R113's exact quotient and divide by r^6. The resulting F2 polynomial has 74 nonzero monomials and special fiber `h^2`. Its lower hull is `(0,3),(2,0),(3,3),(7,31)` and its sole positive edge again has slope 3/2 with residual `1+T^2=(1+T)^2`. The first h-linear term occurs at r order 3. Thus this purely radical substitution has not produced a simple Hensel root; repeating it is not justified by the present data.

A better coordinate remains inside the original function field. R113's slope `v_x(v)=3/2` gives normalized orders `v_P(x)=2`, `v_P(v)=3` on the degree-2 edge if its local factor is irreducible; then `t=v/x` has order 1. Substituting `v=x*t` into the original R113 polynomial should allow x to be solved as a series in t, avoiding a purely inseparable square root. The coprime slope/edge-length irreducibility criterion and complete local-degree accounting must be checked before entering this as a normalized place theorem. No branch count or N51 valuation is claimed in this round.

The source/protocol were committed before data. Fresh replay was byte-identical; a changed R113 input failed the hash gate. An independent verifier substituted `x=r^2` and `y=1+r^2+r^4+r^7(1+h)` directly into all eight original f51 coefficients, divided by r^18, reproduced all 74 terms and checked the hull. Lean 4.34.1 checked the repeated edge and candidate uniformizer arithmetic. No producer/replay/verifier failure.

Result SHA-256: `c7c5664c9956f9af5f709bd08cc6bf4ae91c2ba465224b5f0af3356b7dd71bc0`. Raw archive SHA-256: `fc2ac6aea07d37f7fd0b975b98a454306e5a88ddf0627455fc89f19f6c2778e7` (976 bytes). Replay: `python3 research/koblitz_modular_ic_20261018/rounds/114/wild114.py <output-directory>`; independent check: `python3 research/koblitz_modular_ic_20261018/rounds/114/verify114.py`.

Sources: R51 donor, R105 parser, R113 wild edge, [Stacks integral closure](https://stacks.math.columbia.edu/tag/00GH), [Stacks Henselian lifting](https://stacks.math.columbia.edu/tag/04GE), [Newton polygon theorem](https://www.math.columbia.edu/~gyujinoh/ANT_revised.pdf).
