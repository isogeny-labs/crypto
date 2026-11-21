# Fixed-curve rational coordinate reduction modulo 47

For the exact rational model `E: y²=x³−47x²+751x−3997`, a rational affine point with `x=a/b²`, `b>0`, and `b` nonzero modulo 47 has `y=c/b³` for an integer `c`. Lean proves that

`(a/b² mod 47, c/b³ mod 47)`

is a **nonsingular point** of `E₀: y²=x³−x−2` over `F₄₇`. If its reduced coordinates equal the frozen target `(10,1)`, the point necessarily satisfies `a≡10b²` and `c≡b³` modulo 47. Lean also checks the rational point `Q=(17,10)` and finite target `(10,1)` directly against their respective curve predicates.

The proof clears denominators in the rational equation to obtain

`c²=a³−47a²b²+751ab⁴−3997b⁶`.

The integer `c` exists because `(yb³)²` is integral and `ℤ` is integrally closed in `ℚ`. Casting this polynomial identity into `ZMod 47` gives `c²=a³−ab⁴−2b⁶`; division by the nonzero powers of `b` yields the reduced equation. Lean checks that the reduced discriminant is nonzero, so the resulting point is nonsingular.

The [protocol](protocol.json) froze the exact models, source hashes, target, and proof scope before formalization. The pinned [Lean module](formal/Reduction.lean) proves:

| Result | Lean declaration | Status |
| --- | --- | --- |
| `Q=(17,10)` and target `(10,1)` belong to their curves | `rational_Q_on_curve`, `reduced_target_on_curve` | Checked |
| Existence of integral `c` and cleared-denominator identity | `cube_denominator_numerator`, `integral_model_equation` | Checked |
| Reduced equation and nonzero discriminant | `reduced_equation`, `reduced_discriminant_ne_zero` | Checked |
| Reduced coordinates define a nonsingular point | `reduced_point_nonsingular`, `rational_point_reduces` | Checked |
| Target coordinate congruences | `target_coordinate_congruences` | Checked |

The [build output](formal/build-output.txt) contains the `#print axioms` results; all list only standard Lean foundations. Stderr is empty. No declaration uses an admitted claim, custom research axiom, or unverified native evaluation.

This closes the **coordinate reduction** part of the fixed-curve proof map, together with the [primitive square-denominator theorem](../xedni_fixed_curve_lean_bridge_20261121/README.md). The rational reduction map as a group homomorphism, order 43 of the reduced generator, the Mordell–Weil generator certificate, and the canonical-height comparison remain separate formalization obligations. The nonzero-denominator hypothesis is explicit; the theorem makes no assertion at points where reduction is not defined by these affine coordinates.

Replay from the repository root:

```sh
sh research/exotic_ecdlp_20261021/xedni_reduction_lean_bridge_20261121/formal/build.sh
```
