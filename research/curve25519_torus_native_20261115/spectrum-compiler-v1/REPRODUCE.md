# Reproduce the declared correctness component

Hydrate selected source/raw subjects using the repository's maintained artifact
tool and real manifests before inspecting them. Check SOURCE_QUALIFICATION.json
and native-v1/RETAINED_BYTES.json. No remote receipt is inferred from a local hash.

```sh
python3 tools/artifacts/artifacts.py pull
sha256sum --check research/curve25519_torus_native_20261115/spectrum-compiler-v1/SOURCES.sha256
cargo test --locked --release --manifest-path tools/torus-spectrum-compiler/Cargo.toml
cargo fmt --check --manifest-path tools/torus-spectrum-compiler/Cargo.toml
cargo clippy --locked --release --all-targets --manifest-path tools/torus-spectrum-compiler/Cargo.toml -- -D warnings
cargo build --locked --release --manifest-path tools/torus-spectrum-compiler/Cargo.toml
```

Use a fresh source extraction and new output directory; never overwrite frozen
evidence. The original native command is the maintained versioned shell
transport `tools/torus-spectrum-compiler/run-control.sh IMMUTABLE_BINARY NEW_DIR
CVC5_BINARY`, under600seconds and2GiB. Its two native invocations are fixed
section correctness and backend admission, not a coefficient-search campaign.
The native archive contains every raw output and both complete SMT exports.
The gzip copies are byte-equal after decompression; raw size/hash pins are in
RETAINED_BYTES.json. Source archive roundtrip was checked for all 76 entries.

Backend: official `cvc5-Linux-x86_64-static-gpl.zip` at
https://github.com/cvc5/cvc5/releases/tag/cvc5-1.4.2.
Archive hash, executable hash and actual --show-config are retained in
native-v1/backend-release-admission.json. The release includes CoCoA and uses
GPLv3; no third-party executable is committed here. Finite-field SMT values use
the observed native `#fVALUE mMODULUS` compact spelling without the space.
The scalar admission parser checks the requested modulus and canonical value;
it is not yet an arbitrary spectrum-model parser.

Editable report: `dot -Tpdf diagram.dot -o diagram.pdf`, then
`pdflatex -interaction=nonstopmode -halt-on-error report.tex` in this directory.
Figures present recorded counts/layout only. Do not infer timings from tool
or test-suite elapsed output. [pcc:unverified]
