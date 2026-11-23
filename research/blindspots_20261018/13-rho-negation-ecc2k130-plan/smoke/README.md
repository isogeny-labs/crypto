Smoke tests run before the cards were frozen (not measured runs; seed offsets 0 and 1000003, disjoint from the measured 7000000 and 9000011):

* `selftest.txt`: the instrument's selftest (two-implementation agreement, positive and negative controls) with the final binary built from `examples/blindspots_13_rho_negation.rs`.
* `all-cells-1-target-summary.md`: `b13 sweep` on every cell (sizes 40, 44, 48 by r 16, 32, 128, 2048, all 14 rules plus `baseb`), one target each, the theta 8 and 14 extension at one target, 4 ideal-map targets per size, then `b13 summary`. It proves no cell panics and the summary reads every record kind. It is a one-target smoke: its gains are noise and carry no information.
* Part B: `ecc2k130-cpu --curve 131 --bench --threads 1` ran 160 launches twice (14.868 and 14.414 M it/s) and `b13 plan` ran on two 60-launch logs; nothing from those runs is kept.
