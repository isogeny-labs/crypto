# Reproduce in a NEW checkout/output directory

Hydrate source-v1.tar.xz at its archived root paths,including unchanged original
gzip. Cargo.lock pins registry dependencies. Build isolated Rust crate:

    CARGO_TARGET_DIR=/tmp/rowspace-build cargo test --release --lib --manifest-path tools/torus-rowspace-audit/Cargo.toml
    CARGO_TARGET_DIR=/tmp/rowspace-build cargo clippy --release --all-targets --manifest-path tools/torus-rowspace-audit/Cargo.toml -- -D warnings
    CARGO_TARGET_DIR=/tmp/rowspace-build cargo build --release --manifest-path tools/torus-rowspace-audit/Cargo.toml
    /tmp/rowspace-build/release/torus-rowspace-audit --identity
    /tmp/rowspace-build/release/torus-rowspace-audit /tmp/NEW-rowspace-output

These are reproduction instructions,not a running/scheduled job. Original
qualification used its separately copied executable/qualifier and distinct
pre-call manifest. Raw directory is create-new; capture stdout separately.
No original witness or root/decomposition solver is used. Whole archive and
NATIVE_ARCHIVE.json retain all seven public systems/keep lists,complete
certificates and summaries. src/check.rs directly reconstructs weighted rows
without invoking producer elimination. Both are producer code,not signed
independent/formal validation. Refusals remain unknown,not absence.
