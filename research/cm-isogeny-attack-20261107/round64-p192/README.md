# Round64: field-resultant jet norms

Read report.md, resultant-proof.md, scope.json and amendment1.json. The routine uses classical field-Euclidean resultants and gcd, not a subresultant PRS or half-gcd. The signed baby-algebra input and complete decoder panel are inherited unchanged from Round63; coefficients and receipts are checked separately.

Hydrate .artifacts.json entries before checking hashes. From this directory:

```sh
CARGO_BUILD_JOBS=1 CARGO_TARGET_DIR=/tmp/field-norm-target timeout 120 cargo build --release --locked --offline --manifest-path native/Cargo.toml
/tmp/field-norm-target/release/signed-baby-response --norm-controls --check norm-controls.json > /tmp/controls.json
/tmp/field-norm-target/release/signed-baby-response --check response.json > /tmp/response.json 2> /tmp/response.log
cmp response.json /tmp/response.json
cmp response.log /tmp/response.log
/tmp/field-norm-target/release/signed-baby-response --dense-reference --check dense-reference.json > /tmp/dense.json 2> /tmp/dense.log
cmp dense-reference.json /tmp/dense.json
cmp dense-reference.log /tmp/dense.log
/tmp/field-norm-target/release/p192 "$PWD" --check P192-norms.json > /tmp/P192.json
cmp P192-norms.json /tmp/P192.json
sha256sum --check dependencies.sha256
sha256sum --check manifest.sha256
```

The qualifier executes field norms, dense norm references and direct streams. norm_backend_stats isolates the two norms on matching already-formed inputs; algebra_reference_counts includes both backends plus common polynomial work. Whole-qualifier totals and all setup/decoder receipts remain. P192 polynomial inputs are explicitly interpolated from four signed pair factors, so its isolated norm counts do not establish a fast public constructor. All scientific standing is unverified until a signed independent verdict.
