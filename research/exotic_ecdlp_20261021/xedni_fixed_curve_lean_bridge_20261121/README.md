# Primitive square denominators for the fixed rational curve

Every nonidentity Mathlib rational point on

`E: y²=x³−47x²+751x−3997`

has `x=a/b²` for integers `a` and `b>0` with `gcd(|a|,b)=1`. The checked Lean result removes the square-denominator representation as an independent assumption in the fixed-curve lift-height argument. The curve-specific Mordell–Weil, reduction, and canonical-height inputs remain separate formalization obligations.

The proof first establishes the representation for **every integral short Weierstrass equation** `y²=x³+A x²+B x+C`, with `A,B,C∈ℤ`. Write `x=a/d` in lowest terms. Clearing denominators gives

`y² = (a³+A a²d+B ad²+C d³)/d³`.

The numerator is congruent to `a³` modulo `d`; therefore it is coprime to `d` and to `d³`. The reduced denominator of the square `y²` is simultaneously `den(y)²` and `d³`. Thus `d³` is a square, so every prime valuation of `d` is even and `d=b²`. Coprimality of `a` and `b` follows from the reduced fraction `a/d`.

The [protocol](protocol.json) froze the source proof, exact curve coefficients, and theorem scope before formalization. The pinned [Lean module](formal/FixedCurveSquareDenominator.lean) contains:

| Result | Lean declaration | Status |
| --- | --- | --- |
| `d³` square implies `d` square | `square_of_cube_square` | Checked |
| Denominator square for any integral `A,B,C` | `square_denominator_of_integral_affine` | Checked |
| Primitive `a/b²` representation | `exists_primitive_square_denominator` | Checked |
| Representation for every nonidentity point of the exact fixed curve | `fixed_curve_point_square_denominator` | Checked |

The actual point theorem uses Mathlib's `WeierstrassCurve.Affine ℚ` point type and its equation predicate. The retained [build output](formal/build-output.txt) reports only `propext`, `Classical.choice`, and `Quot.sound` for all reported declarations; stderr is empty. There are no admitted claims, custom research axioms, or native-evaluation bypasses.

This establishes the coordinate form used in the [height exclusion](../xedni_explicit_height_exclusion_20261121/README.md) and [minimum-height lift](../xedni_minimum_lift_height_20261121/README.md). Connecting those results to a fully Lean-checked curve-specific theorem still requires a formal Mordell–Weil generator certificate, reduction modulo 47, and a canonical-height comparison theorem for this model.

Replay from the repository root:

```sh
sh research/exotic_ecdlp_20261021/xedni_fixed_curve_lean_bridge_20261121/formal/build.sh
```
