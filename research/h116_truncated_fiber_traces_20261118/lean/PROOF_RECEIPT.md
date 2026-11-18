# H116 Lean identity-layer receipt

Toolchain: Lean 4.34.1 with Mathlib revision
`d13f23b723b8a846827a245b89c10fc7d3f11612`.

Source SHA-256:

```text
decee05b15443105a82e29bbb684ae458583a34ea14944a84fb85a6ce24e3763  H116FiberTrace.lean
```

The repository proof gate built both the default target and the explicitly
tracked source, then transitively audited all theorem declarations:

```text
PASS  64 thm  0 trusted  research/h116_truncated_fiber_traces_20261118/lean
```

Receipt SHA-256 values:

```text
f1b4db9afa0d4c3ffeec03cdf730348f7ed2e69bc11f7357cb7abf1cdd4f8fdc  receipts/audit.log
794675580f9fd3f0540a4d3aca6c2fde586605da1df53390941f3fc8e26474a3  receipts/build.log
29e284dc7d8024aa6a333a92d0b23035028d78ada47b37f7dd77d8e1a0293483  receipts/cache.log
84e0cf16eb81ad2cf8a5dce3dfa08b1e35977e0d56207f1d82ac5e3c80b05025  receipts/source-build.log
c26de83abdc9496cd1301470918ec39ecca1cf389ef0ae1c6504da1800d1c431  receipts/status.txt
6a9633829db8cd6a3ef7839ce705874f4295ced6b9d83c4e93f5c3f9f667d589  receipts/concrete-first-build-failure.log
```

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
elliptic identities `V+pi=[t]`, `pi(z)=z^p`, formal subtraction, and
x-coordinate pole-order transfer remain open endpoint obligations.
[pcc:unverified]

The first integration build is retained at
`receipts/concrete-first-build-failure.log`. A broader imported namespace made
the statement's unqualified antidiagonal elaborate to a different declaration.
The corrected theorem uses the definitionally equivalent range sum that the
native loop evaluates; no hypothesis or conclusion was weakened.
