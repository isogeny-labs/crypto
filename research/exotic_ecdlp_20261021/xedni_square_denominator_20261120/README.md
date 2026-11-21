# Canonical square denominator for rational affine points

The Lean theorem `exists_primitive_square_denominator` proves that every rational affine solution of

\[y^2=x^3+9x+1\]

has coordinates \(x=a/b^2\) for an integer \(a\), positive integer \(b\), and \(\gcd(|a|,b)=1\). This closes the canonical x-coordinate representation obligation identified by the height-4096 certificate.

The proof writes \(x=a/d\) in lowest terms and \(n=a^3+9ad^2+d^3\). Coprimality gives \(\gcd(n,d^3)=1\), so the curve equation identifies the reduced denominator of \(y^2\) as \(d^3\). Since `Rat.den_pow` identifies that denominator as \(y.den^2\), \(d^3\) is a square. The checked `square_of_cube_square` lemma uses the parity of each prime factorization to conclude that \(d\) is a square. `square_denominator_of_affine` and `exists_primitive_square_denominator` implement these steps without a valuation axiom.

The follow-on theorem `no_bounded_rational_affine_point` applies the prior finite modular sieve to any rational affine point on this curve whose canonical numerator satisfies \(|x.num|\le4096\), whose canonical square-root denominator satisfies `x.den.sqrt ≤ 4096`, and whose x-coordinate numerator obeys \(x.num\equiv744\,x.den\pmod{1009}\). The stronger `no_bounded_affine_with_reduced_x` accepts the coordinate equality \(x.num/x.den=744\) in `ZMod 1009`, with a unit denominator, and proves the required integer congruence. These results put the finite exclusion directly on a rational point; no separate y-coordinate reduction condition is needed.

## Proof map

| Result | Lean declaration | Exact hypotheses | Status |
| --- | --- | --- | --- |
| Cube-square implication | `XedniSquareDenominator.square_of_cube_square` | `IsSquare (d^3)` for `d : ℕ` | Checked |
| Square denominator | `XedniSquareDenominator.square_denominator_of_affine` | rational `x,y` on `y²=x³+9x+1` | Checked |
| Primitive representation | `XedniSquareDenominator.exists_primitive_square_denominator` | same curve equation | Checked |
| Bounded rational affine exclusion | `XedniSquareDenominator.no_bounded_rational_affine_point` | same equation; canonical numerator and denominator bounds; x numerator congruence modulo 1009 | Checked |
| Reduced x-coordinate exclusion | `XedniSquareDenominator.no_bounded_affine_with_reduced_x` | same equation and bounds; square-root denominator nonzero in `ZMod 1009`; reduced x-coordinate equals 744 | Checked |
| Identification with a library point-reduction map | none | a rational point represented by the library's elliptic-curve type and its good-reduction map | Open |

`formal/build.sh` checks the pinned Lean toolchain and Mathlib manifest against the project installation and builds the actual modules. It captures `#print axioms` for every new theorem. The expected standard foundations are `propext`, `Classical.choice`, and `Quot.sound`; the module uses no `sorry`, `admit`, custom research axiom, or native evaluation. Lean is `leanprover/lean4:v4.35.0-rc4`, and Mathlib revision is `021c46bdd042f67b538b9f9871d556a93713a9a9`. `lean4checker --fresh` is unavailable on this host.
