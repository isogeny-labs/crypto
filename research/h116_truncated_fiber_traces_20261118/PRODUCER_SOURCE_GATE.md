# H116 producer source gate

No H116 cohort moment, predicate score or timing was computed before this gate.

`h116_field.rs` is a byte-identical reuse of the H115 exact extension-field and
elliptic-curve engine at crypto revision `10d380c5df`. Both the original blob
and H116 copy have SHA-256:

```text
577015b44fae9828b9bddf1b2afc0403e4f44d732b3a01325f42253f0982cc82
```

The initial `h116_producer.rs` binding implemented the fixed-length Bostan recurrence, formal-series
inversion/logarithmic derivative, direct fiber enumeration, identical A/A,
λ=2 coordinate control, operation counters and first-mismatch stopping rule.
Its initial SHA-256 was:

```text
10b07f305cfba69a2a79f633db4e5035c0fab5d403e0a1f9a7ae4e66402e5a2c
```

Compiler:

```text
rustc 1.100.0 (0ef1bc386 2026-11-09)
host: x86_64-unknown-linux-gnu
LLVM version: 23.1.5
```

Strict debug and optimized builds succeeded. Binary SHA-256 values:

```text
c1362bd22f3c45d0ef9524babc9fb192351f3032b7dc76bb6c4aadbfd8c7d611  h116_producer.debug
65d3e1b3d59037566bcc6fb5ed8d9c477f9ca8f167793081a91de956fe1bc248  h116_producer.opt
```

Before any H116 moment was produced, source review found that the initial run
loop exposed `target_index` and used scalar order. That violates the frozen
blinding and seeded-order requirements even though the candidate function did
not accept a scalar. The initial binding remains above as a failed pre-data
gate. The corrected producer has two process-separated modes:

- `--prepare-targets` reads scalar indices and emits a shuffled blinded target
  manifest plus a separate scorer-only label map;
- `--run` accepts only the blinded coordinates and SHA-256 target digests. It
  validates the complete nonidentity subgroup census and never opens the label
  map.

The corrected source and binary SHA-256 values are:

```text
bbb313c169806ad0099290f161a3462a1171eb8e49b5923f40bcfcbf7a3cd778  h116_producer.rs
f081399945e967ea75c71b207910a9e6442daf8f5dd891f140dadbd3d2380063  h116_producer.debug
c8d180ff67f011b492e02e37f42c30f8825528b8891a746ee22f8db4825b34be  h116_producer.opt
```

The corrected self-test uses only synthetic controls and the excluded H115
`(p,a,b,r,d)=(31,1,19,37,4)` fixture. It requires:

- rejection of all p=5, p=7 and p=13 characteristic-boundary fixtures;
- 32/32 exact separable-root polynomial moment vectors and rejection of 32/32
  deterministic one-coordinate tampers;
- the FIPS SHA-256 `abc` known-answer vector and deterministic nonidentity
  target shuffle checks;
- exact equality between recurrence coefficients and dense kernel Vélu
  coefficients;
- exact equality of all eight relative X and eight absolute Y moments against
  the 37-point direct fiber for target `[2]G`;
- the same equality after the λ=2 coordinate isomorphism; and
- a complete 37-squared torsion-table control.

Corrected optimized self-test stdout was `h116 producer selftest: pass`; it
exited zero in 0.243s user CPU. The corrected source and binary bindings must be committed and pushed
before `--run` is invoked on `COHORT_V2.json`.

The first complete target-preparation attempt then revealed that the preparer
unnecessarily represented base-field targets in the extension field. It was
stopped with an incomplete, unhashed artifact and produced no moment. The
partial files were deleted. Target preparation alone now uses the exact
degree-one representation `F_p[x]/(x)`; the candidate and direct moment paths
are unchanged and continue to use the selected extension degree. Current
source and binary bindings are:

```text
e5f508ffb20586f500b0c54c8f6529a6157893a339f9dc42cdd5356c689ecdc8  h116_producer.rs
0f4cb9f7d36d5bdff00593515e2397ae30d021181d49961481af90ea720df101  h116_producer.debug
d46f3627db7b75142324b7c926e6840e90b25e9e0890ccb34e938161d6132263  h116_producer.opt
```

The current optimized self-test exited zero in 0.239s user CPU. Deterministic
preparation of all 4560 blinded targets exited zero in 0.023s user CPU and a
second preparation was byte-identical for both outputs.

The pre-data development test initially omitted the normalization `U=a^2u` and
failed the p=31 comparison. That attempt is retained in the session log. The
corrected source above passed without changing the frozen H116 cohort, metrics
or decision rule.
