#set page(paper: "a4", margin: 18mm)
#set text(size: 9.4pt)
= run-20261023T0038Z-quartic_twist_k1_census_p73_seed0_v1
Raw SHA-256: 2c49e75469b6d8a13ba33a3e9575435e79f9749107118a2a01a581e8e0698366
Audit SHA-256: d73a65fe047db4624294ee57a20944391647fac7030a25d1607e89250358d962
Census protocol 09bf98c033; scalar-audit amendment 547a52330a; Shared Intel Xeon Platinum 6975P-C host; L0 exploratory run.
== Execution
`target/release/examples/quartic_twist_k1_census research/twist_index_calculus_20261020/quartic_twist_mixed_v1.ndjson 73 0`
8 GiB address-space cap; 1200 s watchdog.
== Recorded observations
- Exact point-keyed group census and independent scalar-keyed audit both pass: 77 signed k=1 relations across 117 frozen residuals; 57 have zero.
- First hit at zero-based query 1; uniform-occupancy heuristic 54.708 relations, observed/heuristic 1.407.
- Independent BSGS verified 96 physical factor logs and the target log using a 5330-point baby table; all 117 complete signed-set digests match.
- Point-keyed census charged 437,664 setup and 106,603,795 query Fp multiplications; these are reference-oracle costs, not polynomial IC costs.
#image("graph.svg", width: 100%)
Dashed expectation is a first-moment heuristic, not a fitted model or confidence interval. Group-oracle work is outside candidate IC cost.
