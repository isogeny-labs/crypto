#set page(paper: "a4", margin: 18mm)
#set text(size: 9.4pt)
= run-20261023T0037Z-quartic_twist_k1_scale_p97_seed0_v1
Raw SHA-256: 5cfcc2e4af6850d06c65cc56ad47a90712635f31e1cf356f580a3d73b456b2ad
Audit SHA-256: b2043ca81cf2b4ce524a900f28efb3eea0cae13a036aa15a34c00afc06383b59
Source commit a815598a57; Shared Intel Xeon Platinum 6975P-C host; L0 exploratory run. Public target [271]G.
== Execution
`target/release/examples/quartic_twist_k1_scale research/twist_index_calculus_20261020/quartic_twist_mixed_v1.ndjson 97 0`
8 GiB address-space cap; 1200 s watchdog.
== Recorded observations
- Native status `pass`; independent audit `pass`; 310 of 558 frozen residuals; anchored rank 6/6; recovered 271.
- Independent exact k=1 signed sets: 16 reference, 16 polynomial, 0 missing; 0 algebraic system failures.
- Counted IC: 13,839,508,536 Fp multiplications and 53,111 group operations. Same-process twist24 rho: 802,322 Fp multiplications and 5,463 group operations.
- All 72 physical factor logs were independently computed with a 9410-point BSGS baby table; the separate exact oracle built 2520 signed L0 pair entries.
- Public generator anchor log: 1; the first signed physical factor may be the generator or its negation.
#image("graph.svg", width: 100%)
L0 wall is not isolated; hash/allocator CPU and calibrated GAE are absent. No priced IC/rho or external-solver claim.
