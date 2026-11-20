# H116 pointwise rational reconstruction source gate

This gate seals both implementations before the first affected execution of
`H116-POINTWISE-RATIONAL-RECONSTRUCTION-001`.

```text
1b837dfb420183e2ba0702060494687cdf78652f5fd4073d5a64c8af26d277e3  POINTWISE_RATIONAL_RECONSTRUCTION_PROTOCOL.json
8e5d83c35cf640e6e7caa491972de4cf06310a1f5645e3eafcf61b37db6ea676  h116_pointwise_rational_reconstruction.rs
e3abf4ba14ef94b6fb06ce4c7817c51ca085ce63cfbd1c8bf5c5f2f49b52e247  h116_pointwise_rational_reconstruction_verify.py
577015b44fae9828b9bddf1b2afc0403e4f44d732b3a01325f42253f0982cc82  h116_field.rs
469ef0cd5ca96329a9543b4f1afbc55f0c58027e927151dbe56429917df2126c  h116_pointwise_charpoly_verify.py
```

The Rust producer compiled with Rust 1.100.0 using
`--edition=2021 -O -D warnings` and passed its exact linear-interpolation
selftest.  The independent verifier compiled with Python 3.12.14 and passed a
separate implementation of the same fixture.  No affected panel command has
run and no result file exists at this gate.  `rustfmt` is unavailable, so no
formatting-tool claim is made. [pcc:unverified]
