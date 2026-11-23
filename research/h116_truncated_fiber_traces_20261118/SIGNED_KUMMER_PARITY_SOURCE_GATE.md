# H116 signed-y Kummer parity source gate

Protocol `H116-SIGNED-KUMMER-PARITY-001`, the nonsymmetric cover family,
selection statistic, shuffle control, sources and manifest were frozen before
computing any signed-y cover signature.

```text
9a427bcb561a8af3088b343ada5601f5fcb7cfc332bbb8dc2c92afd9bd57a679  SIGNED_KUMMER_PARITY_PROTOCOL.md
159d610449a18ad96eba135839c35b2bd347fe72b9e1d72ac7ac4c47be9313d5  SIGNED_KUMMER_PARITY_LAUNCH.json
4ed68572e94336fb76e06a1ea9ca93f38260ea0d49a34ea70fa9b0e2a486dc1d  h116_signed_kummer_parity.rs
e5f508ffb20586f500b0c54c8f6529a6157893a339f9dc42cdd5356c689ecdc8  h116_producer.rs
577015b44fae9828b9bddf1b2afc0403e4f44d732b3a01325f42253f0982cc82  h116_field.rs
2bafe271f1b2f6e6eb1a100f672f81e2b16bbe30b6db29ac579c0740dfe9eddf  ../../src/hash/sha256.rs
```

Pre-run build gate:

```text
h116 signed Kummer parity selftest: pass rules=184
9c092f9d7ed6f49dc9f00519f1d21153828a21980115cbf0eb89628944edcdd8  /tmp/h116_signed_kummer_parity
```

The source compiled with pinned Rust 1.100.0 under
`--edition=2021 -O -D warnings`. No signed-y signature, selected subset or
shuffle score existed when this gate was written. The first permissible run is
from the first pushed commit containing these exact bytes.
