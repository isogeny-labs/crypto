#set page(width: 210mm, height: 297mm, margin: 17mm)

#set text(font: "DejaVu Sans", size: 10pt)

= ecbench_b24_l0 run report

Session ECBS1hc445922b4993 used spec ECS1h6170df336121 (SHA-256 6170df336121d55d3f6fa6126bb49fda4176e548bcb192599c46ce0ce932037c) and binary SHA-256 56a532e119bb6e58df188afc25d6bc8495fd345a643a8c541f136481f1090324. The host was Intel(R) Xeon(R) 6975P-C. Isolation earned L0 (--cpus none --allow-busy); wall time is descriptive. All curve registry flags were false.

The session requested 324 runs: 324 verified. Three public targets per curve, one warmup and five measured rounds compared rho-neg, ic-native, and an A/A rho arm. The saved comparison has 90 measured IC/rho pairs; its counted S ratio is 35.847. Both arms have unpriced native counters, so that ratio is a lower-bound diagnostic. All six curves have comparable verified IC/rho pairs.

#image("figure.svg", width: 100%)

Figure: per-curve counted S ratios and cluster 95% intervals from the saved ecbench comparison. L0 wall time is not admitted; no twist method ran in this session.
