# Rust integration receipts

The source promoted to `crates/certified-relations` is shared by the study
workers and the root `f4_fp::certified` API. Old source snapshots and timed
worker binaries remain frozen evidence of their original commits; their old
source-file paths need not be active code after this move.

`rust-build-NN.json`, `rust-pre-call-NN.json` and `rust-post-call-NN.json`
are distinct, created without replacing an earlier manifest. Each native test
executable was separately copied, stripped of debug sections, hashed and made
read-only before calls. Post-call hashes confirm the copied bytes. Logs and
the archived executable copies are retained; these are producer development
checks without independent signing, not runtime samples `[pcc:unverified]`.

Round 01 retains the five-check integration pass and explicit six-variable
budget failure. Round 02 retains the original-input rewrite diagnosis. The
failed shared-lean trial retains the later change-of-order failure. Round 03
retains complete six-variable breadth-first cases on both fields and all five
public-API checks. Read INTEGRATION_REPORT.md for the exact scope and counts.

The two `initial-*-build.tar.xz` archives retain the attempted broad root build
outputs. A dedicated native qualifier manifest links the actual root library
without building every unrelated command-line target. Shared, study and
root-library lint/test logs remain separate. No full-root-suite pass is claimed.

Policy-selected archives are migrated by trusted artifact-autofix on the PR:
the authoritative `.artifacts.json` records the content-addressed S3 objects
after full remote-byte replay. Until migration, archive bytes remain on the
source branch. `archives.sha256` records their expected hashes. Hydrate using
the repository's `tools/artifacts` before checking/extracting moved archives.

To rerun the public API tests from a full checkout:

```sh
cargo test --locked --manifest-path research/relation_degree_trace_20261115/rust-integration/Cargo.toml
cargo test --release --locked --manifest-path crates/certified-relations/Cargo.toml
cargo test --release --locked --manifest-path research/relation_degree_trace_20261115/Cargo.toml
```

For recorded evidence, build without running, copy/hash the executable first
and retain a new pre-call manifest. Scientific timing still requires its own
checkpoint/isolation admission and matched protocol.
