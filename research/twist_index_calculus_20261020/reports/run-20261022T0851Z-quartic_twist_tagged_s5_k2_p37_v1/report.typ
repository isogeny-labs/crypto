#set page(paper: "a4", margin: 19mm)
#set text(size: 10pt)
= run-20261022T0851Z-quartic_twist_tagged_s5_k2_p37_v1
Raw: quartic_twist_tagged_s5_k2_p37_v1.json  |  SHA-256: 6fb9841fc27ca4e5aba2571ef05df8a262a0eeb0fd325d418374bbeb58fbf1bf
Current source SHA-256: 45251363eab451979d5eaf18246e63546c3ac5fc74455addb528e91e8c1e9ed0
Measurement source: frozen source commit 98fb8e09f; current source SHA below is a rendering-time snapshot.
Intel Xeon Platinum 6975P-C; shared host; L0 exploratory run. Public target [271]G; p=37, b0=2, r=1873633, seed=0.
== Execution
`target/release/examples/quartic_twist_tagged_s5 --k 2 research/twist_index_calculus_20261020/quartic_twist_mixed_v1.ndjson`
Cap: 8 GiB address space, 1200 s watchdog.
== Recorded observations
- Producer status: mismatch; exact signed-set agreements 3/4; system failures 1056.
- Signed polynomial relations 0; field solutions 0; systems attempted 1056.
- An agreement on an empty set does not make a failed system complete. The independent scalar-keyed pilot audit classifies each arm.
#image("graph.svg", width: 100%)
Counted native field work only. L0 wall time is not isolated; native hash, allocator and partition CPU are missing. No priced IC/rho speed claim.
