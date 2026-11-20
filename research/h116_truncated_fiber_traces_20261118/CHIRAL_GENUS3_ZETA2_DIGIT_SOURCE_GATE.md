# H116 chiral genus-three second-zeta digit source gate

Protocol `H116-CHIRAL-GENUS3-ZETA2-DIGIT-001`, second-power-sum and
second-zeta observables, complete affine ternary families, exact extension
field census, count/Weil/Newton/rescaling/chirality/planted controls, shuffle
statistic, sources and launch manifest were frozen before enumerating any
extension-field point or evaluating any second-zeta score.

```text
14291abc774fdd389cfe8f209ade9013ed6f7ad79352df5928e88a3490bc36a1  CHIRAL_GENUS3_ZETA2_DIGIT_PROTOCOL.md
9f6269de798ee224375a16a6a2cf52f95ca758f6677ce85a93db4c540c117bd5  CHIRAL_GENUS3_ZETA2_DIGIT_LAUNCH.json
9eea726fb43aea703b3f1ef27e440d4a6fc750d0f57bb9339d3a33cf91b57af8  h116_chiral_genus3_zeta2_digit.rs
e5f508ffb20586f500b0c54c8f6529a6157893a339f9dc42cdd5356c689ecdc8  h116_producer.rs
577015b44fae9828b9bddf1b2afc0403e4f44d732b3a01325f42253f0982cc82  h116_field.rs
2bafe271f1b2f6e6eb1a100f672f81e2b16bbe30b6db29ac579c0740dfe9eddf  ../../src/hash/sha256.rs
```

Pre-run build gate:

```text
h116 chiral genus-three zeta2 digit selftest: pass rules=486
1c3a1a8b25b68c30dd959734b3ab53f8f945bb4ac97fe31c99644fccf84a26af  /tmp/h116_chiral_genus3_zeta2_digit
```

The source compiled with pinned Rust 1.100.0 under
`--edition=2021 -O -D warnings`.  No `F_(p^2)` curve census, second Frobenius
power sum, selected rule or shuffle score existed when this gate was written.
The first permissible run is from the first pushed commit containing these
exact bytes.
