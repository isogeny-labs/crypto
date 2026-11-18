# H116 truncated Frobenius-fiber traces

H116 tests a component needed by H115: whether the fixed low-order moments of
the degree-`r` fiber of `phi = V - 1` can be recovered from a short Laurent jet
of the `x`-coordinate map, without constructing or enumerating that fiber.

The proposed identities are frozen as hypotheses until proved and independently
checked. In particular, an efficient moment evaluator is not by itself an ECDLP
algorithm. The separate digit-predicate and decoder obligations remain open.
[pcc:unverified]

## Requested scope and evidence plan

| Requirement | Frozen evidence |
| --- | --- |
| A different non-generic lead | Relative fiber traces/residues of the special endomorphism `V-1`, not a generic walk or relation collection |
| Rigorous algorithm | Native candidate with exact field-operation counters and a direct degree-`r` enumeration reference |
| Controls | identical A/A reference, planted polynomial positive control, characteristic-bound rejection control, coordinate isomorphisms and shuffled target order |
| Scaling | five deterministically selected public toy curves with `p > 19`, all nonidentity targets and three generator labels |
| Independent check | separate cryptanalysis verifier source and tamper tests before scoring |
| Formal proof | Lean identity layer for logarithmic derivatives/Newton sums and residue extraction, with the Frobenius-jet and cost premises kept explicit if their library instances are unavailable |
| Subexponential claim | admissible only if the moment construction, one fixed exact digit predicate, the complete decoder and every setup/recovery cost are all bounded by `r^o(1)` |

## Frozen algebraic target

Let a separable odd-degree isogeny have coordinate map

```text
phi(x,y) = (R(x), y R'(x) / a),   R = N/D,
F_u = N - uD.
```

For a non-2-torsion target `Q=(u,v)`, H116 tests the following coefficient
identities, with fiber points counted with their algebraic multiplicities:

```text
X_j(Q)-X_j(Q0)
  = [x^(-j-1)] d/dx log((R-u)/(R-u0)),

Y_j(Q)
  = a v [x^(-1)] x^(j-1)/(R-u).
```

For `phi=V-1` on `E/F_p`, the formal endomorphism identity

```text
V - 1 = [t-1] - pi,    pi(z)=z^p
```

predicts that the required fixed-order jet is computable from a truncated
formal-group calculation. For the first implementation, the Bostan et al.
normalized-isogeny recurrence is used only when its denominators are units;
the frozen cohort requires `p > 2J+3` with `J=8`. The exact general small-
characteristic formal-group construction is an open successor, not silently
substituted into this run. [pcc:unverified]

The implemented normalization and exact truncated formulas are recorded in
`ALGORITHM.md`. In particular, the recurrence target x-coordinate is
`U=(t-1)^2u`, not raw `u`; this scaling is required to make the isogeny
normalized before applying the Bostan recurrence.

The Lean layer now also defines the exact formal series
`F_U(w)=1-Uw+h₁w²+h₂w³+...` and proves its coefficients, derivative, unit-
constant reciprocal recurrence and finite derivative-convolution used by the
native eight-coefficient loop. The `w=1/x` isogeny substitution and the
Bostan/`V-1` jet constructor remain explicit open obligations.
[pcc:unverified]

For prime `p>19`, the Lean theorem `bostanDenominatorsNonzero` proves that all
eleven denominators used to construct `h₁,...,h₇` are nonzero in
`ZMod p`. This discharges the characteristic-range side condition for the
frozen prime-field cohort; it does not prove recurrence (16) itself.
[pcc:unverified]

The exact finite recurrence is now encoded as `BostanJetRecurrence`. Lean
proves that its seven output coefficients are unique and that any two solutions
induce identical `F_U` coefficients through degree eight. The remaining bridge
is no longer ambiguity about the native formula: it is the geometric statement
that the actual normalized isogeny Laurent jet satisfies this recurrence.
[pcc:unverified]

The coefficient-extraction part of that bridge is now formalized as well.
`NormalizedIsogenySecondOrderEquation` is equation (15) of the cited Bostan et
al. construction after `R(x)=xU(1/x)`. Lean proves that this PowerSeries
equation plus the two initial coefficients implies the exact seven-term native
recurrence, with every division discharged for prime `p>19`. What remains is
to obtain this formal-series equation from the actual normalized isogeny map.
The preceding paper step is now formalized separately:
`normalizedMapEquation_implies_secondOrder` differentiates the normalized
rational-map curve equation and cancels its explicitly nonzero derivative to
obtain equation (15). The transformed equation (14) is also formalized as
`NormalizedIsogenyFirstOrderEquation`, and Lean proves it implies the existing
second-order PowerSeries equation. Its cancelled factor is automatically
nonzero because its constant coefficient is one. The same first-order identity
forces the two initial coefficients `h₁=(A-Atilde)/5` and
`h₂=(B-Btilde)/7`. Thus, in prime characteristic `p>19`, it now implies the
complete frozen recurrence without extra coefficient hypotheses. What remains
is to construct the actual normalized isogeny at-infinity series and prove that
it satisfies this first-order PowerSeries equation; no theorem here constructs
that map. [pcc:unverified]

The open geometric bridge now has an explicit finite proof-carrying interface.
`NormalizedRationalMapPolynomialCertificate` accepts reversed numerator and
denominator polynomials `N,D`, checks the quotient relation and the single
cleared-denominator curve identity, and Lean proves that certificate implies
the first-order equation and hence the complete recurrence for prime `p>19`.
This moves the remaining obligation to producing and independently checking
the actual rational-map polynomials; it does not treat a supplied polynomial
pair as an isogeny without that identity. [pcc:unverified]

The separate [map-certificate control](MAP_CERTIFICATE_EXPERIMENT.md) exercised
this interface on the excluded H115 `p=31`, `r=37` fixture. Rust reconstructed
the full Vélu map, and an independent Python verifier reproduced the cleared
identity, the coefficient prefix `[1,0,20,0,13,7,2,4,8]` and rejection of a
one-coefficient mutation. This is proof/control evidence only: the degree-`r`
construction is not charged as the fixed-length candidate and no gated H116
target was read. [pcc:unverified]

The follow-on [fixed low-jet experiment](JET_CERTIFICATE_EXPERIMENT.md) removes
the full polynomial degree from the **verification** interface. Lean proves
that `D*U=N mod X^11` plus the cleared curve identity modulo `X^10` forces the
nine coefficients needed by the complete recurrence for prime `p>19`. The
excluded `p=31`, `r=37` replay used 22 numerator/denominator coefficient slots
instead of all 75, and passed in-window, derivative-guard and out-of-window
mutation controls. This does not remove the degree-`r` cost of constructing the
correct low jet or certify its intended kernel semantics; those remain the
active algorithmic obligations. [pcc:unverified]

The next [formal-scalar low-jet experiment](FORMAL_SCALAR_JET_EXPERIMENT.md)
constructs that eleven-coefficient prefix from only `(p,A,B,t)` on the excluded
`p=31,r=37` fixture. It computes the elliptic formal logarithm, solves
`L([t-1](z))=(t-1)L(z)`, applies the normalized target scaling, and re-expands
the x-coordinate ratio in `w=1/x`. Its prefix exactly matches both the retained
full-map quotient through coefficient ten and the recurrence through
coefficient eight; wrong-scalar, omitted-scaling, coordinate-isomorphism and
synthetic Frobenius-boundary controls also pass. An independent verifier
recomputes the comparisons and the first-order curve identity. This supports
one fixed-size excluded-fixture construction, not generic map semantics or an
asymptotic advantage. [pcc:unverified]

Lean now proves the associated precision cutoff: for prime `p>19`, a normalized
monomial correction `X^(p-1)` vanishes below degree 22, so it cannot affect the
eleven even z-degrees corresponding to coefficients `w^0,...,w^10`. The theorem
does not identify the actual elliptic Frobenius term with that monomial. A
subsequent generic proof cancels a shared leading local parameter `X` with one
degree of precision loss and shows that inversion and inverse-square preserve
the remaining unit-series prefix. Thus the double-pole core of the x-coordinate
bridge is checked. A further generic theorem now shows that substitution of
prefix-equal zero-constant parameters into any fixed regular unit series, its
inverse, and its product with the inverse-square pole preserve the same output
prefix. The curve-specific instance now defines that series by
`q=1+A X^4q^2+B X^6q^3`, proves the cleared affine identity, proves rescaling to
`(a^4A,a^6B)`, and matches the complete source/target x-ratio used by the
implementation. The remaining formal-group obligation is the parameter
identity derived from `V+pi=[t]` and local Frobenius. The generic filtration
part of that obligation is now discharged against Mathlib's actual
`FormalGroup`: Lean proves `Y | F(X,Y)-X`, proves that an `X^m` correction is
invisible below degree `m`, and composes it with the existing one-degree pole
loss. The endomorphism orientation is now proved as well: from certified
relations `F(actual,omitted)=scalar` and
`F(omitted,inverseCorrection)=0`, associativity derives
`F(scalar,inverseCorrection)=actual`. In a commutative formal group the inverse
inherits the omitted parameter's `X^m` factor; alternatively, an explicit
short-Weierstrass negation certificate `F(omitted,-omitted)=0` avoids a
separately constructed inverse series. A new possibly noncommutative-ring
theorem derives `V+pi=[t]` from the Frobenius characteristic polynomial,
`pi*V=[p]`, left cancellation by `pi`, and centrality of `[t]`. Binding those
abstract premises to additive endomorphisms now removes two certificates:
integer scalar maps are central automatically, and injective `pi` supplies
left-composition cancellation. A checked `ZMod 2` mutation shows injectivity
is necessary: the two equations hold for zero `pi,V`, cardinality two and trace
one, while the sum conclusion fails. The positive Frobenius instance is now
constructed on Mathlib's actual nonsingular affine points by applying
`Affine.Point.map` to the finite-field Frobenius algebra homomorphism. Mathlib
proves this point map additive and injective; over `ZMod p`, Lean proves its
coordinates are exactly `(x^p,y^p)`. Over any algebraic extension of
`ZMod p`, inverse field Frobenius now induces an inverse point endomorphism.
Defining the pointwise candidate `V=[p]*piInverse`, Lean proves
`pi*piInverse=1`, hence `pi*V=[p]`. The characteristic equation is therefore
the only remaining positive point-group algebraic premise needed to derive
`V+pi=[t]`. Identifying this pointwise candidate with the retained
rational/scheme Verschiebung and transporting the equality to the formal
parameter, negation, sign and target scaling remain open.
An eight-cell exhaustive extension-field panel then checks the remaining
pointwise characteristic equation and the constructed `V` on all 5,066
enumerated points. All positive equations and point-count recurrences pass,
and an independent implementation matches every correctness field. The frozen
protocol nevertheless fails one of 24 mutation requirements: on the 35-point
`p=5`, degree-two group, `[5]pi^-1=[5]`, so omitting `pi^-1` is algebraically
invisible. The failed control is retained in
`POINTWISE_CHARPOLY_EXPERIMENT.md`; it is finite evidence, not the missing
generic theorem. [pcc:unverified]
Lean now also rearranges the point identity into the exact candidate
orientation `V-1=[t-1]-pi`. A total observable on actual affine points maps
infinity to zero and `(x,y)` to `z=-x/y`; the two point expressions are proved
to have the same observed `z` value under the explicit characteristic
premise. This closes pointwise orientation but not identification with the
retained rational/scheme morphism or equality of completed local series.
[pcc:unverified]
For finite point groups, the characteristic premise now has a proof-carrying
compressed form. Lean proves that checking the characteristic expression on a
set whose additive-subgroup closure is top proves the global endomorphism
equation, fixes the trace definition to `p+1-Nat.card E(F_p)`, and composes the
certificate to the pointwise `V-1` orientation. This does not extend a finite
certificate to the algebraic closure. [pcc:unverified]
The frozen generator-certificate experiment instantiates that interface on the
eight parent groups. Three groups are cyclic and five use two generators, so
5,066 exhaustive characteristic checks compress to 13 generator checks. Both
coefficient mutations and every dropped-generator control reject, and an
independent implementation matches all generator coordinates, orders and
closures. Generator discovery still enumerates these small groups; this is a
finite verification compression, not a generic algorithmic speedup.
[pcc:unverified]
Lean now also proves an algebraic-point exhaustion theorem. Both coordinates
of any affine point over an algebraic extension lie in one finite intermediate
field, so some positive power of the actual point Frobenius fixes the point.
Vanishing of the characteristic expression on every positive Frobenius-fixed
locus therefore implies the global point-endomorphism equation and the exact
`V-1` orientation. A `ZMod 3` control proves that checking only one fixed locus
is insufficient. The restricted characteristic identities themselves remain
unproved. [pcc:unverified]
The frozen `H116-FROBENIUS-FIXED-LOCI-001` census passes all eight cells with
exact independent replay. Every one of 5,066 points has a period dividing the
extension degree. The base locus omits 5,002 points, and the union of every
proper-period stratum omits 4,924 full-period points; the correct
characteristic expression has zero failures in all strata.
[pcc:unverified]
Lean now also proves a finite rational-function separation bridge: equality on
a finite pole-free sample implies equality after cross multiplication when
both cross-product degrees are strictly below the sample size. A `ZMod 3`
control proves that strictness cannot be weakened. The frozen
`H116-RATIONAL-SEPARATION-CAPACITY-001` experiment finds 2,534 distinct
affine x-values across the same eight cells, with per-cell raw capacities from
16 to 1,236, and independently reconstructs the nonzero degree-|S| polynomial
that vanishes on every sample. This measures capacity only; the actual H116
map, poles and degree certificate remain open. [pcc:unverified]
The first attempt to instantiate that bridge retained a useful failure: it
mistook `ker(V-1)` for `E(F_p)`. Lean now proves the corrected kernel
eigenrelation `pi(P)=[t-1]P` for every `V`-fixed point and its iterated
fixed-locus form. The frozen v2 joint reconstruction then recovers three full
base-field rational x-maps of degrees `(7,6)`, `(5,4)` and `(14,13)` from two
independent interpolation windows. All eight extension cells agree within
their base-curve family, all 2,524 regular evaluations match, and the predicted
visible-kernel sizes match. Generic degree, cyclicity, scheme semantics and
local-series transport remain open. [pcc:unverified]
At the abstract endomorphism-ring layer, Lean also proves the exact factor
identity `(V-1)(pi-1)=p+1-t`. Thus the algebraic norm calculation behind the
predicted degree is closed; its scheme-theoretic degree/kernel interpretation
is the remaining geometric step. [pcc:unverified]
Lean now packages that geometric step as an exact paired-degree certificate:
multiplicativity of degree, `deg[N]=N^2`, and equality of the degrees of
`V-1` and `pi-1` imply `deg(V-1)=N`. A numeric mutation shows the equality
premise is essential: unequal degrees 4 and 9 still have product `6^2`. The
pinned Mathlib snapshot supplies no elliptic-isogeny, dual-isogeny, or morphism-
degree API, so those three inputs remain explicit obligations rather than
assumptions hidden in the conclusion. [pcc:unverified]
The frozen `H116-KERNEL-X-ORBIT-001` experiment independently checks a finer
consequence of the corrected eigenspace model. Across eight cells, every root
of the three reconstructed denominators matches the union of the
`lambda^d=+1` and `lambda^d=-1` eigenspaces modulo `P~-P`, including exact
simple/double multiplicities and zero numerator cancellations. The two locked
sign-sensitive cells contain five denominator roots that point enumeration
cannot see: three at `p=5,d=3` and two at `p=7,d=2`. This repairs the retained
v1 interpretation on the finite panel but does not prove generic cyclicity or
scheme semantics. [pcc:unverified]
The frozen `H116-DUAL-PAIR-COMPOSITION-001` experiment then reconstructs the
other norm factor `pi-1` rather than assuming its kernel polynomial. Its three
stable rational x-maps have the same respective degrees `(7,6)`, `(5,4)` and
`(14,13)` as the recovered `V-1` maps. Direct group evaluation verifies
`(V-1)((pi-1)P)=[N]P` on all 5,066 points with zero failures; omitting the
right-factor subtraction and changing `[N]` to `[N+1]` reject in every cell.
This is finite evidence for the paired-degree certificate, not a proof that
the maps are scheme-theoretic dual isogenies. [pcc:unverified]
The next Lean layer imports the finite field-tower degree law and Mathlib's
exact division-polynomial degrees. For any field where `n` is nonzero,
`Phi_n/Psi_n^2` has numerator/denominator degrees
`(n^2,n^2-1)` and leading coefficients `(1,n^2)`. When `n` vanishes
in the field and `|n|>1`, Lean proves the denominator degree strictly
collapses; a `ZMod 3` control records a concrete case. The frozen
[scalar division-polynomial experiment](SCALAR_DIVPOLY_DEGREE_EXPERIMENT.md)
independently constructs these maps for `N=7,5,14`, matches direct scalar
multiplication on 5,058 affine points with zero failures, and measures the
`[p]` denominator-degree collapses `24->20`, `48->42`, and
`120->110`. The concrete pullback tower, point-map identification,
coordinate-to-scheme degree theorem, and duality remain open.
[pcc:unverified]
The next Lean checkpoint removes the abstract coordinate-degree assumption.
For two compatible function-field towers above the pulled-back target x-line,
quadratic source and target coordinate projections force the curve-map degree
to equal the rational coordinate extension degree. Mathlib's exact reduced
rational-function theorem then identifies that coordinate degree as
`max(deg num(f),deg denom(f))`. The remaining scalar-specific steps are to
prove generic-`n` no-cancellation, construct the actual H116
pullback fields and identify the quotient with the curve map. The
[source-linked report](reports/search-20261118T1152Z/REPORT.md) contains the
tower diagram, references, audit receipt and unequal-projection control.
[pcc:unverified]
The following Lean checkpoint closes the generic displayed-quotient gap:
for nonzero `q` with `IsCoprime p q`, canonical `RatFunc` normalization keeps
the degrees of both `p` and `q`, and the coordinate extension therefore has
degree `max(deg p,deg q)`. The separately frozen
[`H116-SCALAR-DIVPOLY-COPRIMALITY-001`](SCALAR_DIVPOLY_COPRIMALITY_PROTOCOL.md)
panel targets the generic premise `IsCoprime Phi_n Psi_n^2` with exact gcds
and two negative controls.  A new scalar-two Lean instance no longer waits on
that panel: the explicit Bezout constant is `4*A^3+27*B^2`, so every
nonsingular short-Weierstrass curve outside characteristic two has coprime
Mathlib division polynomials `Phi_2,Psi_2^2`.  Combining this with the reduced
quotient theorem proves the exact coordinate extension degree
`[F(X):F(Phi_2/Psi_2^2)]=4`.  Arbitrary `n`, the curve-map identification and
the H116 pullback fields remain open.  The next Lean checkpoint repeats the
construction at scalar three with a degree-eight extended-Euclid witness whose
constant is `64*(4*A^3+27*B^2)^2`.  It proves coprimality of Mathlib's
`Phi_3,Psi_3^2` and exact reduced coordinate degree nine outside the stated
characteristic exclusions; the singular `A=B=0` pair over `ZMod 5` is a
proved negative control.  These fixed-scalar results do not establish the
generic-`n` premise.  A further generic Lean reduction now removes the
`X*PsiSq_n` multiple from Mathlib's exact `Phi_n` definition and proves that
three certificates suffice: `prePsi_(n+1)`, `prePsi_(n-1)`, and the parity
factor must each be coprime to `PsiSq_n`.  An integer mutation shows that one
certificate cannot simply be omitted.  The unresolved generic theorem is now
this adjacent-EDS/parity coprimality package rather than an opaque rational-
function cancellation claim.  Lean also closes the next root-semantic layer:
polynomials over an algebraically closed field with no common root are
coprime, and a group element killed by `[n]` and `[n+1]` is zero.  Hence a
common-root-to-nonzero-adjacent-torsion certificate would discharge each
adjacent polynomial obligation.  The exact missing bridge is now to construct
that nonzero elliptic point from Mathlib's division-polynomial roots; a zero-
point control proves the nonzero clause is necessary. [pcc:unverified]
The source-linked [universal resultant bridge](UNIVERSAL_RESULTANT_BRIDGE.md)
now records that Schmidt's published multiplication-polynomial formula, once
aligned with the pinned Mathlib `Phi,PsiSq` conventions, implies arbitrary-
scalar coprimality and exact coordinate extension degree `n^2`.  Lean proves
that implication with the formula kept as an explicit premise.  This removes
the mathematical novelty claim from the fixed-scalar corridor; convention
alignment, the H116 map semantics and the scalar predicate remain open.
[pcc:unverified]
At the coordinate layer Lean now defines `z=-x/y` and proves exactly that
`(x,-y)` sends `z` to `-z`, that field maps commute with `z`, and that a
coordinatewise `p`-power endomorphism sends `z` to `z^p`; the bridge from
these coordinate identities to the actual formal group remains open.
[pcc:unverified]

The frozen [prefix-transfer boundary control](PREFIX_TRANSFER_EXPERIMENT.md)
then tests that precision loss on 20 finite-field series cases. In every case a
correction first visible at degree `m+1` in the local parameter is first visible
at degree `m` after normalized inverse-square transfer, and the boundary
coefficient matches `-2 c f(0)^-3`. Zero and one-degree-shifted controls pass in
all 20 cases. This demonstrates that the proved one-degree loss is sharp; it
does not establish the endomorphism parameter identity.
[pcc:unverified]

The frozen [regular-unit substitution control](REGULAR_SUBSTITUTION_EXPERIMENT.md)
adds a nonconstant outer unit. Across the same 20 prime/cutoff cells,
substitution and its inverse first differ at `m+1`, the full normalized pole
coordinate first differs at `m`, and all three closed-form boundary deltas
match. Zero-correction, constant-outer and nonunit-outer controls all pass.
This tests the generic composition theorem; the new Lean curve-coordinate
instance supplies its elliptic algebra but not the `V-1` semantics.
[pcc:unverified]

The frozen [curve-coordinate control](CURVE_COORDINATE_EXPERIMENT.md)
instantiates that algebra on the p=31 fixture and four completed OOS models.
An independent recurrence verifier reproduced all 24 coefficients of the
source and target units: curve identities, `(a^4A,a^6B)` rescaling, double
substitution and the complete normalized x-ratio match in all five cells.
Wrong target scaling and wrong local parameter first differ at coefficient
four in every cell. Identity certificate `IDC1h692b3e04c6a4f0cd` and the
inspected [PDF report](reports/run-20261118T0618Z/REPORT.pdf) bind the algebraic
and experimental records. This does not discharge the actual local
endomorphism parameter identification. [pcc:unverified]

The preregistered
[first omitted-Frobenius boundary experiment](FROBENIUS_BOUNDARY_EXPERIMENT.md)
tests the coefficient immediately beyond the matched low jet. On the excluded
p=31, r=37 fixture, the retained normalized `V-1` map and an independently
constructed scalar `[-6]` map agree through coefficient 14, first differ at
the predicted coefficient `(p-1)/2=15`, and have the predicted delta
`2/(t-1)=10 mod 31`. Both map identities pass and all three controls reject.
The first producer's wrong duplication sign is retained as a failed positive
control; v2 changed only that sign under a new source freeze. The
[inspected report](reports/run-20261118T0645Z/REPORT.pdf) includes the data and
formal dependency chain. This is one small-curve boundary result, not a
growing-family theorem or ECDLP speedup. [pcc:unverified]

The preregistered
[out-of-sample formal-scalar panel](FORMAL_SCALAR_OOS_EXPERIMENT.md) kept the
candidate source unchanged and sought a different-j curve model in each of five
frozen trace classes. The p=107 cell exhausted all models without a qualifying
second j-invariant and was retained as a selection failure. On the other four
cells, the candidate exactly matched full Vélu maps through coefficient ten at
degrees 197, 257, 397 and 547. An independent JSON-only verifier repeated model
selection, quotient division, the recurrence, both curve identities, codomain
scaling and mutation rejection. Thus the same constructor now matches five
full maps including the earlier degree-37 fixture; the finite panel still does
not replace the generic formal-group bridge. [pcc:unverified]

At the formal-series level, the target/reference denominators are now proved
to share every coefficient except `-Uw`: their difference is exactly
`(U0-U)w`. This is conditional on one supplied normalized jet `h`; identifying
that jet with the actual isogeny is still open. [pcc:unverified]

Target generation and moment production are process-separated. The target
preparer applies the frozen per-curve seeds and emits (1) coordinates plus an
opaque SHA-256 digest and (2) a separate scalar-label map. The moment producer
accepts only the first artifact, checks that it is a complete nonidentity
subgroup census, and serializes only the digest. The label map is reserved for
the independent scorer after identity verification.

## Cost boundary

The candidate target for fixed `J` is `O(M(J) log p)` base-field operations,
including construction of the scalar `[t-1]` jet. The direct control performs
`Theta(rJ)` extension-field work after constructing the kernel generator. The
run records cold setup, genuinely reusable setup, per-target work, recovery and
verification separately. No stage ratio will be reported as an end-to-end
ECDLP ratio. [pcc:unverified]

Files are added in this order: literature and protocol freeze, producer and
Lean source, independent verifier binding, checkpointed EKS execution, raw
export, report/diagram/PDF, and only then any claim record.

The independent verifier source is now bound in
`INDEPENDENT_VERIFIER_GATE.md`. Its target-admission run has not started
because the mandatory cryptanalysis checkpoint upload credential is absent;
this blocks the producer run rather than weakening the gate. [pcc:unverified]
