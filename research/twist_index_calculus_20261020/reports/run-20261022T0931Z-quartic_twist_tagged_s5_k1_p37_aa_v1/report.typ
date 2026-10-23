#set page(paper: "a4", margin: 19mm)
#set text(size: 10pt)
= run-20261022T0931Z-quartic_twist_tagged_s5_k1_p37_aa_v1
Raw: quartic_twist_tagged_s5_k1_p37_aa_v1.json  |  SHA-256: ab9b02795c6b02c4b4e7a774e87a9acf4647231bab00e5cbc161218c8ee66ba5
Current source SHA-256: 45251363eab451979d5eaf18246e63546c3ac5fc74455addb528e91e8c1e9ed0
Measurement source: final source; current SHA below is the rendering-time snapshot.
Intel Xeon Platinum 6975P-C; shared host; L0 exploratory run. Public target [271]G; p=37, b0=2, r=1873633, seed=0.
== Execution
`target/release/examples/quartic_twist_tagged_s5 --k 1 research/twist_index_calculus_20261020/quartic_twist_mixed_v1.ndjson`
Cap: 8 GiB address space, 1200 s watchdog.
== Recorded observations
- Producer status: pass; exact signed-set agreements 4/4; system failures 0.
- Signed polynomial relations 2; field solutions 2; systems attempted 96.
- An agreement on an empty set does not make a failed system complete. The independent scalar-keyed pilot audit classifies each arm.
#image("graph.svg", width: 100%)
Counted native field work only. L0 wall time is not isolated; native hash, allocator and partition CPU are missing. No priced IC/rho speed claim.
