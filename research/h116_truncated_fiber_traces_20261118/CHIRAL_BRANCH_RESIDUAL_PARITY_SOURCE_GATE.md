# H116 chiral branch-residual parity source gate

Protocol `H116-CHIRAL-BRANCH-RESIDUAL-PARITY-001`, target-tethered branch
cubic, residual quadratic and closed discriminant, multiplier/chirality order,
complete affine Boolean family, binary recursion, reconstruction,
discriminant-factorization, root-count, chord-sum and chiral-swap controls,
shuffle statistic, sources and launch manifest were frozen before evaluating
any residual-branch tuple against a scalar label.

```text
e276d3c87789fbd740f098485eedacdae64b99f37ca8b84960f747902448a70d  CHIRAL_BRANCH_RESIDUAL_PARITY_PROTOCOL.md
208ec154b5585f1a45efab9c896571896493f8b2e6a7b33aa706a874ced314c9  CHIRAL_BRANCH_RESIDUAL_PARITY_LAUNCH.json
b6f7bde25d6c42ac6b5135b8c7463ab80ecb955c7926eaca7f9404df6001e81b  h116_chiral_branch_residual_parity.rs
e5f508ffb20586f500b0c54c8f6529a6157893a339f9dc42cdd5356c689ecdc8  h116_producer.rs
577015b44fae9828b9bddf1b2afc0403e4f44d732b3a01325f42253f0982cc82  h116_field.rs
2bafe271f1b2f6e6eb1a100f672f81e2b16bbe30b6db29ac579c0740dfe9eddf  ../../src/hash/sha256.rs
```

Pre-run build gate:

```text
h116 chiral branch residual parity selftest: pass rules=512
625785da744d52c2dad68069d458774a155709b5443943af87a1d6d3047173dd  /tmp/h116_chiral_branch_residual_parity
```

The source compiled with pinned Rust 1.100.0 under
`--edition=2021 -O -D warnings`.  No target residual tuple, selected rule or
shuffle score existed when this gate was written.  The first permissible run
is from the first pushed commit containing these exact bytes.
