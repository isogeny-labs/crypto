# Native contract validation receipt

Status: producer-side contract validation only; no experiment was launched and
no performance observation was made `[pcc:unverified]`.

## Commands and outcomes

```text
cargo test --manifest-path research/ghs_frontier_3_7_20261119/ws5_matched_ghs/Cargo.toml
2 passed; 0 failed

cargo run --quiet --manifest-path research/ghs_frontier_3_7_20261119/ws5_matched_ghs/Cargo.toml --bin ghs-m3-validate -- selftest
GHS M3 native validator self-test PASS

cargo run --quiet --manifest-path research/ghs_frontier_3_7_20261119/ws5_matched_ghs/Cargo.toml --bin ghs-m3-validate -- protocol protocol.json request_templates.json
{"calibrations":33,"diagnostic_runs":27,"grid_sizes":11,"launch_ready":false,"primary_runs":141,"status":"contract_valid"}

cargo run --quiet --manifest-path research/ghs_frontier_3_7_20261119/ws5_matched_ghs/Cargo.toml --bin ghs-m3-analyze -- selftest
GHS M3 native analysis self-test PASS
```

The native tamper matrix rejected changes to the first grid q, the composed
primary arm, the USD 5 ceiling, and the unbound request template's
`launch_ready` flag. It also rejects changing the absent runner to present or
claiming that the command bridge can earn L2 without a prospective protocol
change. The analyzer refused `protocol.json` as an analysis input with exit 2
and wrote no output.

## Source hashes at validation

```text
44edc2c54627130d57bac89baa28ed50f99e732e37512d78e62f9ee68338d8ce  protocol.json
3383ce7186d78eeaba8f3ecee7174f7eceb78b29d7ea16288e17dcc71fe82eff  request_templates.json
f3863ad8b9ad03c059ea8ec6f094ad9dfdd6da6591979547b8e092effe09de5f  src/lib.rs
24bd8436c49094a10e64c499a005c1c2a3a4e1a4b3e22addd67f07419ae9607e  src/bin/validate.rs
fc3e6d03a6cf886b315329249b092c4813e72a56c366ec3ac575aff23abc3bd7  src/bin/analyze.rs
```

These hashes identify this uncommitted validation pass. The parent integration
must rerun the checks and regenerate the hashes after any edit or source-bound
commit. They are not image, binary, execution, replay, or independent-validation
receipts.

## Remaining gates

- A source-bound runner that emits all required cold and online phase fields.
- Exact source, binary, runner, image, protocol, and request bindings.
- Live budget, checkpoint, all-tenant pool, quota, and Git-custody admission.
- The four split executions and sealed historical archive readback.
- Complete result-manifest acceptance, independent raw-record recomputation,
  replay, PCC standing, and publication regeneration.

