# H116 Lean identity-layer receipt

Toolchain: Lean 4.34.1 with Mathlib revision
`d13f23b723b8a846827a245b89c10fc7d3f11612`.

Source SHA-256:

```text
2723ee048d728507d8e6dab1a35a8a4ceea26f412bf69be126639ae1d336e7e7  H116FiberTrace.lean
```

The repository proof gate built both the default target and the explicitly
tracked source, then transitively audited all theorem declarations:

```text
PASS  19 thm  0 trusted  research/h116_truncated_fiber_traces_20261118/lean
```

Receipt SHA-256 values:

```text
214dc4c7c2fe876676575ba3105b6aa4951b88c741a5eb8f37a279c0ead7c424  receipts/audit.log
50256013897fa97d9f03b7089760d799e2a70d7034136cf248a9ad6a20ed4a0d  receipts/build.log
29e284dc7d8024aa6a333a92d0b23035028d78ada47b37f7dd77d8e1a0293483  receipts/cache.log
7a564775860d84abc6573144a048d3d3f6e0031eec063183d7bb939cdd76339b  receipts/source-build.log
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
It does not yet identify the concrete `w=1/x` series with the isogeny fiber,
prove the Bostan recurrence or construct the `V-1` formal jet, a scalar
predicate or an ECDLP decoder. [pcc:unverified]

The first integration build is retained at
`receipts/concrete-first-build-failure.log`. A broader imported namespace made
the statement's unqualified antidiagonal elaborate to a different declaration.
The corrected theorem uses the definitionally equivalent range sum that the
native loop evaluates; no hypothesis or conclusion was weakened.
