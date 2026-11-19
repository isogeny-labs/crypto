# H116 Prym Frobenius-splitting parity source gate

Protocol `H116-PRYM-FROBENIUS-SPLITTING-PARITY-001`, auxiliary primes, bit
convention, ordered feature tuple, complete affine Boolean family, binary
recursion, inherited oracle controls, direct polynomial-root control, shuffle
statistic, sources and launch manifest were frozen before evaluating any
splitting bit.

```text
e86291efb1448d3847dca371f7dffcac4a7665a8dbae154d86984d547f631ce9  PRYM_FROBENIUS_SPLITTING_PARITY_PROTOCOL.md
5ec86cde97ef3ba5df0ddaaf5ee35d2c29fa422b4ac54ba8953e21039bdecac1  PRYM_FROBENIUS_SPLITTING_PARITY_LAUNCH.json
5aae5ea3e95b0a53720d51a12079aa662ad7881701e2bfc640e7f8ffa0365d26  h116_chiral_genus3_zeta2_digit.rs
e5f508ffb20586f500b0c54c8f6529a6157893a339f9dc42cdd5356c689ecdc8  h116_producer.rs
577015b44fae9828b9bddf1b2afc0403e4f44d732b3a01325f42253f0982cc82  h116_field.rs
2bafe271f1b2f6e6eb1a100f672f81e2b16bbe30b6db29ac579c0740dfe9eddf  ../../src/hash/sha256.rs
```

Pre-run build gate:

```text
h116 chiral genus-three zeta2 digit selftest: pass ternary_rules=486 binary_rules=512
232febdd3319d42e6ea5f48e8ee4ecc434d46026f2c41977ce318e756dde57e0  /tmp/h116_chiral_genus3_prym_splitting
```

The source compiled with pinned Rust 1.100.0 under
`--edition=2021 -O -D warnings`.  No auxiliary-prime splitting tuple,
selected mask or shuffle score existed when this gate was written.  The first
permissible run is from the first pushed commit containing these exact bytes.
