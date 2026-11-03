# ecbench_b36_pin_reg_l0 run report

Session ECBS1h718e3a9ec867 used spec ECS1hfb39cae6ba00 (SHA-256 fb39cae6ba004918548301e469269e70cb399a52441e0ebc8fa102f0aeb5c97e) and binary SHA-256 b0c9d8093e01d748ec8b850548ae63347a0e69e9131badae7444a0b4211dc635. The host was Intel(R) Xeon(R) 6975P-C. Isolation earned L0 (--cpus none --allow-busy); wall time is descriptive. All curve registry flags were true.

The session requested 324 runs: 324 verified. Three public targets per curve, one warmup and five measured rounds compared rho-neg, ic-native, and an A/A rho arm. The saved comparison has 90 measured IC/rho pairs; its counted S ratio is 31.630. Both arms have unpriced native counters, so that ratio is a lower-bound diagnostic. All six curves have comparable verified IC/rho pairs.

![Per-curve S ratios](figure.svg)

Figure: per-curve counted S ratios and cluster 95% intervals from the saved ecbench comparison. L0 wall time is not admitted; no twist method ran in this session.

Sources: [sealed session](../../sessions/ecbench_b36_pin_reg_l0/), [comparison](../../sessions/ecbench_b36_pin_reg_l0/comparisons/rho-neg__ic-native.json), [audit](../../ecbench_b36_pin_reg.audit.json).
