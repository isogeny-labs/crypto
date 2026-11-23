# Reproduce v2 software controls

Run from the repository root with Rust1.100.0 and the43 source inputs in
SOURCES.sha256. Verify hashes first; the archive retains exact bytes and the
v1 fixture. Use a NEW output directory so old records are never overwritten.

```sh
cargo test --release --locked --manifest-path tools/torus-coset-circuit/Cargo.toml
cargo fmt --check --manifest-path tools/torus-coset-circuit/Cargo.toml
cargo clippy --release --locked --all-targets --manifest-path tools/torus-coset-circuit/Cargo.toml -- -D warnings
cargo build --release --locked --manifest-path tools/torus-coset-circuit/Cargo.toml --bin aggregation_control
tools/torus-coset-circuit/target/release/aggregation_control NEW-OUTPUT-DIRECTORY
```

Actual commands used /home/agent/.cargo/bin/cargo. Development logs retain
the first failed build and corrected source attempts. Final suite:two tests,
none ignored. The executable writes pre-call.json before control computation,
including source/fixture/binary digests and fixed caps. Raw records are
native-v2/controls.json, first-six-member-witness.json,
six-member-system.json.gz and toy-target-acceptance.bin. The record gives
the full trace layout, compressed/uncompressed hashes and public inputs.

Source qualification checks the old38-input v1 archive,37 unchanged working
inputs, the new43-input archive round-trip and ten compiled-in hashes.
Only earlier native lib.rs evolves; direct Lean bindings and old results
remain unchanged. This is fixed development correctness, not sampling,
timing, a solver campaign or full actual-base enumeration. Signed standing
remains unverified.
