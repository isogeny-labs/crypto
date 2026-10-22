# Formal proof map

Lean 4.34.1 and Mathlib commit
`d13f23b723b8a846827a245b89c10fc7d3f11612` are pinned in the toolchain and
dependency lock. `lake build` passed; the repository's transitive axiom auditor
checked 13 declarations (nine explicit theorems and four generated defining
equations), with zero failures and zero compiler-trusted proofs. The only
foundations were `propext`, `Classical.choice` and `Quot.sound` as recorded in
[the build and audit receipts](receipts/).

| Statement | Lean declaration | Exact scope |
|:--|:--|:--|
| E00 discriminant identity | `TwistRoutes.trace_quartic_discriminant` | Every commutative ring; cleared polynomial identity |
| E00 chord division | `TwistRoutes.trace_chord_polynomial_division` | Every commutative ring; exact polynomial division identity |
| E00 complement A | `TwistRoutes.cover_complement_A_cleared` | Every commutative ring; denominator 3 cleared |
| E00 complement B | `TwistRoutes.cover_complement_B_cleared` | Every commutative ring; denominator 27 cleared |
| E00 complement substitution | `TwistRoutes.cover_complement_change_of_variables` | Every commutative ring; cleared coordinate identity |
| Pure anti-invariant trace | `TwistRoutes.anti_invariant_trace_zero` | Actual Mathlib nonsingular Weierstrass point group, an additive endomorphism, pi(P)=-P |
| Rational trace | `TwistRoutes.rational_trace_double` | Same point group and pi(P)=P |
| Kernel-coset trace equality | `TwistRoutes.kernel_translate_same_trace` | Same point group and U+pi(U)=0 |
| Mixed relation implies 2B=0 | `TwistRoutes.mixed_relation_twist_double_zero` | Same point group; R=A+B, pi(R)=R, pi(A)=A, pi(B)=-B |

The point-group statements use Mathlib's actual group law rather than a
renamed toy group. Their endomorphism/eigenvalue hypotheses are explicit. The
formal bridge from concrete finite-field Frobenius and the quadratic-twist
embedding to those hypotheses remains **open**. The odd-order subgroup
elimination, native field/tower representations, exceptional charts, divisor
class maps, subgroup recovery and complexity/cost statements are also not
formally proved here. Passing a polynomial identity does not discharge any of
those requirements. No custom axiom assumes a bridge or research conclusion.

Reproduce with the pinned toolchain:

```sh
cd research/twist_routes_1_3_20261021/lean
lake update
lake exe cache get Mathlib/AlgebraicGeometry/EllipticCurve/Affine/Point.lean Mathlib/Tactic/Ring.lean Mathlib/Tactic/Abel.lean
lake build
lake env lean -R ../../.. --run ../../../tools/lean/AuditAxioms.lean TwistRoutes
```

The local execution reused a pinned checkout and its verified dependency lock
to avoid fetching all Mathlib tags. Failed initial environment/root-path
commands and interrupted redundant fetch logs are retained; they are not proof
failures. A cache filesystem error was resolved by a writable XDG cache. The
passing build and transitive audit were run in this project against its lock.
