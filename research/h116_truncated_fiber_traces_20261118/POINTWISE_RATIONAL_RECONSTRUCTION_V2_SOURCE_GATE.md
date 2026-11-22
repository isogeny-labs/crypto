# H116 pointwise rational reconstruction v2 source gate

This gate seals the corrected joint-interpolation implementations before the
first affected v2 execution.

```text
2db6297b935f4d5edf4c5976205e7aa3eed1252d26f4602cf14a9a08b1854153  POINTWISE_RATIONAL_RECONSTRUCTION_V2_PROTOCOL.json
194064065ae955ea53c5b2b60ae95c8837cbc8079bf271881596a6a92e7f6767  h116_pointwise_rational_reconstruction_v2.rs
354ab9975e8fe8bf2cea2e3704e2aade762c2824465e0fd7f6ec11d365da43ec  h116_pointwise_rational_reconstruction_v2_verify.py
577015b44fae9828b9bddf1b2afc0403e4f44d732b3a01325f42253f0982cc82  h116_field.rs
469ef0cd5ca96329a9543b4f1afbc55f0c58027e927151dbe56429917df2126c  h116_pointwise_charpoly_verify.py
```

The producer compiled with Rust 1.100.0 under
`--edition=2021 -O -D warnings`; the independent verifier compiled with Python
3.12.14.  Both passed four frozen eigenspace fixtures covering the observed
v1 splitting patterns.  No v2 panel command has run and no v2 result exists at
this gate.  `rustfmt` is unavailable, so no formatting-tool claim is made.
[pcc:unverified]
