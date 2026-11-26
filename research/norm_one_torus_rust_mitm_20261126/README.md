# Rust m=5 norm-one-torus decomposition

This study implements and checks a five-term (`m = 5`) meet-in-the-middle
decomposition over the production Prime31 norm-one-torus/Dickson factor base.
The optimized arm combines projective arithmetic, Montgomery batch inversion,
and a canonical quotient of the three-sum table by elliptic negation.

The public-precommit producer run recorded a 48.356 ms control median and a
10.355 ms candidate median, a 4.669821x local end-to-end ratio. Setup
inversions fell from 57,052 to 2. These wall times were not isolated and the
PCC claim has no independent signatures, so they are diagnostic and
unverified, not a validated speed record. [pcc:unverified]

The source report is [REPORT.md](REPORT.md). The public seed, raw rows, witness
certificates, and proof-status artifacts are under
[`public-precommit/`](public-precommit/). Claim
`PCC-20261126-7a50b7` currently has `unverified` standing: both proof adapters
are structurally accepted but `not_covered` without independent keyring
signatures.

## Rust entry points

- [`examples/norm_one_torus_mitm.rs`](../../examples/norm_one_torus_mitm.rs):
  the control and optimized witness-recovery solvers.
- [`examples/norm_one_torus_mitm_verify.rs`](../../examples/norm_one_torus_mitm_verify.rs):
  a separately implemented affine exhaustive-mask and witness verifier.
- [`examples/support/norm_one_torus_mitm.rs`](../../examples/support/norm_one_torus_mitm.rs):
  the seed, production fixture, target, and certificate contract.

## Reproduce

```sh
cargo test --release --example norm_one_torus_mitm \
  --example norm_one_torus_mitm_verify
cargo build --release --example norm_one_torus_mitm \
  --example norm_one_torus_mitm_verify
target/release/examples/norm_one_torus_mitm \
  --manifest research/norm_one_torus_rust_mitm_20261126/public-precommit/manifest.json \
  --certificates /tmp/norm-one-torus-certificates.json
target/release/examples/norm_one_torus_mitm_verify \
  --manifest research/norm_one_torus_rust_mitm_20261126/public-precommit/manifest.json \
  --certificates /tmp/norm-one-torus-certificates.json
```

The retained certificate uses create-new/byte-identical semantics. Reusing its
repository path is therefore a replay check rather than an overwrite.
