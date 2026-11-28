# H116 chiral marked two-isogeny halving parity source gate

Protocol `H116-CHIRAL-MARKED-TWO-ISOGENY-HALVING-PARITY-002`, ordered marked
quartic models, source/dual kernel-halving bits, complete affine Boolean
family, binary recursion, inherited branch/resultant/genus-one controls,
resolvent and two-isogeny coefficient identities, exhaustive group-law and
eliminated halving controls, point-count controls, shuffle statistic, sources
and launch manifest were frozen before evaluating any halving tuple against a
scalar label.  The preserved `-001` preflight stopped on singular quartics
before rule construction or label scoring; `-002` adopts the inherited
singular-model abstention convention and changes no smooth-model observable.

```text
3fa910e5cd84a5b281c81d4d0b40bb6f0f4cc437c26e95ce978571d0d8af615b  CHIRAL_MARKED_TWO_ISOGENY_HALVING_PARITY_PROTOCOL.md
7027fd16d5d691f6c0807d633d1571a278c89350b64928c7db752d76b8d60eb4  CHIRAL_MARKED_TWO_ISOGENY_HALVING_PARITY_LAUNCH.json
837b1727ac55c4df6429d84e38f8d283869326663ecbb81b47f485f789b08587  CHIRAL_MARKED_TWO_ISOGENY_HALVING_PARITY_PREFLIGHT_FAILURE.md
b48493cda3173af581e375e5c61e90600c85f790d8c75c543023c865244a529d  h116_chiral_branch_residual_parity.rs
e5f508ffb20586f500b0c54c8f6529a6157893a339f9dc42cdd5356c689ecdc8  h116_producer.rs
577015b44fae9828b9bddf1b2afc0403e4f44d732b3a01325f42253f0982cc82  h116_field.rs
2bafe271f1b2f6e6eb1a100f672f81e2b16bbe30b6db29ac579c0740dfe9eddf  ../../src/hash/sha256.rs
```

Pre-run build gate:

```text
h116 chiral branch residual parity selftest: pass rules=512 resultant_rules=812 trace_rules=243 invariant_rules=512 kummer_rules=512
7943a0a8cd3ecc89f3a60e4ec3074125c1318d2f41e0c52def1449ee5c4a1fb6  /tmp/h116_chiral_marked_two_isogeny_halving_parity
```

The source compiled with pinned Rust 1.100.0 under
`--edition=2021 -O -D warnings`.  No target halving tuple, selected mask or
shuffle score existed when this corrected gate was written.  The first
permissible scored run is from the first pushed commit containing these exact
bytes.
