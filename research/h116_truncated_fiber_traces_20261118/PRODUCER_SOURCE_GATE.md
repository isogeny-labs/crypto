# H116 producer source gate

No H116 cohort moment, predicate score or timing was computed before this gate.

`h116_field.rs` is a byte-identical reuse of the H115 exact extension-field and
elliptic-curve engine at crypto revision `10d380c5df`. Both the original blob
and H116 copy have SHA-256:

```text
577015b44fae9828b9bddf1b2afc0403e4f44d732b3a01325f42253f0982cc82
```

`h116_producer.rs` implements the fixed-length Bostan recurrence, formal-series
inversion/logarithmic derivative, direct fiber enumeration, identical A/A,
λ=2 coordinate control, operation counters and first-mismatch stopping rule.
Its SHA-256 is:

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

The self-test uses only the excluded H115 `(p,a,b,r,d)=(31,1,19,37,4)` fixture.
It requires:

- rejection of the p=13 characteristic-boundary fixture;
- exact equality between recurrence coefficients and dense kernel Vélu
  coefficients;
- exact equality of all eight relative X and eight absolute Y moments against
  the 37-point direct fiber for target `[2]G`;
- the same equality after the λ=2 coordinate isomorphism; and
- a complete 37-squared torsion-table control.

Optimized self-test stdout was `h116 producer selftest: pass`; it exited zero in
0.238s user CPU. The source and binary bindings must be committed and pushed
before `--run` is invoked on `COHORT_V2.json`.

The pre-data development test initially omitted the normalization `U=a^2u` and
failed the p=31 comparison. That attempt is retained in the session log. The
corrected source above passed without changing the frozen H116 cohort, metrics
or decision rule.
