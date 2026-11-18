# H116 rational-map certificate control

Status: excluded-fixture proof/control only. No H116 target or label file was
read, and the frozen producer was not changed or invoked with `--run`.

## Question

Can a full normalized Vélu rational map on the excluded H115 `p=31`, `r=37`
fixture emit the finite reversed numerator/denominator certificate now accepted
by the H116 Lean proof, and does that certificate reproduce the first eight
recurrence coefficients? [pcc:unverified]

## Construction

`h116_map_certificate.rs` independently reconstructs `ker(V-1)` over
`F_(31^4)`, chooses one point from each `{Q,-Q}` pair and builds

```text
D(x) = product_Q (x-x(Q))²,
R(x) = x + sum_Q [v(Q)/(x-x(Q)) + u(Q)/(x-x(Q))²] = N(x)/D(x).
```

It reverses `N,D` at infinity, checks the Vélu codomain against the
`(t-1)`-scaled source curve, evaluates the cleared identity used by
`NormalizedRationalMapPolynomialCertificate`, divides the reversed series
through degree eight and compares that prefix with the frozen recurrence.
The negative control adds one to reversed numerator coefficient two and
requires the cleared identity to fail. [pcc:unverified]

`h116_map_certificate_verify.py` is a separate Python standard-library
implementation of polynomial arithmetic, the certificate identity, series
division, the recurrence and the same mutation control. It consumes only the
retained JSON result; it does not call the Rust source. [pcc:unverified]

## Commands

```text
PATH=/workspace/.h116-cargo/bin:$PATH \
RUSTUP_HOME=/workspace/.h116-rustup \
CARGO_HOME=/workspace/.h116-cargo \
rustc --edition=2021 -O -D warnings h116_map_certificate.rs \
  -o /tmp/h116_map_certificate
/tmp/h116_map_certificate
python3 -m py_compile h116_map_certificate_verify.py
python3 h116_map_certificate_verify.py MAP_CERTIFICATE_CONTROL.json
```

The current Rust source also passes `rustfmt --edition 2021 --check` under the
repository's installed stable formatter. [pcc:unverified]

## Result

The retained record is `MAP_CERTIFICATE_CONTROL.json`.

| measure | value |
| --- | ---: |
| extension states examined | 33 |
| kernel half-set size | 18 |
| numerator degree | 37 |
| denominator degree | 36 |
| extension additions / multiplications / inversions | 3,883 / 5,077 / 35 |
| base additions / multiplications | 138,376 / 122,844 |
| recorded construction time | 868,899 ns |
| recorded identity/prefix/control time | 182,635 ns |

The Vélu codomain matched `(Atilde,Btilde)=(25,19)`, the cleared identity
passed, the independently recomputed quotient prefix and recurrence both equal

```text
[1, 0, 20, 0, 13, 7, 2, 4, 8],
```

and the one-coefficient mutation was rejected by both implementations.
[pcc:unverified]

Evidence SHA-256 values:

```text
492d6ee838009672ad81059f59a03cacdd84577079785683f7ff8db93cec8901  h116_map_certificate.rs
704c1fded6772f3e561840201e1ced17727dc5f154c59df79adbd7aa7b59a017  h116_map_certificate_verify.py
74aa7a18de613499e9d8e2ea31114f89203f02876f0e66016ec46404c8896bb0  MAP_CERTIFICATE_CONTROL.json
```

## Boundary

This is one excluded toy fixture, not an admitted H116 measurement. Constructing
the full degree-`r` map costs degree-`r` data and is therefore a proof/control
path, not the fixed-length candidate algorithm. The experiment validates the
new certificate interface and the recurrence on one actual Vélu map; it does
not establish a subexponential ECDLP algorithm, the frozen H116 cohort result,
or the still-open fast construction of the required low jet. [pcc:unverified]
