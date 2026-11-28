# H116 Miller-doubling Kummer parity source gate

Protocol `H116-MILLER-KUMMER-PARITY-001`, the structured Miller-function
family, selection statistic, shuffle control, sources and manifest were frozen
before computing any Miller signature.

```text
6c941a29a7c4646a77f0d7910ce056e27085e649b769ff357154b07cab8d0785  MILLER_KUMMER_PARITY_PROTOCOL.md
c696e90fe0507362bd19f514ba08dc9644a914a2e6d0ac17aa967b36fe5be000  MILLER_KUMMER_PARITY_LAUNCH.json
f040bef5e1dc969b6cbf6bb57f3d667da4472bf194dd259ae4c066d02cf40a5d  h116_miller_kummer_parity.rs
e5f508ffb20586f500b0c54c8f6529a6157893a339f9dc42cdd5356c689ecdc8  h116_producer.rs
577015b44fae9828b9bddf1b2afc0403e4f44d732b3a01325f42253f0982cc82  h116_field.rs
2bafe271f1b2f6e6eb1a100f672f81e2b16bbe30b6db29ac579c0740dfe9eddf  ../../src/hash/sha256.rs
```

Pre-run build gate:

```text
h116 Miller Kummer parity selftest: pass rules=184
893edd4509db9e04f4b1b9c5e41b4836fc2450e33b9ed474f6ae30baad4e87d5  /tmp/h116_miller_kummer_parity
```

The source compiled with pinned Rust 1.100.0 under
`--edition=2021 -O -D warnings`. No Miller signature, selected subset or
shuffle score existed when this gate was written. The first permissible run is
from the first pushed commit containing these exact bytes.
