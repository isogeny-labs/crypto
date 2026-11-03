# Validation and execution record

Host: Linux 6.18.54-121.166.amzn2023.x86_64, x86_64. Structural runs on
2026-10-18 UTC; no isolation tier or performance comparison is claimed.
Rustc 1.99.1 (58cf8f9c3 2026-10-13). Lean 4.34.1
(5045d0056413266e57c625dcd7c365b10e377c52). Dependencies are pinned in
`native/Cargo.lock` and `lean-toolchain`.

Experiments used source revision
`4fdf4f39d2af49e758cc4d03a9c6b338a0253468`. The PR is transplanted onto
main to exclude unrelated experiment commits. Imported native research
modules are byte-identical on main and that experiment source revision.
Raw logs and the first large certificate encoding are bundled in
`evidence.tar.gz` (digest in `evidence.sha256`). Receipt names below are
paths within that archive. Extract to a fresh scratch directory:

```sh
mkdir /tmp/koblitz-carrier-receipts
tar -xzf evidence.tar.gz -C /tmp/koblitz-carrier-receipts
```

| Check | Result | Receipt |
| --- | --- | --- |
| Native standalone release tests | 34 passed, 0 failed | `native-tests-registered-order.log` |
| Whole-library release tests, sandboxed | 4,371 passed, 9 local-socket PermissionDenied failures, 117 ignored | `root-tests.log` |
| Whole-library release tests, localhost enabled | 4,380 passed, 0 failed, 117 ignored | `root-tests-unsandboxed.log` |
| Prime-character scan, ell<=10,000 | PASS, ten n values and both orders | `results.json` |
| Norm vs monic-division integer coefficients | 20/20 equal, including both n=131 signs | `results.json` |
| Generalized-kernel complete coset controls | 6/6 agree | `results.json` |
| Klein extension counts | 8/8 agree | `results.json` |
| Composite n=9 control | dimension 8 vs primitive dimension 6 | `results.json` |
| Cyclic-window integer replay | 30 RH/spectral candidates, 0 survive the branch constraint | `cyclic-window.json`, `cyclic-run.log` |
| Frozen scan replay | byte-for-byte equality | `replay.log` and `cmp` commands below |
| Large certificate round trip and replay | PASS, IDC1h650837b4b977a1d0 | `large-identity-balanced.log`, `replay.log` |
| Conditional Lean theorem | PASS; propext, Classical.choice, Quot.sound only | `lean-check.log` |
| PDF review | all pages rendered and visually inspected | `report.pdf`, `report.typ`, `build_report.py` |

The first Rust build failed because it treated the certificate verdict enum
as a struct with an `accepted` field. This compile error was corrected to
match `Verdict::Accept`; no research output was emitted by that build.
The first Lean elaboration used a tactic unavailable in Std; its failure
is preserved in `lean-attempt-01.log`. The successful file contains no
`sorry`, `admit`, or custom axioms. Its conclusion is conditional on the
explicit RH, spectral and branch hypotheses, not a formal geometry theorem.

The initial large-certificate Horner encoding was valid but produced a
92 MB pretty-printed file. Its exact bytes are preserved compressed in
`archive/`. The balanced compact certificate is JSON-round-trip checked
and rechecked by the same repository verifier. This is an encoding change,
not a different mathematical result.

The Python file only lays out recorded values and renders the report.
All cryptographic arithmetic, scans, verification, and evidence generation
use Rust. Document tools: typst 0.15.0, pymupdf 1.28.2, in a temporary
virtual environment. Offline installation was unavailable; installation
then succeeded with network access. No managed-environment status tool or
`/etc/codex/network-policy.json` was exposed in this workspace.

## Commands

```sh
cargo run --release --offline --locked --manifest-path research/koblitz_modular_ic_20261018/native/Cargo.toml -- research/koblitz_modular_ic_20261018
cargo test --release --offline --locked --manifest-path research/koblitz_modular_ic_20261018/native/Cargo.toml
CARGO_TARGET_DIR=/home/ec2-user/crypto/target cargo test --release --lib --offline --locked
cargo run --release --offline --locked --manifest-path research/koblitz_modular_ic_20261018/native/Cargo.toml -- --cyclic-only research/koblitz_modular_ic_20261018/cyclic-window.json
cargo run --release --offline --locked --manifest-path research/koblitz_modular_ic_20261018/native/Cargo.toml -- --large-identity research/koblitz_modular_ic_20261018/identity-n131-balanced.json
cargo run --release --offline --locked --manifest-path research/koblitz_modular_ic_20261018/native/Cargo.toml -- /tmp/koblitz-carrier-final-replay
cmp research/koblitz_modular_ic_20261018/results.json /tmp/koblitz-carrier-final-replay/results.json
cmp research/koblitz_modular_ic_20261018/identity-n131-balanced.json /tmp/koblitz-carrier-final-replay/identity-n131-balanced.json
cd research/koblitz_modular_ic_20261018
lean CyclicWindow.lean
```

The first commands require fresh output paths; the driver refuses to
overwrite frozen result files. Source digests of imported implementations:

```text
9f1148e1ecd60e8d77470380639c8dd73856bb22ae2163a2fca23d6e37ddabf4  src/cryptanalysis/curve_construction/weil.rs
048d106dec860cfbd5ef7b0a49095132c35c07a3ea2265e4c3995dfd5c970871  src/cryptanalysis/curve_construction/modular.rs
5c65e8789ce137643e7d256b11b0d1fca9a7770b34d492527749d8ff8efe011e  src/cryptanalysis/curve_construction/gf2k.rs
d81ee915ad769a87d7aac5d9664631239607f4a0deb27850cdb52cea84e34270  src/cryptanalysis/identity_certificate.rs
```

CI adds a bounded native replay and pinned Lean check. Existing main
programs, examples, benchmark data, and ledger rows are untouched.
