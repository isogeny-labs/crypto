# Reproduce fixed public controls

From repository root, check `SOURCES.sha256` and the three source archives in
BUILD_MANIFEST.json. Restore their exact inputs in a separate checkout if
working sources have evolved. Rust1.100.0 and the pinned Cargo.lock were used.

```
sha256sum --check research/curve25519_torus_native_20261115/worklist-solver-v1/SOURCES.sha256
cargo test --release --lib --manifest-path tools/torus-worklist-solver/Cargo.toml
cargo fmt --check --manifest-path tools/torus-worklist-solver/Cargo.toml
cargo clippy --release --all-targets --manifest-path tools/torus-worklist-solver/Cargo.toml -- -D warnings
cargo build --release --manifest-path tools/torus-worklist-solver/Cargo.toml
tools/torus-worklist-solver/target/release/torus-worklist-solver NEW_OUTPUT_DIRECTORY
```

The frozen qualifier additionally checks the retained original binary before
invoking it; the path is producer-local. A rebuild need not reproduce its
binary hash on another host. Check the new executable's pre-call source pins
before comparing raw control semantics. Do not overwrite native-v1 or call
a rebuilt binary the original retained binary.

Inspect original outputs without an execution:

```
cd research/curve25519_torus_native_20261115/worklist-solver-v1
sha256sum --check native-v1.tar.xz.sha256
tar -tJf native-v1.tar.xz
tar -xJOf native-v1.tar.xz native-v1/controls.json
```

The actual fixture is q1/k2/two columns. Target creation uses the known
public sum of those columns, but only System/branch-role indices/Limits enter
solve. Frozen13 toy targets enter as public point coordinates. The main
executable has no label/sign/witness query input. Original index/group replay,
input construction and serialization are outside partial arithmetic counters.
No speed comparison is reproduced by these commands.
