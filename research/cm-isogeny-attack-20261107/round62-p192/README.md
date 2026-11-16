# Round62: horizontal marked norm transport

Read report.md, transport-proof.md, B-only-proof.md, scope.json and both amendments. Native code uses only inherited fixtures, complete small norms and actual P192 maps. The scalar reference in the small control is separately checked after the coefficient contraction. Hydrate any .artifacts.json entries before hashes/replay.

From this directory with the sibling inputs in dependencies.sha256:

```sh
CARGO_BUILD_JOBS=1 CARGO_TARGET_DIR=/tmp/horizontal-target timeout 120 cargo build --release --locked --offline --manifest-path native/Cargo.toml
/tmp/horizontal-target/release/horizontal-jets --check transport.json > /tmp/transport.json
cmp transport.json /tmp/transport.json
/tmp/horizontal-target/release/horizontal-jets --cofactor-reference --check cofactor-reference.json > /tmp/cofactors.json
cmp cofactor-reference.json /tmp/cofactors.json
/tmp/horizontal-target/release/p192 "$PWD" --check P192-transport.json > /tmp/P192.json
cmp P192-transport.json /tmp/P192.json
/tmp/horizontal-target/release/horizontal-jets --B-only --check B-only.json > /tmp/B-only.json
cmp B-only.json /tmp/B-only.json
/tmp/horizontal-target/release/horizontal-jets --B-only --cofactor-reference --check B-reference.json > /tmp/B-reference.json
cmp B-reference.json /tmp/B-reference.json
sha256sum --check dependencies.sha256
sha256sum --check manifest.sha256
```

Each fresh source replay uses the locked code in an independent target directory. Mutation fixtures change one retained coefficient or point coordinate and must reject with101. Counts retain all declared cases, graph constructors, constants, weighted factors and additional work. They are not timings. P192 runs no full norm. The required broad legacy check has its own retained exit/log; it does not certify this publication head. Scientific standing remains unverified until a signed independent PCC verdict.
