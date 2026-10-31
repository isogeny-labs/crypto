#set page(paper: "a4", margin: 18mm)
#set text(size: 9.4pt)
= run-20261023T0038Z-quartic_twist_k1_census_p97_seed1_v1
Raw SHA-256: 35c418cfcf8f03c75fbc354854dc25b0d80138891736a1b5b3fc47c6cabebd79
Audit SHA-256: 5a71c18d2f2013cbed729ed996d207444541fa1efe81c49191ce0d199476a43b
Census protocol 09bf98c033; scalar-audit amendment 547a52330a; Shared Intel Xeon Platinum 6975P-C host; L0 exploratory run.
== Execution
`target/release/examples/quartic_twist_k1_census research/twist_index_calculus_20261020/quartic_twist_mixed_v1.ndjson 97 1`
8 GiB address-space cap; 1200 s watchdog.
== Recorded observations
- Exact point-keyed group census and independent scalar-keyed audit both pass: 32 signed k=1 relations across 458 frozen residuals; 427 have zero.
- First hit at zero-based query 19; uniform-occupancy heuristic 21.274 relations, observed/heuristic 1.504.
- Independent BSGS verified 72 physical factor logs and the target log using a 9410-point baby table; all 458 complete signed-set digests match.
- Point-keyed census charged 244,440 setup and 237,328,003 query Fp multiplications; these are reference-oracle costs, not polynomial IC costs.
#image("graph.svg", width: 100%)
Dashed expectation is a first-moment heuristic, not a fitted model or confidence interval. Group-oracle work is outside candidate IC cost.
