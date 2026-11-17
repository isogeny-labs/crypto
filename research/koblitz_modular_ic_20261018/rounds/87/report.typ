#set page(paper: "a4", margin: 20mm)
#set text(size: 10pt)
= R87: the full relative equation under diamond pullback

The retained inverse-base source algebra has the quadratic equation
`E(V,z)` and a monic degree-16 equation `P(T,V,z)`, with `T=lambda`.
The R85/R86 diamond substitution gives `z1=1/h`, `V1=N/h²`,
and `T1=A/D`. Multiplying by `h⁶⁴D¹⁶` clears every denominator
of the 145 source monomials.

#image("relative87.svg", width: 100%)

The frozen Singular attempt built the complete 4,612-term pullback,
reduced it to zero modulo `(E,P)`, and checked a full two-generator
ideal lift. A shifted lambda update produced a nonzero remainder.
Its source bytes and four donors are hash-pinned. Lean proves the
characteristic-two shift identities for the powers used by that
negative control; the large ideal lift remains a native certificate.

Next derive the inverse upper-field formula, then transport every
generator of the accepted rank-32 RR PID basis with complete module
lifts in both directions. That will test regularity at the chart
boundaries.
