# Reproduction and local artifacts

The exact commands, inputs, pre/post hashes, timestamps, limits and outputs are
under `evidence/phases`. Native source is `native/`, with its exact Cargo.lock.
Shared generic laws are owned by crypto-lean commit
`1c57c092460bee5093072602d3bb058d13bbbcf1`; the source binding file pins hashes.

For the native scalar check, use original Rust1.99.1, one worker:

```sh
cargo build --release --locked --manifest-path native/Cargo.toml
cargo clippy --release --locked --manifest-path native/Cargo.toml -- -D warnings
timeout 60 native/target/release/root-precision-controls
```

The original used an exclusive target directory, cached dependencies with
`--offline`, copied read-only binary, pre-call hashes and the fixed physical
guard. A command on another machine is a fresh reproduction, not the original
run. The executable emits exact symbolic certificates and bounded numeric rows;
it intentionally does not serialize the enormous integers as decimal data.

For focused formal reproduction, use official Lean4.35.0 and a fresh output
directory containing a `CryptoLeanCore/` subdirectory. Set LEAN_PATH to that
output directory followed by the official Lean library. Compile the pinned
shared TaperedPrecision.lean and TaperedPrecisionRoot.lean into that subdirectory,
then formal/RootControls.lean into RootControls.olean. The commands are the
source-equivalent `lean -j1 -M0 -o OUTPUT SOURCE` calls recorded in the phase
receipts; compile from each owning source root. Run the unchanged repository
proof gate for RootControls and the shared root module with that LEAN_PATH,
then `leanchecker --fresh -v RootControls`. Std is the only imported library;
no Mathlib rebuild or dependency change is needed for these focused targets.
The complete-library/CI gate remains a separate integration check.

Review manifests preserve their original local source-snapshot references.
Those full snapshot directories, copied qualified binary, .olean files, all
failed attempts and local launchers are retained in the local root checkpoint.
The original absolute paths in receipts identify the original run, not paths
to assume on a new host. No uploaded/S3 artifact is asserted. The publication
receipt will identify the local capsule's hash and byte count.

Two additional lossless local archives retain completed generated caches and
all distinct duplicate checkpoint/readbacks, including the earlier CPU umask
failure. Their `.sha256` and compare receipts are in evidence. They changed
storage layout only; original canonical handoff capsules remain untouched.

Formal PCC759751 has a local verifier pass, while `pcc standing` returns
unverified (exit2); native PCC4b3d64 is also unsigned. Current-main ledger
prefix conflict at1564 is retained rather than rewriting either history.
Source review and fresh kernel replay must not be labeled independent signed
execution. [pcc:unverified]
