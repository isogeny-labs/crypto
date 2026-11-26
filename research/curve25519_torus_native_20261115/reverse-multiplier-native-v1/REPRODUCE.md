# Reproduction and version discipline

Current source: SOURCES_V2.sha256, point95 plus eight new inputs in four
archives. SOURCE_QUALIFICATION_V2.json gives every archive digest and the
qualified source commit/publication. Revision1 SOURCES.sha256/source-v1
archive are historical and must not be called current after the main/wrapper
amendment. All prior protocol/source/criteria bytes remain frozen.

```
sha256sum -c research/curve25519_torus_native_20261115/reverse-multiplier-native-v1/SOURCES_V2.sha256
cargo test --release --locked --lib --manifest-path tools/torus-reverse-multiplier/Cargo.toml
cargo fmt --check --manifest-path tools/torus-reverse-multiplier/Cargo.toml
cargo clippy --release --locked --all-targets --manifest-path tools/torus-reverse-multiplier/Cargo.toml -- -D warnings
cargo build --release --locked --manifest-path tools/torus-reverse-multiplier/Cargo.toml
```

These are source-reproduction commands, not a statement that a later checkout
has already run them. Native primary reproduction requires the same source/
binary qualifier, fresh output directory, explicit600second/2GiB cap and
write-once manifests. Do not rerun the frozen worker merely to refresh a
metadata handoff. API solve(Spec,R,cap) receives no known multiplier, scalar,
root/candidate list or supplied point tuple. [pcc:unverified]

Extract the complete record tree from native-v2/native-run-v2.tar.xz into a
fresh scratch directory. RAW_MANIFEST.json binds all members,319552-byte
controls JSONL and20224-byte archive. It includes every branch polynomial,
root factor, multiplicity, candidate, raw point, image, geometric row and
group/complete summary, plus qualification/transport/build/failure logs.
No scientific value is reconstructed by Python; metadata/byte checks may
use Python as in the publication workflow. [pcc:unverified]

All96toy target sets are compared against complete separate enumeration
of the declared102torus parameters through a direct group path, sharing
native field/group primitives. Four fixed groups execute even if an earlier
group fails; errors disclose unattempted arms within that group. The actual
source-qualified run exited0 with110records and all4groups successful.
Source/compiled/binary pre/post match and native create-new collision refuses.
Producer unsigned standing is separate from independently signed verification.
[pcc:unverified]
