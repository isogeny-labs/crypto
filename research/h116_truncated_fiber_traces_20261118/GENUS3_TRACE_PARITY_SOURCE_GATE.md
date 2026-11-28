# H116 genus-three cover trace parity source gate

Protocol `H116-GENUS3-TRACE-PARITY-001`, Pila complexity boundary, cover
family, squarefreeness gate, affine rule family, Weil/rescaling/planted
controls, shuffle statistic, sources and launch manifest were frozen before
evaluating any cover trace.

```text
adb308e27e2c65ca70b1d68441fd097f0a006f2ccf98da8c4461e0d02967eedc  GENUS3_TRACE_PARITY_PROTOCOL.md
122fc5ef219175384769d8fb5ebba3dfca8826ff83552f0e8bab2babdc847f58  GENUS3_TRACE_PARITY_LAUNCH.json
3c345b70a0d0d8380e0af77febe06c69e5957d7eb653f7a6544361f8a0e373ba  h116_genus3_trace_parity.rs
e5f508ffb20586f500b0c54c8f6529a6157893a339f9dc42cdd5356c689ecdc8  h116_producer.rs
577015b44fae9828b9bddf1b2afc0403e4f44d732b3a01325f42253f0982cc82  h116_field.rs
2bafe271f1b2f6e6eb1a100f672f81e2b16bbe30b6db29ac579c0740dfe9eddf  ../../src/hash/sha256.rs
```

Pre-run build gate:

```text
h116 genus-three trace parity selftest: pass rules=512
212383dc49e89f98ac7ce99f11a757de9c734e00c1bf29475ab2f642f186365a  /tmp/h116_genus3_trace_parity
```

The source compiled with pinned Rust 1.100.0 under
`--edition=2021 -O -D warnings`.  No cover trace, selected rule or shuffle
score existed when this gate was written.  The first permissible run is from
the first pushed commit containing these exact bytes.
