# Common 2+2 infinity chart

Status: conditional algebraic chart formalization. The projective local ring,
its valuation, the principal divisor, and the `Pic⁰` class equation are open.

For the normalized quintic
`y²=x⁵+a3·x³+a2·x²+a1·x+a0`, set `z=1/x` and `w=y/x³` on the overlap
where `x≠0`. `Common22InfinityChart.curve_chart_equation` proves

`w²=z·u(z)`, where `u(z)=1+a3·z²+a2·z³+a1·z⁴+a0·z⁵`.

The Lean theorem `chart_factor_at_origin` proves `u(0)=1`. Thus the
projective point represented by `(z,w)=(0,0)` has an algebraic chart equation
whose `z`-derivative is `-1` there. The derivative and the simple-root
statement for the `w=0` section are now kernel-checked in the
[local-parameter packet](INFINITY_LOCAL_PARAMETER.md). The standard
smooth-curve argument then
makes `w` a local parameter, `u(z)` a unit, and `z=w²/u(z)`. The corresponding
orders would be `ord∞(w)=1`, `ord∞(z)=2`, `ord∞(x)=-2`, and
`ord∞(y)=-5`. The local-ring/DVR implication remains **unproved** in Lean;
a separate conditional theorem shows that a supplied chart stalk with
maximal ideal `(z,w)` has maximal ideal `(w)`.

For the source-style interpolant
`n(x)=-C·x³+n2·x²+n1·x+n0` and nonzero clearing factor `B`, put
`g=n(x)-B·y`. `Common22InfinityChart.interpolant_chart` proves

`z³g=-C+n2·z+n1·z²+n0·z³-B·w`.

The source coefficients are `n2=-(C·q1+A)`,
`n1=B·s1-C·r1-A·q1`, and `n0=B·t1-A·r1`, as in
`Common22SourceHead.sourceE`. `source_interpolant_chart` substitutes the
exact `Common22SupportHead.coefficientData` tuple into the generic identity.
The Lean residue lemmas show that the chart
expression has constant `-C`, and at `C=0` its `w` coefficient is `-B≠0`.
Given the unproved local-parameter implication above, this yields pole
order six when `C≠0` and pole order five when `C=0,B≠0`; in the latter case
all `z` terms have order at least two and cannot cancel the linear `w` term.

The precise next formal theorem is: for squarefree monic quintic `f` in odd
characteristic, the local ring of the smooth projective completion at its
unique infinity point is a DVR, the chart above maps into it, `w` is a
uniformizer, and `u(z)` is a unit. A separate zero-divisor theorem must
identify the source output Mumford pair with the residual intersection,
including repeated roots and Weierstrass points. Only then can the principal
divisor of `g` be transported to `Pic⁰`. The existing abstract
`RuntimeSemantics.residual_represents_input` does not construct that map.

No numerical cost or cohort result changes. The Lean receipt is
`infinity-chart-build.log`; it proves the displayed algebraic identities
only, with standard Lean axioms listed there.
