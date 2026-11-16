# Complete post-merge validation of PR #2815

The full default-feature root-library suite on merged source
`0044aaf9dfffa0f0f73d50ee2e5d141dc05b6036` completed successfully:
**5,143 passed, zero failed, 123 ignored, zero filtered out**, Cargo exit 0.
The native twist/factor-base package passed all ten tests, strict release
Clippy, and formatting. No source or lockfile was changed to obtain these results.

| Required check | Verified result |
| --- | --- |
| `cargo test --release --locked --lib` | 5,143 passed; 0 failed; 123 ignored; 0 filtered out; exit 0 |
| Native package release tests | 5 library + 2 binary + 3 CLI tests passed; exit 0 |
| Native strict release Clippy and formatting | Both exit 0 |

Root source tree: `8d449a2437c7ef52e29cf9aa84817e84ef3d6a75`.
Toolchain: Rust/Cargo 1.99.1, x86_64 Linux. Build jobs and test threads were
both four. Default features and Cargo's existing ignored-test behavior apply;
this is not an all-feature, all-target, or ignored-test run.

The root command ran from 2026-10-29T14:33:28Z to 15:23:26Z, including
compilation. Its terminal harness summary reports 2,305.35 seconds for tests.
The final isogeny-walk end-to-end test passed. A nested one-test subprocess
summary appears earlier in the log; it is not the full-suite result.
The terminal summary and captured Cargo exit 0 establish completion together.
The separate native checks finished at 15:23:40Z.
These are correctness-validation receipts, not speedup measurements or an
independent mathematical validation certificate.

## Reproduce

On source `0044aaf9dfffa0f0f73d50ee2e5d141dc05b6036`, apply the repository's
minimal sparse profile, including the inputs that earlier runs lacked:

```sh
bash scripts/sparse-checkout.sh apply --profile minimal \
  --path backlog/ --path gpu/certicom/ --path isogeny_algos/ \
  --path research/ecbench_bounds_challenges_20261006/
export CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=4
cargo test --release --locked --lib
cargo fmt --manifest-path tools/twist-relations/Cargo.toml --check
cargo clippy --release --locked --manifest-path tools/twist-relations/Cargo.toml --all-targets -- -D warnings
cargo test --release --locked --manifest-path tools/twist-relations/Cargo.toml
```

The original source checkout was clean before and after validation. A separate
worktree preserved the authoring workspace. Compilation used its existing
Cargo target cache; Cargo rebuilt the changed source and dependencies.
References reviewed before launch: `AGENTS.md`, the repository
`report-evidence` skill, `docs/sparse-checkout.md`,
`scripts/sparse-checkout.sh`, and the official
[Cargo test reference](https://doc.rust-lang.org/cargo/commands/cargo-test.html).

## Evidence and unsuccessful attempts

The repository's artifact job moved the archive into the existing artifact
store. Its tracked `.artifacts.json` retains the SHA-256 and 83,592-byte size;
both match the locally verified archive. First hydrate this report directory
using the [artifact-store pull instructions](../../docs/artifacts.md#quick-start)
with configured read access. Then extract and check the immutable raw records:

```sh
sha256sum -c runs.tar.xz.sha256
tar -xJf runs.tar.xz
(cd runs && sha256sum -c SHA256SUMS)
```

`runs/crypto-library-tests-0044-full.log` and
`runs/pr2815-full-library-exit.json` retain the full result and its
SHA-256 (`742047528fa4d6d16f999186a065f645c92ffc3ae7edac972a905286d115b477`).
`runs/pr2815-native-tests-0044-full.log` and the combined exit receipt retain
the native result. Metadata, exact launcher, input preparation and all record
hashes accompany them. The launcher records this workspace's absolute paths;
use the commands above for a new checkout.

The first attempt on the same merged source was killed during dependency
compilation. Its shell and Cargo both had Linux wait status 9, with no final
harness summary. The observed container OOM-kill counter was zero; the cause is
unknown. Its separate raw log, metadata and interruption receipt are retained.
The successful attempt used a tool-managed foreground session.

The older baseline `4c87143544eab488543cefacd59a3e86f59c6592` log remains
incomplete and is archived separately as `old-baseline-4c871-library-incomplete.log`.
Its previously observed SIGKILL and lack of a full summary are not reclassified
as a pass. The six-device and one-bounds rerun logs and original native/backlog
logs are also retained. This new result validates merged source `0044aaf9`,
not that older baseline or later revisions of main.

No assertion was weakened and no new skip or quarantine was introduced.
No unrelated failure was patched. Conductor was attempted for these documentation/evidence paths but is
unavailable (exit 127); no shared reservation is claimed. No new research
panel, cloud job, runner registration, funding action, performance admission,
leaderboard entry or mathematical claim was made.

The raw tree is a deterministic archive with run-date mtime, numeric owner/group
zero, GNU tar ordering and xz single-thread compression, per AGENTS.md. Repack:

```sh
tar --sort=name --mtime='2026-10-29 00:00:00Z' --owner=0 --group=0 \
  --numeric-owner --mode='u+rwX,go+rX,go-w' --format=gnu -cf - runs \
  | xz -6 -T1 > runs.tar.xz
```
