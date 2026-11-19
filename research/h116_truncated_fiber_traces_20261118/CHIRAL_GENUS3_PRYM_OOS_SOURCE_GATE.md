# H116 fixed Prym-discriminant out-of-sample source gate

Protocol `H116-CHIRAL-GENUS3-PRYM-OOS-001`, three fresh curve specifications,
the single unchanged discriminant rule, complete inherited oracle controls,
fixed-rule shuffle decision criterion, sources and launch manifest were frozen
before enumerating any point or evaluating any Prym invariant on the OOS
curves.

```text
5e33a96961a9dbc53c337865574b6f3fdbccd92d1aa6cfb33c96544b2bb06ae5  CHIRAL_GENUS3_PRYM_OOS_PROTOCOL.md
efd01d515da4bcb57d579845b6ef9248a0407d1ecbf5dd262b43e335502a954a  CHIRAL_GENUS3_PRYM_OOS_LAUNCH.json
0250c8940fd15a9806cf759375eea162c845a781f029dc7d372fdc10cc552469  h116_chiral_genus3_zeta2_digit.rs
e5f508ffb20586f500b0c54c8f6529a6157893a339f9dc42cdd5356c689ecdc8  h116_producer.rs
577015b44fae9828b9bddf1b2afc0403e4f44d732b3a01325f42253f0982cc82  h116_field.rs
2bafe271f1b2f6e6eb1a100f672f81e2b16bbe30b6db29ac579c0740dfe9eddf  ../../src/hash/sha256.rs
```

Pre-run build gate:

```text
h116 chiral genus-three zeta2 digit selftest: pass rules=486
3da18783c985d7a1fe3b33b54abf6ace9a495e20016c8d10979c6a89365a9627  /tmp/h116_chiral_genus3_prym_oos
```

The source compiled with pinned Rust 1.100.0 under
`--edition=2021 -O -D warnings`.  The OOS path instantiates one rule even
though the shared selftest retains the 486-rule discovery-family census.  No
OOS point, invariant, match count or shuffle score existed when this gate was
written.  The first permissible run is from the first pushed commit
containing these exact bytes.
