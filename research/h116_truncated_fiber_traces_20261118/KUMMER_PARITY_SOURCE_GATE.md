# H116 quadratic-Kummer parity source gate

Protocol `H116-KUMMER-PARITY-001`, the exact cover family, selection statistic,
shuffle control, sources and launch manifest were frozen before computing any
Kummer signature on the discovery curves.

## Frozen hashes

```text
3c5eb4e6a6a6987c42c0b135066a371aab0926fa7694b1d1e6f162e2bdb98ea7  KUMMER_PARITY_PROTOCOL.md
069a58720ce396d75e4214c173f4eca7572eaca398d9c7a6da2f89271fbd1e35  KUMMER_PARITY_LAUNCH.json
daf91206631fbd26f400c4c0ebc4de26e46a6580c91a630640c281417c91e8eb  h116_kummer_parity.rs
e5f508ffb20586f500b0c54c8f6529a6157893a339f9dc42cdd5356c689ecdc8  h116_producer.rs
577015b44fae9828b9bddf1b2afc0403e4f44d732b3a01325f42253f0982cc82  h116_field.rs
2bafe271f1b2f6e6eb1a100f672f81e2b16bbe30b6db29ac579c0740dfe9eddf  ../../src/hash/sha256.rs
```

## Pre-run build gate

```text
/workspace/.h116-rustup/toolchains/1.100.0-x86_64-unknown-linux-gnu/bin/rustc \
  --edition=2021 -O -D warnings h116_kummer_parity.rs \
  -o /tmp/h116_kummer_parity
/tmp/h116_kummer_parity --selftest
h116 Kummer parity selftest: pass rules=184
d69a847546bb11d3d29688341a486708715e80afeefd219476ce7020d566de03  /tmp/h116_kummer_parity
```

The self-test checks the exact 184-rule census, subset-product zero semantics,
binary statistic, and equality of the bitset match count with its reference
case. No signature, selected subset or shuffled score existed when this gate
was written. The first permissible run is from the first pushed commit
containing these exact bytes.
