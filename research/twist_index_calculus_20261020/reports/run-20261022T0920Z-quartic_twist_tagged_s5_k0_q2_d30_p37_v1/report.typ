#set page(paper: "a4", margin: 19mm)
#set text(size: 10pt)
= run-20261022T0920Z-quartic_twist_tagged_s5_k0_q2_d30_p37_v1
Raw: quartic_twist_tagged_s5_k0_q2_d30_p37_v1.json  |  SHA-256: b865e9162e859cca9c9bdf156035702f75e99ccb9b287b17533c183535c998f5
Current source SHA-256: 45251363eab451979d5eaf18246e63546c3ac5fc74455addb528e91e8c1e9ed0
Measurement source: final source; current SHA below is the rendering-time snapshot.
Intel Xeon Platinum 6975P-C; shared host; L0 exploratory run. Public target [271]G; p=37, b0=2, r=1873633, seed=0.
== Execution
`target/release/examples/quartic_twist_tagged_s5 --k 0 research/twist_index_calculus_20261020/quartic_twist_mixed_v1.ndjson --query-index=2 --s5-degree=30`
Cap: 8 GiB address space, 1200 s watchdog.
== Recorded observations
- Producer status partial; system outcome: border unreachable at this degree; failure count 1.
- Signed-set agreement False; polynomial relations 0; reference relations 1.
- This bounded degree increase is a failed diagnostic. It does not prove the S5 system has no solution or that a different solver cannot find one.
#image("graph.svg", width: 100%)
Counted native field work only. L0 wall time is not isolated; native hash, allocator and partition CPU are missing. No priced IC/rho speed claim.
