#set page(paper: "a4", margin: 18mm)
#set text(size: 9.4pt)
= run-20261023T0038Z-quartic_twist_k1_census_p73_seed1_aa_v1
Raw SHA-256: 3026abbfe9f20672381e7f69e011bc4926f5fc0a9e4fee11281ba4e7e8345c21
Audit SHA-256: f4301ea80526d7caa1968c112b6a72dffc0b0993fb17c5774b6343d881b82710
Census protocol 09bf98c033; scalar-audit amendment 547a52330a; Shared Intel Xeon Platinum 6975P-C host; L0 exploratory run.
== Execution
`target/release/examples/quartic_twist_k1_census research/twist_index_calculus_20261020/quartic_twist_mixed_v1.ndjson 73 1`
8 GiB address-space cap; 1200 s watchdog.
== Recorded observations
- Exact point-keyed group census and independent scalar-keyed audit both pass: 45 signed k=1 relations across 120 frozen residuals; 80 have zero.
- First hit at zero-based query 7; uniform-occupancy heuristic 56.111 relations, observed/heuristic 0.802.
- Independent BSGS verified 96 physical factor logs and the target log using a 5330-point baby table; all 120 complete signed-set digests match.
- Point-keyed census charged 437,664 setup and 109,335,363 query Fp multiplications; these are reference-oracle costs, not polynomial IC costs.
#image("graph.svg", width: 100%)
Dashed expectation is a first-moment heuristic, not a fitted model or confidence interval. Group-oracle work is outside candidate IC cost.
