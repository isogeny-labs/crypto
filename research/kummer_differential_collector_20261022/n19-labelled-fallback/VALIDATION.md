# N19 reproduction and review data

The 21 frozen models and 1,260 public synthetic targets are the N11
source-verified cohort. The [protocol](PROTOCOL.md) was frozen before the
managed timed run. The full run's [receipt](results/cohort-receipt.json)
records UTC start/finish, exit code, unit name and primary validation mode;
[inputs.sha256](results/inputs.sha256) pins the executable, producer,
paired runner and protocol. `run_cohort.sh` uses an exclusive `flock` and
unsets target caps and diagnostic transition validation. The full run
finished with `completed=1260 solved=1260` and `paired_rho=1260` in
[cohort-run.log](results/cohort-run.log).

Large raw files are in the approved artifact store. After checkout, hydrate
them by running from the repository root:

```bash
python3 tools/artifacts/artifacts.py pull research/kummer_differential_collector_20261022/n19-labelled-fallback
python3 tools/artifacts/artifacts.py verify research/kummer_differential_collector_20261022/n19-labelled-fallback --remote
```

The manifests in `results/paired/{collector,rho}/.artifacts.json` and
`results/analysis/.artifacts.json` give immutable SHA-256 hashes and byte
counts. The 17 stored files include all 14 paired raw slice files,
`cost-records.jsonl`, `replay.jsonl`, and the 45,360-row
`target-price.csv`. The 288-row `price-grid.csv` and summary remain in
Git. Source model/transfer setup is the frozen
`../results/{cold-model-setup,cold-target-setup}.jsonl`; it is charged
once for each of the 1,260 targets.

Reproduce the checks and presentation from the hydrated bytes:

```bash
CARGO_TARGET_DIR=/dev/shm/n19-target TMPDIR=/dev/shm cargo test --release \
  --manifest-path research/kummer_differential_collector_20261022/Cargo.toml \
  --bin paired_labelled
python3 research/kummer_differential_collector_20261022/n19-labelled-fallback/analyze.py
python3 research/kummer_differential_collector_20261022/n19-labelled-fallback/validate.py
python3 research/kummer_differential_collector_20261022/n19-labelled-fallback/render_report.py
```

The Rust test checks 21/21 models, a forced zero xADD branch, a valid
degree-two zero-numerator divisor, the independent C2 chart oracle, and
three regular transitions per model. The preserved diagnostic
`results/gate/validated-target-16.jsonl` additionally has one real paired
solve with 556 charged transition checks; its rho control is
`results/gate/rho-target-16.jsonl`. The primary 1,260-target run has
`transition_validation_enabled=false` on every row. The independent
analyzer joins candidate, rho, archived source control and cold setup by
workload ID and recomputes source `kP=Q`; the separate validator checks
all 45,360 target/cell rows and 288 grid rows. Its machine receipt is
[validation.log](results/validation.log).

The first attempted test was run with Cargo's package working directory
and failed to locate the repository-relative theta input. The test now
sets the repository root explicitly; subsequent release test runs pass.
Strict `cargo clippy -D warnings` reports six pre-existing warnings in
the included, unchanged `walk_impl_uncached.rs` and
`recover_impl_uncached.rs`. Ordinary `cargo clippy` passes with those six
warnings and no new warning from N19 code. No timed run was launched
before the finite correctness gate passed.

`target-price.csv` has one row per public target and price cell; a failed
candidate would retain its cold cost and status with ratio fields null.
`price-grid.csv` has one row per bit slice or full cohort and price cell,
with `verified`, `failures`, `candidate_wins`, `candidate_losses`, and
`null_ratios`. Ratios divide sums of cold costs over the named slice.
The graph and PDF are derived from this grid; they do not overwrite it.
