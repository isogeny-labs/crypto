# run-20261023T0038Z-quartic_twist_k1_census_p97_seed0_v1

[Raw record](../../quartic_twist_k1_census_p97_seed0_v1.json) SHA-256 `53cf0c91749939920717b6f4ae6503c8ec1bb1eda4a22c4e5b86c3fe3900594c`; [independent audit](../../quartic_twist_k1_census_p97_seed0_audit_v1.json) SHA-256 `aca4b60921ccd77cfa839feb467efbf28830d46e215332eb413dd5709d0852d0`.

Census protocol commit `09bf98c033`; independent scalar-audit amendment `547a52330a`; [point-keyed producer](../../../../examples/quartic_twist_k1_census.rs) SHA-256 `36f5cc62c06bab71fb560ef386d985ec6af5576d16ee117c33c305831fddda80`; [scalar-keyed auditor](../../../../examples/quartic_twist_k1_census_audit.rs) SHA-256 `411cea2d8e4709a479b639f0eed783d1f1f17ddc2c6807a49f41cb626615f49d`.

## Execution

Shared Intel Xeon Platinum 6975P-C host; L0 exploratory run. Entire frozen group-oracle stream, including queries after polynomial IC stopping.

Command: `target/release/examples/quartic_twist_k1_census research/twist_index_calculus_20261020/quartic_twist_mixed_v1.ndjson 97 0` with 8 GiB address-space cap and 1,200 s watchdog.

## Recorded observations

- Exact point-keyed group census and independent scalar-keyed audit both pass: 27 signed k=1 relations across 558 frozen residuals; 535 have zero.
- First hit at zero-based query 70; uniform-occupancy heuristic 25.919 relations, observed/heuristic 1.042.
- Independent BSGS verified 72 physical factor logs and the target log using a 9410-point baby table; all 558 complete signed-set digests match.
- Point-keyed census charged 244,440 setup and 289,137,634 query Fp multiplications; these are reference-oracle costs, not polynomial IC costs.

![Exact signed occupancy and cumulative heuristic](graph.svg)

The dashed curve is a first-moment heuristic, not a fitted model or a confidence interval. Relations within a stream can be dependent. The group-oracle cost is outside the candidate polynomial IC pipeline.

[PDF](report.pdf).
