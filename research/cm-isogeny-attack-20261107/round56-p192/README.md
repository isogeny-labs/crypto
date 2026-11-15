# Cyclic marked trace and Frobenius normalization

Read report.md and the inspected PDF. This replaces the full-torsion trace by a public-GQ cyclic trace for the rational marked-action extractor. The degree bounds, balanced rational merge, raw normalization and full-width P-192 normalizer are derived and checked at the stated scope. PCC-20261108-46c184 is unsigned. [pcc:unverified]

Run in a disposable study-layout copy, hydrating earlier store-backed records and checking dependencies.sha256 first:

```sh
export CARGO_TARGET_DIR=/tmp/p192-round56-check
CARGO_BUILD_JOBS=1 cargo build --locked --release --bins --manifest-path native/Cargo.toml
"$CARGO_TARGET_DIR/release/cyclic-trace-frobenius" "$PWD" > replay.json
"$CARGO_TARGET_DIR/release/verify" "$PWD" > verification-replay.json
"$CARGO_TARGET_DIR/release/p192" "$PWD" > p192-replay.json
cmp replay.json certificate.json
cmp verification-replay.json verification.json
cmp p192-replay.json P192-normalizer.json
```

Add --offline to the build only with dependencies cached. Cargo.lock pins the standalone native implementation. Producer inputs are public parameters, points and the fixed quadratic shifts. The verifier uses packed quadratic-field arithmetic, complete point enumeration and line-intersection addition, plus unpaired and balanced coefficient reconstructions. No Python research process is invoked. [pcc:unverified]

The pure logical sign-selection lemma builds with `lean +leanprover/lean4:v4.19.0 lean/Fiber.lean`. It exposes its two geometric hypotheses and does not formalize the elliptic binding or local trace proof. See proof-map.json. The full trace and curve-map identities are written arguments with native exact controls, pending trusted scientific review. [pcc:unverified]

The supported binaries write JSON to stdout. Keep the sealed directory intact; use copies for replay and mutations. compression-audit.json specifies the next paired-weight resultant interface. Fast normalizer evaluation is available; fast P-192 weighted trace evaluation remains the next construction task.
