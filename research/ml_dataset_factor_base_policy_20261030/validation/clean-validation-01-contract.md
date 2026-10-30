# Clean package validation 01

Classification: SOFTWARE readiness checks for the retained CLI build. Freeze
before execution. Source package commit:
`1d858fb9e4b8035b38f93db4423880627b09211f`.
Build receipt SHA-256:
`0473dc80b618a89ce38d0f3b91c8d5cf6943f28f6e9e3963d604fc968abf4867`.

Run sequentially, stopping on the first failed phase:

1. `cargo test --locked --release --example dataset_startup_cli_fixture -- --test-threads=1`
   Require exactly 31 passed, 0 failed, 0 ignored.
2. `cargo test --locked --release --lib ml_dataset:: -- --test-threads=1`
   Require exactly 479 passed, 0 failed, 2 ignored. The two pre-existing large
   panels stay explicitly ignored; this is not their execution evidence.
3. `bash scripts/clippy-maintained.sh`
   Require exit 0 under the package's maintained target policy and strict lint
   flags. Do not exclude an additional target or weaken lint rules to pass.

Rust 1.98; 8 Cargo jobs; 8 Rayon threads; one test thread; shared target directory
`/home/ec2-user/crypto-ml-dataset-20261028/target`. One own Cargo command at a time.
Each phase has a 1800-second command timeout and 30-second termination grace.
No automatic retry. Retain failure output and source checks on failure.

Bind each phase to the successful build receipt, actual package commit, exact
source manifests, clean Git status, original executable hashes, runner and
guard hashes. Check them before and after execution. Record precise commands,
exit codes, raw output, and terminal test summaries. A zero-test or changed-count
result is not accepted without diagnosis and a prospective contract revision.

The retained executables live outside the shared Cargo target and must remain
unchanged. These checks qualify their source package; test harnesses are separate
compiled executables. Do not relabel old dirty-overlay test receipts as new ones.
Do not generate cells or grant cloud execution in these phases. The previous
full-library timeout remains separate; this plan does not repeat that suite.
