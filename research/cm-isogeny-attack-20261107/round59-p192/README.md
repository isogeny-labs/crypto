# Round59: signed carriers and the CM degree budget

Read `report.md`, the full written argument `carrier-theorem.md`, and `scope.json`. This round supplies explicit small-scalar signed carriers, a conditional scalar decoder, and exact certificates for global weighted-degree constraints on three P-192 fixtures. It does not bound arbitrary arithmetic circuits or equations valid only on baby subsets. PCC standing remains unsigned until trusted review.

Native replay, from this directory:

```sh
CARGO_BUILD_JOBS=1 CARGO_TARGET_DIR=/tmp/carrier-target cargo build --release --locked --manifest-path native/Cargo.toml
/tmp/carrier-target/release/signed-carrier "$PWD" > /tmp/carriers.json
cmp carriers.json /tmp/carriers.json
/tmp/carrier-target/release/signed-carrier "$PWD" --jacobian --check carriers.json > /tmp/jacobian.json
cmp jacobian.json /tmp/jacobian.json
/tmp/carrier-target/release/recover "$PWD" > /tmp/recovery.json
cmp recovery.json /tmp/recovery.json
```

The commands consume the committed sibling Round52 input, Round57 degree-audit fixtures and Round25 scalar labels. Their hashes and the inherited Round4 discriminant certificate are in dependencies.sha256. The affine producer and Jacobian reference share field helpers but use different point-generation formulas and normalization. Every match list is retained. `--wrong-sign` must reject with exit101; the checker also rejects a changed match count.

Source/scope publication receipts and the decoder amendment precede affected outputs. Large files externalized by CI must be hydrated through `.artifacts.json` and the repository artifact tools before SHA-256 verification or replay. Do not write build outputs inside this sealed evidence directory.
