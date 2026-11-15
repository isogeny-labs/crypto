# Affine clearing certificate control

This small native issuer imports the repository's unchanged identity checker
and portable SHA256 module by relative source paths. It certifies only the
polynomial rearrangement d(1-zi)=d-z(di), not the unit constraint di=1.
The unit relation and full quantified ring theorem are checked in Lean.
The certificate ID is IDC1had86c2d70ea770ef. Native replay with extra points
accepts and the naive replacement is rejected as IssueError::False, rather
than merely as malformed [pcc:unverified]. No independent standing is asserted.

From repository root:

```sh
cargo run --release --locked --manifest-path research/prime_common_scalar_20261108/identity/Cargo.toml > /tmp/affine-clearing.json
cmp /tmp/affine-clearing.json research/prime_common_scalar_20261108/identity/affine-clearing.json
cargo test --release --locked --manifest-path research/prime_common_scalar_20261108/identity/Cargo.toml
```

Use an external CARGO_TARGET_DIR to keep binaries outside the evidence tree.
Recorded execution used the pinned local stable Rust toolchain, cached/offline
dependencies, and /workspace/audit/contraction-identity-target. tests-001.log
records 22 unchanged checker/hash module tests, zero failures. No root library
source or dependency changed, and the full repository suite was not rerun.
Build logs are compiler diagnostics, not measured performance evidence.
