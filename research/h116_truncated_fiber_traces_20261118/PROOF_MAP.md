# H116 proof map and Lean instance gate

This map is written before the first H116 Lean declaration.

## Six instance-gate fields

1. **Structure above.** The algebraic core is over a commutative field `K`,
   `Polynomial K`, formal Laurent coefficients at infinity, finite multisets
   of roots and rational functions `N/D`. The elliptic endpoint additionally
   assumes an odd-degree separable isogeny coordinate map and its differential
   scalar `a`.
2. **Theorem used as stated.** The exact Mathlib declarations are
   `Polynomial.derivative'`, `Polynomial.derivative_prod_finset`,
   `Derivation.leibniz`, `Derivation.leibniz_inv`,
   `Derivation.leibniz_div`, `LaurentSeries.derivative_iterate_coeff`, and
   `RatFunc.coe_coe`. No curve-specific group law is re-proved in the study.
3. **Object as an instance.** `F_u=N-uD` is a polynomial; its root multiset is
   the coordinate image of the fiber. `R=N/D` is a rational function. The
   study endpoint instantiates generic polynomial identities with the isogeny
   coordinate data supplied as hypotheses.
4. **Assembly probe.** Before the first proof edit, a Lean 4.34.1/Mathlib
   `d13f23b7` probe typechecked generic product log-derivative, division,
   denominator cancellation, root-product, abstract coefficient/power-sum and
   residue-cancellation theorems. Probe source SHA-256 is
   `254caca6449fa388ea5d58331087c8b3df9f400c256a746d91a24f1575630456`;
   exit zero output SHA-256 is
   `e9f3b1c2e61cd9f42e2c075ca74448cf6c796e0e950d2b36487f24e024e7f08c`.
5. **Missing components only.** The generic finite reciprocal recurrence,
   derivative-times-reciprocal coefficient convolution and the exact formal
   series `F_U(w)=1-Uw+h₁w²+...` are now defined and proved at coefficient
   level. The formal normalized-map series is now proved to share every
   coefficient except the target-dependent linear term. For prime `p>19`, all
   denominators used by the native `h₁,...,h₇` recurrence are now proved
   nonzero in `ZMod p`. The exact finite recurrence is encoded, proved unique
   on `h₁,...,h₇`, and proved to determine `F_U` through degree eight. The
   remaining geometric bridge identifies the series at `w=1/x` with the actual
   isogeny Laurent expansion and proves that its jet satisfies the encoded
   recurrence. The formal-group congruence for `V-1` is a separate endpoint
   premise unless an existing verified elliptic formal-group theorem is
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
| reciprocal-series coefficient recurrence | generic law | H116 Lean module | field, two power series | unit constant coefficient, product equals one |
| derivative-times-reciprocal coefficient convolution | generic law | H116 Lean module | field, two power series | none |
| relative derivative convolution | generic law | H116 Lean module | field, four power series | none |
| concrete `F_U` coefficient and derivative formulas | instance law | H116 Lean module | field, `U`, coefficient function `h` | none |
| concrete `F_U` reciprocal recurrence | instance law | H116 Lean module | field, `U`, `h`, reciprocal series | product equals one |
| target/reference denominators share the same higher jet | instance law | H116 Lean module | field, `U`, `U0`, coefficient function `h` | none |
| native Bostan denominators are nonzero | instance law | H116 Lean module | `ZMod p` | `p` prime and `p>19` |
| finite Bostan recurrence determines `h₁,...,h₇` and `F_U` through degree eight | instance law | H116 Lean module | field, source/target coefficients, two coefficient functions | both functions satisfy `BostanJetRecurrence` |
| `Y_j` fiber identity | endpoint | H116 | isogeny coordinate data | normalized differential formula, non-2-torsion target |
| Frobenius low-jet specialization | instance obligation or explicit premise | crypto-lean candidate | elliptic formal group | ordinary `E/F_p`, `V+pi=[t]` |
| conditional decoder cost | endpoint | H116/H115 theorem reuse | cyclic prime-order group | exact public digit predicate and `r^o(1)` evaluator |

## Inheritance table

| Operation/law | Inheritance |
| --- | --- |
| polynomial multiplication, derivative, evaluation | Mathlib instances plus `Polynomial.derivative'` |
| finite product derivative | `Polynomial.derivative_prod_finset`, used as stated |
| finite sums and coefficient extraction | Mathlib finite sums plus proved H116 power-series reciprocal/convolution, concrete `F_U` coefficient lemmas and recurrence uniqueness through degree eight; reciprocal-at-infinity isogeny substitution remains open |
| field division/cancellation | `Field K` laws; `bostanDenominatorsNonzero` proves the concrete native divisions are defined for prime `p>19` |
| isogeny homomorphism and differential coordinate formula | hypothesis at study endpoint; crypto-lean lookup pending |
| `V+pi=[t]` and `pi(z)=z^p` | H115 algebra plus formal-group instance; lookup pending |
| radix-six decoder | reuse H115 generic theorem after source import/binding |

The assembly lookup found no ready-made `Derivation K (RatFunc K) (RatFunc K)`
instance. The proof therefore states the exact algebraic laws over an arbitrary
field equipped with a derivation, which is stronger at the identity layer, and
keeps the rational-function/formal-group instantiation explicit. It does not
claim that the missing Frobenius jet constructor exists or is fast.

No `maxHeartbeats`, `maxRecDepth`, `native_decide`, copied curve group law or
per-curve theorem is permitted.
