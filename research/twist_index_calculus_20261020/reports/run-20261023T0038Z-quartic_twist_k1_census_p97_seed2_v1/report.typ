#set page(paper: "a4", margin: 18mm)
#set text(size: 9.4pt)
= run-20261023T0038Z-quartic_twist_k1_census_p97_seed2_v1
Raw SHA-256: 6ea6a83d7a5eadeede640935d10b053977dcf5339f0f5422302c7a9167803367
Audit SHA-256: 9af0b819cea1e5f7a8d17d5dfcf3df03a70268a39ad03eac501256038bc52e63
Census protocol 09bf98c033; scalar-audit amendment 547a52330a; Shared Intel Xeon Platinum 6975P-C host; L0 exploratory run.
== Execution
`target/release/examples/quartic_twist_k1_census research/twist_index_calculus_20261020/quartic_twist_mixed_v1.ndjson 97 2`
8 GiB address-space cap; 1200 s watchdog.
== Recorded observations
- Exact point-keyed group census and independent scalar-keyed audit both pass: 20 signed k=1 relations across 481 frozen residuals; 461 have zero.
- First hit at zero-based query 39; uniform-occupancy heuristic 22.343 relations, observed/heuristic 0.895.
- Independent BSGS verified 72 physical factor logs and the target log using a 9410-point baby table; all 481 complete signed-set digests match.
- Point-keyed census charged 244,440 setup and 249,233,829 query Fp multiplications; these are reference-oracle costs, not polynomial IC costs.
#image("graph.svg", width: 100%)
Dashed expectation is a first-moment heuristic, not a fitted model or confidence interval. Group-oracle work is outside candidate IC cost.
