#set page(paper: "a4", margin: 18mm)
#set text(size: 9.4pt)
= run-20261023T0046Z-quartic_twist_k1_scale_p97_seed1_aa_v1
Raw SHA-256: 75bea42ba1addc558e306b8dc6e72c9804e73c8ef652824016c994cf7e9f974c
Audit SHA-256: e5ff58f8756b5215e55278338ea710a5164cc4281e407583bd6d341f58006687
Source commit a815598a57; Shared Intel Xeon Platinum 6975P-C host; L0 exploratory run. Public target [271]G.
== Execution
`target/release/examples/quartic_twist_k1_scale research/twist_index_calculus_20261020/quartic_twist_mixed_v1.ndjson 97 1`
8 GiB address-space cap; 1200 s watchdog.
== Recorded observations
- Native status `pass`; independent audit `pass`; 75 of 458 frozen residuals; anchored rank 6/6; recovered 271.
- Independent exact k=1 signed sets: 6 reference, 6 polynomial, 0 missing; 0 algebraic system failures.
- Counted IC: 3,349,559,070 Fp multiplications and 17,475 group operations. Same-process twist24 rho: 500,318 Fp multiplications and 3,822 group operations.
- All 72 physical factor logs were independently computed with a 9410-point BSGS baby table; the separate exact oracle built 2520 signed L0 pair entries.
- Public generator anchor log: 88538352; the first signed physical factor may be the generator or its negation.
#image("graph.svg", width: 100%)
L0 wall is not isolated; hash/allocator CPU and calibrated GAE are absent. No priced IC/rho or external-solver claim.
