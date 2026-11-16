# run-20261023T0038Z-quartic_twist_k1_census_p97_seed1_v1

[Raw record](../../quartic_twist_k1_census_p97_seed1_v1.json) SHA-256 `35c418cfcf8f03c75fbc354854dc25b0d80138891736a1b5b3fc47c6cabebd79`; [independent audit](../../quartic_twist_k1_census_p97_seed1_audit_v1.json) SHA-256 `5a71c18d2f2013cbed729ed996d207444541fa1efe81c49191ce0d199476a43b`.

Census protocol commit `09bf98c033`; independent scalar-audit amendment `547a52330a`; [point-keyed producer](../../../../examples/quartic_twist_k1_census.rs) SHA-256 `36f5cc62c06bab71fb560ef386d985ec6af5576d16ee117c33c305831fddda80`; [scalar-keyed auditor](../../../../examples/quartic_twist_k1_census_audit.rs) SHA-256 `411cea2d8e4709a479b639f0eed783d1f1f17ddc2c6807a49f41cb626615f49d`.

## Execution

Shared Intel Xeon Platinum 6975P-C host; L0 exploratory run. Entire frozen group-oracle stream, including queries after polynomial IC stopping.

Command: `target/release/examples/quartic_twist_k1_census research/twist_index_calculus_20261020/quartic_twist_mixed_v1.ndjson 97 1` with 8 GiB address-space cap and 1,200 s watchdog.

## Recorded observations

- Exact point-keyed group census and independent scalar-keyed audit both pass: 32 signed k=1 relations across 458 frozen residuals; 427 have zero.
- First hit at zero-based query 19; uniform-occupancy heuristic 21.274 relations, observed/heuristic 1.504.
- Independent BSGS verified 72 physical factor logs and the target log using a 9410-point baby table; all 458 complete signed-set digests match.
- Point-keyed census charged 244,440 setup and 237,328,003 query Fp multiplications; these are reference-oracle costs, not polynomial IC costs.

![Exact signed occupancy and cumulative heuristic](graph.svg)

The dashed curve is a first-moment heuristic, not a fitted model or a confidence interval. Relations within a stream can be dependent. The group-oracle cost is outside the candidate polynomial IC pipeline.

[PDF](report.pdf).
