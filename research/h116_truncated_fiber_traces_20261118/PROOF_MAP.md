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
| isogeny homomorphism and differential coordinate formula | hypothesis at study endpoint; no Mathlib isogeny object was found, so a finite `N,D` certificate is the checked interface; producing it from the actual kernel and independently validating map semantics remain open |
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
