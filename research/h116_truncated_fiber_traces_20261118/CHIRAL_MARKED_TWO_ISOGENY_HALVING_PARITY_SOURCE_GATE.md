# H116 chiral marked two-isogeny halving parity source gate

Protocol `H116-CHIRAL-MARKED-TWO-ISOGENY-HALVING-PARITY-001`, ordered marked
quartic models, source/dual kernel-halving bits, complete affine Boolean
family, binary recursion, inherited branch/resultant/genus-one controls,
resolvent and two-isogeny coefficient identities, exhaustive group-law and
eliminated halving controls, point-count controls, shuffle statistic, sources
and launch manifest were frozen before evaluating any halving tuple against a
scalar label.

```text
85e08eae6fa9f911ba9a4e9e7101eebdf6b2e05d7739a5736f04a4d590359d3e  CHIRAL_MARKED_TWO_ISOGENY_HALVING_PARITY_PROTOCOL.md
cff82b8f77826b5f49aa14a00b37b13f096c88a5f8188a33b2334fb6548b5e40  CHIRAL_MARKED_TWO_ISOGENY_HALVING_PARITY_LAUNCH.json
46a78d6e20812e540c72e3bb2a5422b9f214d3fcafa895e652fc7ec79947ea01  h116_chiral_branch_residual_parity.rs
e5f508ffb20586f500b0c54c8f6529a6157893a339f9dc42cdd5356c689ecdc8  h116_producer.rs
577015b44fae9828b9bddf1b2afc0403e4f44d732b3a01325f42253f0982cc82  h116_field.rs
2bafe271f1b2f6e6eb1a100f672f81e2b16bbe30b6db29ac579c0740dfe9eddf  ../../src/hash/sha256.rs
```

Pre-run build gate:

```text
h116 chiral branch residual parity selftest: pass rules=512 resultant_rules=812 trace_rules=243 invariant_rules=512 kummer_rules=512
d2478ff1c67063b70ff894c77e89b6b7470d37295a3818eb1fafb82b27fc7b98  /tmp/h116_chiral_marked_two_isogeny_halving_parity
```

The source compiled with pinned Rust 1.100.0 under
`--edition=2021 -O -D warnings`.  No target halving tuple, selected mask or
shuffle score existed when this gate was written.  The first permissible run
is from the first pushed commit containing these exact bytes.
