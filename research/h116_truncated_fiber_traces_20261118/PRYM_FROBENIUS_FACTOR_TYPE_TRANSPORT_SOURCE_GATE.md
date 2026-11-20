# H116 Prym Frobenius factor-type transport source gate

Protocol `H116-PRYM-FROBENIUS-FACTOR-TYPE-TRANSPORT-001`, auxiliary primes,
factor-type codebook, ordered categorical tuple, fixed leave-one-curve-out
transport decoder, binary recursion, inherited oracle controls, exact monic
factorization/reconstruction/irreducibility/evaluation controls, shuffle
statistic, sources and launch manifest were frozen before evaluating any
factor-type tuple against a scalar label.

```text
b7238254dd673ffbe126d025ca0d742e80b7e56537cef53cc261ebecbf5ef966  PRYM_FROBENIUS_FACTOR_TYPE_TRANSPORT_PROTOCOL.md
706b67b0b33416bf2662394fda81b4a9ce533c3c7292e41ee7b6dfcd8d55f67f  PRYM_FROBENIUS_FACTOR_TYPE_TRANSPORT_LAUNCH.json
bb225d0ae85209c6028e03dbf13375ff1a29b09d13b3a6ef5358036dcb1441d6  h116_chiral_genus3_zeta2_digit.rs
e5f508ffb20586f500b0c54c8f6529a6157893a339f9dc42cdd5356c689ecdc8  h116_producer.rs
577015b44fae9828b9bddf1b2afc0403e4f44d732b3a01325f42253f0982cc82  h116_field.rs
2bafe271f1b2f6e6eb1a100f672f81e2b16bbe30b6db29ac579c0740dfe9eddf  ../../src/hash/sha256.rs
```

Pre-run build gate:

```text
h116 chiral genus-three zeta2 digit selftest: pass ternary_rules=486 binary_rules=512
7a59dbdd1414de0cef419044cfdf0e5f7eb0c67328cee14c5cd9d8767ba314e7  /tmp/h116_chiral_genus3_prym_factor_type
```

The source compiled with pinned Rust 1.100.0 under
`--edition=2021 -O -D warnings`.  No target factor-type tuple, categorical
collision, transport prediction, or shuffle score existed when this gate was
written.  The first permissible run is from the first pushed commit containing
these exact bytes.
