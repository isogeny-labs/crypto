#set page(paper: "a4", margin: 19mm)
#set text(size: 10pt)
= run-20261022T0913Z-quartic_twist_mixed_polynomial_ic_p37_aa_v2
Raw: quartic_twist_mixed_polynomial_ic_p37_aa_v2.json  |  SHA-256: f57bd62dcada7f812a8a247c222c6614000ca5607e4f5edeca4732aaea246af0
Current source SHA-256: cb1caa67733660b09271eca8b0fbc6a72307f28c59a8eafb1bf79e3432f1d24e
Measurement source: intermediate modified checkout; exact historical source hash not retained; final-source parity tested separately.
Intel Xeon Platinum 6975P-C; shared host; L0 exploratory run. Public target [271]G; p=37, b0=2, r=1873633, seed=0.
== Execution
`target/release/examples/quartic_twist_mixed_polynomial_ic research/twist_index_calculus_20261020/quartic_twist_mixed_v1.ndjson`
Cap: 8 GiB address space, 1200 s watchdog.
== Recorded observations
- Status: pass; anchored rank 4/4; recovered scalar 271.
- Processed 7 frozen residuals and retained 4 rank-raising signed relations; all factor logs verified.
- Counted IC: 455,838,426 Fp multiplications; 8,123 group operations. Same-process twist24 rho: 348,941 Fp multiplications; 2,745 group operations.
- Final-source A/A control: compare all non-timing JSON fields against the other final-source repeat. The initial pilot source is preserved separately.
#image("graph.svg", width: 100%)
Counted native field work only. L0 wall time is not isolated; native hash, allocator and partition CPU are missing. No priced IC/rho speed claim.
