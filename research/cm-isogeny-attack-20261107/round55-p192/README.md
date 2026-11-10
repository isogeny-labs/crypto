# Regularized mixed-trace action: Round55

Read report.md or the inspected report.pdf for the principal-part identity, sign selection, conditional auxiliary-input algorithm and complete costs. P-192 is the primary target. The actual P-192 check covers division-polynomial values, derivatives and normalization; complete marked-action scalar recovery is confined to the declared order7 control. Scientific standing is unsigned. [pcc:unverified]

Replay the supported native path in a disposable study-layout copy:

```sh
CARGO_BUILD_JOBS=1 CARGO_TARGET_DIR=/tmp/p192-round55-check \
  cargo run --locked --release --manifest-path native/Cargo.toml -- "$PWD" \
  > replay.json
cmp replay.json native-verification.json
```

Rust/Cargo and the dependencies pinned by native/Cargo.lock are required. Add --offline only if the crates are already cached. Run from round55-p192, with the exact earlier round54/certificate.json and round52/input.json at the relative paths in dependencies.sha256. Hydrate artifact-store files before checking hashes when necessary. The checker reads saved records and prints JSON; it never launches Python. native/src contains independent field/curve arithmetic, complete dense numerator reconstruction from the saved graph maps, and the original division-polynomial jet recurrence. Archived *.py files preserve the prior source provenance; they are not the supported execution path.

certificate.json retains all six kernels' dense A,B coefficients,36 actions and18 auxiliary recoveries. P192-certificate.json holds the saved full-width normalization cases. native-verification.json and fresh-replay.json retain the exact native readback; mutation logs retain both rejection attempts. The historical build.log and P192.log are preserved. Exact arithmetic check counts are not performance measurements. [pcc:unverified]

No P-192 numerator is constructed. Its compressed evaluation, including kernel and numerator setup, is the next construction task. The Cheon stage is established prior work, and the total conditional cost is stated in report.md. This directory follows merged PR #3681 without modifying earlier sealed rounds.
