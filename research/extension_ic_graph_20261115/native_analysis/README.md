# Native local comparison analysis

This Rust program checks the complete frozen execution schedule, source and
instance identity, graph/pivot relation fingerprints, scalar replays, planned
warmups/rounds and both A/A controls. Failed and incomplete blocks remain in
spent-work diagnostics and invalidate their stratum. It counts duplicate raw
executions separately from unique execution keys. [pcc:unverified]

```sh
cargo test --release --manifest-path research/extension_ic_graph_20261115/native_analysis/Cargo.toml
cargo build --release --manifest-path research/extension_ic_graph_20261115/native_analysis/Cargo.toml
research/extension_ic_graph_20261115/native_analysis/target/release/cubic-graph-analysis \
  RAW.jsonl RUNTIME-PLAN.json NEW-OUTPUT-DIR
```

The runtime plan derives from `../local-pilot-plan.json` and pins the actual
published source and native binary hash before launch. The output directory
must not exist. Outputs are `summary.json` and a Plotters SVG with paired
geometric means and deterministic bootstrap intervals over rounds.

`large_order_local_reference_parity` requires all planned data, successful
replays, matching inputs, passing local A/A timing controls, a subgroup order
at least the frozen minimum and an upper interval at or below one. It is
restricted to the existing cubic r-adding reference; it does not establish
fastest-external comparison, standard EKS/L2 admission or a general crossover.
Small fixtures cannot trigger that field. [pcc:unverified]

Seven tests cover retained warmups, rejection of tiny-instance parity,
failed/incomplete cohorts, source/instance/duplicate/parse problems, and
fingerprint/A/A failures, sampler metadata and paired-oracle stream identity.
An optional frozen `paired_oracles` list checks equal input digests, group work,
residual counts and recovered scalars across arithmetic implementations while
allowing different field-work counts. Per-oracle SVGs accompany the complete
plot. Numerical estimator assertions use a floating-point
tolerance; scientific acceptance thresholds are unchanged. [pcc:unverified]
