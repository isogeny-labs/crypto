# Source and control reproduction

SOURCES.sha256 lists111native inputs: old multiplier103 plus new crate8,
in five constituent archives from SOURCE_CONTEXT.json. Six compiled-in
crate hashes and retained binary match. Source ee1a401f1f was published as
f1f9348521 before copied immutable qualification/transport/primary call.
Source/fixture/caps did not change afterward. [pcc:unverified]

```
sha256sum -c research/curve25519_torus_native_20261115/torsion-column-transport-v1/SOURCES.sha256
cargo test --release --locked --lib --manifest-path tools/torus-column-transport/Cargo.toml
cargo fmt --check --manifest-path tools/torus-column-transport/Cargo.toml
cargo clippy --release --locked --all-targets --manifest-path tools/torus-column-transport/Cargo.toml -- -D warnings
cargo build --release --locked --manifest-path tools/torus-column-transport/Cargo.toml
```

These are reproduction commands, not evidence that a later checkout already
ran them. Primary reproduction requires a new immutable source/binary qualifier,
fresh output,600second/2GiB VM limits and write-once build/pre/post records.
Do not rerun the frozen worker to refresh publication metadata. The native
--transport mode runs no mathematical controls and checks actual wrapper/raw
create-new ownership with stdout kept separate. [pcc:unverified]

Hydrate the exact native-v1/native-run-v1.tar.xz through a genuine artifact
receipt when available, or use its exact temporary Git staging blob. Verify
140084bytes/SHA256f86428a9b3762ed561333b46c78618c5b4e60c4da72f74971e29c5c761b3123f
before extraction to fresh scratch. RAW_MANIFEST.json binds all28members and
the full1,708,591byte/35record JSONL, including all128raw-sector root calls,
every same-row transport, matrix, row operation and kernel vector. No Python
field/group/rank computation or native-control launcher is used.
[pcc:unverified]

The four groups are toy, actual, guards and evaluated identity. The recorded
call completed all4with zero failures. Toy reference shares field/group and
original raw-map primitives while bypassing root factorization. Native
matrix replay is producer-controlled, not independently signed validation.
See lean/REPRODUCE for the separately scoped two numeric obligations.
[pcc:unverified]
