# H116 chiral genus-three Prym digit source gate

Protocol `H116-CHIRAL-GENUS3-PRYM-DIGIT-001`, exact elliptic-factor removal,
Prym middle coefficient and real-Weil discriminant, complete affine ternary
families, inherited exact oracle controls, new factorization/Weil controls,
shuffle statistic, sources and launch manifest were frozen before evaluating
any Prym digit score.

```text
463eef2fe9b9a1ba7ca7fec58b66c641699344642e0f41d755aaa22150fba099  CHIRAL_GENUS3_PRYM_DIGIT_PROTOCOL.md
a2a9cb939a76d57e15e71d117fdf63c46748f82c70afa9ed98f64f87e895c387  CHIRAL_GENUS3_PRYM_DIGIT_LAUNCH.json
639777e93bba3919420ad4e78d56753dd07879751bc5aa667d5acb2298a3acab  h116_chiral_genus3_zeta2_digit.rs
e5f508ffb20586f500b0c54c8f6529a6157893a339f9dc42cdd5356c689ecdc8  h116_producer.rs
577015b44fae9828b9bddf1b2afc0403e4f44d732b3a01325f42253f0982cc82  h116_field.rs
2bafe271f1b2f6e6eb1a100f672f81e2b16bbe30b6db29ac579c0740dfe9eddf  ../../src/hash/sha256.rs
```

Pre-run build gate:

```text
h116 chiral genus-three zeta2 digit selftest: pass rules=486
a3fc0b04ffcfa6510ff708f50e6fca344deaf63c4b664f3823f4b7b0127f0cc7  /tmp/h116_chiral_genus3_prym_digit
```

The source compiled with pinned Rust 1.100.0 under
`--edition=2021 -O -D warnings`.  No Prym rule or score existed when this gate
was written.  The first permissible run is from the first pushed commit
containing these exact bytes.
