# Native algebraic identity record

This small Rust client uses the repository's unchanged native
`src/cryptanalysis/identity_certificate.rs` and SHA256 implementation. Their
source hashes are retained in `source-pins.sha256`. It constructs the expression
trees for three rotations and checks both coordinates through the identity

```
z * first(S^3(a,b)) + second(S^3(a,b)) = z*a+b.
```

The record is **evaluated tier** over `F_(2^61-1)`, as the v1 format requires.
It is not the 3-adic coefficient ring or the F67 label field, and does not
certify packing, locality, precision or complexity. There is separately a
checked arbitrary-additive-group Lean theorem `TernaryPacket.rot_thrice`;
the expression-tree serialization bridge is not formalized, so no proved-tier
pointer is asserted by the certificate.

The recorded runs used official Rust 1.99.1, now pinned in
`rust-toolchain.toml`; package versions are locked. From this directory:

```sh
sha256sum -c source-pins.sha256
cargo run --release --locked -- issue ../rotation.identity.json
cargo run --release --locked -- check ../rotation.identity.json 86420319
cargo test --release --locked
```

The committed certificate has 64 statement-derived points; verification adds
64 points from a domain-separated checker stream. The streams are deterministic,
not an independent probabilistic proof. Ordinary Lean kernel checking supplies
the separate general algebraic proof. No cryptographic workload or timing
experiment is run by this harness.
