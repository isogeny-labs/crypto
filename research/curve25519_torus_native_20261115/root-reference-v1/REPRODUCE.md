# Reproduce fixed public-input controls

Verify SOURCES.sha256 against the composite snapshot in FROZEN.md. All43
dependency source inputs are unchanged. Delta inputs are six new crate
source/manifest files and the frozen v2 public-target fixture. Use Rust1.100.0
and a NEW output directory; the executable refuses to replace an old one.

```sh
cargo test --release --locked --manifest-path tools/torus-reference-solver/Cargo.toml
cargo fmt --check --manifest-path tools/torus-reference-solver/Cargo.toml
cargo clippy --release --locked --all-targets --manifest-path tools/torus-reference-solver/Cargo.toml -- -D warnings
cargo build --release --locked --manifest-path tools/torus-reference-solver/Cargo.toml
tools/torus-reference-solver/target/release/torus-reference-solver NEW-OUTPUT-DIRECTORY
```

Actual invocations used /home/agent/.cargo/bin/cargo. The fixed test and
output executable perform no solver sampling, benchmark, paid run or full
actual-base enumeration. Pre-call source/fixture/binary hashes are written
before controls. The full root records, retained basis data, evaluated
identity and complete compressed comparison trace are in native-v1/.

Trace byte bit0 is the full-family disjoint-completion answer, bit1 the
compressed-family answer. Its exact ordering/offsets are in controls.json.
The185760-byte trace is preserved as488-byte gzip with both hashes. Root
vectors replay through the unchanged v2 original circuit and native row
adapter. Selected event counts omit normalized field work and timings;
they support no performance ratio. Full setup is charged in each cost
interpretation; diagnostic target reuse is not batch speed evidence.

The initial Clippy failure/source is retained. Corrected focused release
test, fmt and strict all-target Clippy pass. No old source/evidence changes.
