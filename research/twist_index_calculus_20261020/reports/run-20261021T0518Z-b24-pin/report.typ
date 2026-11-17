#set page(width: 210mm, height: 297mm, margin: 17mm)

#set text(font: "DejaVu Sans", size: 10pt)

= ecbench_b24_pin_l0 run report

Session ECBS1h5fd0fb0c4a9f used spec ECS1he81bc8c40350 (SHA-256 e81bc8c40350e481e5b2de063328dce46729975eaec5315d87b77462d3042cb7) and binary SHA-256 56a532e119bb6e58df188afc25d6bc8495fd345a643a8c541f136481f1090324. The host was Intel(R) Xeon(R) 6975P-C. Isolation earned L0 (--cpus none --allow-busy); wall time is descriptive. All curve registry flags were false.

The session requested 324 runs: 324 verified. Three public targets per curve, one warmup and five measured rounds compared rho-neg, ic-native, and an A/A rho arm. The saved comparison has 90 measured IC/rho pairs; its counted S ratio is 17.887. Both arms have unpriced native counters, so that ratio is a lower-bound diagnostic. All six curves have comparable verified IC/rho pairs.

#image("figure.svg", width: 100%)

Figure: per-curve counted S ratios and cluster 95% intervals from the saved ecbench comparison. L0 wall time is not admitted; no twist method ran in this session.
