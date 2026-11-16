# Controller v3 validation status

Source is an isolated sparse worktree based on `bb1b6ab5f9c507a05a0f2dfac0eb45cd453b82d0`.
The changed source has not yet been committed or pushed. Rust 1.99.1 is set
by an explicit `RUSTC` path, Cargo uses `--release --locked --offline -j 1`,
and `CARGO_TARGET_DIR` and `TMPDIR` are under `/Volumes/SSD990/tmp/`.

| Attempt | Command or gate | Observed result |
| --- | --- | --- |
| 1 | `cargo test --release --example n83_k600_m6_budget --locked --offline -j 2` | Exit 101 before compile: sparse checkout omitted `tools/factor-base/Cargo.toml`. |
| 2 | Same after adding `tools/factor-base` | Exit 101 before compile: omitted `tools/volcano-cli/Cargo.toml`. |
| 3 | Same after adding `tools` | Exit 101: default Rust 1.93.1 is below the AWS SDK dependency minimum 1.94.1. |
| 4 | Explicit Rust 1.99.1, one Cargo job | Exit 130 during dependency compilation as boot free space fell to about 3.0 GiB. No test verdict. |
| 5 | Explicit Rust 1.99.1, one Cargo job, `TMPDIR` on secondary volume | Exit 101 while compiling the library: sparse checkout omitted six tracked `include_str!` inputs in `docs/ic`, `docs/ecbench` and `docs/curves`. No example test verdict. |
| 6 | Same after adding those tracked `docs/` paths | Exit 130 during `crypto` compilation when concurrent Rust work drove boot free space below the 5 GiB host floor. No example test verdict. |
| 7 | Exact controller source copied byte-for-byte into a minimal offline Rust crate with repository `serde_json=1.0.151` and `sha2=0.10.9`; `cargo test --release --offline -j 1` | Exit 0, six controller tests passed. Source copy SHA-256 `c1b517e582401fd09a28587d5ed03411d0056ed33a06ec13c1097cb42cb12df0` matched the worktree source at launch. macOS `rust-objcopy` emitted a missing-`libLLVM.dylib` strip warning, but the test binary ran and passed. This is a focused code check, not the required repository release suite. |
| 8 | Repository `cargo test --release --example n83_k600_m6_budget --locked --offline -j 1` with explicit Rust 1.99.1 and boot guard | Exit 0 after 11m 01s; all six example tests passed. The library emitted 57 pre-existing warnings, and macOS `rust-objcopy` warned about missing `libLLVM.dylib` while stripping; the executable test binary ran successfully. No boot-stop marker was created. |
| 9 | Repository `cargo test --release --lib --locked --offline -j 1` in the sparse checkout | Stopped with exit 130 before a test verdict after the compiler reported 45 missing tracked `include_str!` research fixtures. `git sparse-checkout add research` then materialized those files, including `research/efd_coordinate_charts_20261108/Charts.lean`. The stop preserved boot margin and the partial target. |
| 10 | Same full library command after adding `research` | Stopped with exit 130 before a test verdict after five further missing tracked `include_str!` fixtures appeared under `docs/units`, `docs/identity-certificates`, and `tests/fixtures`. The full tracked `docs` and `tests` directories were materialized, and all five paths were checked before retry. |
| 11 | Repository `cargo test --release --lib --locked --offline -j 1` after materializing tracked `research`, `docs`, and `tests` | Exit 101 after 538.86 test seconds: 6,340 passed, 25 failed, 138 ignored. The compressed raw log is retained in the [artifact manifest](.artifacts.json); this is a failed macOS suite, not a passing gate. Five failures require tracked `gpu/certicom` files omitted by this sparse checkout; five require an ignored 2,293,097-byte targetwalk record that is archived in S3; twelve require Linux `/proc`; one wide-prime test reached its deadline; and two wide-prime tests reported missing files. None of those modules or fixtures is changed by this controller patch. |

`rustfmt --edition 2021` and `git diff --check` passed on the source before
attempt 6. The missing paths were materialized from the pinned commit using
Git sparse checkout. Attempts 4 and 6 retained their partial Cargo target;
neither is a passing test. No arm solver, relation search, rank calculation,
or S3 write was launched by this controller validation.

The [full-fixture Linux release-library result](../k600-colima-full-fixture-20261116/RESULT.md)
passed 6,440 tests with zero failures and 138 ignored on source
`a702129de8e5`. `git diff --name-only a702129de8e5
bb1b6ab5f9c507a05a0f2dfac0eb45cd453b82d0 -- src Cargo.toml Cargo.lock tests gpu tools`
returned zero files. This patch changes only the example and this contract
directory, so it does not alter library or test code. The older Linux run is
source-equivalent for those paths but is not a green validation of this new
commit and its full fixture set. The macOS deadline and missing-file failures
remain failed observations; their individual root causes have not all been
replayed under Linux at this commit.

The attempt-11 raw log is 617,713 bytes, SHA-256
`84659ddd4fb4eee71a318776f3c745dbf164d3089a1be78abd32bddef3dcd7f2`.
Its deterministic `gzip -n` copy is 118,476 bytes, SHA-256
`105fe96e6339fa34d3044ba96e9dbd2424d9bb48081f2f1a08ff6ba5d02e4a7b`;
decompression reproduced the raw hash. Artifact migration commit
`2378a90aef5f` replaced the Git blob with the [manifest](.artifacts.json).
The durable object is
`s3://setec-astronomy-artifacts/store/sha256/10/105fe96e6339fa34d3044ba96e9dbd2424d9bb48081f2f1a08ff6ba5d02e4a7b`.
A fresh `GetObject` through profile `ecc-challenges-new` returned 118,476
bytes with the expected compressed SHA-256; decompression reproduced the raw
SHA-256 above. Retrieve with `aws s3 cp` from that exact object key to a new
local file, then check its SHA-256 before decompressing. This custody check
does not change the failed validation result.

Next gate: review the exact diff and carry this validation status into the PR.
Run a clean full-fixture Linux suite on the new head when host resources permit;
do not report the macOS suite as passing or infer a factor-base runtime from
the controller tests. The passing example test establishes controller behavior
only; it does not launch the K600 arm.

The later merge of `main` and its exact validation are recorded separately in
[MERGE_VALIDATION.md](MERGE_VALIDATION.md); the attempts above remain the
pre-merge history and are not rewritten as merged-head results.
