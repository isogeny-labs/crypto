# H116 Lean identity-layer receipt

Toolchain: Lean 4.34.1 with Mathlib revision
`d13f23b723b8a846827a245b89c10fc7d3f11612`.

Source SHA-256:

```text
e2d60b4de7719f2309df671eb71d9ad27d17358cfd1d3e00aa75e805e6d6cbad  H116FiberTrace.lean
```

The repository proof gate built both the default target and the explicitly
tracked source, then transitively audited all theorem declarations:

```text
PASS  26 thm  0 trusted  research/h116_truncated_fiber_traces_20261118/lean
```

Receipt SHA-256 values:

```text
3315564a25a534f38f9ac556762ed717806b31ba7f0d39f776150a8e15ca2eac  receipts/audit.log
79fccc1cda04dc7b127aaeec4b2dca8dc11e97f0a7a84954b017f87d3f87fee7  receipts/build.log
29e284dc7d8024aa6a333a92d0b23035028d78ada47b37f7dd77d8e1a0293483  receipts/cache.log
465749ce8d9a1f4e3481eb51856902450e66aa5c8e6794bf1f9e2603102882d8  receipts/source-build.log
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
eight. It does not yet identify the formal normalized-map series with the
actual isogeny at `w=1/x`, prove that the geometric jet satisfies the encoded
recurrence or construct the `V-1` formal jet, a
scalar predicate or an ECDLP decoder. [pcc:unverified]

The first integration build is retained at
`receipts/concrete-first-build-failure.log`. A broader imported namespace made
the statement's unqualified antidiagonal elaborate to a different declaration.
The corrected theorem uses the definitionally equivalent range sum that the
native loop evaluates; no hypothesis or conclusion was weakened.
