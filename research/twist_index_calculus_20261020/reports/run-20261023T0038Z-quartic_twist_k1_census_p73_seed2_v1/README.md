# run-20261023T0038Z-quartic_twist_k1_census_p73_seed2_v1

[Raw record](../../quartic_twist_k1_census_p73_seed2_v1.json) SHA-256 `b692e36582a243f8fd8db27b5647a5c5a18905e36e9dee5730f9b3e49c9de408`; [independent audit](../../quartic_twist_k1_census_p73_seed2_audit_v1.json) SHA-256 `a2dd6f163554d6f2ca405fd15e6a5168472d43604c75d707ec7b0c5ce6c1db1d`.

Census protocol commit `09bf98c033`; independent scalar-audit amendment `547a52330a`; [point-keyed producer](../../../../examples/quartic_twist_k1_census.rs) SHA-256 `36f5cc62c06bab71fb560ef386d985ec6af5576d16ee117c33c305831fddda80`; [scalar-keyed auditor](../../../../examples/quartic_twist_k1_census_audit.rs) SHA-256 `411cea2d8e4709a479b639f0eed783d1f1f17ddc2c6807a49f41cb626615f49d`.

## Execution

Shared Intel Xeon Platinum 6975P-C host; L0 exploratory run. Entire frozen group-oracle stream, including queries after polynomial IC stopping.

Command: `target/release/examples/quartic_twist_k1_census research/twist_index_calculus_20261020/quartic_twist_mixed_v1.ndjson 73 2` with 8 GiB address-space cap and 1,200 s watchdog.

## Recorded observations

- Exact point-keyed group census and independent scalar-keyed audit both pass: 45 signed k=1 relations across 122 frozen residuals; 88 have zero.
- First hit at zero-based query 1; uniform-occupancy heuristic 57.046 relations, observed/heuristic 0.789.
- Independent BSGS verified 96 physical factor logs and the target log using a 5330-point baby table; all 122 complete signed-set digests match.
- Point-keyed census charged 437,664 setup and 111,169,075 query Fp multiplications; these are reference-oracle costs, not polynomial IC costs.

![Exact signed occupancy and cumulative heuristic](graph.svg)

The dashed curve is a first-moment heuristic, not a fitted model or a confidence interval. Relations within a stream can be dependent. The group-oracle cost is outside the candidate polynomial IC pipeline.

[PDF](report.pdf).
