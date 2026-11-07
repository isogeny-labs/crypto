# Conditional DVR valuation of the common 2+2 chart

Status: five Lean theorems compile with only standard Lean axioms. They are
conditional on a discrete valuation ring `R` and a uniformizer `w`; this
packet does **not** construct the smooth projective completion, its stalk,
or the embedding of the source interpolation function into its fraction
field.

For `w²=z·u`, with `u` a unit and `w` irreducible in `R`,
`chart_z_order` proves `addVal(z)=2`. For the exact chart polynomial

`G=-C+n2·z+n1·z²+n0·z³-B·w`,

`chart_interpolant_order_zero` proves `addVal(G)=0` when `C` is a unit and
`z,w` are in the maximal ideal. `chart_interpolant_order_one` proves
`addVal(G)=1` when `C=0`, `B` is a unit, and `w²=z·u`. The latter factors
`G=w·(-B+w·u⁻¹·(n2+n1·z+n0·z²))`; the parenthesis is a unit in the local
ring, so cancellation cannot raise the order. These theorems retain all
coefficient terms and require no distinct-root assumption.

The chart identity in `Common22InfinityChart.lean` says `G=z³g` on the
affine overlap. The separate [fraction-field packet](FRACTION_POLE.md)
now proves the valuation arithmetic in Lean under supplied field-valuation
and chart hypotheses, giving `v(g)=v(w)⁻⁶` for unit `G` and
`v(g)=v(w)⁻⁵` for `v(G)=v(w)`. For the *intended* projective stalk these
would be additive pole orders six and five. The geometric specialization to
that stalk and the identification of its uniformizer are **not** Lean
theorems in this packet.

The next minimal formal obligation is to construct the smooth projective
completion of the squarefree monic quintic, identify its unique infinity
stalk with a DVR, identify chart `w` as a uniformizer and `u(0)=1` as a
unit, and prove the affine overlap's `G=z³g` equality in its fraction
field. A separate divisor theorem must account for repeated roots and
Weierstrass points, identify the source residual Mumford pair, and then
transport the principal divisor to `Pic⁰`. Rust semantics remains a
separate gate. No numerical result changes.

Receipt: `local-valuation-build.log`.
