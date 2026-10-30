# Conditional fraction-field pole identity

Status: two Lean theorems compile with only `propext`, `Classical.choice`, and
`Quot.sound`. The packet proves the fraction-field valuation arithmetic after
the intended infinity stalk and its valuation are supplied. It does not
construct either object or prove the source interpolant's divisor class.

For a valuation `v` on a field, suppose `w != 0`, `w^2 = z*u`, `v(u)=1`, and
`z^3*g=G`. The kernel-checked implications are:

| Theorem | Additional hypothesis | Exact conclusion |
| --- | --- | --- |
| `unit_chart_pole` | `v(G)=1` | `v(g)=v(w)^(-6)` |
| `linear_chart_pole` | `v(G)=v(w)` | `v(g)=v(w)^(-5)` |

These are the two valuation branches of the source-style common 2+2
interpolant. `Common22InfinityChart.source_interpolant_chart` establishes the
cleared equation on the affine overlap. `Common22LocalValuation` establishes
the corresponding chart orders in an abstract DVR. Neither file constructs
the actual smooth projective completion, embeds its local ring in the
function field, or proves that the supplied valuation is its place at
infinity. In particular, the displayed exponents become additive pole orders
six and five only after `w` is identified as the normalized local
uniformizer. The principal divisor, Mumford reduction, `Pic^0` class
equation, and compiled Rust correspondence are still open.

The result changes no operation count, target solve, or rho ratio. The CSV
is a two-row standardized theorem inventory, and the graph depicts the
exponents under the stated hypotheses rather than a measured cost.

Build and audit from this directory:

```sh
lake env lean Common22FractionPole.lean
```

The retained successful stdout is in `fraction-pole-build.log`. The source
is `Common22FractionPole.lean`; the graph is `fraction-pole.svg` and the
one-page review rendering is `FRACTION_POLE.pdf`.
