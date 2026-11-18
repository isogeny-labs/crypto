# H116 curve-coordinate source gate

Protocol `H116-CURVE-COORDINATE-001` was frozen before execution. No result
file existed when this receipt was written. [pcc:unverified]

Source hashes:

```text
d8db7bb580c47343cf24f919a41d6c88abf7e25cbce6dce5baf75a438ce1f8dd  CURVE_COORDINATE_PROTOCOL.json
4f8111c3b85e9d15e26c9a772a63e8f3ecf53743472be0d27f73672fa6b1fcac  CURVE_COORDINATE_PROTOCOL.md
8f57ec8b5948dabfa8f3af62c41b577d26598d18a074cd2dcd160acd642ff556  h116_curve_coordinate_control.rs
1e5facd4ec26a182892e6fa643ba2142bba439eaca0d5ae1d4d13b6553bd6ed7  h116_curve_coordinate_verify.rs
```

Both programs compiled with the pinned workspace Rust toolchain before this
gate was committed:

```sh
RUSTUP_HOME=/workspace/.h116-rustup \
CARGO_HOME=/workspace/.h116-cargo \
/workspace/.h116-cargo/bin/rustc --edition=2021 -O -D warnings \
  h116_curve_coordinate_control.rs -o /tmp/h116_curve_coordinate_control

RUSTUP_HOME=/workspace/.h116-rustup \
CARGO_HOME=/workspace/.h116-cargo \
/workspace/.h116-cargo/bin/rustc --edition=2021 -O -D warnings \
  h116_curve_coordinate_verify.rs -o /tmp/h116_curve_coordinate_verify
```

The canonical experiment-protocol skill file in the sibling checkout is
VLT1-encrypted and cannot be read in this runtime. This receipt therefore uses
the repository's existing H116 native freeze convention and discloses that
limitation. [pcc:unverified]
