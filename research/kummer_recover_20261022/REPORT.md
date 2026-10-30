# Fast-Kummer signed recovery gate

Study ID: `KUMMER-RECOVER-20261022`. The [PDF](RECOVER_GATE.pdf),
[graph](recover-cohort.svg), [summary](results/summary.json), and
[row dictionary](DATA_DICTIONARY.md) expose the complete result.

The fixed cohort contains 1,260 source-verified targets across 21 genus-two
models. Every one of the 175 previously observed zero-coordinate xADD cases
was repaired on the quotient by a recorded rational two-torsion shift and
checked against its independent Jacobian class. The signed recovery program
returned the exact source-derived Mumford divisor for all 1,260 targets:
1,257 through the corrected generic equations and three through a bounded
algebraic exceptional lift. There were no wrong or inapplicable final rows.

The literal displayed equations in [Chung–Costello–Smith, §§6.4 and
7.4](https://www.microsoft.com/en-us/research/wp-content/uploads/2016/02/recovery.pdf)
produced 1,257 incorrect signed divisors and three `D=0` cases in this
implementation's coordinate convention. Those raw rows and the unmodified
literal source are preserved. Reducing their stated
`(G3*x-G4)*a_P(x)` modulo `a_Q(x)` yields the coefficient
`-(xi2*Z1+Z2)*G3+Z1*G4` for `b1(Q)`, whereas displayed Eq. (12) has a
positive coefficient on `G3`. This is an exact polynomial identity under the
recorded Mumford convention. Direct `p=739` row 0 also gives `T=498` from
`f6-G1²-G3²` while the displayed Eq. (11) rational expression gives `569`.
For our monic quintics with `f6=0`, comparing the `f5` coefficient of
`G3² D²` from `b_Q² ≡ f (mod a_Q)` and Eq. (6) gives
`f5 Z3 (xi3 Z2+xi2 Z3)`; Eq. (11) displays `-f5 Z3 Z4` with
`Z4=xi3 Z2-xi2 Z3`. The difference is `2 f5 xi3 Z2 Z3`, equal to
`423 mod 739` on this row. The correction obtains `G3²` independently from
Eq. (6).
These statements concern the displayed equations under the explicit local
coordinate convention; the study does not classify every possible convention.

For each of the three `D=0` targets, the exceptional branch enumerated roots
of `b1²`, solved `b(x)²=f(x) mod a(x)`, and selected the signed lift by checking
`K(Q+P)` against the signed anchor. This branch runs in `O(p)` field search and
uses Jacobian addition, so it is recorded separately from fast Recover.

The gate proves quotient continuation and source-label recovery on this
cohort. A closed label-carrying collector and paired rho pricing were not run
in this study. Candidate cost, rho cost, and ratio are null. The graph and PDF
display only verified gate counts; no win/loss inference is made.
