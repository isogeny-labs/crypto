# Clean build 01

Classification: SOFTWARE. Build the actual crypto CLI and native fixture
controller from the successful clean-source-package-01 root commit.

Admission: package receipt exit_code=0, source preflight/readback true, verified
bundle and package manifests, package archive fresh-download comparison, clean
Git identity, no concurrent Cargo invocation using this task's target directory.
The exact source closure must match snapshot 09; the package is a standalone
validation commit and does not represent upstream integration.

Fixed command: `cargo build --locked --release --bin crypto --example
dataset_startup_cli_fixture`. Rust 1.98; Cargo jobs 8; Rayon threads 8; shared
target `/home/ec2-user/crypto-ml-dataset-20261028/target`. Timeout 1800 seconds
plus 30 seconds TERM grace. Embed the actual package commit and dirty=false
only after verifying Git HEAD, index, file-set, byte hashes, and file modes.
Recheck all source bindings after the command, including on command failure.

On success copy both executables into a new retained output directory and
record SHA-256 and length. Do not execute dataset workers during this step.
Stop on failure; retain the command log and structured receipt. No automatic
retry, source rewrite, fixture-gate override, or substitution of prior test
receipts. Wall duration is diagnostic and not a comparative benchmark.

Subsequent work: actual tests, maintained clippy, official protocol schemas,
and runtime preflight receipts for this source/binary before the separately
bounded 26-cell native acceptance workflow. Full-library timeout 01 remains
a retained regression limitation, not a passed gate.
