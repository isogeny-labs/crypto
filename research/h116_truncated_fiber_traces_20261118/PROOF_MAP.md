# H116 proof map and Lean instance gate

This map is written before the first H116 Lean declaration.

## Six instance-gate fields

1. **Structure above.** The algebraic core is over a commutative field `K`,
   `Polynomial K`, formal Laurent coefficients at infinity, finite multisets
   of roots and rational functions `N/D`. The elliptic endpoint additionally
   assumes an odd-degree separable isogeny coordinate map and its differential
   scalar `a`.
2. **Theorem used as stated.** Planned Mathlib dependencies are polynomial
   derivative-of-product, derivative of `X-C a`, finite-sum coefficient laws,
   and field fraction cancellation. Exact declaration names and verification
   status remain a pre-edit assembly-probe obligation because the research KB
   theorem index was unavailable. No curve-specific group law will be
   re-proved in the study. [pcc:unverified]
3. **Object as an instance.** `F_u=N-uD` is a polynomial; its root multiset is
   the coordinate image of the fiber. `R=N/D` is a rational function. The
   study endpoint instantiates generic polynomial identities with the isogeny
   coordinate data supplied as hypotheses.
4. **Assembly probe.** Before the first proof edit, type-check a theorem whose
   inputs are `N D`, two target coordinates, a factorization of each `F_u`, and
   the nonzero denominators; its conclusion is the finite coefficient identity.
   A second probe applies the residue identity given `F'_u(alpha)=D(alpha)R'(alpha)`.
5. **Missing components only.** A finite reciprocal-series coefficient API may
   be missing. If so, define it generically for polynomials and prove its product
   and derivative laws once. The formal-group congruence for `V-1` is a separate
   endpoint premise unless an existing verified elliptic formal-group theorem is
   located. [pcc:unverified]
6. **Inheritance table.** Below. A blank or `lookup pending` entry blocks the
   first Lean edit.

## Paper proof chain

1. Factor `F_u=c prod_alpha (X-alpha)`.
2. Differentiate and divide to obtain
   `F'_u/F_u = sum_alpha 1/(X-alpha)`.
3. Expand each reciprocal at infinity; the coefficient of `X^(-j-1)` is the
   `j`th root power sum.
4. Substitute `F_u=D(R-u)` and subtract the corresponding identity for `u0`;
   the `D'/D` contribution cancels.
5. At a simple root `alpha`, use
   `F'_u(alpha)=D(alpha)R'(alpha)` and partial fractions for
   `D/F_u=1/(R-u)`.
6. Multiply the residue coefficient by `a*v` using
   `phi_y=yR'/a` to recover the `Y_j` moment.
7. Separately instantiate `V-1=[t-1]-pi`, `pi(z)=z^p`, and the precision bound
   needed by the native recurrence. This line is not allowed to disappear into
   the generic polynomial theorem.

## Statement table

| Statement | Kind | Owner | Quantified structure | Hypotheses |
| --- | --- | --- | --- | --- |
| product log-derivative coefficient equals root power sum | generic law | Mathlib or reusable study module | field, polynomial, finite multiset | factored polynomial, nonzero leading coefficient |
| relative fiber power-sum identity | endpoint | H116 | `N,D,u,u0` | two factorizations, `R=N/D` |
| residue coefficient equals reciprocal derivative sum | generic law | reusable study module | field, separable polynomial | simple roots |
| `Y_j` fiber identity | endpoint | H116 | isogeny coordinate data | normalized differential formula, non-2-torsion target |
| Frobenius low-jet specialization | instance obligation or explicit premise | crypto-lean candidate | elliptic formal group | ordinary `E/F_p`, `V+pi=[t]` |
| conditional decoder cost | endpoint | H116/H115 theorem reuse | cyclic prime-order group | exact public digit predicate and `r^o(1)` evaluator |

## Inheritance table

| Operation/law | Inheritance |
| --- | --- |
| polynomial multiplication, derivative, evaluation | Mathlib instances; exact declarations lookup pending |
| finite product derivative | Mathlib lemma; exact declaration lookup pending |
| finite sums and coefficient extraction | Mathlib instances; reciprocal-at-infinity wrapper may be new |
| field division/cancellation | `Field K` laws |
| isogeny homomorphism and differential coordinate formula | hypothesis at study endpoint; crypto-lean lookup pending |
| `V+pi=[t]` and `pi(z)=z^p` | H115 algebra plus formal-group instance; lookup pending |
| radix-six decoder | reuse H115 generic theorem after source import/binding |

No `maxHeartbeats`, `maxRecDepth`, `native_decide`, copied curve group law or
per-curve theorem is permitted.
