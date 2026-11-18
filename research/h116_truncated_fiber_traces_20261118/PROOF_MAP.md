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
   on `h₁,...,h₇`, and proved to determine `F_U` through degree eight.
   Equation (15), transformed by `R(x)=xU(1/x)`, is formalized as a PowerSeries
   identity and is proved to imply the recurrence. Differentiating the
   normalized rational-map curve equation to equation (15) is also proved,
   under explicit constant-coefficient and nonzero-derivative hypotheses. The
   transformed equation (14) is formalized as a PowerSeries identity and is
   proved to imply the transformed equation (15); its cancelled factor is
   nonzero because its constant coefficient is one. The first identity's
   degree-two and degree-three coefficients are proved to force the two initial
   recurrence coefficients, so for prime `p>19` that identity alone implies the
   full finite recurrence. The remaining geometric bridge constructs the actual
   normalized isogeny at-infinity series and proves that it satisfies
   transformed equation (14). The formal-group congruence for `V-1` is a
   separate endpoint premise unless an existing verified elliptic formal-group
   theorem is located. A finite reversed numerator/denominator certificate is
   now proved sufficient for transformed equation (14), so the map bridge can
   be discharged by a checked polynomial identity rather than a trusted map
   evaluator. One excluded `p=31`, `r=37` Vélu control constructs this
   certificate and passes an independent Python replay; this is not an admitted
   H116 run or a fast-construction proof. A second formal bridge now proves
   that only eleven quotient coefficients and ten cleared-identity
   coefficients are required to force the same nine recurrence coefficients;
   the excluded fixture passes the exact boundary and mutation controls.
   [pcc:unverified]
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
| normalized rational-map curve equation implies equation (15) | generic law | H116 Lean module | differential field, coordinate `x`, map `R`, curve coefficients | coefficients constant under the derivation, `D x=1`, `D R≠0`, equation (14) |
| transformed equation (14) implies transformed equation (15) | instance law | H116 Lean module | field, source/target coefficients, normalized PowerSeries | `NormalizedIsogenyFirstOrderEquation`; the normalized series definition proves the cancelled factor nonzero |
| transformed equation (14) forces the initial coefficients and finite recurrence | instance law | H116 Lean module | field or `ZMod p`, source/target coefficients, normalized PowerSeries | nonzero 5, 7 and recurrence denominators; prime specialization uses `p>19` |
| reversed numerator/denominator certificate implies transformed equation (14) and the recurrence | instance law | H116 Lean module | field or `ZMod p`, normalized series, finite polynomials `N,D` | `D(0)=1`, `DU=N`, one displayed cleared-denominator polynomial identity; prime specialization uses `p>19` |
| fixed rational-map jet certificate implies the recurrence | instance law | H116 Lean module | field or `ZMod p`, normalized series, finite prefixes of `N,D` | `D(0)=1`, `DU=N mod X^11`, cleared identity modulo `X^10`; prime specialization uses `p>19` |
| transformed normalized-isogeny equation implies the finite recurrence | instance law | H116 Lean module | field or `ZMod p`, source/target coefficients, coefficient function | PowerSeries equation (15), initial `h₁,h₂`; generic denominator premises or prime `p>19` |
| `Y_j` fiber identity | endpoint | H116 | isogeny coordinate data | normalized differential formula, non-2-torsion target |
| Frobenius low-jet specialization | instance obligation or explicit premise | crypto-lean candidate | elliptic formal group | ordinary `E/F_p`, `V+pi=[t]` |
| conditional decoder cost | endpoint | H116/H115 theorem reuse | cyclic prime-order group | exact public digit predicate and `r^o(1)` evaluator |

## Low-jet certificate instance gate

The fixed-size certificate is expressed by equality after
`PowerSeries.trunc n`; `trunc n` retains exactly the coefficients of degrees
strictly below `n`.  The local definition is
`PowerSeriesPrefixEq n f g := trunc n f = trunc n g`.  It adds no new
structure above Mathlib power series and polynomials.

The exact proof dependencies frozen before the first Lean source edit were:

| Required step | Theorem used as stated / planned local lemma | Precision effect |
| --- | --- | --- |
| replace either factor by an equal prefix | `PowerSeries.trunc_trunc_mul`, `PowerSeries.trunc_mul_trunc`, `PowerSeries.trunc_trunc_mul_trunc` | preserves `n` |
| replace a power by an equal prefix | `PowerSeries.trunc_trunc_pow` | preserves `n` |
| differentiate an equal prefix | `PowerSeries.trunc_derivative` plus congruence of `Polynomial.derivative` | input `n+1`, output `n` |
| read an individual coefficient | `PowerSeries.coeff_coe_trunc_of_lt` | requires `k<n` |
| cancel a unit series | `PowerSeries.inv_mul_cancel`, after multiplying both prefixes by the inverse | preserves `n`; requires nonzero constant coefficient |
| lower the known precision | `PowerSeries.trunc_trunc_of_le` | from `n` to any `m<=n` |

The certificate object consists of the reversed numerator and denominator
series `N,D`, the normalized quotient prefix `U`, and the two cleared factors
already used by `NormalizedRationalMapSeriesCertificate`.  It requires only:

1. `D(0)=1`;
2. `D*U=N` modulo `X^11`;
3. the displayed cleared rational-map identity modulo `X^10`.

The assembly probe established prefix reflexivity, symmetry,
transitivity, monotonicity, addition, subtraction, multiplication, powers,
derivation and unit cancellation in a temporary file importing the pre-edit
H116 module. Only after those laws typechecked was the fixed certificate added
to the project source. No limit override was used.

The precision budget is explicit.  The quotient identity loses one
coefficient when differentiated, giving the first-order curve equation modulo
`X^10`.  Differentiating that equation loses one more coefficient, giving the
normalized ODE modulo `X^9`.  Coefficients of degrees zero through eight then
force the same two base coefficients and seven recurrence steps already proved
for the full identity.  Thus the certificate carries eleven coefficients
(degrees zero through ten) to certify the nine recurrence coefficients; the
last two are derivative guards, not decoded moments.
[pcc:unverified]

This gate discharges only fixed-size verification.  Constructing those eleven
correct coefficients from a generic prime-field isogeny, proving that they
represent the intended kernel quotient, and bounding that construction below
generic square-root work remain separate endpoint obligations.  The generic
prefix congruence lemmas are plausible `crypto-lean` candidates, but remain
local to H116 until the study demonstrates a reusable theorem boundary.

## Formal-scalar Frobenius-cutoff instance gate

1. **Structure above.** The existing H116 `PowerSeriesPrefixEq`, itself
   equality under Mathlib `PowerSeries.trunc`, represents equality through a
   finite local-parameter precision. The only operations touched here are the
   Mathlib monomial `PowerSeries.X ^ n`, truncation and coefficient extraction.
2. **Theorems used as stated.** `PowerSeries.trunc_X_pow_self_mul` states that a
   monomial of order `m` vanishes under `trunc m`;
   `PowerSeries.coeff_X_pow` identifies its first nonzero coefficient. The
   prime-gap step uses `Nat.Prime` plus finite arithmetic for the only integers
   strictly between 19 and 23.
3. **Object as an instance.** After the normalized local parameter loses the
   leading factor `z`, the first Frobenius correction is represented by
   `X^(p-1)`. Its invisibility is a `PowerSeriesPrefixEq 22` instance, not a new
   formal-series type.
4. **Assembly probe.** A temporary module importing `H116FiberTrace` first
   wrapped `PowerSeries.trunc_X_pow_self_mul` at its exact precision
   `trunc m (X^m)=0`, then used the pre-existing prefix monotonicity theorem to
   instantiate it at `n=22`, `m=p-1` under `[Fact p.Prime]` and `19<p`. The
   exact wrapper avoided the default-heartbeat timeout triggered by coercing a
   truncated polynomial back to a series inside a more general wrapper. The
   successful probe uses the default limits.
5. **Missing components only.** The H116-specific component is the arithmetic
   fact that a prime strictly above 19 is at least 23. The elliptic identities
   `V+pi=[t]`, `pi(z)=z^p`, the formal-group subtraction law and the regular
   unit factor in the elliptic x-coordinate remain separate semantic
   obligations; this cutoff lemma must not claim them.
6. **Inheritance table.** Monomial formation and coefficients come from
   Mathlib PowerSeries; prefix equality and monotonicity come from the existing
   H116 wrapper; primality comes from `Fact p.Prime`; the `22 <= p-1` bound is
   the only local instance proof. There is no limit override, copied formal
   group law or curve-specific computation.

| Statement | Kind | Owner | Quantified structure | Hypotheses |
| --- | --- | --- | --- | --- |
| a monomial of order `m` is zero below degree `m` | generic wrapper | H116 over Mathlib `PowerSeries.trunc_X_pow_self_mul` | semiring power series | none; lower precision uses existing `PowerSeriesPrefixEq.mono` |
| normalized Frobenius correction is invisible below degree 22 | instance obligation | H116 Lean module | `ZMod p` power series | `Fact p.Prime`, `19<p`, supplied identification of the correction with `X^(p-1)` |
| the first eleven normalized x-map coefficients equal the scalar-formal jet | endpoint obligation, not established by the cutoff alone | future elliptic formal-group owner | ordinary elliptic curve and normalized `V-1` map | `V+pi=[t]`, local Frobenius, regular x-coordinate unit substitution |

[pcc:unverified]

The implemented wrapper and three specialization theorems passed the repository
gate at `64 thm, 0 trusted` under the default limits. The first attempted
general wrapper coerced a truncated polynomial back to a series and exceeded
the default heartbeat budget; it was discarded. The checked implementation
uses Mathlib's theorem at its exact precision and the pre-existing H116
monotonicity lemma, so no limit was raised and no theorem was weakened.
[pcc:unverified]

## Generic leading-parameter prefix-transfer gate

1. **Structure above.** The same `PowerSeriesPrefixEq` truncation relation is
   used; no elliptic curve object is introduced. A local parameter is factored
   as `X*f`, where `f` has nonzero constant coefficient, and the normalized
   double-pole term is `f⁻¹²`.
2. **Theorems used as stated.** `PowerSeries.coeff_X_pow_mul` performs the
   coefficient shift behind cancellation of `X`; Mathlib's
   `PowerSeries.inv_mul_cancel` and `mul_inv_cancel` discharge inversion of
   unit power series. `PowerSeries.coeff_subst'` expands substitution
   coefficientwise. The existing H116 multiplication, power and coefficient
   congruences assemble the normalized pole result.
3. **Object as an instance.** `leading_X_inv_sq` is quantified over every field,
   cutoff and pair of power series. It consumes a prefix through degree `n`
   for `X*f` and `X*g` and returns a prefix through degree `n-1` for `f⁻¹²`
   and `g⁻¹²`; only the two visible unit hypotheses are required.
   `leading_X_inv_sq_mul_subst_inv` additionally composes any fixed regular
   outer unit with the two zero-constant parameters.
4. **Assembly probe.** The first proof let unrestricted simplification rewrite
   `X` as `X^1` and hit the default recursion-depth limit. The checked proof
   instead invokes `coeff_X_pow_mul` explicitly. `lake build` then passed at
   default limits, and the exact repository axiom auditor reported
   `73 theorems, 0 failing, 0 trusted`.
5. **Missing components only.** This now proves the double-pole and regular-unit
   substitution algebra for a supplied outer series. Identifying that series
   with the actual elliptic x-coordinate unit, then proving `V+pi=[t]`, local
   Frobenius and formal subtraction still require instances.
6. **Inheritance table.** Truncation, coefficient shifts and unit inverses are
   Mathlib operations; prefix arithmetic is the pre-existing H116 wrapper. The
   only new local laws reconstruct a prefix from coefficients, cancel `X`,
   preserve inverse/inverse-square prefixes and add an invisible `X^m`
   multiple. The new substitution law uses Mathlib's coefficient formula and
   `finsum` congruence. No curve law, limit override or native decision
   procedure appears.
   [pcc:unverified]

| Statement | Kind | Owner | Quantified structure | Hypotheses |
| --- | --- | --- | --- | --- |
| cancel a shared leading `X` with one-degree loss | generic law | H116 over Mathlib coefficient shift | field power series | input prefix through degree `n` |
| inversion and inverse square preserve prefixes | generic law | H116 over Mathlib unit inversion | field power series | nonzero constant coefficients |
| normalized double-pole prefix transfer | generic assembly | H116 | field power series | leading-`X` prefix and two unit hypotheses |
| regular-unit substitution and inverse | generic law | H116 over Mathlib substitution | field power series | zero inner constants and nonzero outer constant |
| normalized pole with regular unit | generic assembly | H116 | field power series | leading-`X` prefix and unit hypotheses |
| actual normalized elliptic x-ratio prefix | endpoint obligation | future elliptic formal-group owner | ordinary elliptic curve | identification of the regular series plus endomorphism identities |

The current source and direct repository-auditor receipts are pinned in
`lean/PROOF_RECEIPT.md`. The proof-gate cache-fetch preflight itself is recorded
as infrastructure-incomplete because the managed workspace lacked room for a
second Mathlib cache; the pinned build and exact axiom audit both completed.
[pcc:unverified]

The separately frozen `H116-PREFIX-TRANSFER-001` boundary control provides the
tightness check. Across 20 finite-field cases the local-parameter difference is
first visible exactly one degree after the normalized inverse-square
difference, and the boundary coefficient matches its independent closed form.
The zero and shifted-degree controls pass in all cases. This is experimental
support for the precision boundary, not an elliptic instance.
[pcc:unverified]

## Inheritance table

| Operation/law | Inheritance |
| --- | --- |
| polynomial multiplication, derivative, evaluation | Mathlib instances plus `Polynomial.derivative'` |
| finite product derivative | `Polynomial.derivative_prod_finset`, used as stated |
| finite sums and coefficient extraction | Mathlib finite sums plus existing H116 power-series reciprocal/convolution, concrete `F_U` coefficient lemmas and recurrence uniqueness through degree eight; reciprocal-at-infinity isogeny substitution remains open |
| field division/cancellation | `Field K` laws; `bostanDenominatorsNonzero` proves the concrete native divisions are defined for prime `p>19` |
| isogeny homomorphism and differential coordinate formula | hypothesis at study endpoint; no Mathlib isogeny object was found, so a finite `N,D` certificate is the checked interface; producing it from the actual kernel and independently validating map semantics remain open |
| `V+pi=[t]` and `pi(z)=z^p` | H115 algebra plus formal-group instance; lookup pending |
| radix-six decoder | reuse H115 generic theorem after source import/binding |

The assembly lookup found no ready-made `Derivation K (RatFunc K) (RatFunc K)`
instance. The proof therefore states the exact algebraic laws over an arbitrary
field equipped with a derivation, which is stronger at the identity layer, and
keeps the rational-function/formal-group instantiation explicit. It does not
claim that the missing Frobenius jet constructor exists or is fast.

No `maxHeartbeats`, `maxRecDepth`, `native_decide`, copied curve group law or
per-curve theorem is permitted.
