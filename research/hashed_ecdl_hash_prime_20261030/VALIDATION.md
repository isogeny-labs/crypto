# Development validation commands and scope

These are native development/public-proof-fixture checks, not scientific,
profiling or benchmark runs. Raw elapsed times below are unisolated diagnostics.
CHECKPOINT_SKIP named that development scope; no scientific gate was bypassed.
The S3 checkpoint upload has already succeeded separately; native image/execution
and matched measurement requirements remain open.

All commands completed with exit 0 except the retained initial example compile
failure (exit 101), corrected with explicit usize shift operands.

- `cargo run --locked --release --manifest-path crates/hashed-ecdl-tensor/Cargo.toml --example prepare-certified-hash`: preparation.txt; initial failure in preparation-initial.txt.
- `cargo test --locked --release --manifest-path crates/hashed-ecdl-tensor/Cargo.toml --test certified_hash_prime -- --nocapture`: targeted-tests.txt.
- `cargo test --locked --release --manifest-path crates/hashed-ecdl-tensor/Cargo.toml`: crypto-tests.txt (44 passing tests).
- `cargo test --locked --release --manifest-path bindings/rust/Cargo.toml -p hashed-ecdl-tensor`: cryptanalysis-tests.txt (55 passing tests).
- Strict all-target clippy with `-- -D warnings` using each manifest above: crypto-clippy.txt, cryptanalysis-clippy.txt.
- `cargo +1.78.0 check --locked` with each manifest/package and `--all-targets`: crypto-msrv.txt, cryptanalysis-msrv.txt.
- `cargo fmt --manifest-path <package Cargo.toml> --check`: both exit 0, no output.
- Feature additions/removals for all four changed native source files match across repositories after normalizing paths and diff offsets. New proof JSON, preparation example and integration tests are byte-identical.
- The inherited strict point/JSON/bounded-read and functional-pruning changes in cryptanalysis main are preserved. Whole source trees are not asserted identical.
- Native SHA256 manifests pin the actual respective source contexts, not old frozen snapshots.
- `build-report.sh`: Graphviz/LaTeX/Ghostscript succeeded; the one-page PDF was rendered and visually inspected. No overfull/error diagnostics. Both PDFs are below 64 KiB. Intermediate TeX files are excluded from the commit.
- `git diff --check` passed. No dependencies, source recursion constants, old fixture or frozen protocol/report/source manifest changed.

Primary sources read: pinned Bochum v0.2 section 3 (PDF Git blob
44b1e3de53927ff576b2e0f761f5f39e1df30a61) and HAC Fact 4.40 / chapter notes.
GNU factor supplied `18446744073709551628: 2 2 7 658812288346769701` as untrusted
65-bit fixture preparation; the native test independently checks the actual
prime factor and Pocklington proof conditions. This is not a new theorem or an
independent audit of the general implementation.
