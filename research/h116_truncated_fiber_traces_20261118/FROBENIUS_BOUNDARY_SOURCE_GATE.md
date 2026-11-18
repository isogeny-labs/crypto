# H116 Frobenius-boundary source gate

Protocol `H116-FROBENIUS-BOUNDARY-001`, its `[2]([3])` producer and its
division-polynomial verifier were compiled before execution. No boundary result
file existed when this receipt was written. [pcc:unverified]

```text
4beb56127583e78cfb9715abc407f85c5567fe4353a1b3aae268e006e9d258fe  FROBENIUS_BOUNDARY_PROTOCOL.json
27a79a028c19d00579b17e5f2c78cfe7ec1a6d9d8f81572ba6bb9f76a3adb054  FROBENIUS_BOUNDARY_PROTOCOL.md
73dbc70e5642a1c64b89fc76903e97c509b57f9f10c65ee63f1df031c6ffd678  h116_frobenius_boundary.rs
c4af847dd450b9ce1b2893fe35f1123cc02f407ccf5745011eca43607ea4a52d  h116_frobenius_boundary_verify.rs
```

Both sources compile with the pinned workspace Rust toolchain, optimization and
warnings denied. The retained degree-37 map coefficients are copied exactly
from `MAP_CERTIFICATE_CONTROL.json`; the producer and verifier reconstruct the
degree-36 scalar map by different formulas. [pcc:unverified]

