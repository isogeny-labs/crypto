#set page(paper: "a4", margin: 22mm)
#set text(font: "Liberation Sans", size: 10pt)
= Mixed-fiber Semaev-S4 all-roots DLP

*Command:* `mixed-fiber-polynomial-dlp` and `mixed-fiber-polynomial-analyze` from `native/Cargo.toml`. *Host:* shared Intel Xeon 6975P-C; L0 operation counts only.

The native solver built the Semaev-S4 resultant for every tuple in each prescribed three-fiber system, enumerated all rational polynomial zeros, tested every sign lift, inserted only group-law-valid relations, solved the modular rank system, and recomputed kP for each recovered scalar. It completed all 540 candidate arm records on 180 exact-order targets across nine eligible prime-field curves. The same target/fiber/rho records match the earlier pair-table cohort exactly. It performed 3,783,011 polynomial evaluations, found 21,849 polynomial roots, and attempted 165,669 sign lifts, retaining failed lifts.

At M+0.5S+10I and 0.1M per declared native logical operation, homogeneous/mixed/matched-random cost 57.8840×/32.2975×/34.4846× source rho. Against the corresponding pair-table arms the ratios are 15.8869×/10.0233×/9.2752×. All 36 pooled price cells lose to both references for all three arms.

#figure(image("cost-ratios.svg", width: 100%), caption: [Complete cold operation-cost ratios versus matched source rho; 180 targets per arm.])

*Raw records:* `correctness/mixed-fiber-polynomial-dlp-v1.jsonl`; *full 36-price per-curve grid:* `results/mixed-fiber-polynomial-dlp-price-grid.csv`. This exhaustive all-roots polynomial evaluator is not F4/msolve. The fastest external Gröbner comparison and qualified wall-speed comparison remain open.
