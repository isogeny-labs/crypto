# H116 Lean identity-layer receipt

Toolchain: Lean 4.34.1 with Mathlib revision
`d13f23b723b8a846827a245b89c10fc7d3f11612`.

Source SHA-256:

```text
c42ec0c4fda6f69a23b6dcc62e342d363289c850663e2153a5996930e5d22609  H116FiberTrace.lean
```

The repository proof gate built both the default target and the explicitly
tracked source, then transitively audited all theorem declarations:

```text
PASS  32 thm  0 trusted  research/h116_truncated_fiber_traces_20261118/lean
```

Receipt SHA-256 values:

```text
a215abe5811ff67e2241059445c918e4fa1a1728771b6b45d404bcb14a3c435b  receipts/audit.log
526edc8cddb8f0df6fac0c4477364f6bb6a933232bad92e1013f13274082ec89  receipts/build.log
29e284dc7d8024aa6a333a92d0b23035028d78ada47b37f7dd77d8e1a0293483  receipts/cache.log
4270e5cf3f389cef9136b9b900ab22cfbfafc53879c670a5ac425f044f16e095  receipts/source-build.log
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
its constant coefficient. It does not yet construct the actual normalized
isogeny at-infinity series, prove that series satisfies transformed equation
(14), or construct the `V-1` formal jet, a scalar predicate or an ECDLP
decoder. [pcc:unverified]

The first integration build is retained at
`receipts/concrete-first-build-failure.log`. A broader imported namespace made
the statement's unqualified antidiagonal elaborate to a different declaration.
The corrected theorem uses the definitionally equivalent range sum that the
native loop evaluates; no hypothesis or conclusion was weakened.
