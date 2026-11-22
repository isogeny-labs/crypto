# H116 Prym Frobenius-eigenline parity source gate

Protocol `H116-PRYM-FROBENIUS-EIGENLINE-PARITY-001`, auxiliary primes, bit
convention, ordered feature tuple, complete affine Boolean family, binary
recursion, inherited oracle controls, direct full-Weil root enumeration,
full/real Weil quotient identity, shuffle statistic, sources and launch
manifest were frozen before evaluating any eigenline bit against a scalar
label.

```text
a05904770c70ef5acdf538ae27b6deea7cde3a023d5e4aa6645b617a9653ff3e  PRYM_FROBENIUS_EIGENLINE_PARITY_PROTOCOL.md
9b256b3574f1623dbf6cc0ec2eca3b212cfb1796a714090ec8d2a8192a617877  PRYM_FROBENIUS_EIGENLINE_PARITY_LAUNCH.json
9b65b10fb67e8041a93ca5725c05c178e5f75743a70c77fa4a0d42cb61511489  h116_chiral_genus3_zeta2_digit.rs
e5f508ffb20586f500b0c54c8f6529a6157893a339f9dc42cdd5356c689ecdc8  h116_producer.rs
577015b44fae9828b9bddf1b2afc0403e4f44d732b3a01325f42253f0982cc82  h116_field.rs
2bafe271f1b2f6e6eb1a100f672f81e2b16bbe30b6db29ac579c0740dfe9eddf  ../../src/hash/sha256.rs
```

Pre-run build gate:

```text
h116 chiral genus-three zeta2 digit selftest: pass ternary_rules=486 binary_rules=512
f9807621e62feb5e895b9fe91363d429ef5144b67d34d86ea3f6a696a0af53ba  /tmp/h116_chiral_genus3_prym_eigenline
```

The source compiled with pinned Rust 1.100.0 under
`--edition=2021 -O -D warnings`.  No full-Weil eigenline tuple, selected mask,
or shuffle score existed when this gate was written.  The first permissible
run is from the first pushed commit containing these exact bytes.
