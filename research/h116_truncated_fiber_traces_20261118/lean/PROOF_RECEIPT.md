# H116 Lean identity-layer receipt

Toolchain: Lean 4.34.1 with Mathlib revision
`d13f23b723b8a846827a245b89c10fc7d3f11612`.

Source SHA-256:

```text
8afac8222c1ea6656e6ff167ae2e7ae31885bf2bda11f268945a16035447833e  H116FiberTrace.lean
```

The pinned toolchain built the explicitly tracked source, then the repository's
`AuditAxioms.lean` transitively audited all theorem declarations:

```text
PASS_DIRECT  201 thm  0 trusted  research/h116_truncated_fiber_traces_20261118/lean
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
76d2694729ae99d4f3877d3ae8ead42973d6dd1c4d7879aa34a86354e3583a72  receipts/rational-separation-build.txt
b469ffae036437ff01c0318340771c04e62b1fded7b7eb2775d16631f71cbcc3  receipts/rational-separation-audit.txt
91f194daf08ff0c6c4d6455a9d30c86ee468a35b172bdc17a29fc0f43eeed1a7  receipts/rational-separation-first-build-failure.txt
673b57e3ca150bf88154f6193e14278944aae9de240501e4ae32d5036c665744  receipts/rational-separation-second-build-failure.txt
b7efb787302ea2dfa9aa223d42ac1aec148da29a8a2c358b08a34b1b59c4dc5f  receipts/kernel-eigenspace-build.txt
64bc81bfa8577086492d0220e137ffb82dac8306a3b6df28300096475c456d9e  receipts/kernel-eigenspace-audit.txt
f6b367568557629e55b46748360a01a251768e8c0d8ce4ed53953eb8e65a9e58  receipts/norm-factor-build.txt
f61a96a76811c2cd4736b7515e2f2a4126a1b500e397894ff6c090dc4a8864c2  receipts/norm-factor-audit.txt
543996048dcfc71e49530f69165e90f4a49322258d91b93101ced228ee8f0389  receipts/degree-certificate-build.txt
1e567fafe38bf9cdc1f29d0399a60b69f0272055ff039e49bb168e78d2689b2f  receipts/degree-certificate-audit.txt
6f25eceebc85cce9cc43bcd1b0265beb8596c3a32043580266e51aa107b886df  receipts/scalar-degree-tower-build.txt
369a478a267c3adc7bfe821025d5a5b31546d1f96e1f2e3071b2e45f7c3d8d51  receipts/scalar-degree-tower-audit.txt
a9be1857e0b978b1b0a5cff9ca238ae04b13e02c00023384e481b1ec45d70eb2  receipts/function-field-sandwich-build.txt
523f28dceb06d84847126a8066a7a15d6059bff9a215504fbc2c03a7ccf82b1b  receipts/function-field-sandwich-audit.txt
9134d32b084b1163d4e4c54d4dcccc1c093f67da34cbd66a5c086914348d9f5b  receipts/no-cancellation-build.txt
af1f55a38f86370e51a9e4038009738a845709279204f56dbc3e487737715f11  receipts/no-cancellation-audit.txt
ea01d069a7682bd04bb64a5aa113cba204622e24b66ef16a0fd376403eb8f584  receipts/scalar-two-coprimality-build.txt
597fdddd8ad379ed42f458a779b17367c87ff4a4878412bcd2e30d57a274a73c  receipts/scalar-two-coprimality-audit.txt
912ad7b074051d0da17a004b556754ab5aeb2c4932d8aa788c23a3225afdf3f9  receipts/scalar-three-coprimality-build.txt
5731fc9c7bc418dc7f99cdd3b422f4618771e1843776691afc5736f5b66aaecc  receipts/scalar-three-coprimality-audit.txt
912ad7b074051d0da17a004b556754ab5aeb2c4932d8aa788c23a3225afdf3f9  receipts/generic-coprimality-reduction-build.txt
1932e63c3f78d6ed02cba849b8c5040c19c4fccee3c541e930c35b3848c8a664  receipts/generic-coprimality-reduction-audit.txt
912ad7b074051d0da17a004b556754ab5aeb2c4932d8aa788c23a3225afdf3f9  receipts/adjacent-root-semantics-build.txt
c2024fa98bfb039e1e161b9ee6128ef176a1c04af4c80e3bcaa9952c64f80356  receipts/adjacent-root-semantics-audit.txt
27c565391615e9df1a9b95db99860bd257b2a3ef95b816033ad5cbae69b9f27d  receipts/algebraic-closure-descent-build.txt
e0bd5924933c7d079a435effee25447e7a4a1b3feb4abdcc007e0c17ebd08f02  receipts/algebraic-closure-descent-full-build.txt
e94fe66e359305ddbae083f9d86559ad5984ca577e5ccede00fc8cbfaaf36fc6  receipts/algebraic-closure-descent-audit.txt
118466f845c4c9efdfd729cfcdb9254fbac8a198c9a260c3bfa7da9752a4734a  receipts/affine-point-lift-build.txt
2bf8d5c08ebb3d7273efbe0fd0035fc5765ff1367afa1c766f0dd0c72d2d613d  receipts/affine-point-lift-full-build.txt
16c2e15deef97c7e76e47a664f9c7b0822b6e511abb322d5d17087290fa8063a  receipts/affine-point-lift-audit.txt
f5beab595d8df926ce8d0c4cd16d80c35c58d22fd24cb2f618f89f4ec3db9603  receipts/odd-eds-parity-build.txt
3b6b4e6857cbaf5bf4d464ab4bb7bb49ba43f7f52ce33f18bdcbeadc1e40f74c  receipts/odd-eds-parity-full-build.txt
11a8caa16d979f24c3d3019e5b2fd9c44453505557e266bbce39aaef46f6097d  receipts/odd-eds-parity-audit.txt
ee6ea7e75626e2699c5ee765246480ec2ec6183de2604b89bdf1a65318c85b34  receipts/scalar-four-bootstrap-build.txt
fc6bd72d5b93ee2d369243a94342a924ab1a640e60b08f6b5c0006037822a3c1  receipts/scalar-four-bootstrap-full-build.txt
2d0c1fa8ac87304c62c9473c57727eaba88342d2e89c3834e6a0b20883e2f1a6  receipts/scalar-four-bootstrap-audit.txt
3aede61bd0d0a8e98614966570d22e58e3145afafbee48099d9ebf57b2621564  receipts/scalar-five-bootstrap-build.txt
7e627e64916a948ec3a65944f76b62e333d95f03b8a7e2c653cf3765a71fd3f6  receipts/scalar-five-bootstrap-full-build.txt
883eaa9ccd73b5cedd8a50fa4200a8fe2fafc6981fb0e495552983861db6ec38  receipts/scalar-five-bootstrap-audit.txt
7f9ddfb534f85f11e68a751767bc5e3a5f8dd410d24404b248183a3048b3725a  receipts/scalar-six-bootstrap-build.txt
04b9838bc270b57f7cc9cebdcef7f50dba2783257cfea30e217d750e8a185d8e  receipts/scalar-six-bootstrap-full-build.txt
4d709be2432832e9635842df1f72edee24f07792482475f54a6e5b02bf9b4a1a  receipts/scalar-six-bootstrap-audit.txt
a74ee17eea454b66f6fc1e3f8acd75fd7de2bfa723c56a9b9193a3858bc4ca8c  receipts/generic-two-factor-bootstrap-build.txt
988f903ed14a3cda6f1ba517dc407c00957773c6bdb73bce3dc2055244832238  receipts/generic-two-factor-bootstrap-full-build.txt
aeffe27752d9cd4d28b01dcf47992e3b5b5e11396627debbc60ffb685361db05  receipts/generic-two-factor-bootstrap-audit.txt
18d5c962ccb50c179bcf616b0c5987e13b7f9353ccaf71ff98f94b7da97ac2fa  receipts/generic-successor-scalar-seven-build.txt
463f9e1442a6219c211a0d6cea358eb8f217d97cf830fce4f9d654fe08ff3f84  receipts/generic-successor-scalar-seven-full-build.txt
a31892ff2095c514583f502bfef1ba389f69a4c4a3f39435f87b9029aed57af6  receipts/generic-successor-scalar-seven-audit.txt
d93b32bc5be9adc6229888fec8e59edb7182d3afbcf2dd4d7abbccee9280a0b5  receipts/second-successor-scalar-eight-build.txt
ae1b03718e01355a281f31e8c00e0ddb3a652bb091df4f5ba74f80794518cbc2  receipts/second-successor-scalar-eight-full-build.txt
ac3832511710d08fdaec1912b38116d7702503b3dca339b58c64c250ab88f93d  receipts/second-successor-scalar-eight-audit.txt
b67421427618cb5c1774e9bbb66583a164d9dbf7c8772fc06c0ec4f44e46fb49  receipts/third-successor-scalar-nine-build.txt
62633ade5c49413ba2bfa42efefb3a28112cea149c61137843762d298ee79964  receipts/third-successor-scalar-nine-full-build.txt
7829dc01b1738d829455dd90dfb4bfdb17a6a5ed17c4eb21a4d29da8d948e3e6  receipts/third-successor-scalar-nine-audit.txt
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

The 121-theorem checkpoint adds a finite rational-map separation interface.
Two supplied numerator/denominator pairs have equal cross products when they
agree on a finite set where both denominators are nonzero and the set size is
strictly greater than both cross-product degrees.  A `ZMod 3` control proves
the strict inequality is necessary: `X^3-X` is nonzero, has degree equal to
the field cardinality, and vanishes everywhere.  The interface does not
construct the H116 rational map, bound its cross-product degree, or audit its
poles; those remain explicit hypotheses, so no algorithmic or asymptotic
conclusion follows. [pcc:unverified]

The 124-theorem checkpoint formalizes the kernel-eigenspace mechanism exposed
by the retained reconstruction failure.  From `V+pi=[trace]`, every point fixed
by `V` satisfies `pi(P)=[trace-1]P`; iteration gives
`pi^n(P)=[(trace-1)^n]P`, so the degree-`n` Frobenius fixed-locus condition on
`ker(V-1)` is exactly the corresponding scalar condition.  This proves the
eigenvalue reduction but not the cyclic-kernel cardinality formula used as a
finite-panel prediction. [pcc:unverified]

The 125-theorem checkpoint proves the exact noncommutative-ring factorization
`(V-1)(pi-1)=p+1-trace` from the norm relation `V*pi=p` and sum relation
`V+pi=trace`. This is the algebraic norm identity behind the expected degree
`#E(F_p)` of `V-1`; interpreting it as a degree and kernel-cardinality theorem
still requires the missing elliptic morphism infrastructure. [pcc:unverified]

The 128-theorem checkpoint isolates that remaining degree infrastructure as a
proof-carrying interface.  A multiplicative degree, scalar-degree identity
`deg [N]=N^2`, and equal degrees for the factor pair turn the proved norm
factorization into `deg(V-1)=N`.  The composed theorem applies this directly to
`(V-1)(pi-1)=[p+1-t]`.  A checked `N=6` control gives unequal factor degrees
4 and 9 with product 36, proving that norm degree alone cannot replace the
dual/equal-degree premise.  The pinned Mathlib snapshot has no elliptic-isogeny,
dual-isogeny, or morphism-degree API, so instantiating the three geometric
premises remains explicit and open. [pcc:unverified]

The 134-theorem checkpoint proves finite field-tower degree multiplication and
its equal-adjacent-degree consequence, then exposes Mathlib's exact
`(n^2,n^2-1)` numerator/denominator degrees and `(1,n^2)` leading
coefficients for the division-polynomial coordinate quotient when `n` is
nonzero in the field. It also proves strict denominator-degree collapse when
`n` vanishes in the field and `|n|>1`; a `ZMod 3` control supplies a
concrete instance. The
remaining bridge is geometric: construct the pullback tower, identify the
division-polynomial quotient with scalar multiplication as a curve morphism,
identify coordinate and morphism degrees, and prove the dual-pair theorem.
[pcc:unverified]

The 137-theorem checkpoint compares the two compatible field towers that sit
above a pulled-back target x-line. If both curve fields are quadratic over
their x-lines, the full curve-map degree equals the source coordinate-line
degree. It also exposes Mathlib's exact theorem that
`[K(X):K(f)] = max(deg num(f),deg denom(f))` for the canonical reduced rational
function. A checked `4*3=2*6` control shows why equal projection degrees are
necessary. Constructing the H116 field embeddings, proving that the displayed
division polynomials are coprime, identifying their quotient with the curve
map, and proving the dual-pair theorem remain open. [pcc:unverified]

The 139-theorem checkpoint closes the generic no-cancellation bridge for a
displayed rational quotient. Given `q != 0` and `IsCoprime p q`, Mathlib's
canonical `RatFunc` normalization preserves the numerator and denominator
degrees, so the coordinate extension degree is exactly
`max(deg p,deg q)`. The proof uses the public gcd normalization and rational-
function APIs; it does not assume that `Phi_n` and `Psi_n^2` satisfy the
coprimality premise. [pcc:unverified]

The 142-theorem checkpoint closes that premise for scalar two on every
nonsingular short-Weierstrass curve outside characteristic two.  An explicit
Bezout identity has constant `4*A^3+27*B^2`; Mathlib's exact `Phi 2` and
`PsiSq 2` formulas then give `IsCoprime (Phi 2) (PsiSq 2)`.  Composing this
with the generic reduced-quotient theorem proves
`[F(X):F(Phi_2/Psi_2^2)]=4`.  This is an exact coordinate-field result for
`n=2`; it does not establish arbitrary-scalar coprimality, identify the
quotient with the scheme morphism, or prove a subexponential ECDLP bound.
[pcc:unverified]

The 150-theorem checkpoint extends the same concrete bridge to scalar three.
An explicit degree-eight extended-Euclid witness proves the scalar-three
numerator coprime to `Psi_3`; `IsCoprime.pow_right` and Mathlib's exact
division-polynomial formulas prove `IsCoprime (Phi 3) (PsiSq 3)`.  Outside
characteristics two and three the reduced quotient consequently satisfies
`[F(X):F(Phi_3/Psi_3^2)]=9`.  A singular `A=B=0` control over `ZMod 5` is
proved not coprime by mapping evaluation at zero.  This remains a fixed-scalar
coordinate result, not arbitrary-scalar coprimality or a curve-map/ECDLP
complexity theorem. [pcc:unverified]

The 153-theorem checkpoint extracts the exact generic algebraic reduction from
Mathlib's definition of `Phi n`.  If `prePsi(n+1)`, `prePsi(n-1)`, and the
parity factor are each coprime to `PsiSq n`, then `Phi n` is coprime to
`PsiSq n`: the `X*PsiSq n` term is removed as a denominator multiple and the
three factor certificates compose.  An integer mutation proves that omitting
one factor premise is unsound.  The adjacent-EDS and parity-factor premises
are now the explicit remaining generic obligations. [pcc:unverified]

The 157-theorem checkpoint proves the next semantic layer.  Over an
algebraically closed field, no common polynomial root implies coprimality via
the Euclidean gcd.  In any additive commutative group, a point killed by
adjacent integer scalars is zero.  Therefore a certificate mapping every
common root to a nonzero adjacent-torsion point implies polynomial
coprimality.  A zero-point control proves the nonzero clause is essential.
Constructing that point and proving the division-polynomial root semantics
remain open. [pcc:unverified]

The 159-theorem checkpoint descends that semantic layer to the original
coefficient field.  For any field homomorphism into an algebraically closed
field, absence of common mapped roots proves coprimality over the base field;
the adjacent-torsion semantic wrapper now has the same base-field endpoint.
The proof composes Mathlib's exact `Polynomial.isCoprime_map` equivalence with
the preceding audited theorems.  This closes algebraic-closure descent but
does not yet construct the elliptic point or prove division-polynomial
root-to-scalar-multiplication semantics. [pcc:unverified]

The 163-theorem checkpoint closes the root-to-nonzero-point construction.
For every x-coordinate over an algebraically closed field, Lean constructs a
root of the monic quadratic Weierstrass equation in `y`; on an elliptic curve,
Mathlib's `Affine.Point.mk` turns it into a certified nonzero group point.  A
new assembly theorem reduces base-field adjacent coprimality to only the two
pointwise scalar equations.  The negative control proves that
`y^2=x^3+x+1` over `ZMod 5` has no point above `x=1`, so the algebraic-closure
hypothesis cannot simply be removed.  The remaining gap is the generic
division-polynomial root-to-scalar-multiplication theorem. [pcc:unverified]

The 170-theorem checkpoint closes the generic parity-factor premise on
nonsingular short curves outside characteristic two.  A strong induction on
Mathlib's exact odd normalized-EDS recurrence proves every odd term coprime to
`s` from the base pair; a compact Bezout identity with constant
`4*(4*A^3+27*B^2)` supplies that pair for `Psi3,Psi2Sq`.  Consequently the
generic `Phi n,PsiSq n` no-cancellation theorem now requires only the two
adjacent EDS factors for every natural scalar.  The singular `A=B=0` pair over
`ZMod 5` is proved non-coprime by evaluation at zero.  Adjacent-factor
coprimality remains the sole generic no-cancellation obligation in this
specialization. [pcc:unverified]

The 175-theorem checkpoint bootstraps exact scalar-four no-cancellation from
the earlier scalar-three theorem and Mathlib's EDS recurrence.  It proves
`Psi3` coprime to `prePsi4`, then `prePsi4` coprime to `prePsi5`, assembles
`IsCoprime (Phi 4) (PsiSq 4)`, and derives coordinate extension degree
sixteen.  This avoids a separate high-degree resultant certificate.  The
singular `A=B=0` scalar-four pair over `ZMod 5` is proved non-coprime by
evaluation at zero.  Arbitrary adjacent-EDS coprimality remains open.
[pcc:unverified]

The 180-theorem checkpoint extends the recurrence bootstrap through scalar
five.  It proves `Psi3` coprime to `prePsi5`, then `prePsi5` coprime to
`prePsi6`, assembles `IsCoprime (Phi 5) (PsiSq 5)`, and derives coordinate
extension degree twenty-five when the characteristic does not divide five.
Coprimality itself requires only the prior nonsingularity and characteristic-
two hypotheses, separating cancellation from inseparable degree collapse. A
singular `A=B=0` control over `ZMod 7` is proved non-coprime. Arbitrary
adjacent-EDS coprimality remains open. [pcc:unverified]

The 184-theorem checkpoint reaches scalar six through a new two-factor
bootstrap. With `t=prePsi5-prePsi4^2`, Lean proves `prePsi6=Psi3*t` and rewrites
`prePsi7` as `(Psi3^3-prePsi4^2)*t-prePsi4^4`; prior pair and parity
certificates then prove the adjacent pair coprime. This assembles
`IsCoprime (Phi 6) (PsiSq 6)` and coordinate extension degree thirty-six
outside characteristics two and three. The singular `A=B=0` control over
`ZMod 7` is non-coprime. The factorization is a candidate invariant for an
arbitrary-index proof, which remains open. [pcc:unverified]

The 186-theorem checkpoint abstracts that scalar-six argument to an arbitrary
commutative ring. If `c=b*s^2-a^3`, `a` is coprime to `b` and `s`, and `b` is
coprime to `c`, then `a*(c-b^2)` is coprime to
`c*a^3-b^3*s^2`. Scalar six is now a direct instance. The integer mutation
`a=2,b=1,s=2,c=-4` retains the other pair premises but yields `-10,-36`,
proving that `a` coprime to `s` is essential. Companion recurrence stencils
and recursive preservation of all premises remain open. [pcc:unverified]

The 191-theorem checkpoint proves the companion successor stencil under the
same three premises. Lean derives `c` coprime to `s` automatically, rewrites
the next inner factor as `e-c*(c-b^2)`, and proves the recurrence-shaped
successor pair coprime. Instantiating this identity proves `prePsi7` coprime
to `prePsi8`, `Phi7` coprime to `PsiSq7`, and coordinate extension degree
forty-nine outside characteristics two and seven. The singular `A=B=0`
control over `ZMod 11` is non-coprime. Arbitrary parity-branch closure remains
open. [pcc:unverified]

The 196-theorem checkpoint proves a second companion successor stencil with
no additional premise.  For `d=a*(c-b^2)`,
`e=c*a^3-b^3*s^2`, and `u=a^2*d-c^2`, the decisive identity is
`a^2*(d*b^3*s^2-a*c^3)=b^3*s^2*u-c^2*e`.  The preceding successor
certificate then proves the next recurrence pair coprime.  Its short-curve
instance proves `prePsi8` coprime to `prePsi9`, `Phi8` coprime to `PsiSq8`,
and coordinate extension degree sixty-four outside characteristic two.  The
singular `A=B=0` control over `ZMod 11` is non-coprime.  Arbitrary recurrence
closure remains open. [pcc:unverified]

The 201-theorem checkpoint proves a third companion successor stencil under
the same invariant.  Factoring the new odd output as `g=a*w` exposes the next
inner factor as `v=w-t*u`; factor extraction from the preceding certificate
gives `w` coprime to `u`, while the base relation gives `w` coprime to `t`.
The exact short-curve instance proves `prePsi9` coprime to `prePsi10`,
`Phi9` coprime to `PsiSq9`, and coordinate extension degree eighty-one
outside characteristics two and three.  The singular `A=B=0` control over
`ZMod 11` is non-coprime.  A motivational exhaustive sweep over 249,580 valid
tuples in 13 small prime fields found no counterexample, but was not a frozen
benchmark and is not used as proof.  Arbitrary recurrence closure remains
open. [pcc:unverified]

The first integration build is retained at
`receipts/concrete-first-build-failure.log`. A broader imported namespace made
the statement's unqualified antidiagonal elaborate to a different declaration.
The corrected theorem uses the definitionally equivalent range sum that the
native loop evaluates; no hypothesis or conclusion was weakened.
