# Round58: pole-compatible signed trace

Read `report.md` and `scope.json`; the two amendments preserve the frozen panel and specify the P-192 and precision checks before their affected data. The native programs check exact algebra and point actions, with no timing claim. `poles.json` covers456 complete toy actions; `P192-poles.json` covers32 selected P-192 blocks. The standalone CM correspondence criterion is a written derivation; it is not a general representation lower bound.

Use locked native dependencies and a target directory outside this immutable evidence directory:

```sh
CARGO_BUILD_JOBS=1 CARGO_TARGET_DIR=/tmp/round58-target cargo build --release --locked --manifest-path native/Cargo.toml
/tmp/round58-target/release/pole-trace > /tmp/poles.json
cmp poles.json /tmp/poles.json
/tmp/round58-target/release/pole-trace --root-product --check poles.json > /tmp/root-product.json
cmp root-product.json /tmp/root-product.json
P192_PRECISION_CHECK=1 /tmp/round58-target/release/pole-trace --check poles.json > /tmp/precision.json
/tmp/round58-target/release/p192_poles "$PWD" > /tmp/P192-poles.json
cmp P192-poles.json /tmp/P192-poles.json
/tmp/round58-target/release/p192_poles "$PWD" --root-product > /tmp/P192-root-product.json
cmp P192-root-product.json /tmp/P192-root-product.json
```

P-192 commands require the committed Round52 input and Round54 quadratic-shift record in sibling directories. Both are hashed in dependencies.sha256. `--mutate` must fail with exit101. The root-product checker also rejects a changed certificate coefficient. Shared field/curve code is retained in the source; the reference norm uses root products instead of Gaussian elimination.

If CI externalized a large file, hydrate `.artifacts.json` entries using the repository artifact tools before checking `manifest.sha256` or replaying. Report PNG pages are derived inspection artifacts. Claims remain unsigned producer evidence pending trusted review.
