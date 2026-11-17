#set page(paper: "a4", margin: 18mm)
#set text(size: 9.4pt)
= run-20261023T0038Z-quartic_twist_k1_census_p97_seed0_aa_v1
Raw SHA-256: 53cf0c91749939920717b6f4ae6503c8ec1bb1eda4a22c4e5b86c3fe3900594c
Audit SHA-256: aca4b60921ccd77cfa839feb467efbf28830d46e215332eb413dd5709d0852d0
Census protocol 09bf98c033; scalar-audit amendment 547a52330a; Shared Intel Xeon Platinum 6975P-C host; L0 exploratory run.
== Execution
`target/release/examples/quartic_twist_k1_census research/twist_index_calculus_20261020/quartic_twist_mixed_v1.ndjson 97 0`
8 GiB address-space cap; 1200 s watchdog.
== Recorded observations
- Exact point-keyed group census and independent scalar-keyed audit both pass: 27 signed k=1 relations across 558 frozen residuals; 535 have zero.
- First hit at zero-based query 70; uniform-occupancy heuristic 25.919 relations, observed/heuristic 1.042.
- Independent BSGS verified 72 physical factor logs and the target log using a 9410-point baby table; all 558 complete signed-set digests match.
- Point-keyed census charged 244,440 setup and 289,137,634 query Fp multiplications; these are reference-oracle costs, not polynomial IC costs.
#image("graph.svg", width: 100%)
Dashed expectation is a first-moment heuristic, not a fitted model or confidence interval. Group-oracle work is outside candidate IC cost.
