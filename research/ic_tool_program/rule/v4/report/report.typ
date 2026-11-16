#set page(paper: "a4", margin: 16mm)
#set text(size: 9pt)
#set par(justify: true)
= Rule v4: a recipe against v3's recipe, paired, on the Table A sizes

*Outcome: PARTIAL.* 1 of 6 sizes improve under the frozen rule (paired cold ratio v4 over v3 with a 95% bootstrap interval below 1); 0 regress. Class: engineering where it improves (S falls, the method is the same). Source: `analysis.json`, `recipe.json`, `training/`, `PROTOCOL.md` (frozen before any timed row; amendments A1 to A7 in it), `replay-certificate.json`. The v4 arm is the v3 recipe with K, the collection window and the unit size retuned on training seeds (lever (ii)); lever (i), free relations from the pair table's duplicate keys, was implemented and measured on the training grid (72 paired runs a size: 5 to 9% *more* cost) and is off. Same binary for both arms; one `isolated_bench` process per row; 16 recipe seeds, 2 targets each; this host has no GFNI.

== Paired cold ratio, v4 recipe over v3 recipe
#table(columns: 8, align: (right, right, left, left, left, left, right, right),
[n], [log2 r], [cold, v4 / v3 [95%]], [verdict], [set-up [95%]], [online [95%]], [A/A], [pos. ctl],
[47], [36.6], [1.007 [0.960, 1.056]], [NOT SEPARATED], [0.999 [0.951, 1.048]], [1.42 [1.13, 1.79]], [0.963], [1.10], [57], [38.0], [0.953 [0.904, 0.998]], [IMPROVEMENT], [0.951 [0.901, 0.995]], [1.12 [0.99, 1.33]], [0.972], [1.13], [41], [39.0], [0.963 [0.910, 1.014]], [NOT SEPARATED], [0.965 [0.912, 1.018]], [0.71 [0.44, 0.99]], [0.978], [1.11], [53], [44.3], [1.010 [0.964, 1.060]], [NOT SEPARATED], [1.009 [0.961, 1.061]], [1.26 [0.98, 1.76]], [0.947], [1.14], [59], [44.5], [1.013 [0.966, 1.061]], [NOT SEPARATED], [1.013 [0.965, 1.060]], [1.10 [1.00, 1.24]], [0.961], [1.15], [61], [47.2], [0.966 [0.917, 1.018]], [NOT SEPARATED], [0.967 [0.918, 1.019]], [0.69 [0.51, 0.89]], [1.019], [1.12])
The A/A is the v3 recipe against itself (its interval must contain 1; otherwise the size is UNRESOLVED). The positive control is the v3 recipe with the free-relation lever on (a slowdown of known direction, about 1.05 to 1.09 in the training pairs).

#image("paired-cold.svg", width: 100%)

== Distance remaining to rho (index calculus cold / rho cold, rho in the same process)
S is cost / sqrt(r) in the process's own batched-addition unit. Canonical step: rho priced at one batched addition plus the table canonicalisation. As built: rho's measured walk. Above 1, rho is cheaper.
#table(columns: 8, align: (right, right, right, right, right, left, right, right),
[n], [S_IC v3], [S_IC v4], [S_rho canonical], [gap v3 canon.], [gap v4 canon. [95%]], [gap v3 as built], [gap v4 as built],
[47], [8.85], [8.91], [0.902], [9.89], [9.89 [9.22, 10.65]], [5.55], [5.44], [57], [8.53], [8.13], [0.884], [9.56], [9.20 [8.58, 9.89]], [5.42], [5.25], [41], [10.74], [10.35], [0.624], [17.01], [16.57 [14.59, 19.12]], [9.05], [8.58], [53], [14.73], [14.87], [0.547], [26.33], [27.17 [24.29, 30.22]], [13.59], [13.71], [59], [14.02], [14.21], [0.586], [24.08], [24.25 [19.91, 30.49]], [12.82], [12.82], [61], [21.97], [21.23], [0.504], [43.46], [42.12 [37.09, 49.28]], [23.41], [21.52])

#image("gap-to-rho.svg", width: 100%)

== Section 5a band (counted group-addition equivalents; unpriced native work at 0, 0.1, 1 GAE)
#table(columns: 8, [n], [gap v4 w=0], [w=0.1], [w=1], [v4/v3 w=0], [w=0.1], [w=1], [order holds],
[47], [17.88], [18.84], [23.14], [1.092], [1.101], [1.136], [no], [57], [18.30], [19.36], [24.08], [1.004], [1.013], [1.045], [no], [41], [25.96], [27.56], [34.62], [0.888], [0.900], [0.939], [yes], [53], [37.01], [39.79], [52.19], [1.079], [1.078], [1.072], [no], [59], [35.73], [38.41], [50.43], [1.099], [1.098], [1.094], [no], [61], [46.98], [50.40], [65.70], [0.782], [0.779], [0.769], [yes])

Counted GAE is a lower bound: the build's cache misses, the linear algebra and the descent's bookkeeping are not priced, so it can rank two recipes differently from wall time (it prices a larger table's build at one addition a pair). "Order holds" = v4 below v3 at all three weights, interval upper end below 1.

== The reference: rho against the strong fixture, T01 to T08, ns per step
ParallelRho is the rho of the `ic price` process. At the second half of n = 41 and at n = 53, 59 and 61 those processes ran with the neighbours' jobs active (amendment A8); the strong fixture rows ran with none, so ParallelRho reads slower against it there than it would. Units: the strong fixture's ns a step over this arm's batched-addition unit.
#table(columns: 8, [n], [ParallelRho ns/step], [strong ns/step], [ratio], [admissible], [strong units/step], [ParallelRho units/step], [canonical units/step],
[47], [122.9], [120.1], [1.02], [False], [8.7], [9.2], [3.77], [57], [140.5], [116.7], [1.20], [False], [8.5], [10.0], [4.33], [41], [120.4], [111.8], [1.08], [False], [8.2], [8.1], [3.75], [53], [119.0], [158.5], [0.75], [True], [11.1], [8.2], [4.00], [59], [129.9], [141.0], [0.92], [True], [10.3], [8.8], [4.33], [61], [142.4], [131.5], [1.08], [False], [9.3], [8.7], [4.39])

== Lever (i) on the training grid: the same configuration and seed, lever off and on
#table(columns: 6, [n], [pairs], [cost on / off], [se (log)], [trials on / off, v3 K], [build on / off, v3 K],
[47], [72], [1.062], [0.016], [0.911], [1.43], [57], [72], [1.058], [0.018], [0.976], [1.41], [41], [72], [1.056], [0.019], [0.995], [1.47], [53], [72], [1.078], [0.021], [0.964], [1.40], [59], [72], [1.088], [0.019], [0.995], [1.47], [61], [72], [1.053], [0.023], [0.955], [1.54])

== The recipe selected on training seeds (disjoint from the evaluation targets)
v3 / v4 values; the lever is off in the v4 arm (it cost 5 to 9% in the training pairs). Predicted: the fitted cost of the v4 configuration over v3's on the training seeds (it is a prediction, not a result).
#table(columns: 5, [n], [columns], [window], [unit trials], [predicted v4/v3],
[47], [56 / 48], [164 / 262], [164 / 164], [0.944], [57], [64 / 54], [228 / 365], [181 / 181], [0.979], [41], [80 / 96], [205 / 328], [623 / 623], [0.960], [53], [264 / 224], [874 / 874], [1014 / 254], [0.992], [59], [256 / 218], [944 / 944], [935 / 935], [0.950], [61], [320 / 464], [1220 / 732], [3504 / 876], [0.890])

== Checks
Replay: 493 processes replayed in a separate process with `icprog`'s oracle, 0 failures; 24 untimed re-runs. Status verified.
