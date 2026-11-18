# H116 Lean identity-layer receipt

Toolchain: Lean 4.34.1 with Mathlib revision
`d13f23b723b8a846827a245b89c10fc7d3f11612`.

Source SHA-256:

```text
c7e24fef4e4b1a4d53194dd1b1a55af9185505d8b3820dcd25901f3ae38c1360  H116FiberTrace.lean
```

The pinned toolchain built the explicitly tracked source, then the repository's
`AuditAxioms.lean` transitively audited all theorem declarations:

```text
PASS_DIRECT  119 thm  0 trusted  research/h116_truncated_fiber_traces_20261118/lean
```

Receipt SHA-256 values:

```text
f1b4db9afa0d4c3ffeec03cdf730348f7ed2e69bc11f7357cb7abf1cdd4f8fdc  receipts/audit.log
794675580f9fd3f0540a4d3aca6c2fde586605da1df53390941f3fc8e26474a3  receipts/build.log
29e284dc7d8024aa6a333a92d0b23035028d78ada47b37f7dd77d8e1a0293483  receipts/cache.log
84e0cf16eb81ad2cf8a5dce3dfa08b1e35977e0d56207f1d82ac5e3c80b05025  receipts/source-build.log
c26de83abdc9496cd1301470918ec39ecca1cf389ef0ae1c6504da1800d1c431  receipts/status.txt
6a9633829db8cd6a3ef7839ce705874f4295ced6b9d83c4e93f5c3f9f667d589  receipts/concrete-first-build-failure.log
8e6e042fd064c36089b12d0eb28fa670d0fecdb7b8b40a4064767b147ea9ad4f  receipts/prefix-transfer-build.log
d42999adfc5a4d88469b491c5ae9f72d329172b50d322d8c69b45c9e5f4a77cc  receipts/prefix-transfer-audit.log
eddb483c58d97a0aab4af93e933272a8f37d6208040be118d6826a5c3202776e  receipts/regular-substitution-build.log
378ac3322b032621070229de6c5f6e222ecd31f5729b8d198cd9b5e39282303b  receipts/regular-substitution-audit.log
b9a7d56d16d41dc867cb768b114d87dd81b1cd3c745b2968adca395e95dca634  receipts/curve-coordinate-build.log
621d395aacdb96c526881fd73cab63ea937df6f7779cb10d69eca36669e8ac5f  receipts/curve-coordinate-audit.log
43aa3a63525c4f4249a311b4ace2f04f1510ceb6d73f1666a60913ee6e2b6579  receipts/formal-group-build.log
4f1d11d86bd8040b77073409c5d5e07f2f75bb18257ef6b9d5f0a5f31b9f4ac4  receipts/formal-group-audit.log
e799ca27fd505fd3bd6d62389f8a609bf2a99e23765c810eb2c8ab8fff7210e1  receipts/formal-cancellation-build.txt
555c4dbcbbc1bf204023d35b37e47b1211b2dfc2eacc0aa1b4c9d5be673849fd  receipts/formal-cancellation-audit.txt
d0b4471bb8940fdc9338cdafd1a2067462d9b7dcaa8d872b3ab3b17b35ad047e  receipts/endomorphism-specialization-build.txt
79e02a8aac2abe78f8053a1c859a4617a379e25cb973f0beacaa86992af0243e  receipts/endomorphism-specialization-audit.txt
b07aea6e5114e29df4209e1c4548be6a807dffb4853c91323e2ceb574a994c2c  receipts/affine-frobenius-build.txt
9b5f3da4fc8ef2c694fdd9942086c3a7f42e936a4512ba3c0067ca94257164f4  receipts/affine-frobenius-audit.txt
cca767c7b41617791f74533e98d52d7dff185195899aff1e711d6f38dcdf2832  receipts/pointwise-verschiebung-build.txt
d394c8993e2163bbf80c4cdb313da455d456402cdee0eb4d62edbf559431157d  receipts/pointwise-verschiebung-audit.txt
7032125ddbcf9a9ed2cbfabea7cc42f7bb3fbc045637bf72133e57158de58902  receipts/point-observable-first-build-failure.txt
7089cef1d434cf9a73ca41ef2d0e1a3b03fe7ffdd1016e7e03c19c8344e123b7  receipts/point-observable-build.txt
47b5ab662d6ffb77e3b5e0a098741d3c2bc293d3c7a05f69764b4f39a133a7ab  receipts/point-observable-audit.txt
6c773a22800689a181321a476fa59f95e0478132e0a1da72007d1bc318ca39cd  receipts/generator-certificate-build.txt
8c9eb79f902c872b7860f8089d6bc5b280bfce3151aa6428e346df874a04b6a0  receipts/generator-certificate-audit.txt
4165925670b0621960cfbf9f0361276bf60b7b3c134653a847aeb2377ca9f2fb  receipts/periodic-fixed-loci-build.txt
b04c0644adf3fbb0fe15e4cbe36456b2bec88be2f226f29e2c6376066161068a  receipts/periodic-fixed-loci-audit.txt
```

The earlier repository-wide gate receipts above cover the first 64 theorems.
For the 80-theorem checkpoint its mandatory cache-fetch preflight could not
complete because the managed 32-GiB workspace had insufficient free space;
the incomplete task-owned cache was deleted. The same pinned `lake build`
completed, and the gate's exact `AuditAxioms.lean` auditor passed directly over
the resulting module. This is recorded as `PASS_DIRECT`, not silently reported
as a successful cache preflight. [pcc:unverified]

Every theorem reports only `propext`, `Classical.choice`, and `Quot.sound`.
There is no `sorry`, custom axiom, `native_decide`, trusted declaration, raised
limit or curve-specific group-law copy.

Scope: the artifact proves the generic product logarithmic derivative,
finite-root reciprocal sum, abstract coefficient-to-power-sum implication,
relative denominator cancellation, simple-root residue cancellation, the
exact unit-constant reciprocal-series recurrence, the derivative-times-
reciprocal coefficient convolution and its relative subtraction form. It also
defines the exact concrete series
`F_U(w)=1-Uw+h₁w²+h₂w³+...`, proves its constant, linear, higher and derivative
coefficients, and discharges the unit premise of its reciprocal recurrence.
It proves that the target contributes exactly the linear term `-Uw` and that
two target denominators with the same jet differ in only that coefficient. It
also proves that every denominator used by the native `h₁,...,h₇` Bostan
recurrence is nonzero in `ZMod p` when `p` is prime and `p>19`. It also encodes
the exact finite recurrence, proves its seven coefficients unique, and proves
that any two solutions induce identical `F_U` coefficients through degree
eight. It formalizes transformed equation (15) as a PowerSeries identity and
proves that this identity plus the two initial coefficients implies the exact
recurrence, including the prime `p>19` specialization. It also proves that
differentiating equation (14) and cancelling an explicitly nonzero map
derivative gives equation (15). It encodes transformed equation (14) as a
PowerSeries identity and proves that it implies the already formalized
transformed equation (15), with nonzeroness of the cancelled factor proved from
its constant coefficient. It proves that the degree-two and degree-three
coefficients of transformed equation (14) force the two initial jet identities,
then packages the chain showing that this one first-order identity implies the
complete recurrence for prime `p>19`. It does not yet construct the actual
normalized isogeny at-infinity series or prove that series satisfies transformed
equation (14). It does, however, define a finite reversed polynomial
numerator/denominator certificate, prove the cleared-denominator identity
implies transformed equation (14), and compose that certificate to the complete
prime-field recurrence. It now also formalizes equality of finite power-series
prefixes using `PowerSeries.trunc`, proves that multiplication, powers,
differentiation and unit cancellation respect the required precision, and
defines a fixed rational-map jet certificate. Eleven coefficients of the
quotient relation and ten of the cleared identity imply nine coefficients of
the normalized ODE, which are proved sufficient for both base identities and
all five indexed recurrence steps. The prime-field endpoint composes this
fixed certificate directly to `BostanJetRecurrence` for every prime `p>19`.
Producing the correct eleven coefficients without the full degree-`r` map,
validating their intended kernel semantics, the `V-1` formal jet, a scalar
predicate and an ECDLP decoder remain open. [pcc:unverified]

The formal-scalar follow-up also packages Mathlib's monomial truncation theorem
in `PowerSeriesPrefixEq` and proves that, for every prime `p>19`, a normalized
`X^(p-1)` correction vanishes below degree 22 and at the eleven even degrees
used by the frozen x-map prefix. This is a precision theorem only. The actual
elliptic identities `V+pi=[t]`, `pi(z)=z^p`, and formal subtraction remain open
endpoint obligations. The follow-up now proves the generic double-pole core:
coefficientwise reconstruction, cancellation of a shared nonunit leading `X`
with exactly one degree of precision loss, unit-series inversion, inverse-square
preservation, and invisibility of an `X^m` multiple. The regular elliptic
x-coordinate unit-factor algebra is now generic as well: substitution of
prefix-equal zero-constant parameters preserves a fixed regular series,
preserves its inverse when the outer constant is a unit, and composes with the
inverse-square pole term. Identifying the supplied outer unit with the actual
elliptic x-coordinate series is now discharged by
`WeierstrassCoordinateUnit`: its fixed-point equation forces constant one and
the regularized affine curve identity. Rescaling proves the target coefficients
`(a⁴A,a⁶B)`, and double substitution proves the exact target/source identity
when `a*zNormalized=zScalar`. The 84-theorem checkpoint additionally imports
Mathlib's actual `FormalGroup` and proves that `Y` divides `F(X,Y)-X`, that a
substituted correction factors multiplicatively, and that an `X^m` correction
preserves the formal parameter prefix below degree `m`. It composes this with
the leading-`X` pole bridge at one degree less. The 94-theorem checkpoint adds
the endomorphism-oriented cancellation step: associativity turns certified
relations `F(actual,omitted)=scalar` and
`F(omitted,inverseCorrection)=0` into
`F(scalar,inverseCorrection)=actual`. For a commutative formal group it also
proves that a certified inverse inherits an `X^m` factor from an omitted
parameter with that factor, then composes this fact through the normalized pole
prefix theorem. An explicit-negation endpoint avoids a separately constructed
inverse series: it takes `F(omitted,-omitted)=0` as the visible certificate and
derives the correction order directly. Instantiating this generic result with
the actual elliptic `V+pi=[t]`, short-Weierstrass negation, the order-`p`
Frobenius parameter, and sign/scaling identifications remains open. The same
checkpoint defines `z(x,y)=-x/y` and proves coordinatewise that negation sends
`z` to `-z`, every field ring homomorphism commutes with `z`, and a supplied
`p`-power coordinate endomorphism sends `z` exactly to `z^p`. This narrows but
does not construct the missing formal-group/endomorphism instance.
It additionally proves, over a possibly noncommutative ring, that the
Frobenius characteristic equation and norm relation imply `V+pi=trace` when
left composition by `pi` is cancellable and the scalar trace commutes with
`pi`. The 95-theorem checkpoint specializes this result to the concrete ring
`AddMonoid.End G`: preservation of integer multiples proves scalar centrality,
and pointwise injectivity of `pi` proves left-composition cancellation. Hence
injective `pi`, `pi*V=[p]`, and `pi^2-[t]*pi+[p]=0` imply `V+pi=[t]` as
additive endomorphisms. A checked `ZMod 2` mutation control sets `pi=V=0`,
`p=2`, and `t=1`: the norm and characteristic equations still hold while the
sum conclusion fails, demonstrating that injectivity is a necessary boundary.
Constructing the actual elliptic maps, proving the three positive certificates,
and transporting pointwise equality to the local formal group remain explicit
endpoint obligations. [pcc:unverified]

The 101-theorem checkpoint constructs the positive Frobenius map on Mathlib's
actual nonsingular affine point group. For a curve over a finite field `k`
base-changed to a field `E`, `FiniteField.frobeniusAlgHom k E` induces an
injective additive point endomorphism whose affine coordinates are raised to
`#k`; over `ZMod p` they are exactly raised to `p`. The composed prime-field
theorem therefore needs only a supplied Verschiebung plus `pi*V=[p]` and the
Frobenius characteristic equation to derive `V+pi=[t]` on points. Constructing
that Verschiebung, proving those two endomorphism equalities, and transporting
point equality to the local formal-group series remain open. [pcc:unverified]

The 105-theorem checkpoint constructs inverse Frobenius on the same affine
point group over any algebraic extension of `ZMod p`. It defines the pointwise
candidate `V=[p]*piInverse`, proves `pi*piInverse=1` and therefore proves the
norm relation `pi*V=[p]` without an extra premise. The Frobenius characteristic
equation alone then implies `V+pi=[t]` on affine points. This additive
point-group construction does not identify `V` with the retained rational or
scheme morphism, and it does not transport equality of point endomorphisms to
equality of the selected local formal power series. Those semantic
identifications and the characteristic equation remain open; no runtime or
asymptotic conclusion follows. [pcc:unverified]

The 109-theorem checkpoint rearranges the point-endomorphism identity into the
exact candidate orientation `V-1=[t-1]-pi`. It defines a total observable on
Mathlib affine points, sending infinity to zero and `(x,y)` to the existing
short-Weierstrass parameter `-x/y`, then proves that the two point expressions
have equal observed parameter values under the explicit characteristic
premise. This closes additive orientation and pointwise observation only.
Identifying the constructed pointwise map with the retained rational or scheme
morphism, and transporting that representation to equality of completed local
power series, remain open. [pcc:unverified]

The 112-theorem checkpoint adds a proof-carrying finite-group interface.
Mathlib's additive-subgroup closure theorem promotes equality on a supplied
generating set to equality of additive endomorphisms. H116 instantiates it with
the Frobenius characteristic expression, defines the trace as
`p+1-Nat.card E(F_p)`, and composes generator checks directly to the pointwise
`V-1` orientation. Producing a finite-extension closure certificate remains an
external verification task. Such a certificate does not prove the equation on
the algebraic closure or identify rational/formal maps. [pcc:unverified]

The 119-theorem checkpoint proves an exact algebraic-point exhaustion bridge.
Any coordinate pair in an algebraic extension of `ZMod p` lies in one finite
intermediate field, so a common positive Frobenius power fixes both
coordinates. Iterating Mathlib's point Frobenius therefore proves that every
affine algebraic point lies in some positive fixed locus. A generic
endomorphism lemma promotes vanishing on every such locus to global vanishing,
and H116 composes it to the exact pointwise `V-1` orientation. The checked
`ZMod 3` mutation control shows why one fixed locus is insufficient: negation
fixes only zero at period one, where the identity mutation is invisible, while
period two exposes it at one. This reduction does not prove the characteristic
identity on the finite fixed loci, identify rational/scheme morphisms, or
imply an ECDLP speedup. [pcc:unverified]

The first integration build is retained at
`receipts/concrete-first-build-failure.log`. A broader imported namespace made
the statement's unqualified antidiagonal elaborate to a different declaration.
The corrected theorem uses the definitionally equivalent range sum that the
native loop evaluates; no hypothesis or conclusion was weakened.
