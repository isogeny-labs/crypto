# R89 pole-bound erratum (post-run, not a protocol amendment)

The frozen R89 protocol used `14+d=51`, taking `d=37` from the R50 PID
matrix. That bound omitted the degree in the relative coordinate `T`.
The strict native-error gate rejected `transport89-01` at column 1 in both
directions. Its later result line and pass marker are artifacts of
`require89` using `ERROR` inside a Singular procedure, which returned to
the top level. Neither line certifies a column.

A uniform **upper bound** for the pole order of an R50 normalized generator
at the removed original `u=0` fibers is 61 times the local ramification
index. Write its numerator as a sum of `z^k T^i V^j`, where `k<=37`,
`i<=15`, `j<=1`, and divide by the monic `F(z)` of degree 37. At the A
fiber, `ord(z)=-e`, `ord(V)>=-e`, `ord(T)>=-4e`, and
`ord(F)=-37e`. Thus each summand has order at least
`(37-k-4i-j)e >= -61e`. At the B fiber, the corresponding
orders are `ord(V)>=-2e` and `ord(T)>=-e`, giving at least
`(37-k-i-2j)e >= -17e`. The same bound applies to each copy
of the source under the accepted two-way diamond formulas, with the
appropriate overlap function vanishing on the removed fiber.

This bounds possible poles; it does not prove that 61 is attained or that
every transported generator has polynomial coordinates after that power.
Freeze a new protocol with bound 61, fix the fatal-error handling, and
rerun from a new attempt ID. Do not relabel or modify `transport89-01`.

Inputs: R27 `CM.md` cusp valuation table; R50
`NEXT_SOURCE_NORMALIZATION.md` overlap geometry and R50 saved PID basis;
R85-R88 diamond maps. The R50 note's `14+d` sentence is superseded by this
bound only for this all-generator transport experiment.
