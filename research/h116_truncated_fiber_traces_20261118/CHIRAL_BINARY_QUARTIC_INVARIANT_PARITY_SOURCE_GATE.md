# H116 chiral binary-quartic invariant parity source gate

Protocol `H116-CHIRAL-BINARY-QUARTIC-INVARIANT-PARITY-001`, ordered quartic
models and `I,J` characters, complete affine Boolean family, binary recursion,
inherited branch/resultant/genus-one controls, independent quartic
discriminant and `4I^3-J^2=27 disc(f)` identity, shuffle statistic, sources and
launch manifest were frozen before evaluating any invariant tuple against a
scalar label.

```text
5e5df4112b91436c965ae0aa124aa7386626e34dac71be468c5188c7f9e6675e  CHIRAL_BINARY_QUARTIC_INVARIANT_PARITY_PROTOCOL.md
457d02312e045c7e205ce97120739bc30b3ddd66248a4d86a0dc8f5a44b1379a  CHIRAL_BINARY_QUARTIC_INVARIANT_PARITY_LAUNCH.json
cbc63b64ea5810fe9f385454022d7e831766cc8476582229b624ca184768610d  h116_chiral_branch_residual_parity.rs
e5f508ffb20586f500b0c54c8f6529a6157893a339f9dc42cdd5356c689ecdc8  h116_producer.rs
577015b44fae9828b9bddf1b2afc0403e4f44d732b3a01325f42253f0982cc82  h116_field.rs
2bafe271f1b2f6e6eb1a100f672f81e2b16bbe30b6db29ac579c0740dfe9eddf  ../../src/hash/sha256.rs
```

Pre-run build gate:

```text
h116 chiral branch residual parity selftest: pass rules=512 resultant_rules=812 trace_rules=243 invariant_rules=512
209acc714f1acf3c020c4e4844dd5a848313a6258a57be84c0395ea687fca1db  /tmp/h116_chiral_binary_quartic_invariant_parity
```

The source compiled with pinned Rust 1.100.0 under
`--edition=2021 -O -D warnings`.  No target invariant tuple, selected mask or
shuffle score existed when this gate was written.  The first permissible run
is from the first pushed commit containing these exact bytes.
