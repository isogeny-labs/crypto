#set page(paper: "a4", margin: 18mm)
#set text(size: 9.4pt)
= run-20261023T0031Z-quartic_twist_k1_scale_p73_seed1_v1
Raw SHA-256: 6f970c1d0be37d0948ec3e2d3bac0fbc24f4ddb04801fa00b41651fb73360389
Audit SHA-256: 84a44f6bc0f11d7d50c0d6eff37aa28e49932bc72531923dd596269c8c0c3df7
Source commit a815598a57; Shared Intel Xeon Platinum 6975P-C host; L0 exploratory run. Public target [271]G.
== Execution
`target/release/examples/quartic_twist_k1_scale research/twist_index_calculus_20261020/quartic_twist_mixed_v1.ndjson 73 1`
8 GiB address-space cap; 1200 s watchdog.
== Recorded observations
- Native status `partial`; independent audit `partial`; 20 of 120 frozen residuals; anchored rank 8/8; recovered 271.
- Independent exact k=1 signed sets: 11 reference, 9 polynomial, 2 missing; 1 algebraic system failures.
- Counted IC: 1,183,617,228 Fp multiplications and 10,891 group operations. Same-process twist24 rho: 366,965 Fp multiplications and 3,030 group operations.
- All 96 physical factor logs were independently computed with a 5330-point BSGS baby table; the separate exact oracle built 4512 signed L0 pair entries.
- Public generator anchor log: 28408032; the first signed physical factor may be the generator or its negation.
#image("graph.svg", width: 100%)
L0 wall is not isolated; hash/allocator CPU and calibrated GAE are absent. No priced IC/rho or external-solver claim.
