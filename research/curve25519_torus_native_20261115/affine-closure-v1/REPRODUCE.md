# Reproduce the declared development controls

Read `PROTOCOL.md`, `FROZEN.md` and `INTERFACE_PROVENANCE.md` first. Hydrate the
core archive with maintained `tools/artifacts/artifacts.py pull` after a genuine
store receipt exists; until then it remains tracked/local at its real path.
`PUBLISHING.md` records the current transport state.

From the repository root, inspect `BUILD_MANIFEST.json` and confirm all 64 source
inputs and the four source archives. The pre-call qualifier checks their frozen
digests and the separate executable before running. Do not substitute a newly
built executable under the old binary hash.

```sh
sha256sum -c research/curve25519_torus_native_20261115/affine-closure-v1/SOURCES.sha256
cargo test --release --lib --manifest-path tools/torus-affine-closure/Cargo.toml
cargo fmt --manifest-path tools/torus-affine-closure/Cargo.toml -- --check
cargo clippy --release --all-targets --manifest-path tools/torus-affine-closure/Cargo.toml -- -D warnings
```

These are native fixed development controls, not a timing/scaling campaign.
The immutable core qualifier takes its documented fresh output location; inspect
the script before invoking it, and never overwrite `native-v1/` or the frozen
archives. The Rust executable's public file interface is:

```text
affine-closure-control-v1 NEW_OUTPUT_DIRECTORY
affine-closure-control-v1 verify REQUEST_JSON PREPROCESS_JSON
```

`FILE_INTERFACE_PLAN.tsv`, `FILE_INTERFACE_INPUTS.sha256` and
`file-interface-qualifier-v1.sh` define the corrected three positive and ten
negative standalone coefficient checks. Its full raw round is in
`file-interface-shell-v1.tar.xz`. Root discovery is not rerun by that round.

`native-v1/controls.json` indexes the 34 sealed quadratic requests, original and
appended systems, exact certificates, before/after solver artifacts, replay and
small-system truth/refusal controls. Extract the archive into scratch storage
and check member sizes/hashes against `NATIVE_ARCHIVE.json`. No known labels,
signs, scalar, point list or witness is a solver query input. Prime-field
validity for solving remains external; all standing remains unsigned/unverified.
