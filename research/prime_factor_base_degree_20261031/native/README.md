# Prime bivariate certificate and pilot exporter

```sh
cargo test --release --locked --manifest-path research/prime_factor_base_degree_20261031/native/Cargo.toml
cargo build --release --locked --manifest-path research/prime_factor_base_degree_20261031/native/Cargo.toml
```

Input:

```json
{"p":17,"max_degree":4,"polynomials":[[{"x":0,"y":0,"c":1},{"x":2,"y":0,"c":1}],[{"x":0,"y":0,"c":1},{"x":1,"y":0,"c":1},{"x":2,"y":0,"c":1}]]}
```

A term represents `c*x^x*y^y`, with canonical coefficients in 1..p−1,
ordinary exponents and sorted unique exponent pairs. Empty polynomial lists
represent zero. The example is an artificial UNSAT unit fixture, not a
summation-polynomial result.

After experiment admission:

```sh
prime-degree-certificate prepare SPEC.json FRESH-DIRECTORY
prime-degree-certificate certify SYSTEM.json FRESH-CERTIFICATE.json
prime-degree-certificate verify SYSTEM.json CERTIFICATE.json
```

Certificates bind input bytes by SHA-256 and certify bounded-Macaulay
refutation degree with upper/lower identities. Field size is limited to
checked primes 5..65521, degree24 and 4,096 generated rows. Inputs are
bivariate, with ordinary polynomial multiplication; no Boolean quotient
is used. All JSON schemas reject unknown fields. Certificate outputs are
write-once and replayed before publication.

`prepare` is a small control exporter, limited to p≤257 and 2..12 abscissae.
It requires a prime-order nonsingular full curve group, constructs the
shifted-power intersection after rational-lift filtering, and exports
size-matched low-x and hash controls. Target representatives are one per
± pair; exact two-point references include both signs and repeated points.
Its metadata explicitly marks canonical identity registration pending. It
does not produce an accepted FB1 object or a structured-chain system.

The maximum degree field is a research stop bound. No refutation through
that bound is unresolved, even when the ideal is inconsistent. It is not
a lower bound on iterative F4 solving degree. The checker does not report
homogeneous regularity or claim an elliptic-curve improvement.
