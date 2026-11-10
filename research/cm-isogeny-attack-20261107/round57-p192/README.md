# Signed paired-resultant adapter and P-192 degree audit

Read report.md or its inspected PDF. The native adapter evaluates the cyclic trace at the declared safe probes using signed product-Kummer blocks. It includes a separate actual P-192 rational-interpolation audit and independent rank/point checks. The formal and performance scopes are explicit in proof-map.json and cost-contract.json. Scientific standing is unsigned. [pcc:unverified]

Replay in a disposable study-layout copy. Hydrate any artifact-store records first and verify dependencies.sha256. Build with Rust/Cargo using the locked dependencies:

```sh
export CARGO_TARGET_DIR=/tmp/p192-round57-check
CARGO_BUILD_JOBS=1 cargo build --locked --release --bins --manifest-path native/Cargo.toml
"$CARGO_TARGET_DIR/release/paired-trace-resultant" "$PWD" > blocks-replay.json
"$CARGO_TARGET_DIR/release/degree" "$PWD" > degrees-replay.json
"$CARGO_TARGET_DIR/release/verify_degree" "$PWD" > degree-verification-replay.json
"$CARGO_TARGET_DIR/release/p192_pairs" "$PWD" > pairs-replay.json
cmp blocks-replay.json blocks.json
cmp degrees-replay.json P192-degrees.json
cmp degree-verification-replay.json degree-verification.json
cmp pairs-replay.json P192-pairs.json
```

Add --offline only with dependencies cached. The degree verifier reads the saved P192-degrees.json; copy degrees-replay.json into that name only inside a disposable copy when verifying regenerated data. Every binary writes JSON to stdout. No research Python process is used.

The P-192 degree matrix is generated from public points. The independent verifier uses Jacobian arithmetic and determinant recovery from matrix powers/Newton identities, sharing modular arithmetic, polynomial utilities and interpolation-matrix construction. Positive controls also satisfy the full known rational-map identities. All negative results are bounded by the frozen degrees and sample conditions. [pcc:unverified]

Run `lean +leanprover/lean4:v4.19.0 lean/Pair.lean` for the integer trace identity. Its geometric binding is not supplied. Keep this sealed directory intact; use copies for replay and mutation checks. Blocks.json may be moved to the repository artifact store by CI; its original digest remains in manifest.sha256. Follow the repository hydration workflow before checking that manifest.
