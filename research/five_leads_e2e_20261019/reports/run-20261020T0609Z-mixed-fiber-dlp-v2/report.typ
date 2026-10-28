#set page(paper: "a4", margin: 22mm)
#set text(font: "Liberation Sans", size: 10pt)
= Mixed-fiber pair-table DLP run

*Receipt written:* 2026-10-20T06:09Z UTC. *Command:* `mixed-fiber-dlp` from `native/Cargo.toml`. *Host:* shared Intel Xeon 6975P-C; operation counts are L0, wall time unqualified.

The native solver ran homogeneous, mixed and matched-random factor bases on nine eligible prime-order curves over F97, F193 and F257. Each arm received 20 distinct exact-order targets per curve. All 540 candidate scalars and all matched rho controls were source-verified; there were no failures or exhaustions. Exact byte replay is bound by `correctness/mixed-fiber-dlp-v2-replay.sha256`.

At M+0.5S+10I and 0.1M per declared native logical operation, homogeneous/rho is 3.643509×, mixed/rho is 3.222234×, and random/rho is 3.717951×. Each ratio divides summed complete one-target costs for 180 matched targets.

#figure(image("cost-ratios.svg", width: 100%), caption: [Recorded pooled pair-table DLP cost ratios; 180 targets per arm; dashed line is equal cost.])

*Raw source:* `correctness/mixed-fiber-dlp-v2.jsonl`; native auditor `native/mixed_fiber_dlp_analyze.rs`; full price grid `results/mixed-fiber-dlp-native-v3-price-grid.csv`. This run tests a pair-table oracle, not a heterogeneous polynomial/F4 solver.
