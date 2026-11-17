#set page(paper: "a4", margin: 18mm)
#set text(size: 9.4pt)
= run-20261023T0033Z-quartic_twist_k1_scale_p73_seed0_aa_v1
Raw SHA-256: f186bd068ab2da3c3dcea5d308eacdca1b0a99b2ca49769fa976eda83ce2627f
Audit SHA-256: a470c2bc86f9cc9fad48024363aec812e7545912b53deff60ff085ec47f647e8
Source commit a815598a57; Shared Intel Xeon Platinum 6975P-C host; L0 exploratory run. Public target [271]G.
== Execution
`target/release/examples/quartic_twist_k1_scale research/twist_index_calculus_20261020/quartic_twist_mixed_v1.ndjson 73 0`
8 GiB address-space cap; 1200 s watchdog.
== Recorded observations
- Native status `pass`; independent audit `pass`; 15 of 117 frozen residuals; anchored rank 8/8; recovered 271.
- Independent exact k=1 signed sets: 8 reference, 8 polynomial, 0 missing; 0 algebraic system failures.
- Counted IC: 888,203,272 Fp multiplications and 9,988 group operations. Same-process twist24 rho: 496,841 Fp multiplications and 3,735 group operations.
- All 96 physical factor logs were independently computed with a 5330-point BSGS baby table; the separate exact oracle built 4512 signed L0 pair entries.
- Public generator anchor log: 28408032; the first signed physical factor may be the generator or its negation.
#image("graph.svg", width: 100%)
L0 wall is not isolated; hash/allocator CPU and calibrated GAE are absent. No priced IC/rho or external-solver claim.
