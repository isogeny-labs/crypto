# Round60: transverse cross-norm

Read report.md, cross-norm-proof.md and scope.json. This round constructs a Frobenius-compressed zero-order norm, two target-specific first coefficients, an inverse-affine point action and an ordinary-action bridge. The complete native norm enumerates small-curve graphs; the full P-192 first variations are not evaluated.

From this directory, with the committed sibling fixtures available:

```sh
CARGO_BUILD_JOBS=1 CARGO_TARGET_DIR=/tmp/transverse-target cargo build --release --locked --manifest-path native/Cargo.toml
/tmp/transverse-target/release/transverse-norm > /tmp/norms.json
cmp norms.json /tmp/norms.json
/tmp/transverse-target/release/transverse-norm --cofactor-reference --check norms.json > /tmp/cofactors.json
cmp cofactors.json /tmp/cofactors.json
/tmp/transverse-target/release/transverse-norm --transfer-check --check norms.json > /tmp/transfer.json
cmp norms.json /tmp/transfer.json
/tmp/transverse-target/release/p192 "$PWD" > /tmp/P192.json
cmp P192-blocks.json /tmp/P192.json
/tmp/transverse-target/release/p192 "$PWD" --cofactor-reference > /tmp/P192-cofactors.json
cmp P192-cofactors.json /tmp/P192-cofactors.json
```

The public transfer checker uses no scalar label. Its exact call/lookup counts are in transfer-check.json; controls and fresh replays are additional work. The P-192 selected inputs are deliberately U=[j](G-Q) for indices in the declared block. They do not establish containment for an arbitrary input.

Preserve the sealed source, data, failures and before-data receipts. Hydrate any .artifacts.json entries through the repository artifact tools before SHA-256 verification or replay. Claims remain unsigned producer evidence until trusted review. The proof describes the trivial zero-target branch; native target panels are nonzero.
