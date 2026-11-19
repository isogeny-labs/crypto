# H116 chiral genus-three trace digit source gate

Protocol `H116-CHIRAL-GENUS3-TRACE-DIGIT-001`, related-work boundary,
chiral cover family, discriminant gate, ternary rule family, conditional
recursion, Weil/count/rescaling/swap/planted controls, shuffle statistic,
sources and launch manifest were frozen before evaluating any chiral-cover
trace.

```text
ab4f6eabe4f2cb1025ba7c380199c90ca31cadffb378f2fa1d3d8a115be5f6b4  CHIRAL_GENUS3_TRACE_DIGIT_PROTOCOL.md
2093f37286873ea254ce53b0ffdb3b378a41f45cb4e7cfab36f22d751cb32150  CHIRAL_GENUS3_TRACE_DIGIT_LAUNCH.json
8d01318eacffaa730ab60e52223b173f58e0688cce2bfc777df83736342da9ae  h116_chiral_genus3_trace_digit.rs
e5f508ffb20586f500b0c54c8f6529a6157893a339f9dc42cdd5356c689ecdc8  h116_producer.rs
577015b44fae9828b9bddf1b2afc0403e4f44d732b3a01325f42253f0982cc82  h116_field.rs
2bafe271f1b2f6e6eb1a100f672f81e2b16bbe30b6db29ac579c0740dfe9eddf  ../../src/hash/sha256.rs
```

Pre-run build gate:

```text
h116 chiral genus-three trace digit selftest: pass rules=243
71c1ea1f19c8c42f07bf99670fd631a102612ac95811895b21fe54963abefd0e  /tmp/h116_chiral_genus3_trace_digit
```

The source compiled with pinned Rust 1.100.0 under
`--edition=2021 -O -D warnings`.  No chiral-cover trace, selected rule or
shuffle score existed when this gate was written.  The first permissible run
is from the first pushed commit containing these exact bytes.
