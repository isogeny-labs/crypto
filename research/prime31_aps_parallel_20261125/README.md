# Prime31 APS row split: development parity record

Status: source-specific local development, `[pcc:unverified]`. This checks a
four-worker split of **one target's** factored S3 pair search on the existing
prime31 curve. It is not an isolated-host performance run or an APS/rho
crossover result. The serial path remains unchanged unless the new CLI flag
is used.

`public_targets.json` contains only the 48 public Q coordinates from
`../prime31_ic_20261115/native/APS_RHO_FOLD_SENSITIVITY_48Q_EXACT_CI_20261116.json`.
The caller-known scalar stays in that archive and is read only by
`verify_panel.py`, after the producer has finished. The producer receives
only the Q coordinates. `DEVELOPMENT_PANEL.json` preserves every case and
worker report, including assigned, started and unstarted rows, S3 pairs,
roots, candidate checks, verification additions and worker wall clocks.
The panel command compares serial and parallel witnesses on the same
generated labelled base; serial runs first, so its clocks are diagnostic.

Reproduce with the source in this PR, Rust 1.99.1 and the committed archive:

```sh
rustup run 1.99.1 cargo test --locked --release \
  --manifest-path research/prime31_ic_20261115/native/Cargo.toml \
  --example prime31_aps_one_relation
rustup run 1.99.1 cargo run --quiet --locked --release \
  --manifest-path research/prime31_ic_20261115/native/Cargo.toml \
  --example prime31_aps_one_relation -- --parallel-row-panel 4 \
  < research/prime31_aps_parallel_20261125/public_targets.json \
  > research/prime31_aps_parallel_20261125/DEVELOPMENT_PANEL.json
python3 research/prime31_aps_parallel_20261125/verify_panel.py \
  research/prime31_aps_parallel_20261125/DEVELOPMENT_PANEL.json \
  research/prime31_ic_20261115/native/APS_RHO_FOLD_SENSITIVITY_48Q_EXACT_CI_20261116.json
```

The native example's 24 release tests passed. The checker found 48/48
identical serial and parallel witness records, checked each against the
archived caller-only scalar and signed coefficient congruence, and checked
all four worker ranges and pair totals. It counted 1,569,408 serial S3 pairs
and 2,731,008 worker pairs, **1.740× as much pair work**. The row cutoff
prevents new rows at or beyond the earliest verified row known to a worker,
but in-flight rows and work before another worker publishes a witness are
charged. The output is scheduling-dependent in its work counts.
These checks are source-specific development evidence [pcc:unverified].

SHA-256 of the preserved output is
`e1529ea430731d57f8918b820fa6ffdb432d8e252df02feaa7b9b630a6a245f1`;
the input archive SHA-256 is
`d42f2ebc49f0c0192a07fefe8b7fe6654a21aee0f1c6082327724f2d313ca1ec`.
No process CPU-time or peak-memory data was recorded, and rho was not
parallelized in this run. A source-frozen, isolated, equal-core panel under
the [comparison protocol](../aps_parallel_equal_core_20261124/PROTOCOL.md)
is still needed.
