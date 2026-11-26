# Reproduce the native adapter controls

Restore/hydrate exact tracked inputs and the old compiler archive through
maintained tooling. Check SOURCES.sha256. SOURCE_QUALIFICATION.json names both
archive constituents and their roundtrips; do not substitute the delta for
the full85-input context. Cargo.lock pins ordinary third-party dependencies.

```sh
sha256sum --check research/curve25519_torus_native_20261115/model-solver-v1/SOURCES.sha256
cargo test --locked --release --manifest-path tools/torus-spectrum-solver/Cargo.toml
cargo fmt --check --manifest-path tools/torus-spectrum-solver/Cargo.toml
cargo clippy --locked --release --all-targets --manifest-path tools/torus-spectrum-solver/Cargo.toml -- -D warnings
cargo build --locked --release --manifest-path tools/torus-spectrum-solver/Cargo.toml
```

The original fixed final invocation used run-control.sh IMMUTABLE_BINARY
NEW_DIRECTORY PINNED_CVC5 under600seconds/2GiB. It runs synthetic full-model
replay, native small-system protocol/cap controls, then exactly one declared
known-consistent public curve query. Do not automatically rerun the frozen
curve query or overwrite evidence. A new comparison/campaign requires its
own protocol and backend authorization. Native scalar backend identity stays
754bc9abb49bcd2ae095bf07a9ba2f68759bf0f232e71f601ac642f802a59fc8.

Extract native-v1/native-run-v1.tar.xz to a fresh directory. Its raw six streams
match native-v1/RAW_MANIFEST.json; all captured queries/responses and declared
truncation flags are retained. No remote receipt or signed standing is inferred
from local hashes. The new archive is retained directly under actual policy;
older pending S3 objects are untouched. [pcc:PCC-20261123-ab3e5b]

The process driver uses native Rust only, no Python math/replay/solver wrapper.
Primary source links and KB-unavailable scope are in PROTOCOL.md. Editable
report diagram: dot -Tpdf diagram.dot -o diagram.pdf, then pdflatex report.tex.
The report presents outcomes, not elapsed-time or performance measurements.
[pcc:unverified]
