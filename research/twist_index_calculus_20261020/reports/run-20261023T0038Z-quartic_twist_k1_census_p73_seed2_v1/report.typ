#set page(paper: "a4", margin: 18mm)
#set text(size: 9.4pt)
= run-20261023T0038Z-quartic_twist_k1_census_p73_seed2_v1
Raw SHA-256: b692e36582a243f8fd8db27b5647a5c5a18905e36e9dee5730f9b3e49c9de408
Audit SHA-256: a2dd6f163554d6f2ca405fd15e6a5168472d43604c75d707ec7b0c5ce6c1db1d
Census protocol 09bf98c033; scalar-audit amendment 547a52330a; Shared Intel Xeon Platinum 6975P-C host; L0 exploratory run.
== Execution
`target/release/examples/quartic_twist_k1_census research/twist_index_calculus_20261020/quartic_twist_mixed_v1.ndjson 73 2`
8 GiB address-space cap; 1200 s watchdog.
== Recorded observations
- Exact point-keyed group census and independent scalar-keyed audit both pass: 45 signed k=1 relations across 122 frozen residuals; 88 have zero.
- First hit at zero-based query 1; uniform-occupancy heuristic 57.046 relations, observed/heuristic 0.789.
- Independent BSGS verified 96 physical factor logs and the target log using a 5330-point baby table; all 122 complete signed-set digests match.
- Point-keyed census charged 437,664 setup and 111,169,075 query Fp multiplications; these are reference-oracle costs, not polynomial IC costs.
#image("graph.svg", width: 100%)
Dashed expectation is a first-moment heuristic, not a fitted model or confidence interval. Group-oracle work is outside candidate IC cost.
