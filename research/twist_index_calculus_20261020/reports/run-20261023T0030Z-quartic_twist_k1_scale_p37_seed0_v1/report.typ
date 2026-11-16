#set page(paper: "a4", margin: 18mm)
#set text(size: 9.4pt)
= run-20261023T0030Z-quartic_twist_k1_scale_p37_seed0_v1
Raw SHA-256: b3c3287b2f5cd3b3cfe41d9dfb4eca4a7d397be05ea9aae5505c65f0eadc304e
Audit SHA-256: ae9fc983d4720ffbcef96ab375e3d3e53ffd38a8b57f7d03c69b345bbc032c78
Source commit a815598a57; Shared Intel Xeon Platinum 6975P-C host; L0 exploratory run. Public target [271]G.
== Execution
`target/release/examples/quartic_twist_k1_scale research/twist_index_calculus_20261020/quartic_twist_mixed_v1.ndjson 37 0`
8 GiB address-space cap; 1200 s watchdog.
== Recorded observations
- Native status `pass`; independent audit `pass`; 8 of 61 frozen residuals; anchored rank 4/4; recovered 271.
- Independent exact k=1 signed sets: 4 reference, 4 polynomial, 0 missing; 0 algebraic system failures.
- Counted IC: 116,050,118 Fp multiplications and 3,936 group operations. Same-process twist24 rho: 348,941 Fp multiplications and 2,745 group operations.
- All 48 physical factor logs were independently computed with a 1369-point BSGS baby table; the separate exact oracle built 2520 signed L0 pair entries.
- Public generator anchor log: 1; the first signed physical factor may be the generator or its negation.
#image("graph.svg", width: 100%)
L0 wall is not isolated; hash/allocator CPU and calibrated GAE are absent. No priced IC/rho or external-solver claim.
