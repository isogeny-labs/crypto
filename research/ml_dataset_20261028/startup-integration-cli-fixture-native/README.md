# Native release CLI software fixture

This is the Rust successor to the frozen historical Python fixture at
`../startup-integration-cli-fixture/`. That earlier protocol, runner, manifest
and receipts remain unchanged evidence. Use this native runner for new checks.
No Python process performs cryptographic work or orchestration here.

The fixed software coverage remains two local prime-curve cells: n=7, m=3,
F4, seed42, curve_seed7, generator_index1, key_index1, both twists. The plan
has 64 cells; only shards62/64 and63/64 execute with max_cases1, factor-base2,
degree bound4 and work limit512. Each native worker retains its 20-second,
512MiB and 16MiB polled trace limits. Each CLI command has a 45-second
watchdog; the whole command sequence has a 240-second deadline. These are
software termination controls, not performance measurements or campaign budgets.

The ten real CLI commands cover plan, preflight, generation/verify/status for
both cells, hash-only status on a copied cell, then semantic rejection of that
copy after restricting an auxiliary domain and changing one S3 coefficient.
The copy's artifact hashes and sizes are recomputed before rejection. Exact
source/binary/plan/protocol/run/configuration bindings and all original outcome
scope checks are required. Neither a PDP solution nor a resampled target is
required; no cell enters the research corpus. Startup activation, cloud jobs,
publication, training, probability, ECDLP recovery and speed are outside scope.

Commit this successor's complete manifest and protocol before execution. The
manifest binds README.md, protocol.json and run.rs. The protocol independently
binds the native runner source and its canonical body excluding `freeze`.
All protocol numbers are integers. Execution refuses changed files, symlink
artifacts, invalid identities, an existing output directory and a dirty or
wrong-source native preflight. The supplied fixture commit is a declaration;
the caller must verify that exact Git filing separately.

```sh
cargo +1.98.0 build --release --example dataset_startup_cli_fixture
cargo +1.98.0 test --release --example dataset_startup_cli_fixture
target/release/examples/dataset_startup_cli_fixture \
  --fixture /absolute/checkout/research/ml_dataset_20261028/startup-integration-cli-fixture-native \
  --binary /absolute/path/to/immutable/crypto \
  --binary-sha256 ACTUAL_64_HEX_BINARY_SHA256 \
  --source-commit ACTUAL_40_HEX_BINARY_SOURCE_COMMIT \
  --fixture-commit ACTUAL_40_HEX_FIXTURE_COMMIT \
  --scratch /absolute/existing/scratch-parent/new-native-fixture-run
```

The Rust runner gives children only PATH, HOME, USER, LOGNAME, fixed locale,
RAYON_NUM_THREADS=1 and its private TMPDIR. It never adds publish, remote or
budget flags. This is a local execution selection, not a firewall proof.
SIGINT/SIGTERM handlers set an atomic cancellation flag rather than unwinding
across process creation. Watchdog/cancellation kills and reaps the child
process group immediately; unlike the historical runner, this successor has
no five-second termination grace. A drop guard also cleans up on wait errors.
SIGKILL, host loss or storage failure can prevent a final receipt and require
explicit reconciliation. There is no automatic retry or resume.

Before work, the runner makes a fresh read-only fixture copy and writes its
execution-inputs.json. Every command retains argv, stdout/stderr, hashes,
Unix-second timestamps, exit status, timeout/cancellation and binary identity.
The final v2 receipt includes both cells, the tamper control and a full file
inventory. Every outcome remains a software observation with zero corpus rows
and zero verified cloud dataset cells. Failed attempts retain their prefix.

Sources reviewed: the historical runner and its frozen protocol,
`src/ml_dataset/mod.rs` preflight/replay/status and native command contracts,
and [the existing prime-chain literature/encoding contract](../../../docs/ml-dataset/PRIME-F4-CHAIN-CONTRACT.md).
This is a native language repair of known finite software coverage; it adds no
mathematical mechanism, timing comparison or scientific validation promotion.
Actual execution results are filed separately after this preparation commit.
