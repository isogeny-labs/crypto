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
   transformed equation (14). The generic formal-group filtration congruence
   is now proved against Mathlib's actual `FormalGroup`; identifying the
   supplied `V-1` map with that formal sum remains a separate elliptic endpoint
   premise. A finite reversed numerator/denominator certificate is
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
   `V+pi=[t]`, `pi(z)=z^p`, the elliptic negation certificate and the regular
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
   `80 theorems, 0 failing, 0 trusted` after the curve-coordinate instance was
   added.
5. **Missing components only.** This now proves the double-pole and regular-unit
   substitution algebra. The curve-coordinate instance below identifies the
   supplied series. Proving `V+pi=[t]`, local Frobenius and the actual elliptic
   negation certificate still requires an instance; the generic cancellation
   orientation is discharged by the later formal-group gate.
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
| actual normalized elliptic x-ratio prefix | curve-coordinate instance plus open endpoint | H116/future formal-group owner | ordinary elliptic curve | checked `q` equation plus endomorphism parameter identity |

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

`H116-REGULAR-SUBSTITUTION-001` separately tests the new regular-unit assembly.
All 20 substitution and inverse-substitution differences occur at degree
`m+1`; all complete normalized pole-coordinate differences occur at degree
`m`; and all three independent boundary formulas plus the frozen control
triplets pass. This still supplies no elliptic identification of the outer
series. [pcc:unverified]

## Short-Weierstrass coordinate-unit instance gate

1. **Structure above.** `WeierstrassCoordinateUnit A B q` is the exact fixed-
   point equation `q=1+A X⁴q²+B X⁶q³` obtained from the short-Weierstrass curve
   after writing `x(z)=z⁻²q(z)⁻¹`. No curve group law is copied.
2. **Theorems used as stated.** Mathlib constant coefficients show `q(0)=1`;
   unit cancellation proves the regularized affine identity. Mathlib power-
   series substitution, composition and coefficient maps prove coordinate
   rescaling and double substitution.
3. **Object as an instance.** `normalizedWeierstrassXRatio` is exactly the
   implementation expression `qSource*f⁻²*qTarget(Xf)⁻¹`. The generic prefix
   theorem is instantiated with source and target coordinate-unit equations.
4. **Assembly probe.** The source builds at default limits and the exact axiom
   auditor reports `80 theorems, 0 failing, 0 trusted`. The first rescaling
   proof exposed the distinction between `MvPowerSeries.C` and the univariate
   wrapper `PowerSeries.C`; rewriting through Mathlib's definitional bridge
   resolved it without changing the statement.
5. **Missing components only.** The algebra now proves the target coefficients
   `(a⁴A,a⁶B)` and the exact identity
   `qTarget(zNormalized)=qSource(zScalar)` whenever
   `a*zNormalized=zScalar`. The remaining endpoint must derive that parameter
   equality from `V+pi=[t]`, `pi(z)=z^p` and the actual elliptic negation law.
6. **Inheritance table.** Curve-unit arithmetic uses PowerSeries ring laws;
   rescaling uses `subst_comp_subst_apply`; prefix transfer reuses the prior
   H116 theorem. No isogeny semantics, decoder, complexity bound, limit
   override or native decision procedure is introduced. [pcc:unverified]

| Statement | Kind | Owner | Hypotheses |
| --- | --- | --- | --- |
| `q(0)=1` and cleared affine identity | curve-coordinate instance | H116 | coordinate-unit equation |
| `(A,B,q) -> (a⁴A,a⁶B,q(aX))` | generic scaling law | H116 over Mathlib substitution | field and scale |
| target/source substitution identity | generic assembly | H116 | zero-constant normalized parameter and `a*zNormalized=zScalar` |
| normalized x-ratio prefix | curve-coordinate assembly | H116 | source/target coordinate units plus leading-parameter prefix |
| derive normalized/scalar parameter identity | endpoint obligation | future formal-group owner | `V+pi=[t]`, local Frobenius and elliptic negation; generic cancellation is proved below |

`H116-CURVE-COORDINATE-001` is the finite instance control for this table. Its
fixed-point producer and independent recurrence verifier agree through 24
coefficients on five curves; wrong scaling and wrong parameter controls are
rejected in every cell. Identity certificate `IDC1h692b3e04c6a4f0cd` seals
the polynomial rescaling step and points to the audited Lean source. The
control supplies no replacement for the open endpoint row. [pcc:unverified]

## Frobenius--Verschiebung characteristic-polynomial instance gate

1. **Structure above.** The quantified object is an arbitrary possibly
   noncommutative ring containing elements `pi`, `verschiebung`, `trace` and
   `cardinality`. Only left cancellation by `pi` and centrality of `trace`
   relative to `pi` are exposed; the theorem does not assume the whole
   endomorphism ring is commutative.
2. **Theorems used as stated.** Ring multiplication distributes over addition;
   `Commute.eq` rewrites `trace*pi` as `pi*trace`; additive `abel` only
   rearranges the characteristic-polynomial terms. No elliptic theorem is
   re-proved inside this layer.
3. **Object as an instance.** The characteristic equation is supplied exactly
   as `pi^2-trace*pi+cardinality=0`, and the norm relation as
   `pi*verschiebung=cardinality`. Left cancellation then derives
   `verschiebung+pi=trace`.
4. **Assembly probe.** A temporary module first checked the commutative-ring
   specialization, then the stronger noncommutative statement with only the
   explicit commute premise. Both pass at default limits. After integration,
   the tracked module rebuilt at default limits and the exact auditor reported
   `94 theorems, 0 failing, 0 trusted`.
5. **Missing components only.** The actual elliptic endomorphism ring must
   still supply left cancellation of Frobenius, the norm relation, the
   Frobenius characteristic polynomial and central scalar interpretation.
   This gate derives the sum identity but does not construct those certificates.
6. **Inheritance table.** All manipulation is inherited from Mathlib ring and
   additive-group laws. H116 contributes only their exact composition into the
   endomorphism identity needed by the local-parameter endpoint. No curve law,
   isogeny implementation, runtime claim, trusted decision procedure or limit
   override is introduced. [pcc:unverified]

| Statement | Kind | Owner | Quantified structure | Hypotheses |
| --- | --- | --- | --- | --- |
| `V+pi=trace` from characteristic and norm equations | abstract endomorphism algebra | H116 over Mathlib ring laws | possibly noncommutative ring | left cancellation by `pi`, `pi` commutes with `trace`, `pi*V=cardinality`, characteristic equation |
| actual elliptic `V+pi=[t]` | endpoint instance | future elliptic formal-group owner | elliptic endomorphism ring | bind the four abstract premises to Frobenius, Verschiebung and scalar maps |

## Additive-endomorphism specialization instance gate

1. **Structure above.** The quantified object is `AddMonoid.End G` for an
   arbitrary additive commutative group `G`. Mathlib equips this type with a
   generally noncommutative ring structure whose multiplication is function
   composition, so `(pi * V)(P)=pi(V(P))`. Integer casts are the scalar
   endomorphisms `P -> n • P`.
2. **Theorems used as stated.** `AddMonoid.End.coe_mul` fixes the composition
   orientation; `AddMonoidHom.map_zsmul` proves that every additive
   endomorphism commutes with every integer scalar map; extensionality and
   `Function.Injective pi` prove cancellation of left composition by `pi`.
   The preceding abstract
   `frobeniusVerschiebung_sum_of_characteristic` theorem performs the remaining
   ring calculation.
3. **Object as an instance.** `pi` and `V` are additive endomorphisms of `G`;
   `trace` and `cardinality` are integers, coerced to scalar endomorphisms.
   From pointwise injectivity of `pi`, `pi*V=[cardinality]`, and
   `pi^2-[trace]*pi+[cardinality]=0`, the specialization derives
   `V+pi=[trace]` as equality of additive endomorphisms.
4. **Assembly probe.** A temporary module importing the tracked H116 module
   and `Mathlib.Algebra.Group.Hom.End` checked scalar centrality, cancellation,
   and the composed theorem at default limits. Probe source SHA-256 is
   `662669a9b4d377d30dda3d03cffe5d61f86e4dfe7b65af6454a8a2a36ae77b46`;
   successful output SHA-256 is
   `822260e51bb3c89e64865a33a1fde187afdf55670fa38e51b32c4a6d71b95b93`.
   A second probe added the frozen noninjective mutation control on
   `AddMonoid.End (ZMod 2)`; source SHA-256 is
   `43346a0e463c9f238ba179aff9c02b48a95b86efb69722b4f763d506e14e5df2`
   and successful output SHA-256 is
   `46055e171c5b3dd5f6753c2d0a0ac3e7ca4c82cd0546d98a958ccd9bd9da98ba`.
5. **Missing components only.** Scalar centrality and the abstract
   left-cancellation premise are no longer endpoint certificates. An actual
   elliptic instance must still bind this theorem to the chosen point group and
   supply Frobenius, Verschiebung, norm and characteristic-polynomial data.
   The following affine-point gate discharges Frobenius construction and
   injectivity. Equality on points must still be connected to the local
   formal-group series; this specialization does not silently identify the two
   categories.
6. **Inheritance table.** Composition, the endomorphism ring, integer scalar
   maps, preservation of integer multiples, and function extensionality are
   inherited from Mathlib. H116 only specializes its preceding abstract ring
   identity. No curve law, point-counting theorem, endomorphism constructor,
   decoder, complexity claim, trusted decision procedure or limit override is
   introduced. [pcc:unverified]

| Statement | Kind | Owner | Quantified structure | Hypotheses |
| --- | --- | --- | --- | --- |
| integer scalar maps are central in `AddMonoid.End G` | inherited instance law | H116 over `map_zsmul` | additive commutative group | none |
| injective `pi` cancels left composition | inherited instance law | H116 over function extensionality | additive commutative group | `Function.Injective pi` |
| `V+pi=[trace]` on an additive point group | endomorphism specialization | H116 | `AddMonoid.End G` | injective `pi`, norm equation, characteristic equation |
| noninjective mutation rejects the sum conclusion | negative control | H116 | `AddMonoid.End (ZMod 2)` | `pi=V=0`, `trace=1`, `cardinality=2`; norm and characteristic equations hold but the conclusion fails |
| affine elliptic-point refinement | following instance gate | H116 over Mathlib | nonsingular points of a base-changed Weierstrass curve | bind `pi` via field Frobenius; leave `V`, norm and characteristic equations explicit |

## Affine elliptic-point Frobenius instance gate

1. **Structure above.** Let `K` be a finite field, `F` a field extension of
   `K`, and `W` an affine Weierstrass curve over `K`. Mathlib's nonsingular
   point type on `W.baseChange F` is an additive commutative group. The
   endomorphism is `WeierstrassCurve.Affine.Point.map` applied to
   `FiniteField.frobeniusAlgHom K F`, which raises coordinates to `#K`.
2. **Theorems used as stated.** `Affine.Point.map` is already an additive group
   homomorphism induced by a base-preserving algebra map;
   `Affine.Point.map_injective` proves it injective;
   `FiniteField.coe_frobeniusAlgHom` identifies its field action with the
   `#K`-power map; and `ZMod.card` specializes `#(ZMod p)` to `p`.
3. **Object as an instance.** `affineFrobeniusPointEnd W` is an actual
   `AddMonoid.End` of nonsingular points on the base-changed curve. On an
   affine point `(x,y)` it maps coordinates to `(x^(#K),y^(#K))`; for
   `K=ZMod p` these are exactly `(x^p,y^p)`. The composed theorem derives
   `V+pi=[t]` from only `pi*V=[p]` and
   `pi^2-[t]*pi+[p]=0`, because construction and injectivity of `pi` are now
   internal.
4. **Assembly probe.** A temporary module importing the tracked H116 module,
   Mathlib's affine point API and finite-field Frobenius API checked the
   constructor, injectivity, coordinate formula, generic finite-field sum
   theorem and `ZMod p` specialization at default limits. Probe source SHA-256
   is `8482c86b0afb73ef9b5ac7fbce0a4155ad0f652b1b2b64c497069daaa4c2e217`;
   successful output SHA-256 is
   `7a5401d3108c1e58a5838a9bbfacb8356c0a5b163bcfa484e73282388a660aa4`.
5. **Missing components only.** The point Frobenius map, its additive law,
   injectivity and exact prime-field coordinate action are discharged. The
   following gate constructs the point-group Verschiebung and proves the norm
   relation over an algebraic extension. The characteristic-polynomial
   equality and transport from point-endomorphism equality to equality of the
   chosen local formal-group series remain open. A decidable-equality instance
   for the extension field is explicit because the pinned affine group-law API
   requires it; a classical instance is admissible but no computability claim
   is made.
6. **Inheritance table.** The curve, nonsingular point group, base change,
   coordinate map, additive-homomorphism proof, injectivity proof, finite-field
   Frobenius and `ZMod` cardinality are all inherited from Mathlib. H116 names
   their composition and applies the previously audited endomorphism theorem.
   No curve addition formula, isogeny law, point-counting result, decoder,
   complexity claim, trusted decision procedure or limit override is added.
   [pcc:unverified]

| Statement | Kind | Owner | Quantified structure | Hypotheses |
| --- | --- | --- | --- | --- |
| coordinatewise finite-field Frobenius is an injective additive point endomorphism | elliptic instance | H116 over Mathlib | nonsingular points of `W.baseChange F` | finite base field, field extension, decidable equality used by pinned point API |
| over `ZMod p`, point coordinates map as `(x,y) -> (x^p,y^p)` | prime-field instance | H116 over Mathlib | base-changed affine Weierstrass points | `Fact p.Prime` |
| `V+pi=[t]` on actual affine points | composed endpoint | H116 | `AddMonoid.End ((W.baseChange F).Point)` | supplied `V`, norm equation, characteristic equation |
| pointwise Verschiebung and norm refinement | following instance gate | H116 over Mathlib | algebraic extension affine points | construct `[p]*piInverse`; prove norm internally; leave characteristic equation and local-series semantics explicit |

## Algebraic-extension pointwise Verschiebung instance gate

1. **Structure above.** Specialize the extension field `E/ZMod p` to an
   algebraic field extension. Mathlib's
   `FiniteField.frobeniusAlgEquivOfAlgebraic` makes the field Frobenius an
   algebra automorphism. Mapping its inverse through the same affine-point
   functor gives an inverse Frobenius point endomorphism. Define the pointwise
   Verschiebung candidate as `[p]` composed with that inverse.
2. **Theorems used as stated.** `Affine.Point.map_map` identifies composition
   of point maps with composition of algebra maps; the Frobenius algebra
   equivalence's `apply_symm_apply` law makes the composite the identity.
   `AddMonoidHom.map_nsmul` centralizes `[p]` against Frobenius. The preceding
   prime-field affine-point theorem then consumes the proved norm relation.
3. **Object as an instance.** `zmodAffineFrobeniusPointInvEnd W` is the point
   map of inverse Frobenius, and
   `zmodAffineVerschiebungPointEnd W=[p]*piInverse`. Lean proves
   `pi*piInverse=1`, hence `pi*V=[p]`. Consequently the characteristic equation
   alone implies `V+pi=[t]` on the actual nonsingular affine point group.
4. **Assembly probe.** A temporary module importing the tracked H116 module
   checked inverse composition, the pointwise Verschiebung definition, the norm
   equation and the characteristic-only sum theorem at default limits. Probe
   source SHA-256 is
   `f1f896488d960bfb19ae6f72277371f0bceb7e60d4927511c209695b74d7db7e`;
   successful output SHA-256 is
   `428c1eaefa5fbef1303cb11a5e250595caf4da3a9f6c0524ac38ac0ae8edc730`.
5. **Missing components only.** This constructs an additive point-group map
   with the defining norm identity of Verschiebung. It does not prove that the
   map is represented by the retained rational/scheme morphism, nor that point
   equality implies equality of local formal power series. The Frobenius
   characteristic equation remains an explicit premise. Those three items are
   the semantic endpoint; no asymptotic claim follows from the pointwise
   construction.
6. **Inheritance table.** Algebraicity, Frobenius automorphism, inverse laws,
   affine-point functoriality, scalar multiplication and additive composition
   are inherited from Mathlib. H116 only defines `[p]*piInverse` and composes
   the inherited laws. No curve addition formula, isogeny evaluator,
   point-counting proof, decoder, runtime claim, trusted decision procedure or
   limit override is introduced. [pcc:unverified]

| Statement | Kind | Owner | Quantified structure | Hypotheses |
| --- | --- | --- | --- | --- |
| inverse field Frobenius induces an inverse point endomorphism | elliptic point instance | H116 over Mathlib | nonsingular affine points after algebraic base change | `Algebra.IsAlgebraic (ZMod p) E` |
| pointwise `V=[p]*piInverse` satisfies `pi*V=[p]` | constructed norm identity | H116 | additive point endomorphism ring | same algebraic extension |
| characteristic equation implies `V+pi=[t]` | composed point theorem | H116 | actual affine point group | supplied characteristic equation only |
| identify pointwise `V` with retained rational/scheme map and local series | endpoint obligation | future elliptic owner | rational morphism and local formal group | morphism semantics, characteristic equation, equality-to-series transport |

The separately frozen `H116-POINTWISE-CHARPOLY-001` panel exhaustively checks
the supplied characteristic equation and constructed pointwise `V` on 5,066
points across eight extension-field cells. All positive equations and the
point-count recurrence pass, and a separate Python implementation matches
every correctness field. The protocol result remains a failure because one
missing-inverse mutation is identical to the correct map on the 35-point
`p=5`, degree-two group. This is finite falsification evidence for the control
design only; it supplies no generic theorem and leaves the endpoint row above
unchanged. [pcc:unverified]

## Pointwise `V-1` orientation and observable instance gate

1. **Structure above.** Work first in `AddMonoid.End G` for an arbitrary
   additive commutative group, then specialize `G` to Mathlib's nonsingular
   affine point group. The observable is an arbitrary function on points; the
   chosen elliptic instance sends infinity to zero and an affine point
   `(x,y)` to `-x/y`.
2. **Theorems used as stated.** Additive-group rearrangement turns
   `V+pi=[t]` into `V-1=[t-1]-pi`. `DFunLike.congr_fun` evaluates equality of
   additive endomorphisms at a point. The preceding pointwise Verschiebung
   theorem supplies the sum identity under the explicit characteristic
   premise.
3. **Object as an instance.** The left side is the actual constructed
   pointwise `V` applied to `P`, then subtracts `P`. The right side is
   `(t-1)P-pi(P)`. Applying the selected local-parameter function transports
   the point equality without asserting equality of rational maps or formal
   power series.
4. **Assembly probe.** A temporary module checked the endomorphism
   rearrangement, its characteristic-polynomial specialization, the total
   local-parameter definition and the observed point equality at default
   limits. Probe source SHA-256 is
   `0f62a2522b7f2c538df1df4d3bdd5b5e236269dad570b2a00e9aeee6df2e0aec`;
   successful empty output SHA-256 is
   `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`.
5. **Missing components only.** Equality after evaluation on algebraic points
   does not construct the retained rational/scheme morphism or identify its
   completed local-ring map. A later theorem must show that the retained map
   represents the constructed pointwise endomorphism and transport it to the
   selected formal parameter series.
6. **Inheritance table.** Endomorphism subtraction, integer scalar casts,
   point-group subtraction, the constructed Frobenius and pointwise `V` are
   inherited from Mathlib and the preceding H116 declarations. This gate adds
   only additive rearrangement and congruence through a total observable. It
   introduces no point-counting theorem, rational-map evaluator, series
   equality, runtime claim or trusted procedure. [pcc:unverified]

| Statement | Kind | Owner | Quantified structure | Hypotheses |
| --- | --- | --- | --- | --- |
| `V-1=[t-1]-pi` from `V+pi=[t]` | generic endomorphism algebra | H116 | `AddMonoid.End G` | supplied sum identity |
| characteristic equation implies pointwise `V-1` orientation | affine-point specialization | H116 | algebraic prime-field extension points | supplied characteristic equation |
| equality survives an arbitrary point observable | generic congruence | H116 | function from point group to any type | endomorphism orientation |
| `z=-x/y` agrees on the two point expressions | elliptic observable instance | H116 | actual affine points | supplied characteristic equation |
| equality of retained rational/formal maps | endpoint obligation | future morphism owner | rational morphism and completed local ring | representation and local-series transport |

## Generating-set characteristic-certificate gate

1. **Structure above.** Work in `AddMonoid.End G` and a set `S : Set G` whose
   additive-subgroup closure is top. For the elliptic specialization, `G` is a
   finite extension-field affine point group and the trace is tied by
   definition to `p+1-Nat.card E(F_p)`.
2. **Theorems used as stated.** `AddMonoidHom.eq_of_eqOn_dense` promotes
   equality of additive homomorphisms on `S` to global equality when
   `AddSubgroup.closure S=top`. Evaluation of the characteristic-polynomial
   endomorphism on `S` supplies the equality with the zero homomorphism.
3. **Object as an instance.** The certificate contains a generating set, a
   checked closure-to-top proof and one characteristic-equation value per
   generator. The resulting global equation feeds the already audited
   pointwise `V-1` orientation and observable theorems.
4. **Assembly probe.** A temporary module checked the generic
   zero-on-generators theorem, the characteristic-polynomial wrapper, the
   point-count trace definition and the affine-point specialization at default
   limits. Probe source SHA-256 is
   `f928c423eca1b6b279ea54f79f7fd5675ac4bc6b66a3c15fd98787d5cda3bccc`;
   successful empty output SHA-256 is
   `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`.
5. **Missing components only.** A certificate for one finite extension point
   group does not prove equality on the algebraic closure or equality of
   rational maps. Generic Frobenius theory or a separating morphism theorem is
   still required for the family-level endpoint.
6. **Inheritance table.** Additive subgroup closure and homomorphism extensionality
   are inherited from Mathlib. H116 packages the characteristic expression and
   the already constructed point maps. No enumeration algorithm, curve group
   law, point-count proof, asymptotic claim or trusted procedure enters Lean.
   [pcc:unverified]

| Statement | Kind | Owner | Quantified structure | Hypotheses |
| --- | --- | --- | --- | --- |
| zero on generators implies zero endomorphism | generic group law | Mathlib plus H116 wrapper | additive commutative group | closure of supplied set is top |
| generator checks imply characteristic equation | proof-carrying certificate | H116 | `AddMonoid.End G` | closure certificate and per-generator evaluations |
| trace equals `p+1-#E(F_p)` | definition | H116 over `Nat.card` | prime-field affine point type | finite base field |
| generator certificate implies pointwise `V-1` orientation | elliptic specialization | H116 | finite algebraic extension points | closure and per-generator checks |
| algebraic coordinate pair has a common Frobenius period | finite-subfield theorem | Mathlib plus H116 composition | algebraic prime-field extension | finite-dimensional adjoin of both coordinates |
| algebraic affine points exhaust positive Frobenius-fixed loci | elliptic specialization | H116 | algebraic extension points | coordinate iteration and common period |
| fixed-locus checks imply global characteristic equation | generic endomorphism theorem | H116 | `AddMonoid.End G` | every point periodic and checks on every positive fixed locus |
| fixed-locus characteristic family implies pointwise `V-1` | elliptic specialization | H116 | algebraic extension points | restricted characteristic identity for every positive period |
| restricted characteristic identities and rational-map equality | endpoint obligation | future elliptic owner | elliptic morphisms | standard Frobenius theorem on all fixed loci plus morphism identification |

`H116-GENERATOR-CHARPOLY-002` instantiates the finite certificate on the eight
parent groups. Three cells are cyclic and five use two generators, compressing
5,066 exhaustive evaluations to 13. Both coefficient mutations and every
dropped-generator control reject; the independent verifier has zero field
mismatches. This measures the finite interface only and does not discharge the
generic endpoint row. [pcc:unverified]

The 119-theorem checkpoint replaces the remaining unstructured quantifier over
algebraic points by an exact family of finite fixed loci. The pair-coordinate
adjoin is finite, the `n`th point Frobenius raises coordinates to `p^n`, and
every algebraic point is consequently periodic. Vanishing on every positive
fixed locus promotes to a global endomorphism equality and composes to
pointwise `V-1`. The `ZMod 3` single-locus control retains an identity mutation
that is invisible at period one but exposed at period two. The frozen
fixed-loci census passes all eight cells with exact replay: the base locus
omits 5,002 of 5,066 points, proper-period strata omit 4,924 full-period
points, and the correct characteristic expression has zero stratum failures.
This instantiates but does not prove the restricted characteristic family.
[pcc:unverified]

The 121-theorem checkpoint adds a separating certificate for a supplied pair
of rational functions. Agreement on a finite regular sample implies equality
of cross products when the sample cardinality is strictly larger than both
cross-product degrees. A `ZMod 3` theorem proves strictness is essential:
`X^3-X` is nonzero of degree three but vanishes on all three field elements.
The frozen separation-capacity census independently replays all eight cells.
Their 5,058 affine points give 2,534 distinct x-values and raw strict degree
capacities from 16 through 1,236. In every cell the nonzero monic sample
vanishing polynomial has degree exactly the sample size and vanishes on every
sample, while a constant mutation rejects everywhere. The actual H116
numerators, denominators, pole set and cross-product degree remain open
certificate inputs; no morphism or speedup follows from capacity alone.
[pcc:unverified]

The subsequent split-pole reconstruction deliberately failed: it had
incorrectly identified `ker(V-1)` with `E(F_p)=ker(pi-1)`. The failure exposed
the right eigenspace. On a point fixed by `V`, the sum identity forces
`pi(P)=[t-1]P`; Lean proves this, proves its `n`-fold iteration, and proves that
Frobenius fixed-locus membership on `ker(V-1)` is exactly the scalar fixed
condition. The corrected frozen v2 experiment jointly solves for numerator
and denominator without requiring split roots. Both `2N`-sample windows have
full rank in all eight cells and recover identical degree-`(N,N-1)` base-field
maps. The maps are stable across extension degrees and have zero mismatches on
2,524 regular x-values. The cyclic-kernel prediction
`gcd(N,(t-1)^d-1)` matches every observed kernel, but cyclicity and the generic
degree/cardinality statement remain open elliptic obligations.
[pcc:unverified]

The next audited ring step proves `(V-1)(pi-1)=p+1-t` directly from
`V*pi=p` and `V+pi=t`, even in a noncommutative ring. This supplies the exact
algebraic norm factor behind the observed degree `N`; only the elliptic
morphism theorem translating that factor into degree and kernel cardinality is
missing. [pcc:unverified]

The 128-theorem checkpoint turns that last translation into an explicit
certificate. For an abstract multiplicative degree, a factorization
`left*right=scalar`, scalar degree `N^2`, and equality of the two factor degrees
imply that each factor has degree `N`. Composing with the norm factor gives
`deg(V-1)=N` from degree multiplicativity, `deg[N]=N^2`, and
`deg(V-1)=deg(pi-1)`. The intended elliptic source of the last equality is
duality, `(V-1)^vee=pi-1`. A checked `4*9=6^2` mutation proves that product
degree alone cannot discharge it. The pinned Mathlib tree contains no elliptic-
isogeny, dual-isogeny or morphism-degree object, so these three premises remain
future instance obligations rather than hidden axioms. [pcc:unverified]

The 134-theorem checkpoint discharges two strictly algebraic sub-obligations.
`finiteFieldTower_degree_mul` imports the exact finite-rank tower law, and
`finiteFieldTower_degree_eq_norm_of_paired` shows that an already-constructed
pullback tower with equal adjacent degrees and total degree `N^2` has adjacent
degree `N`. Separately, Mathlib's division-polynomial theorems give exact
coordinate degrees `deg Phi_n=n^2` and `deg Psi_n^2=n^2-1`, with leading
coefficients `(1,n^2)`, when `n` is nonzero in the coefficient field. If
`n` instead vanishes in the field and `|n|>1`, Lean proves the denominator
degree is strictly below `n^2-1`; a `ZMod 3` theorem records a concrete
instance. The exhaustive frozen experiment checks the
coordinate quotient against direct scalar multiplication on 5,058 affine
points with zero failures and independently reproduces the characteristic
degree collapse. Remaining obligations are the concrete function-field
pullback tower, identification of `Phi_n/Psi_n^2` with the point/scheme map,
coordinate degree equals morphism degree, and the duality
`(V-1)^vee=pi-1`. [pcc:unverified]

The frozen kernel x-orbit experiment tests the denominator consequence that
point enumeration misses. For cyclic `ker(V-1)`, a rational x-root occurs when
`(t-1)^d P=+P` or `-P`, because `x(P)=x(-P)`. All eight cells match the locked
union formula, exact simple/double multiplicities, zero numerator cancellation,
and an independent replay. The plus-only model fails exactly its two frozen
negative controls: it misses three roots at `p=5,d=3` and two at `p=7,d=2`.
This repairs the finite denominator interpretation but does not prove the
generic cyclic or scheme-theoretic premises. [pcc:unverified]

The next frozen dual-pair experiment reconstructs `pi-1` from two disjoint
pointwise interpolation windows. Its three base-field maps have the same exact
degrees as the recovered `V-1` maps, all 2,505 regular evaluations and 29 poles
match, and the complete point census verifies
`(V-1)o(pi-1)=[N]` with zero failures on 5,066 points. Omitting the subtraction
from `pi-1` fails 5,040 times; replacing `[N]` by `[N+1]` fails at all 5,058
nonidentity points. This instantiates the paired-degree pattern only on the
finite panel; the abstract dual-isogeny and scheme-degree premises remain open.
[pcc:unverified]

## Quadratic function-field sandwich instance gate

1. **Structure above.** A dominant curve map induces two compatible finite
   function-field towers over the pulled-back target x-line `B`. The source
   tower is `B -> Xs -> E`, where `Xs` is the source x-line and `E` the source
   curve field. The target tower is `B -> Xt -> E`, where `Xt` is the pulled-
   back target curve field. The desired curve-map degree is `[E:Xt]`; the
   rational coordinate degree is `[Xs:B]`.
2. **Theorems used as stated.** `Module.finrank_mul_finrank` supplies
   `[E:B]=[Xs:B][E:Xs]` and `[E:B]=[Xt:B][E:Xt]`. If both curve fields are
   quadratic over their x-lines, then
   `coordinateDegree * 2 = 2 * curveMapDegree`, and natural-number
   cancellation gives equality. Mathlib's
   `RatFunc.finrank_eq_max_natDegree` separately identifies
   `[K(X):K(f)]` with the maximum degree of the canonical reduced numerator
   and denominator of `f`.
3. **Object as an instance.** The Lean statement quantifies over four fields
   `B`, `Xs`, `Xt`, `E`, the two existing `IsScalarTower` instances, the two
   quadratic-degree equalities and the coordinate extension degree. It does
   not define a new curve, rational-map or function-field representation.
4. **Assembly probe.** Build the generic theorem in the tracked H116 module at
   default limits, then run the exact theorem/axiom auditor. No finite curve or
   H116-specific coefficient is needed for elaboration.
5. **Missing components only.** The theorem does not construct `B`, `Xs`,
   `Xt`, or `E` from the retained rational formulas. The rational-function
   wrapper closes `[K(X):K(f)] = max(deg num(f),deg denom(f))`, including
   canonical cancellation, but does not prove that supplied polynomials such
   as `Phi_n` and `Psi_n^2` are already coprime or identify their quotient with
   the curve map. The quadratic field equalities, dominance, separability and
   duality of `V-1` and `pi-1` are also caller obligations. [pcc:unverified]
6. **Inheritance table.** Fields, algebras, scalar towers and `Module.finrank`
   come from Mathlib. The only new endpoint is cancellation between the two
   existing tower equalities. The mutation control uses unequal line degrees
   to show that a shared total degree alone does not identify adjacent degrees.

| Statement | Kind | Owner | Quantified structure | Hypotheses |
| --- | --- | --- | --- | --- |
| `quadraticFunctionFieldSandwich_degree` | generic law | H116 over Mathlib field towers | four fields and two scalar towers | both line-to-curve degrees are two; supplied coordinate extension degree |
| `rationalFunctionCoordinateExtension_degree` | inherited rational-function law | H116 wrapper over Mathlib `RatFunc.finrank_eq_max_natDegree` | one rational function over an arbitrary field | none; numerator and denominator are canonical reduced forms |
| `unequalCoordinateLineDegrees_control` | boundary control | H116 arithmetic | natural numbers | concrete equality `4*3=2*6` with unequal adjacent degrees |

## Reduced quotient and no-cancellation instance gate

1. **Structure above.** For `p,q in F[X]`, the displayed rational function is
   `f=(p:RatFunc F)/(q:RatFunc F)`. Mathlib's `RatFunc.num` and
   `RatFunc.denom` divide by `gcd p q` and scale by a nonzero constant so that
   the denominator is monic.
2. **Theorems used as stated.** `RatFunc.num_div` and `RatFunc.denom_div`
   expose the canonical normalization. `IsCoprime.isRelPrime` together with
   `gcd_isUnit_iff_isRelPrime`, `normalize_eq_one`, and `normalize_gcd` makes
   the normalized gcd equal to one. `Polynomial.natDegree_C_mul` preserves
   degree under the remaining nonzero scalar. The existing
   `rationalFunctionCoordinateExtension_degree` then computes the extension
   degree.
3. **Object as an instance.** The planned theorem is generic over a field and
   two polynomials. It assumes `q != 0` and `IsCoprime p q`; it does not assume
   that any particular division polynomials are coprime.
4. **Assembly probe.** First check the generic reduction in a temporary module
   importing the tracked H116 file, at default limits. Only after the exact
   numerator and denominator degree equalities elaborate may the theorem move
   into the tracked module and enter the exact axiom audit.
5. **Missing components only.** Instantiating the theorem with
   `Phi_n,Psi_n^2` still requires a proof of `IsCoprime`; identifying that
   quotient with scalar multiplication and constructing the curve-field tower
   remain separate. The frozen polynomial-gcd panel below measures this premise
   on out-of-sample toy cells but cannot prove it generically. [pcc:unverified]
6. **Inheritance table.** Polynomial gcd, coprimality, rational-function
   normalization and finite extension degree all come from Mathlib. The local
   theorem merely connects their exact public APIs. A shared-factor mutation
   and a singular cubic are retained as negative controls.

| Statement | Kind | Owner | Quantified structure | Hypotheses |
| --- | --- | --- | --- | --- |
| displayed coprime quotient retains numerator and denominator degrees | generic bridge | H116 over Mathlib RatFunc normalization | field and two polynomials | nonzero denominator and `IsCoprime p q` |
| displayed coprime quotient has extension degree `max(deg p,deg q)` | generic assembly | H116 over the preceding bridge and `RatFunc.finrank_eq_max_natDegree` | rational function field | same hypotheses |
| shared-factor mutation is not coprime | negative control | native finite polynomial arithmetic | fixed toy polynomials | multiply both sides by the same linear factor |

## Scalar-two short-Weierstrass coprimality instance gate

1. **Structure above.** Put `f=X^3+A*X+B`,
   `N=X^4-2*A*X^2-8*B*X+A^2`, and
   `d=4*A^3+27*B^2`.  Direct polynomial expansion gives the explicit
   Bezout identity
   `(d*(4*A+3*X^2))*N + (d*(5*A*X+27*B-3*X^3))*f = d^2`.
   Thus `d != 0` makes `N` and `f` coprime over any field.
2. **Theorems used as stated.** `IsCoprime` is Mathlib's explicit Bezout
   predicate.  `WeierstrassCurve.Phi_two` and `PsiSq_two` expose the exact
   scalar-two division polynomials.  The definitions of `b2,b4,b6,b8`
   specialize the short curve `(0,0,0,A,B)` to `N` and `4*f`.
   `isCoprime_mul_unit_left_right` removes the nonzero constant factor four
   when `2 != 0`.
3. **Object as an instance.** The first theorem proves coprimality of
   the displayed `N` and `f` from the explicit discriminant factor `d != 0`.
   The endpoint theorem instantiates Mathlib's exact `Phi 2` and `PsiSq 2`
   for `W=<0,0,0,A,B>`, with no copied division-polynomial recurrence.  The
   final assembly applies the preceding reduced-quotient theorem and Mathlib's
   exact division-polynomial degrees to prove coordinate extension degree four.
4. **Assembly probe.** Both the Bezout witness and exact Mathlib specialization
   first elaborated in a temporary module importing the tracked H116 file at
   default limits.  The tracked project then completed all 3,162 build jobs;
   the exact repository auditor reports `142 theorems, 0 failing, 0 trusted`.
5. **Missing components only.** This is an actual nonsingular scalar-two
   instance of the no-cancellation premise; it is not a proof for arbitrary
   scalar `n`.  The `2 != 0` premise excludes characteristic two, where this
   short model and the factor-four reduction degenerate.  Identifying the
   quotient with pointwise scalar multiplication and the curve-field map
   remains separate. [pcc:unverified]
6. **Inheritance table.** Field inversion, constant-polynomial embedding,
   division-polynomial definitions and unit-invariance of coprimality are
   inherited from Mathlib.  H116 supplies only the explicit Bezout witness
   and the exact specialization.  The singular case `A=B=0` remains the
   frozen negative control in the independent finite-field experiment.

| Statement | Kind | Owner | Quantified structure | Hypotheses |
| --- | --- | --- | --- | --- |
| explicit `N,f` Bezout certificate | constructive instance law | H116 over polynomial ring laws | arbitrary field and `A,B` | `4*A^3+27*B^2 != 0` |
| `IsCoprime (W.Phi 2) (W.PsiSq 2)` for `W=<0,0,0,A,B>` | exact Mathlib endpoint | H116 over Mathlib division polynomials | short Weierstrass curve over arbitrary field | discriminant factor nonzero and `2 != 0` |
| `[F(X):F(Phi_2/Psi_2^2)]=4` | exact coordinate-degree assembly | H116 over the coprimality endpoint and reduced-quotient theorem | rational function field of the short curve's x-coordinate | same hypotheses |
| singular cubic loses the premise | negative control | frozen Rust verifier | `y^2=x^3` over `F_13` | `A=B=0` |

## Scalar-three short-Weierstrass coprimality instance gate

1. **Structure above.** Put `f=X^3+A*X+B`,
   `q=3*X^4+6*A*X^2+12*B*X-A^2`, and
   `r=X^6+5*A*X^4+20*B*X^3-5*A^2*X^2-4*A*B*X-8*B^2-A^3`.
   The scalar-three numerator is `p=X*q^2-8*f*r`.  With
   `d=4*A^3+27*B^2`, exact extended Euclid gives `U*p+V*q=64*d^2`, where
   `U=27*(15*A*X+27*B+7*X^3)` and `V` is the explicit degree-eight
   polynomial recorded in the Lean witness.  Thus `d != 0` and `2 != 0`
   make `p` and `q` coprime, hence `p` and `q^2` coprime.
2. **Theorems used as stated.** `WeierstrassCurve.Phi_three`, `PsiSq_three`,
   `Psi3`, `prePsi4`, and `Psi2Sq` specialize Mathlib's exact recurrence to
   `p` and `q^2`.  `IsCoprime.pow_right` transfers the constructive witness to
   the squared denominator.  The reduced-quotient theorem and exact degree
   formulas then give coordinate extension degree nine when `3 != 0`.
3. **Object as an instance.** The declarations separate the explicit
   `p,q` Bezout law, the Mathlib division-polynomial endpoint, and the exact
   rational-function extension degree.  No root enumeration, point counting,
   or finite-field decision procedure enters the proof.
4. **Assembly probe.** The full degree-eight witness, every Mathlib
   specialization and the singular control first elaborated in a temporary
   importing module at default limits.  The tracked project then completed all
   3,162 build jobs; the exact repository auditor reports
   `150 theorems, 0 failing, 0 trusted`.
5. **Missing components only.** This extends no-cancellation from scalar two
   to scalar three, not to arbitrary scalar.  The degree-nine assembly assumes
   `3 != 0`; characteristic-three inseparability is intentionally outside that
   statement.  Scheme-map identification and the H116 pullback tower remain
   separate. [pcc:unverified]
6. **Inheritance table.** Polynomial rings, Mathlib division polynomials,
   coprimality under powers, and rational-function degree are inherited.
   H116 supplies the explicit extended-Euclid witness and specialization.  At
   `A=B=0`, both displayed scalar-three polynomials vanish at `X=0`; a mapped
   evaluation contradiction is the negative control.

| Statement | Kind | Owner | Quantified structure | Hypotheses |
| --- | --- | --- | --- | --- |
| explicit scalar-three `p,q` Bezout certificate | constructive instance law | H116 over polynomial ring laws | arbitrary field and `A,B` | `4*A^3+27*B^2 != 0`, `2 != 0` |
| `IsCoprime (W.Phi 3) (W.PsiSq 3)` | exact Mathlib endpoint | H116 over Mathlib division polynomials | short Weierstrass curve over arbitrary field | same hypotheses |
| `[F(X):F(Phi_3/Psi_3^2)]=9` | exact coordinate-degree assembly | H116 over reduced quotients | rational function field of the short curve's x-coordinate | same hypotheses and `3 != 0` |
| singular scalar-three pair is not coprime | negative control | H116 evaluation homomorphism | short singular cubic over `ZMod 5` | `A=B=0` |

## Generic division-polynomial coprimality reduction gate

1. **Structure above.** In any commutative ring,
   `IsCoprime (x*m-a*b*c) m` is equivalent to
   `IsCoprime (a*b*c) m`, because the `x*m` term is a multiple of the
   denominator.  Mathlib defines
   `Phi_n=X*PsiSq_n-prePsi_(n+1)*prePsi_(n-1)*epsilon_n`, with
   `epsilon_n=1` for even `n` and `Psi2Sq` for odd `n`.
2. **Theorems used as stated.** `IsCoprime.mul_sub_right_left_iff` removes the
   multiple of the denominator.  `IsCoprime.mul_left` combines three factor
   certificates.  `WeierstrassCurve.Phi` supplies the exact Mathlib
   definition; no copied recurrence is introduced.
3. **Object as an instance.** One generic ring theorem assembles three factor
   certificates.  Its division-polynomial wrapper reduces generic
   `IsCoprime (Phi n) (PsiSq n)` to coprimality of `prePsi(n+1)`,
   `prePsi(n-1)`, and the parity factor against the full denominator.
4. **Assembly probe.** The generic ring theorem, exact
   `WeierstrassCurve.Phi` wrapper and missing-factor mutation first elaborated
   in a temporary importing module.  The tracked project then completed all
   3,162 build jobs; the exact repository auditor reports
   `153 theorems, 0 failing, 0 trusted`.
5. **Missing components only.** This does not prove the three factor premises.
   It converts the previous opaque generic-`n` no-cancellation gap into exact
   adjacent-EDS and parity-factor obligations.  Proving those obligations for
   nonsingular curves under characteristic hypotheses remains the next
   mathematical task. [pcc:unverified]
6. **Inheritance table.** All algebraic coprimality transformations and the
   division-polynomial definition are inherited from Mathlib.  H116 supplies
   only their exact composition and an integer mutation showing that omitting
   one factor premise is unsound.

| Statement | Kind | Owner | Quantified structure | Hypotheses |
| --- | --- | --- | --- | --- |
| `IsCoprime (x*m-a*b*c) m` from three factor certificates | generic ring law | H116 over Mathlib coprimality | arbitrary commutative ring | each of `a,b,c` coprime to `m` |
| `IsCoprime (W.Phi n) (W.PsiSq n)` | exact division-polynomial reduction | H116 over Mathlib `Phi` | arbitrary Weierstrass curve over a commutative ring | adjacent and parity factors coprime to `PsiSq n` |
| omitted first factor premise can fail | negative control | H116 integer arithmetic | fixed `x=0,m=2,a=2,b=c=1` | only the retained factors are coprime |

## Odd-EDS parity-factor coprimality gate

1. **Structure above.** For `preNormEDS' (s^2) c d`, the odd recurrence
   modulo `s` retains only products of smaller odd-index terms: when the
   recurrence index is even, the first summand contains `s^2`; when it is
   odd, the second does. Strong induction therefore proves every odd term
   coprime to `s` from the single base certificate `IsCoprime c s`.  For a
   short Weierstrass curve, `s=Psi2Sq` and `c=Psi3`.
2. **Theorems used as stated.** Mathlib's `preNormEDS'_odd` supplies the exact
   recurrence. `IsCoprime.mul_left`, `pow_left`,
   `mul_sub_right_left_iff`, and `sub_mul_right_left_iff` propagate the
   induction.  A direct H116 Bezout identity proves the short-curve base case:
   with `d=4*A^3+27*B^2`, `s=4(X^3+AX+B)`, and
   `q=3X^4+6AX^2+12BX-A^2`,
   `3(7AX+9B+3X^3)s-4(4A+3X^2)q=4d`.
3. **Object as an instance.** The generic recurrence theorem is instantiated
   with Mathlib's exact `prePsi'`.  For every natural scalar on a nonsingular
   short curve outside characteristic two, it proves the parity factor
   coprime to `PsiSq n`.  Composing with the earlier generic reduction leaves
   only the two adjacent `prePsi` certificates.
4. **Assembly probe.** The recurrence induction, explicit base certificate,
   Mathlib specialization, parity wrapper, reduced two-premise assembly and
   singular control first elaborated in a temporary importing module.  The
   tracked project then completed all 3,162 build jobs; the exact repository
   auditor reports `170 theorems, 0 failing, 0 trusted`.
5. **Missing components only.** The parity-factor premise is closed for
   natural scalars on short curves.  Coprimality of `prePsi(n-1)` and
   `prePsi(n+1)` against `PsiSq n` remains open; this is now the only generic
   no-cancellation obligation in this specialization. [pcc:unverified]
6. **Inheritance table.** The EDS recurrence and coprimality algebra are
   inherited from Mathlib. H116 supplies the strong-induction composition and
   short-curve Bezout certificate.  The singular `A=B=0` curve over `ZMod 5`
   is a negative control: both base polynomials vanish at zero and are proved
   not coprime.

| Statement | Kind | Owner | Quantified structure | Hypotheses |
| --- | --- | --- | --- | --- |
| every odd `preNormEDS' (s^2) c d` term is coprime to `s` | generic recurrence law | H116 over Mathlib normalized EDS | arbitrary commutative ring | `IsCoprime c s` |
| short-curve `Psi3` is coprime to `Psi2Sq` | constructive instance law | H116 polynomial Bezout certificate | short Weierstrass curve over a field | `4*A^3+27*B^2 != 0`, `2 != 0` |
| every odd `prePsi' n` is coprime to `Psi2Sq` | exact division-polynomial specialization | H116 over Mathlib `prePsi'` | same short curve | odd natural `n` and base hypotheses |
| parity factor is coprime to `PsiSq n` | exact factor closure | H116 | same short curve | natural `n` and base hypotheses |
| `Phi n` coprimality reduces to two adjacent factors | semantic assembly | H116 over generic reduction | same short curve | the two adjacent `prePsi` certificates |
| singular base pair is not coprime | negative control | H116 evaluation homomorphism | `A=B=0` over `ZMod 5` | none |

## Scalar-four recurrence bootstrap gate

1. **Structure above.** The audited scalar-three no-cancellation theorem gives
   `IsCoprime Phi3 (Psi3^2)`. Since
   `Phi3=X*Psi3^2-prePsi4*Psi2Sq`, removing the denominator multiple and then
   a product factor proves `Psi3` coprime to `prePsi4`.  Mathlib's next odd
   recurrence is `prePsi5=prePsi4*Psi2Sq^2-Psi3^3`; removing the
   `prePsi4` multiple proves the next adjacent pair.  The parity theorem then
   supplies the remaining `Psi2Sq` factors in `PsiSq4`.
2. **Theorems used as stated.** `WeierstrassCurve.Phi_three`,
   `PsiSq_three`, `prePsi'_odd`, and `PsiSq_four` are used exactly as stated.
   `IsCoprime.of_mul_right_left`, `of_mul_left_left`, `pow_left`,
   `pow_right`, `mul_right`, and the subtract-multiple equivalence perform the
   algebraic extraction. The preceding two-adjacent-factor theorem assembles
   `Phi4,PsiSq4` coprimality; the reduced rational quotient and exact
   division-polynomial degree theorems give extension degree sixteen.
3. **Object as an instance.** On every nonsingular short curve outside
   characteristic two, the construction proves both adjacent pairs, exact
   scalar-four numerator/denominator coprimality, and
   `[F(X):F(Phi4/PsiSq4)]=16`.
4. **Assembly probe.** Both adjacent-pair derivations, scalar-four assembly,
   degree endpoint and singular control first elaborated in a temporary
   importing module. The tracked project then completed all 3,162 build jobs;
   the exact repository auditor reports
   `175 theorems, 0 failing, 0 trusted`.
5. **Missing components only.** This advances the exact no-cancellation
   endpoint from scalars two and three to scalar four by recurrence. It does
   not yet prove adjacent EDS coprimality for arbitrary `n` or an ECDLP
   runtime bound. [pcc:unverified]
6. **Inheritance table.** All recurrence identities, division-polynomial
   formulas, degree results and coprimality transformations are inherited from
   Mathlib or earlier audited H116 theorems.  H116 supplies their bootstrap
   composition.  The singular `A=B=0` scalar-four pair over `ZMod 5` is proved
   non-coprime by evaluation at zero.

| Statement | Kind | Owner | Quantified structure | Hypotheses |
| --- | --- | --- | --- | --- |
| `Psi3` is coprime to `prePsi4` | recurrence extraction | H116 over scalar-three certificate | short curve over a field | discriminant nonzero, `2 != 0` |
| `prePsi4` is coprime to `prePsi5` | recurrence bootstrap | H116 over Mathlib odd EDS recurrence | same | same |
| `Phi4` is coprime to `PsiSq4` | exact division-polynomial assembly | H116 | same | same |
| scalar-four coordinate extension degree is sixteen | exact degree assembly | H116 | rational function field | same |
| singular scalar-four pair is not coprime | negative control | H116 evaluation homomorphism | `A=B=0` over `ZMod 5` | none |

## Scalar-five recurrence bootstrap gate

1. **Structure above.** The scalar-five step first reduces
   `prePsi5=prePsi4*Psi2Sq^2-Psi3^3` modulo `Psi3`; the scalar-four adjacent
   pair and the odd-parity base certificate prove `Psi3` coprime to
   `prePsi5`. The even recurrence then gives
   `prePsi6=Psi3*(prePsi5-prePsi4^2)`. Both surviving factors are coprime to
   `prePsi5`, so the next adjacent pair follows. Since `PsiSq5=prePsi5^2`,
   the two adjacent factors assemble scalar-five no-cancellation directly.
2. **Theorems used as stated.** Mathlib's `prePsi'_odd`, `prePsi'_even`, and
   `PsiSq_ofNat` supply the exact formulas. The audited scalar-four adjacent
   pairs and parity theorem provide the base certificates. Coprimality product,
   power, symmetry and subtract-multiple laws perform the reductions. The
   generic two-adjacent-factor theorem assembles `Phi5,PsiSq5`; the reduced
   quotient and exact-degree theorem give degree twenty-five when `5 != 0`.
3. **Object as an instance.** On every nonsingular short curve outside
   characteristic two, scalar-five numerator and denominator are coprime.
   Outside characteristic five as well, the coordinate extension has degree
   twenty-five. This explicitly separates cancellation from inseparable
   degree collapse.
4. **Assembly probe.** Both auxiliary pair derivations, scalar-five assembly,
   degree endpoint and singular control first elaborated in a temporary
   importing module. The tracked project then completed all 3,162 build jobs;
   the exact repository auditor reports
   `180 theorems, 0 failing, 0 trusted`.
5. **Missing components only.** Exact no-cancellation now holds through scalar
   five. The proof remains a fixed-depth recurrence bootstrap; an invariant
   closing arbitrary adjacent indices and any ECDLP runtime conclusion remain
   open. [pcc:unverified]
6. **Inheritance table.** EDS identities, coprimality transformations and
   degree formulas are inherited. H116 supplies their new composition. The
   singular `A=B=0` scalar-five pair over `ZMod 7` is proved non-coprime by
   evaluation at zero.

| Statement | Kind | Owner | Quantified structure | Hypotheses |
| --- | --- | --- | --- | --- |
| `Psi3` is coprime to `prePsi5` | recurrence extraction | H116 | short curve over a field | discriminant nonzero, `2 != 0` |
| `prePsi5` is coprime to `prePsi6` | recurrence bootstrap | H116 | same | same |
| `Phi5` is coprime to `PsiSq5` | exact division-polynomial assembly | H116 | same | same |
| scalar-five coordinate extension degree is twenty-five | exact degree assembly | H116 | rational function field | same plus `5 != 0` |
| singular scalar-five pair is not coprime | negative control | H116 evaluation homomorphism | `A=B=0` over `ZMod 7` | none |

## Scalar-six two-factor bootstrap gate

1. **Structure above.** Set `t=prePsi5-prePsi4^2`. The even recurrence gives
   `prePsi6=Psi3*t`.  The next odd recurrence initially gives
   `prePsi7=prePsi5*Psi3^3-prePsi4^3*Psi2Sq^2`; using the scalar-five
   identity, H116 rewrites this exactly as
   `(Psi3^3-prePsi4^2)*t-prePsi4^4`.  Thus coprimality with `prePsi6` splits
   into coprimality with `Psi3` and with `t`, both reducible to earlier pair
   and parity certificates.
2. **Theorems used as stated.** Mathlib's `prePsi'_even` and `prePsi'_odd`
   supply the two recurrences. The scalar-four/five adjacent pairs and
   `Psi3,Psi2Sq` base certificate discharge the factor goals through
   `IsCoprime.mul_left_iff`, power laws and subtract-multiple equivalences.
   The two-adjacent-factor assembly proves scalar-six no-cancellation; the
   exact quotient-degree theorem gives degree thirty-six when `6 != 0`.
3. **Object as an instance.** On every nonsingular short curve outside
   characteristic two, `Phi6` and `PsiSq6` are coprime. Outside characteristic
   three as well, the coordinate extension degree is thirty-six.
4. **Assembly probe.** The two-factor adjacent proof, scalar-six assembly,
   degree endpoint and singular control first elaborated in a temporary
   importing module. The tracked project then completed all 3,162 build jobs;
   the exact repository auditor reports
   `184 theorems, 0 failing, 0 trusted`.
5. **Missing components only.** This reaches scalar six and exposes a
   two-factor recurrence invariant candidate. It does not yet package that
   invariant for arbitrary indices or imply an ECDLP runtime bound.
   [pcc:unverified]
6. **Inheritance table.** EDS identities, coprimality laws and degree formulas
   are inherited. H116 supplies the exact alternate factorization and
   composition. The singular `A=B=0` scalar-six pair over `ZMod 7` is proved
   non-coprime by evaluation at zero.

| Statement | Kind | Owner | Quantified structure | Hypotheses |
| --- | --- | --- | --- | --- |
| `prePsi6` is coprime to `prePsi7` | two-factor recurrence bootstrap | H116 | short curve over a field | discriminant nonzero, `2 != 0` |
| `Phi6` is coprime to `PsiSq6` | exact division-polynomial assembly | H116 | same | same |
| scalar-six coordinate extension degree is thirty-six | exact degree assembly | H116 | rational function field | same plus `3 != 0` |
| singular scalar-six pair is not coprime | negative control | H116 evaluation homomorphism | `A=B=0` over `ZMod 7` | none |

## Adjacent-torsion root-semantics gate

1. **Structure above.** Over an algebraically closed field, two polynomials
   are coprime if they have no common root: a nonunit gcd would
   have nonzero degree, hence a root, and that root propagates through both gcd
   divisibilities.  In any additive group, simultaneous equations
   `[n]P=0` and `[n+1]P=0` force `P=0` by subtraction.
2. **Theorems used as stated.** `gcd_isUnit_iff`,
   `Polynomial.isUnit_iff_degree_eq_zero`, `IsAlgClosed.exists_root`,
   `Polynomial.IsRoot.dvd`, and integer scalar-addition laws supply the two
   generic arguments.  No elliptic group law is copied.
3. **Object as an instance.** A semantic wrapper accepts a certificate that
   every common polynomial root produces a nonzero group element killed by
   adjacent integers.  The group theorem makes that certificate contradictory,
   and the root criterion returns polynomial coprimality.
4. **Assembly probe.** The root criterion, adjacent-scalar group law, semantic
   wrapper and zero-point control first elaborated in a temporary module.  The
   tracked project then completed all 3,162 build jobs; the exact repository
   auditor reports `157 theorems, 0 failing, 0 trusted`.
5. **Missing components only.** The wrapper does not yet construct an elliptic
   point from a root of `prePsi n`, prove that it is nonzero, or prove that the
   two roots encode `[n]P=[n+1]P=0`.  Those exact division-polynomial root
   semantics are now the remaining geometric bridge. [pcc:unverified]
6. **Inheritance table.** Algebraic-closure roots, polynomial gcds and integer
   scalar laws are inherited from Mathlib.  H116 composes them and explicitly
   exposes the root-to-torsion premise.  The zero group element satisfies both
   adjacent equations and is the boundary control.

| Statement | Kind | Owner | Quantified structure | Hypotheses |
| --- | --- | --- | --- | --- |
| no common root implies `IsCoprime p q` | generic algebraic law | H116 over Mathlib gcd/root APIs | algebraically closed field | no shared root |
| adjacent integer torsion intersects trivially | generic group law | H116 over integer scalar addition | arbitrary additive group | `[n]P=0`, `[n+1]P=0` |
| root-to-adjacent-torsion semantics implies coprimality | semantic assembly | H116 over the preceding laws | algebraically closed field plus additive group | each common root supplies a nonzero adjacent-torsion point |
| zero point satisfies both equations | boundary control | H116 group simplification | arbitrary additive group | `P=0` |

## Algebraic-closure descent for root semantics gate

1. **Structure above.** Let `f : F ->+* K` be a homomorphism from an
   arbitrary base field into an algebraically closed field.  If the mapped
   polynomials `p.map f` and `q.map f` have no common root in `K`, they are
   coprime in `K[X]`; field-map invariance then descends coprimality to
   `F[X]`.  The same descent applies after common mapped roots are converted
   to nonzero points killed by adjacent integer scalars.
2. **Theorems used as stated.** The preceding H116 algebraically closed root
   criterion and adjacent-torsion contradiction are composed with Mathlib's
   `Polynomial.isCoprime_map`, whose reverse implication is the required
   descent.  Ring homomorphisms between fields are injective automatically;
   no chosen algebraic closure or coefficient-level inverse is introduced.
3. **Object as an instance.** One wrapper accepts no-common-root evidence for
   `p.map f,q.map f`; a second accepts the mapped-root-to-adjacent-torsion
   semantic certificate.  Both conclude `IsCoprime p q` over the original
   coefficient field.
4. **Assembly probe.** Both wrappers first elaborated in a temporary module.
   The tracked project then completed all 3,162 build jobs; the exact
   repository auditor reports `159 theorems, 0 failing, 0 trusted`.
5. **Missing components only.** This closes coefficient-field descent but
   does not construct an algebraic closure, lift a division-polynomial root to
   an elliptic point, or prove the pointwise scalar-multiplication formula.
   Those geometric obligations remain explicit. [pcc:unverified]
6. **Inheritance table.** Polynomial mapping and coprimality invariance are
   inherited from Mathlib. H116 supplies only the composition with the already
   audited root and group-semantic theorems.

| Statement | Kind | Owner | Quantified structure | Hypotheses |
| --- | --- | --- | --- | --- |
| mapped no-common-root evidence descends to `IsCoprime p q` | generic field-extension law | H116 over Mathlib `Polynomial.isCoprime_map` | base field, algebraically closed extension field, field homomorphism | no shared mapped root |
| mapped root-to-adjacent-torsion semantics descends to `IsCoprime p q` | semantic assembly | H116 over the preceding descent and group law | same fields plus additive commutative group | every shared mapped root supplies a nonzero adjacent-torsion point |

## Algebraic-closure affine-point lift gate

1. **Structure above.** For a Weierstrass equation over an algebraically
   closed field and any chosen `x`, the curve equation is a monic quadratic in
   `y`.  Its degree-two coefficient is one, so it is nonconstant and has a
   root.  On an elliptic affine curve the resulting equation witness builds a
   nonzero `Affine.Point`.  Combining this with the preceding base-change
   theorem removes point construction from the adjacent-torsion premise.
2. **Theorems used as stated.** `IsAlgClosed.exists_root` supplies a root of
   `Y^2+(a1*x+a3)Y-(x^3+a2*x^2+a4*x+a6)`;
   `Affine.equation_iff`, `Affine.Point.mk`, and `Point.some_ne_zero` turn it
   into the exact Mathlib point object.  The previous mapped-root semantic
   bridge supplies the coprimality conclusion.
3. **Object as an instance.** The first theorem returns an equation witness
   over any algebraically closed field.  The second returns the associated
   nonzero elliptic point.  The assembly theorem accepts only the remaining
   pointwise claim that a common mapped root makes this point satisfy the two
   adjacent scalar equations.
4. **Assembly probe.** All three positive statements and the finite-field
   control first elaborated in a temporary importing module.  The tracked
   project then completed all 3,162 build jobs; the exact repository auditor
   reports `163 theorems, 0 failing, 0 trusted`.
5. **Missing components only.** The root-to-point construction is closed.
   The assembly still does not prove that a root of `prePsi_n` is killed by
   scalar `n`; that pointwise division-polynomial formula is the remaining
   geometric obligation. [pcc:unverified]
6. **Inheritance table.** Algebraic-closure root existence, the Weierstrass
   equation and affine point/group structures are inherited from Mathlib.
   H116 supplies the quadratic specialization and composition.  The control
   curve `y^2=x^3+x+1` over `ZMod 5` has no point over `x=1`, verified by
   exhaustive kernel reduction, showing that algebraic closure cannot be
   deleted from the lift.

| Statement | Kind | Owner | Quantified structure | Hypotheses |
| --- | --- | --- | --- | --- |
| every `x` lies below a Weierstrass equation point | generic algebraic-closure law | H116 over `IsAlgClosed.exists_root` | arbitrary affine Weierstrass curve over algebraically closed field | none beyond field closure |
| every `x` lies below a nonzero elliptic affine point | point construction | H116 over Mathlib `Affine.Point.mk` | elliptic affine curve over algebraically closed field | decidable equality for the inherited group instance |
| pointwise adjacent scalar semantics implies base-field coprimality | semantic assembly | H116 over the point lift and mapped-root descent | base field, closed extension, elliptic affine curve | common-root point is killed by `n` and `n+1` |
| point lift can fail over the base finite field | negative control | H116 finite reduction | `y^2=x^3+x+1` over `ZMod 5`, `x=1` | none |

## Short-Weierstrass local-parameter instance gate

1. **Structure above.** The local affine parameter is the field expression
   `z(x,y)=-x/y`. Coordinate negation is `(x,y) -> (x,-y)`, and a coordinate
   endomorphism is represented by a field ring homomorphism applied to both
   coordinates. This gate does not construct the elliptic formal group.
2. **Theorems used as stated.** Field `div_neg` proves
   `z(x,-y)=-z(x,y)`. Ring-homomorphism preservation of negation and division
   proves `z(phi(x),phi(y))=phi(z(x,y))`; a supplied power-map identity for
   `phi` then gives the exact Frobenius parameter formula.
3. **Object as an instance.** The implementation expression is exactly
   `shortWeierstrassLocalParameter x y`. The negation and map theorems quantify
   over arbitrary fields; the Frobenius wrapper quantifies over a ring
   endomorphism `phi` and the explicit premise `phi(a)=a^p`.
4. **Assembly probe.** A temporary module importing `H116FiberTrace` checked
   the definition and all three wrappers at default limits. The tracked module
   then rebuilt at default limits; the exact repository auditor reported
   `93 theorems, 0 failing, 0 trusted`.
5. **Missing components only.** This proves the coordinate-level negation and
   Frobenius formulas. It does not prove that the supplied rational map is
   Verschiebung, construct its local formal-group law, or identify the
   endomorphism relation `V+pi=[t]`.
6. **Inheritance table.** Division, negation, powers and ring-homomorphism laws
   are inherited from Mathlib field/ring instances. H116 adds only the named
   local-parameter definition and compositions. No curve equation, copied
   addition law, complexity claim, trusted decision procedure or resource
   override is introduced. [pcc:unverified]

| Statement | Kind | Owner | Quantified structure | Hypotheses |
| --- | --- | --- | --- | --- |
| `z(x,-y)=-z(x,y)` | coordinate identity | H116 over field laws | arbitrary field | none |
| `z(phi(x),phi(y))=phi(z(x,y))` | coordinate naturality | H116 over ring-hom laws | field ring homomorphism | none |
| coordinate Frobenius sends `z` to `z^p` | instance wrapper | H116 | field endomorphism | supplied identity `phi(a)=a^p` |

## Mathlib formal-group filtration instance gate

1. **Structure above.** The quantified object is Mathlib's
   `FormalGroup R` from `Mathlib.RingTheory.FormalGroup.Basic`, with underlying
   bivariate series `F.toPowerSeries`, identity theorem `F.Xzero_eq_X`, and
   substitution through `MvPowerSeries.HasSubst`. The coordinate endpoint uses
   the pre-existing H116 `PowerSeriesPrefixEq` and normalized pole bridge.
2. **Theorems used as stated.** `MvPowerSeries.X_dvd_iff` converts
   second-variable divisibility into coefficient vanishing;
   `FormalGroup.Xzero_eq_X` identifies the restriction `F(X,0)`;
   `MvPowerSeries.subst_sub`, `subst_mul`, and `subst_X` transport the quotient
   through substitution. `PowerSeriesPrefixEq.add_X_pow_mul` and
   `leading_X_inv_sq_mul_subst_inv` supply the filtration and pole transfer.
   For a commutative formal group, `FormalGroup.comm'` turns a certified right
   inverse into the orientation required by the same correction factorization.
   In the short-Weierstrass specialization, ordinary ring negation gives the
   zero constant and order factorization of `-omitted` directly.
3. **Object as an instance.** A zero-constant scalar parameter `s` and
   correction `c` are substituted into the actual Mathlib formal group as
   `F(s,c)`. If `c=X^m tail`, the resulting series is prefix-equal to `s`
   below degree `m`; after writing both parameters as `X` times units, an
   order-`n+1` correction yields an order-`n` normalized pole prefix. For the
   actual endomorphism orientation, the cancellation bridge takes explicit
   certificates `F(actual,omitted)=scalar` and
   `F(omitted,inverseCorrection)=0`; associativity and the right identity give
   `F(scalar,inverseCorrection)=actual`, after which the same filtered pole
   theorem applies. Commutativity plus `omitted=X^m tail` then proves, rather
   than assumes, `inverseCorrection=X^m inverseTail`. An alternative endpoint
   uses the explicit correction `-omitted`; its only inverse premise is the
   exact certificate `F(omitted,-omitted)=0`.
4. **Assembly probe.** A temporary module importing `H116FiberTrace` and
   Mathlib's formal-group module checked all four declarations at default
   heartbeats. A first direct curve-coordinate wrapper timed out during
   elaboration; replacing it by the generic pole theorem made every component
   explicit and passed without a limit override. A second temporary module
   checked both the associativity-only right-cancellation lemma and its composed
   pole-prefix endpoint at default limits. A third temporary module checked the
   inverse-order transfer and the endpoint specialized to omitted-parameter
   order at default limits. The tracked module then built at default limits and
   the exact repository auditor reported `88 theorems, 0 failing, 0 trusted`.
   A fourth temporary module checked the explicit-negation endpoint at default
   limits; after integration the tracked module rebuilt at default limits and
   the exact auditor reported `89 theorems, 0 failing, 0 trusted`.
5. **Missing components only.** The generic laws no longer assume the desired
   prefix or the desired subtraction equality. The remaining endpoint
   hypotheses are the actual elliptic identification `V+pi=[t]` in the chosen
   formal parameter, the elliptic negation certificate
   `F(piParameter,-piParameter)=0`, the characteristic-`p` formula giving that
   parameter order `p`, and the sign/target scaling that turns
   it into the supplied `V-1` rational map. Parity then translates the
   `z^(p-1)` normalized boundary to `w^((p-1)/2)`; that translation is not
   asserted by the generic theorem.
6. **Inheritance table.** The formal group, identity restriction and
   substitution algebra are inherited from Mathlib; finite prefix arithmetic
   and the double-pole loss are inherited from earlier H116 theorems. The
   filtration lemma extracts the second-variable quotient from `F(X,0)=X`.
   The cancellation bridge uses only `FormalGroup.assoc'`, `add_zero`, and
   zero-constant substitution witnesses; it does not construct an inverse or
   import elliptic semantics. The inverse-order transfer additionally inherits
   `FormalGroup.comm'` and ring factorization; it does not assume the inverse's
   order. The explicit-negation endpoint uses only ring negation and avoids the
   commutativity premise, but leaves the actual elliptic negation identity as a
   visible certificate. No curve equation, endomorphism relation,
   complexity bound, trusted decision procedure or resource-limit override is
   introduced.
   [pcc:unverified]

| Statement | Kind | Owner | Quantified structure | Hypotheses |
| --- | --- | --- | --- | --- |
| `Y` divides `F(X,Y)-X` | generic law, upstream candidate | H116 over Mathlib `FormalGroup` | commutative ring formal group | Mathlib formal-group laws only |
| `F(s,c)=s+cq` for some `q` | generic assembly | H116 | commutative ring formal group and univariate power series | zero constant coefficients |
| order-`m` correction preserves the prefix below `m` | generic assembly | H116 | field formal group | `c=X^m tail` and zero constants |
| order-`n+1` correction preserves normalized pole prefix below `n` | generic assembly | H116 | field formal group and regular outer unit | leading-`X` factorizations and unit constants |
| `F(actual,omitted)=scalar` cancels against an explicit right inverse | generic assembly | H116 over Mathlib `FormalGroup` | commutative ring formal group | zero constants and `F(omitted,inverseCorrection)=0` |
| cancellation plus order-`n+1` inverse preserves normalized pole prefix below `n` | generic endpoint composition | H116 | field formal group and regular outer unit | certified sum, certified inverse, inverse order, leading-`X` factorizations and unit constants |
| a certified inverse has at least the omitted parameter's order | generic commutative-group assembly | H116 over Mathlib `FormalGroup` | commutative formal group over a commutative ring | zero constants, certified right inverse, omitted-parameter factorization |
| cancellation plus order-`n+1` omitted parameter preserves normalized pole prefix below `n` | generic endpoint composition | H116 | commutative field formal group and regular outer unit | certified sum and inverse, omitted order, leading-`X` factorizations and unit constants |
| explicit negation correction preserves normalized pole prefix below `n` | generic endpoint composition | H116 | field formal group and regular outer unit | certified sum, `F(omitted,-omitted)=0`, omitted order, leading-`X` factorizations and unit constants |
| actual `V-1` normalized x-ratio prefix | endpoint obligation | future elliptic formal-group owner | ordinary elliptic curve over `F_p` | `V+pi=[t]`, local Frobenius, sign and scaling identifications |

## Inheritance table

| Operation/law | Inheritance |
| --- | --- |
| polynomial multiplication, derivative, evaluation | Mathlib instances plus `Polynomial.derivative'` |
| finite product derivative | `Polynomial.derivative_prod_finset`, used as stated |
| finite sums and coefficient extraction | Mathlib finite sums plus existing H116 power-series reciprocal/convolution, concrete `F_U` coefficient lemmas and recurrence uniqueness through degree eight; reciprocal-at-infinity isogeny substitution remains open |
| field division/cancellation | `Field K` laws; `bostanDenominatorsNonzero` proves the concrete native divisions are defined for prime `p>19` |
| isogeny homomorphism, dual and degree laws | finite field-tower multiplication and exact division-polynomial coordinate degrees are now proved; no Mathlib elliptic-isogeny or dual object was found, so constructing the actual pullback tower, identifying the scalar coordinate quotient with the scheme map and its degree, proving `(V-1)^vee=pi-1`, and the differential formula remain open |
| formal-group filtration of an `X^m` correction | Mathlib `FormalGroup` plus `FormalGroup.filtered_correction_prefix` and `filtered_pole_prefix`; generic layer proved |
| `V+pi=[t]`, `F(pi,-pi)=0` in parameter coordinates, and the actual local `pi` series of order `p` | elliptic formal-group instance; endpoint identification still pending |
| radix-six decoder | reuse H115 generic theorem after source import/binding |

The assembly lookup found no ready-made `Derivation K (RatFunc K) (RatFunc K)`
instance. The proof therefore states the exact algebraic laws over an arbitrary
field equipped with a derivation, which is stronger at the identity layer, and
keeps the rational-function/formal-group instantiation explicit. It does not
claim that the missing Frobenius jet constructor exists or is fast.

No `maxHeartbeats`, `maxRecDepth`, `native_decide`, copied curve group law or
per-curve theorem is permitted.
