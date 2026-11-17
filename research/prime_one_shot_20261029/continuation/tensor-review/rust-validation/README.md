# Fresh native tensor-crate development validation

2026-10-30 UTC. **Pass:34 tests,0 failures,0 ignored**, using official Rust1.78.0
and the unchanged committed lock. Binary tests and doctests each contain0 tests.
This is a fresh run of existing development controls, not an independent general
implementation proof or an asymptotic/performance result.

## Source and toolchain

Source commit:6b6f42a2de5f1ddbbc2cb4bbf0a3fe072248d248.
Crate tree:712d83360e07bf9fe321c11d3367653949bd9af7.
All24 isolated source files match their recorded Git blob IDs and SHA256s.
The source crate declares MSRV1.78; no root rust-toolchain pin was found.

Cargo/rustc were actually absent from PATH/inspected standard locations. I
installed an unprivileged minimal official1.78.0 toolchain below
/workspace/audit/toolchains, after checking rustup-init against the official
SHA256. Used --no-modify-path; no shell startup, system identity or shared
repository source/index/branch changes. The install and version receipts remain.

Official documentation read before installation:

- https://rust-lang.org/tools/install/
- https://rust-lang.github.io/rustup/installation/index.html
- https://rust-lang.github.io/rustup/installation/other.html

Official rustup-init URL:
https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init
and the adjacent .sha256 file. Verified SHA256:
5be191e8c70a5007074254cb74d78dbfdd25cd5a61c27de7b8e5410122ec24f4.

## Actual execution

From the isolated, blob-verified crate copy, with explicit CARGO_HOME,
RUSTUP_HOME and CARGO_TARGET_DIR below the audit prefix:

    CARGO_BUILD_JOBS=2 RUST_TEST_THREADS=1 cargo test --locked --release

Exit0. The34 controls include independent truth-table/direct contraction
comparisons, nested and coupled creation-precision cases, tapered Hermite,
weighted enumeration, exact Fourier arithmetic, canonical modular words,
point addition, public hash membership, cofactor projection, bounded primality,
unknown/resource errors and a tiny relation certificate equality/replay test.
No tests were skipped or altered; no source or lock change was needed.

A later documented public tiny CLI fixture also finished exit0 and emitted a
receipt reporting verified scalar4. Input: p5,a2,b1,r7,cofactor1,P(0,1),Q(3,2),
slots2,fixture_seed1,stepcap2000000000. The exact input, log and receipt are
retained. A separate verify invocation was not run; finalization was requested
before any further CLI calls. Neither published challenge was attempted.

## Chronology and limits

[EXECUTION.md](rust-tensor-validation/EXECUTION.md) is a **retrospective execution
receipt**. Earlier planning notes were written before the tests but updated
with a CLI addendum after the suite passed; their combined final file must not
be described as a prospectively frozen experiment protocol. These standard
local development checks are not a scientific timing campaign.

The first git archive stalled on a promisor fetch and was interrupted130.
Pinned public raw retrieval then succeeded with all24 blob checks. No source
failure was hidden. The root library suite was not run: its manifest, lock and
src/lib.rs are unmaterialized, with1028 src files and four path dependencies.
That larger build was assessed but not started without parent coordination.
The toolchain absence itself has now been fixed.

No full theorem/implementation certification, scaling claim, EKS admission,
scientific timing, speedup or challenge scalar is established by these tests.
In particular, forced small recursion and Functional finite-size controls do not
execute the contraction theorem at its admitted asymptotic sizes.

## Retained artifacts

Under rust-tensor-validation/: validation.json, EXECUTION.md,
cargo-test-release.log/.exit, tiny-cli-solve.log/.exit, tiny-cli-receipt.json,
source-commit.txt, source-tree.txt, source-blobs.txt, source-download.log,
source-files.sha256, rustc-version.txt, cargo-version.txt,
toolchain-binaries.sha256, executed-binaries.sha256 and execution-files.sha256.
The full isolated source, target binaries and official toolchain remain outside
Git; publication needs only report/logs/metadata plus the pinned source links.

Use the toolchain without changing global PATH:

    CARGO_HOME=/workspace/audit/toolchains/cargo
    RUSTUP_HOME=/workspace/audit/toolchains/rustup
    /workspace/audit/toolchains/cargo/bin/cargo
