# Rational affine bridge for the height-4096 certificate

The Lean theorem `no_bounded_rational_affine_lift` applies the checked height-4096 integer sieve to a rational affine point on

\[y^2=x^3+9x+1.\]

Given a primitive representation \(x=a/b^2\), with \(-4096\le a\le4096\) and \(1\le b\le4096\), the new lemma `cube_denominator_numerator` proves that \(yb^3=c\) for some integer \(c\). Substitution yields \(c^2=a^3+9ab^4+b^6\). If the x coordinate reduces to 744 modulo 1009, the prior finite sieve gives a contradiction. The y coordinate is unconstrained in this theorem. The numerator-integrality step uses Mathlib's integrally closed integer theorem.

## Proof map

| Result | Lean declaration | Exact hypotheses | Status |
| --- | --- | --- | --- |
| Cube-denominator y numerator | `XedniRationalBridge.cube_denominator_numerator` | `b>0`, `x=a/b²`, `y²=x³+9x+1` | Checked |
| Height-4096 x-coordinate sieve | `XedniRationalBridge.no_x_lift_height4096` | integer `a,b,c`; height and coprimality bounds; x numerator congruence; integral curve equation | Checked |
| Bounded rational affine exclusion | `XedniRationalBridge.no_bounded_rational_affine_lift` | previous conditions; `a,b` bounds; `gcd(|a|,b)=1`; x numerator congruence | Checked |
| All rational affine points acquire primitive square-denominator x representation | none | arbitrary rational point on this integral Weierstrass model | Open |
| Algebraic reduction of a rational point to x=744 modulo 1009 | none | good reduction and denominator a unit mod 1009 | Open |

Thus this addition completes the y-integrality and equation bridge, while the canonical x-denominator and point-reduction bridges remain explicit proof obligations. It does not promote the bounded coordinate theorem to a statement about every rational point without those bridges.

## Verification

Run `sh formal/build.sh` from this packet. It checks the pinned Lean toolchain and Mathlib manifest against the project installation, builds the actual theorem modules and prints their axioms. The expected foundations are `propext`, `Classical.choice`, and `Quot.sound`. No `sorry`, `admit`, custom research axioms, or native evaluation occur in this module. The repository's previous certificate pins Lean `leanprover/lean4:v4.35.0-rc4` and Mathlib revision `021c46bdd042f67b538b9f9871d556a93713a9a9`. `lean4checker --fresh` is unavailable in the current environment.
