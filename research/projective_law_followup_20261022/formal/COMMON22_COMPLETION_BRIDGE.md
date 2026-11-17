# Explicit infinity point: proof boundary

The new Lean module `Common22CompletionBridge.lean` works over any field `K`
with arbitrary coefficients `a3 a2 a1 a0`. Its chosen chart is the actual
quotient

`K[z,w]/(w² - z(1 + a3 z² + a2 z³ + a1 z⁴ + a0 z⁵))`.

The declaration `prime_eq_origin_of_z_mem` proves that a prime containing
`z` equals the origin ideal `(z,w)`. It uses the quotient equation to get
`w²` in the prime, primality to get `w`, and maximality of the origin ideal.
`infinity_chart_complement_unique` and `infinity_origin_outside_overlap`
then give `unique_infinity_point`: the complement of the principal overlap
`D(z)` in this affine infinity chart is exactly the origin point. This does
not assume a smoothness or projectivity conclusion.

## Verification

The pinned environment is Lean `v4.34.1` and Mathlib
`d13f23b723b8a846827a245b89c10fc7d3f11612`, as recorded in the adjacent
`lean-toolchain` and `lakefile.toml`. Compile from this directory with

```sh
timeout 180s lake env lean Common22CompletionBridge.lean
```

The bounded command exited 0. Its verbatim output is in
`common22-completion-bridge-build.log`; the command, exit code, and source/log
hashes are in `common22-completion-bridge-exit.txt`. `#print axioms` is
included for all four theorems in the source, and every line reports exactly
`propext`, `Classical.choice`, and `Quot.sound`. No `sorry`, `admit`,
`native_decide`, or custom axiom occurs in this module. The reviewer-facing
table, graph, and PDF are `common22-completion-bridge.csv`,
`common22-completion-bridge.svg`, and `COMMON22_COMPLETION_BRIDGE.pdf`.

## Exact remaining geometric bridge

`Common22TwoChartGlue.gluedCurve` glues the affine `y²=f(x)` chart to the
explicit infinity chart over their checked principal-open isomorphism.
The checked gluing maps are open immersions and jointly cover the resulting
scheme. To identify it with a chosen smooth projective completion, a new
construction must supply either:

1. a concrete projective scheme, with an open cover by these two quotient
   charts and overlap maps matching
   `Common22PrincipalOpenSpec.principalOpenSpecIso`; or
2. a finite morphism from `gluedCurve` to a constructed `P¹_K`, together
   with the finite-type and separation arguments that make the glued curve
   proper, and a proof that its two charts are smooth under an explicit
   squarefree or discriminant hypothesis.

One explicit candidate is weighted `Proj A`, where

`A = K[X,Y,Z]/(Y² - X⁵Z - a3 X³Z³ - a2 X²Z⁴ - a1 XZ⁵ - a0 Z⁶)`

and `deg X = deg Z = 1`, `deg Y = 3`. Formally dehomogenizing at `Z` gives
the affine `y²=f(x)` chart (`x=X/Z`, `y=Y/Z³`), and at `X` gives the infinity
chart (`z=Z/X`, `w=Y/X³`). At `X=Z=0` the equation forces `Y²=0`, so no
relevant homogeneous prime lies outside those two opens. These symbolic
observations specify the needed graded-ring quotient, open-cover and
transition isomorphism; they are **not** formalized in this packet. The
ordinary plane closure has a singular point at infinity and is therefore
not the desired smooth model without normalization.

The pinned Mathlib has `Scheme.GlueData`, `Cover.gluedCover`, and the
isomorphism `Cover.fromGlued` for an *already supplied* open cover. It does
not turn the present two charts into a projective completion automatically.
It also has `Proj.toSpecZero` proper when the graded ring is finite type over
its degree-zero part, so a formalized weighted quotient and chart-cover
equivalence would provide a direct properness route.
No generic hyperelliptic smooth-completion object or theorem was found in
the pinned `Mathlib/AlgebraicGeometry` tree; the existing projective curve
point APIs there are for elliptic curves. Consequently this packet proves a
singleton infinity fiber of the explicit chart, not properness, smoothness,
or an isomorphism to a `Proj` model.

The Picard-group step is further downstream: it needs a divisor theory for
the chosen smooth curve, a principal-divisor identity for the interpolating
function including the now-identified infinity point and its multiplicity,
and a proof that the returned Mumford pair represents the resulting class
in `Pic⁰`. None of those conclusions follows from the singleton-point
statement alone.
