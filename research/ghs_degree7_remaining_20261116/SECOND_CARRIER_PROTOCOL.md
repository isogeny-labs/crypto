# Second trace +22 carrier orbit: exact degree-seven screen

Frozen 16 November 2026 UTC before counted second-carrier runs. The twelve
monic odd-degree trace-`+22` carrier masks in the earlier finite census are
listed in `../ghs_trace22_carrier_variants_20261116/RESULT.md`.
`carrier_orbits.sage` enumerates all 24 projective matrices in
`PGL₂(F3)` and checks the homogeneous polynomial identity

`(cx+d)^12 f_m((ax+b)/(cx+d))=f_n(x)`

for the monic odd-degree models. A matching identity gives the curve
coordinate change `x_old=(ax+b)/(cx+d)`, `y_new=(cx+d)^6 y_old`.
The expected two six-element orbits have representatives 32280 and 35349.
The first is already fully screened in the fixed-carrier theorem. The
second is the exact carrier

`Y_35349:y²=x^11+x^9+2x^8+x^7+x^5+x^4+x³+2x`.

`make_second_carrier_sources.py` copies the five frozen search-arm sources
and changes exactly one carrier-polynomial literal per copy; it asserts
that the original occurs once. The source orbit masks remain 68 and 84.
The prior quadratic-square screen already ran all 24 carrier/source cells,
including this representative. The five copied arms complete the remaining
normalized denominator classes on `Y_35349`: sparse, repeated linear,
cubic square, two branch poles plus linear square, and three branch poles.
Each copied source and generated output is committed before a counted run.
No source model or denominator shape is inferred from a partial result.

A hit must receive independent exact-algebra replay, morphism-degree and
subgroup-image verification, a complete GHS solve, matched rho, and
publication-priority audit. A negative result on both representatives,
combined with the displayed coordinate changes, extends the degree-seven
exclusion to all twelve recorded carrier models and ten exact source models.
It does not cover every genus-five curve in the isogeny class.

Sources checked: the existing twelve-carrier census and its 24 quadratic
screen receipts, all fixed-carrier protocols/results in this directory,
and [Howe's elliptic-cover theorem](https://arxiv.org/pdf/2110.04221).
