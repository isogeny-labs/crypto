# H116 Lean identity-layer receipt

Toolchain: Lean 4.34.1 with Mathlib revision
`d13f23b723b8a846827a245b89c10fc7d3f11612`.

Source SHA-256:

```text
7d77482bed908cc294f8a5828a7227981df487ba66ba6e855ffc8b7789295bb9  H116FiberTrace.lean
```

The repository proof gate built both the default target and the explicitly
tracked source, then transitively audited all theorem declarations:

```text
PASS  60 thm  0 trusted  research/h116_truncated_fiber_traces_20261118/lean
```

Receipt SHA-256 values:

```text
97d6cb403fe102771a210cc38fdc9df47487e37b3e6842b03cc6de64d7494c3e  receipts/audit.log
efa2d5bb0383a722e51fc41647b1727f0449b9c07b989db8b11da91d78e37aa3  receipts/build.log
29e284dc7d8024aa6a333a92d0b23035028d78ada47b37f7dd77d8e1a0293483  receipts/cache.log
4b4996d4b0d404b60305f5a8e7b17c82e47e4ee14294e4f65ec1afc92c64fd24  receipts/source-build.log
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

The first integration build is retained at
`receipts/concrete-first-build-failure.log`. A broader imported namespace made
the statement's unqualified antidiagonal elaborate to a different declaration.
The corrected theorem uses the definitionally equivalent range sum that the
native loop evaluates; no hypothesis or conclusion was weakened.
