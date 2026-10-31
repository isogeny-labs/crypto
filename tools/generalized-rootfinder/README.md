# Native generalized finite-field roots

This completes the native execution path of the bounded generalized solver
archived in `research/generalized_rootfinding_20261016/historical/rootfinder`.
It does not use Python. All returned tuples retain their original variable
coordinates. Complete enumeration, component meet-in-the-middle and the native
Boolean quadratic route return exact counts independently of the output cap.
Supported Boolean systems use the existing AVX2/AVX-512 Gray kernel at22
or more variables when available. Native ablations found scalar faster at16;
the default keeps smaller sizes scalar, with full original-equation verification.
`KIC_GENERALIZED_SCALAR_GRAY=1` forces scalar enumeration for comparisons.
Explicit `method: "native_boolean"` permits wide enumeration at smaller sizes
for ablation. The native Rust `solve_boolean_polynomials` entry accepts
quadratic `F2BoolPoly` inputs directly, retaining exact counts and original
bit coordinates without JSON or BigUint conversion. Output caps never
truncate the counted search; unsupported input/budgets return an error.

```sh
cargo run --release --manifest-path tools/generalized-rootfinder/Cargo.toml -- \
  INPUT.json --max-work 8388608
```

Input uses the historical `field`, `variables`, `equations` and `L` schema.
Coefficients have `coefficient` and `powers`; integers beyond JSON's exact range
should be hexadecimal or decimal strings. Fields are prime, binary, or
polynomial-basis extensions with an explicit monic modulus. Characteristics
above 64 bits require a recursive Pocklington certificate. Binary default is
the historical GF(2^83) polynomial. Moduli are certified by Rabin's test.

`L` accepts ascending polynomial `coefficients` and an optional literal
`denominator`, `linearized` Frobenius terms, or `composition` maps in
inner-to-outer order. Rational compositions require
`"semantics":"partial_circuit"`: no cancellation removes an intermediate
pole. `elements` is an additional explicit-domain convenience. Large domains,
degree limits, exhausted splitting trials or search limits never become an
empty complete solution set. Read the certificate and status, including
`output_truncated`, before interpreting an answer.

`method: "separator"` selects the complete target-independent S3 separator.
Its five source coordinates lie in D; its three auxiliary coordinates range
over the full field. Every original eight-coordinate tuple is retained,
including diagonal and exceptional fibers. It accepts ordinary binary curves
with `b != 0` and squarefree completed cubics in odd characteristic. The two
fixtures in this directory exercise the ordinary solver and the separator.

This is a bounded exact solver. Completing its native implementation does not
establish the old research program's 8x hard all-roots performance target or
make exponential searches cheap. Implicit large-domain hard-family search
retains its separate, unresolved research and performance obligations.
