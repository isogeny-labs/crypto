#set page(paper: "a4", margin: 19mm)
#set text(size: 10pt)
= run-20261022T0906Z-quartic_twist_mixed_polynomial_ic_p37_v1
Raw: quartic_twist_mixed_polynomial_ic_p37_v1.json  |  SHA-256: 035c7ad608aa4ce8b2b0bb3a75775b6f652423ae4e450d19135a2c177ebd17a3
Current source SHA-256: cb1caa67733660b09271eca8b0fbc6a72307f28c59a8eafb1bf79e3432f1d24e
Measurement source: frozen source commit ade6c1916; current source SHA below is a rendering-time snapshot.
Intel Xeon Platinum 6975P-C; shared host; L0 exploratory run. Public target [271]G; p=37, b0=2, r=1873633, seed=0.
== Execution
`target/release/examples/quartic_twist_mixed_polynomial_ic research/twist_index_calculus_20261020/quartic_twist_mixed_v1.ndjson`
Cap: 8 GiB address space, 1200 s watchdog.
== Recorded observations
- Status: pass; anchored rank 4/4; recovered scalar 271.
- Processed 7 frozen residuals and retained 4 rank-raising signed relations; all factor logs verified.
- Recorded polynomial stream: 451,109,265 internal Fp multiplications; 5,125 stream group operations. The total appears in the separate independent audit and final-source record, not as a field of this historical producer schema.
#image("graph.svg", width: 100%)
Counted native field work only. L0 wall time is not isolated; native hash, allocator and partition CPU are missing. No priced IC/rho speed claim.
