#set page(paper: "a4", margin: 18mm)
#set text(size: 9.4pt)
= run-20261023T0048Z-quartic_twist_k1_scale_p97_seed2_aa_v1
Raw SHA-256: d3f0e8361bc52a7a4f62dc0fec0433231e7322ba9a194d629afed2bf7c243071
Audit SHA-256: 4ffb41ad817ffb2b8bb7456200cd447f1568da88afca075b05620ec41bfaea88
Source commit a815598a57; Shared Intel Xeon Platinum 6975P-C host; L0 exploratory run. Public target [271]G.
== Execution
`target/release/examples/quartic_twist_k1_scale research/twist_index_calculus_20261020/quartic_twist_mixed_v1.ndjson 97 2`
8 GiB address-space cap; 1200 s watchdog.
== Recorded observations
- Native status `pass`; independent audit `pass`; 150 of 481 frozen residuals; anchored rank 6/6; recovered 271.
- Independent exact k=1 signed sets: 6 reference, 6 polynomial, 0 missing; 0 algebraic system failures.
- Counted IC: 6,696,848,851 Fp multiplications and 28,588 group operations. Same-process twist24 rho: 791,504 Fp multiplications and 5,395 group operations.
- All 72 physical factor logs were independently computed with a 9410-point BSGS baby table; the separate exact oracle built 2520 signed L0 pair entries.
- Public generator anchor log: 88538352; the first signed physical factor may be the generator or its negation.
#image("graph.svg", width: 100%)
L0 wall is not isolated; hash/allocator CPU and calibrated GAE are absent. No priced IC/rho or external-solver claim.
