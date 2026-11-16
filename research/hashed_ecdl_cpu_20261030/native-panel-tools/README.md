# Native CPU panel validation and export

This Rust crate is the active correctness-test and export path for round18,
round19, round20 and round21. It replaces the Python execution paths identified in PR2980 review.

Round21 support was checked with all 12 synthetic tests using its frozen
protocol and analyzer, including unknown-study rejection. Logs are in
../round21/native-tests.log. This does not execute a benchmark or authorize one.
Historical Python source is preserved byte-for-byte under legacy/ for provenance;
it is not an execution entry point. The historical validation logs remain as
recorded, with their Python provenance visible.

The frozen analyzers, solver binaries, protocols, materialization manifests,
round18 results and reports are unchanged. The new exporter checks the analyzer
binary and source hashes, checks every returned manifest, then runs the frozen
native analyzer. It does not trust a retained PASS string. It creates export.json
exclusively and refuses to overwrite existing evidence. This means the completed
round18 directory must not be passed to it: its existing export is immutable.

## Tests

From the repository root, with the preserved executable bundles present:

```sh
export CARGO_TARGET_DIR=/tmp/native-panel-tools-target
for round in 18 19; do
  PANEL_STUDY="$PWD/research/hashed_ecdl_cpu_20261030/round$round" \
  ANALYZER="/home/ec2-user/hashed-ecdl-cpu-round$round/bundle/analyze" \
  cargo test --offline --locked --release \
    --manifest-path research/hashed_ecdl_cpu_20261030/native-panel-tools/Cargo.toml
done
```

The eleven synthetic tests cover the six original analyzer cases, the three
original export cases, stale-PASS rejection and analyzer-source mismatch.
Synthetic certificates are explicitly labeled, live in uniquely named temporary
directories and never execute a solver. The Rust suite invokes only the native
analyzer. No Python process is launched.

A separate, explicitly invoked historical parity test copies round18 records to
a temporary directory and regenerates the export there. All measurements,
per-run fields, flow counts and artifact hashes must equal the recorded export.
Only export creation time and producer notes are excluded from this comparison.
It does not write to the original results or execute a solver:

```sh
PANEL_STUDY="$PWD/research/hashed_ecdl_cpu_20261030/round18" \
ANALYZER=/home/ec2-user/hashed-ecdl-cpu-round18/bundle/analyze \
cargo test --offline --locked --release \
  --manifest-path research/hashed_ecdl_cpu_20261030/native-panel-tools/Cargo.toml \
  historical_round18_export_parity -- --ignored
```

## Future export

After an admitted round19 run has completed and its records have been downloaded,
build this crate with cargo build --offline --locked --release using the same
manifest and target directory. Invoke the binary with the study directory,
preserved analyzer path and actual export UTC timestamp:

```sh
/tmp/native-panel-tools-target/release/hashed-cpu-panel-tools \
  "$PWD/research/hashed_ecdl_cpu_20261030/round19" \
  /home/ec2-user/hashed-ecdl-cpu-round19/bundle/analyze \
  "$(date -u +%Y-%m-%dT%H:%M:%SZ)"
```

The adapter supports the fixed 24-column unquoted native rows format and exactly
six arms, three fixtures and ten rounds. It preserves the existing export schema
and full-cost boundary; it introduces no statistical inference.

## Validation and handoff

Round20 support was added under T-1028. Twelve synthetic tests now pass against
both round18 and round20, including an explicit unknown-study rejection and
assertions binding every exported backend reference to the actual study name.
New logs reside in ../round20/. Historical logs below remain unchanged. This
extension does not execute the solver or grant launch authorization.

Both eleven-test synthetic suites pass, and the historical parity test passes.
Strict all-target release Clippy and formatting pass. The first round19 native
test attempt failed because preserved binaries lacked executable permission;
permissions were corrected locally and binary hashes stayed unchanged.
No cloud benchmark was submitted. T-1016 owns these changes and the round18/19
documentation and entry-point migration. The account-cost decision remains
separate from this software correction.
