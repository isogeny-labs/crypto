#set page(paper: "a4", margin: 18mm)
#set text(size: 9.4pt)
= run-20261023T0032Z-quartic_twist_k1_scale_p73_seed2_v1
Raw SHA-256: 05a7fc6c5bbee79fe3dbc24695b4f3517d48b09522e099dbd62c7a4b9e7e5ca1
Audit SHA-256: a476a5095f5a8fd7f6861126281e4a872d32484962e26773e919455f091cab42
Source commit a815598a57; Shared Intel Xeon Platinum 6975P-C host; L0 exploratory run. Public target [271]G.
== Execution
`target/release/examples/quartic_twist_k1_scale research/twist_index_calculus_20261020/quartic_twist_mixed_v1.ndjson 73 2`
8 GiB address-space cap; 1200 s watchdog.
== Recorded observations
- Native status `pass`; independent audit `pass`; 43 of 122 frozen residuals; anchored rank 8/8; recovered 271.
- Independent exact k=1 signed sets: 15 reference, 15 polynomial, 0 missing; 0 algebraic system failures.
- Counted IC: 2,542,727,892 Fp multiplications and 15,084 group operations. Same-process twist24 rho: 481,339 Fp multiplications and 3,619 group operations.
- All 96 physical factor logs were independently computed with a 5330-point BSGS baby table; the separate exact oracle built 4512 signed L0 pair entries.
- Public generator anchor log: 1; the first signed physical factor may be the generator or its negation.
#image("graph.svg", width: 100%)
L0 wall is not isolated; hash/allocator CPU and calibrated GAE are absent. No priced IC/rho or external-solver claim.
