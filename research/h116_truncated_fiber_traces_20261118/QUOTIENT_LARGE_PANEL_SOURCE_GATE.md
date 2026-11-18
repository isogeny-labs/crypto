# H116 fixed-ratio large-panel source gate

Protocol `H116-QUOTIENT-LARGE-PANEL-001`, deterministic curve selection,
producer, scorer, includes and launch manifest were frozen before generating
any moment on the new curves.

## Frozen hashes

```text
ff6d6c05d3c2e01cce2b41577f8229a6dcc9039509190fa28ef79f4827e93e0e  QUOTIENT_LARGE_PANEL_PROTOCOL.md
495d42b4ade27674ff68c1090355b4c21ee6a4f36d2ddfbd9c30968c88e5a27f  QUOTIENT_LARGE_PANEL_LAUNCH.json
543ac46354f6ef3693ad2a49170fe272617a2bbbd5599367adfbb5b2f4481570  h116_quotient_large_producer.rs
4b0b608893215ec248516de0b6b9d14a67a56a5483c1de2eb10d38f34cf8f182  h116_quotient_large_scorer.rs
e5f508ffb20586f500b0c54c8f6529a6157893a339f9dc42cdd5356c689ecdc8  h116_producer.rs
577015b44fae9828b9bddf1b2afc0403e4f44d732b3a01325f42253f0982cc82  h116_field.rs
52b28350a37483547165cd6b7efc115005aa3453b1fa802ad1cb85f8177207a7  h116_quotient_digit_scorer.rs
2bafe271f1b2f6e6eb1a100f672f81e2b16bbe30b6db29ac579c0740dfe9eddf  ../../src/hash/sha256.rs
```

## Pre-generation build gate

Both sources compiled with the pinned Rust 1.100.0 compiler under
`--edition=2021 -O -D warnings`. Their self-tests and binary hashes were:

```text
h116 quotient large producer selftest: pass curves=3 targets=34856
h116 quotient large scorer selftest: pass
d19254193287d64005cbb068b422e14c7381a686e8ff4b625159ebed472bcd44  /tmp/h116_quotient_large_producer
477ab345ee169af9bc4827610da613b5c7123020ef371cbeff5a869c22a46b69  /tmp/h116_quotient_large_scorer
```

The producer self-test independently recomputes the point counts and verifies
that each frozen coefficient is the first nonsingular positive coefficient
giving prime group order. No new-panel moment, quotient rank, path statistic or
shuffle score existed when this gate was written. The first permissible run is
from the first pushed commit containing these exact bytes.
