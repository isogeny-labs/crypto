#set page(paper: "a4", margin: 19mm)
#set text(size: 10pt)
= run-20261022T0916Z-quartic_twist_mixed_polynomial_ic_k1_p37_v1
Raw: quartic_twist_mixed_polynomial_ic_k1_p37_v1.json  |  SHA-256: 9e7299c74b69844a575297d43dd3cf3cb6060749664ab36267d4d5c3eb271c4e
Current source SHA-256: cb1caa67733660b09271eca8b0fbc6a72307f28c59a8eafb1bf79e3432f1d24e
Measurement source: frozen source commit ade6c1916; current source SHA below is a rendering-time snapshot.
Intel Xeon Platinum 6975P-C; shared host; L0 exploratory run. Public target [271]G; p=37, b0=2, r=1873633, seed=0.
== Execution
`target/release/examples/quartic_twist_mixed_polynomial_ic research/twist_index_calculus_20261020/quartic_twist_mixed_v1.ndjson --patterns=1`
Cap: 8 GiB address space, 1200 s watchdog.
== Recorded observations
- Status: pass; anchored rank 4/4; recovered scalar 271.
- Processed 8 frozen residuals and retained 4 rank-raising signed relations; all factor logs verified.
- Counted IC: 116,050,118 Fp multiplications; 3,936 group operations. Same-process twist24 rho: 348,941 Fp multiplications; 2,745 group operations.
#image("graph.svg", width: 100%)
Counted native field work only. L0 wall time is not isolated; native hash, allocator and partition CPU are missing. No priced IC/rho speed claim.
