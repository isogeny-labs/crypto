# H116 Lean identity-layer receipt

Toolchain: Lean 4.34.1 with Mathlib revision
`d13f23b723b8a846827a245b89c10fc7d3f11612`.

Source SHA-256:

```text
02bb36792047afa035deec587c9384cf280757b18238cba0e50059601d892890  H116FiberTrace.lean
```

The repository proof gate built both the default target and the explicitly
tracked source, then transitively audited all theorem declarations:

```text
PASS  39 thm  0 trusted  research/h116_truncated_fiber_traces_20261118/lean
```

Receipt SHA-256 values:

```text
7eb41219460d3a3c930e0e274286733919b89920f100b92470e017c84425e24e  receipts/audit.log
f2844539a43530ba73f9528e689930f4178326c39545928acc948bef6687c542  receipts/build.log
29e284dc7d8024aa6a333a92d0b23035028d78ada47b37f7dd77d8e1a0293483  receipts/cache.log
47c0205156651285d6a4d5a8dab1ea2d7cf9817d2769f1de78b4cc2459fe53b0  receipts/source-build.log
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
prime-field recurrence. Producing and independently validating those
polynomials for the actual isogeny, the `V-1` formal jet, a scalar predicate and
an ECDLP decoder remain open. [pcc:unverified]

The first integration build is retained at
`receipts/concrete-first-build-failure.log`. A broader imported namespace made
the statement's unqualified antidiagonal elaborate to a different declaration.
The corrected theorem uses the definitionally equivalent range sum that the
native loop evaluates; no hypothesis or conclusion was weakened.
