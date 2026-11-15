# N83 pilot migration inventory validation, 2026-11-15 UTC

Source branch: `codex/n83-pilot-store-inventory-20261115`; base at audit start: `f8c5a80e310b`. The tested commit was amended only to relocate the report and add this validation record; no root `src/`, root Cargo, or factor-base input file changed. Host: macOS arm64; Rust toolchain `1.99.1-aarch64-apple-darwin`; local isolation L0. This validation reports correctness and custody checks, not algorithm timing.

## Native custody verifier

- `cargo run --release --offline --manifest-path research/koblitz_n83_factor_base_sweep_20261008/pilot-01/migration-inventory-native/Cargo.toml -- research/koblitz_n83_factor_base_sweep_20261008/pilot-01 <fresh-old-bucket-download-root> <new-inventory.json>`: exit 0, 54 / 54 objects, 83,328,324 compressed bytes; both compressed and decompressed BLAKE3 matched the frozen manifest. Rust `sha2` SHA-256 matched system `sha256sum` on 54 / 54 files.
- Repeated the native command into a distinct output path and compared the two JSON files byte for byte: identical.
- Changed the first byte of a copied `a=0` compressed object while the other 53 paths referenced clean downloads: exit nonzero, `compressed BLAKE3 mismatch`, no output file.
- `cargo test --release --offline --manifest-path research/koblitz_n83_factor_base_sweep_20261008/pilot-01/migration-inventory-native/Cargo.toml`: exit 0; this binary crate has zero unit tests. `rustfmt --check` and `git diff --check` passed.
- The nonfatal local `rust-objcopy` warning reports absent `libLLVM.dylib` in this Rust toolchain; the binary ran and the inventory was generated.

## Required repository library gate

Command: `TMPDIR=/Volumes/SSD990-2/tmp RUSTC=/Users/adamburan/.rustup/toolchains/1.99.1-aarch64-apple-darwin/bin/rustc CARGO_TARGET_DIR=/Volumes/SSD990/n83-symmetry-grid-v5-20261109/target CARGO_BUILD_JOBS=4 /Users/adamburan/.rustup/toolchains/1.99.1-aarch64-apple-darwin/bin/cargo test --release --lib -- --test-threads=2`.

Observed result: exit 101; **6,337 passed, 24 failed, 138 ignored**, test execution 1,212.39 s. The [complete stdout/stderr log](cargo-release-lib.log.gz) is gzip-compressed with `gzip -n`. Raw log SHA-256: `62bed3717b5c9095427d64a5a9adc380074d1bf50f5407be256e0a347559da64`; compressed log SHA-256: `989aa843fce8aca79fe657e2628e134c3d538ddae04741a6e8cf8f672f25f0b2`.

Failure classification, limited to this macOS checkout:

| Count | Existing test family | Observed cause or result |
|---:|:--|:--|
| 5 | `cryptanalysis::ecbench::band` / `compare` | The requested `research/curve25519_f1_targetwalk_20261108/sessions/k20-s0/records.jsonl` was absent and is not tracked in this checkout. |
| 4 | `cryptanalysis::ecbench::record::diagnostic_child_v2_tests` | The chosen `TMPDIR` has a group-writable nonsticky `/Volumes/SSD990-2` ancestor; the security guard rejected it. Focused rerun with `TMPDIR=/private/tmp` passed **7 / 7** tests. |
| 1 | `isogeny::class_group_subexp::tests::orders_and_subgroup_indices_are_consistent` | Asserted `210 == 105`; focused rerun reproduced the same failure. |
| 12 | `ml_dataset::report` and `ml_dataset::restore` | Safe worker reconciliation explicitly requires Linux `/proc`. |
| 2 | `ml_dataset` wide-prime central replay | `execution::create` reads `/proc/sys/kernel/random/boot_id`, absent on macOS. |

The [focused diagnostic log](cargo-diagnostic-child.log.gz) has SHA-256 `f2a5d5643411233f2ae1b01cca9e11e04de5e17190ec43bdfeaadcaaf312d7b6`; the [focused isogeny log](cargo-isogeny-recheck.log.gz) has SHA-256 `754ca35dbeba1583f3856bd7a27b1bd3cdfb94decb858508bdd78a98e4a30805`. The PR adds only the nested custody verifier, its inventory, and report files. The listed failures are outside the changed code and data; this is a path-based attribution, not a passing full-library result. Applicable CI on the eventual PR head remains the merge gate.
