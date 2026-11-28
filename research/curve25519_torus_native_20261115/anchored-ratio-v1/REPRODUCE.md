# Version1 source and fixed native control

SOURCES.sha256 binds118inputs: previous111 plus7new crate files. The source
closure is six constituent archives in SOURCE_CONTEXT.json, not a monolithic
118-input archive. New source delta10604bytes. Original source was published
as38faab5cefee before the copied immutable qualifier and sole native call.
Retained1614112byte binary SHA376d8d57938bb86a12419e1e9e4384ee727ee6f119ce240602b68fe50320025f
and five compiled inputs match pre/post. [pcc:PCC-20261124-0a02aa]

```
sha256sum -c research/curve25519_torus_native_20261115/anchored-ratio-v1/SOURCES.sha256
cargo test --release --locked --lib --manifest-path tools/torus-anchored-ratio/Cargo.toml
cargo fmt --check --manifest-path tools/torus-anchored-ratio/Cargo.toml
cargo clippy --release --locked --all-targets --manifest-path tools/torus-anchored-ratio/Cargo.toml -- -D warnings
cargo build --release --locked --manifest-path tools/torus-anchored-ratio/Cargo.toml
```

These commands are for reproduction, not evidence that a future checkout ran
them. A new primary call needs a fresh source/binary qualifier and output,
600second/2GiB envelope and distinct write-once BUILD/PRECALL/POSTCALL. Native
--transport checks actual wrapper/create-new ownership without math controls.
Keep launcher stdout separate from the native-owned raw path.
[pcc:PCC-20261124-0a02aa]

Under native-v1, verify native-run-v1.tar.xz.sha256 and extract into fresh
scratch. The49256byte/36-member archive retains the full364851byte/42-record
JSONL, final logs, source/qualifier receipts, transport/collisions, failed
development source snapshots/logs and exact pre/post metadata. RAW_MANIFEST
binds all members and archive; repacking uses the documented deterministic
24November GNU-tar/xz6 format. It round-trips byte-for-byte. Source/proof
standing is producer unsigned/unverified; no new Lean proof was run.
[pcc:PCC-20261124-0a02aa]
