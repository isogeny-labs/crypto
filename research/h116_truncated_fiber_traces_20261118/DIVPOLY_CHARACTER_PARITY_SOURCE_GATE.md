# H116 fixed division-polynomial character source gate

Protocol `H116-DIVPOLY-CHARACTER-PARITY-001`, related-work boundary, fixed
indices, selection statistic, identity controls, shuffle control, sources and
manifest were frozen before evaluating any panel character.

```text
315e6a76c4ab0db7c301a1262d3a281ac3635f404f0d4858b0f092019dc5d559  DIVPOLY_CHARACTER_PARITY_PROTOCOL.md
b25aeb6bb646b790cf5fc7595c0b0b442808ab6eb20807dd1c3ad097e7f4b8f5  DIVPOLY_CHARACTER_PARITY_LAUNCH.json
4bd07bb6884db2acbd366da656d4d73324ab2a06822ab4521186d9ca480d4dfb  h116_divpoly_character_parity.rs
e5f508ffb20586f500b0c54c8f6529a6157893a339f9dc42cdd5356c689ecdc8  h116_producer.rs
577015b44fae9828b9bddf1b2afc0403e4f44d732b3a01325f42253f0982cc82  h116_field.rs
2bafe271f1b2f6e6eb1a100f672f81e2b16bbe30b6db29ac579c0740dfe9eddf  ../../src/hash/sha256.rs
```

Pre-run build gate:

```text
h116 division-polynomial character selftest: pass rules=184
c1f05f0acd20e9104a906af3fc9ad23f24451a3fc4999563ef5c6f8e77eb4225  /tmp/h116_divpoly_character_parity
```

The source compiled with pinned Rust 1.100.0 under
`--edition=2021 -O -D warnings`.  No panel character, selected subset or
shuffle score existed when this gate was written.  The first permissible run
is from the first pushed commit containing these exact bytes.
