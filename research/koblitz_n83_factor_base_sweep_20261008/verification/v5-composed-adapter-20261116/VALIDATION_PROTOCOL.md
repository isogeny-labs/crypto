# V5 adapter release-library validation

Frozen before the command. The V5 change edits examples and study files only;
`git diff --name-only a91b8b66d0f1ffc175fdce984d1059144ce232f6 HEAD -- src Cargo.toml Cargo.lock tests gpu tools`
must be empty. For the full release-library gate, use the already materialized
`/Volumes/SSD990/tmp/n83-k600-budget-v3-20261116` checkout at
`a8821d64550526c3151eca205a9751ca4d8aa8a5`, which has the same
library-affecting files as merged `a91b8b66d0f1ffc175fdce984d1059144ce232f6`.
The V5 branch's own sparse checkout lacks unrelated research fixtures, so
running its full library suite would conflate fixture absence with code tests.
Record that equivalence check and do not describe this as an exact-head test.

Run once on shared macOS ARM64 with Rust 1.99.1, release, locked/offline,
one Cargo job, and the existing target cache:

```sh
RUSTC=/Users/adamburan/.rustup/toolchains/1.99.1-aarch64-apple-darwin/bin/rustc \
CARGO_TARGET_DIR=/Volumes/SSD990/tmp/n83-k600-budget-v3-target \
TMPDIR=/Volumes/SSD990/tmp \
gtimeout -s TERM -k 30s 7200s cargo test --release --lib --locked --offline -j 1
```

Record full output, UTC endpoints, child exit, test counts and names, disk
samples, and the exact source commits. Stop if boot or SSD available space
falls below 2 GiB. A passing gate needs exit 0 and zero failed tests. A failed
gate remains failed, even if it matches the earlier
[`MERGE_VALIDATION.md`](../k600-budget-v3-contract-20261116/MERGE_VALIDATION.md)
macOS fixture failures. This suite is validation overhead, not a timed IC arm.
