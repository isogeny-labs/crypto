#set page(margin: 1.6cm, numbering: "1")
#set text(size: 9.5pt)
#set table(stroke: 0.4pt)
= Frobenius-folded k-sum against the lazy orbit-member incumbent (K1 n23, n29)

2026-11-15. Frozen protocol commit `9fa38942e4`. Class: engineering. All numbers `[pcc:unverified]`. With one orbit and m = 3 the candidate is a Frobenius-folded generic (square-root-class) method; it cannot beat rho asymptotically. Ratios are cost(candidate) / cost(strictest lazy incumbent), paired by target, 95 % cluster-bootstrap, 3 sessions x 48 targets x 3 rounds per curve, every arm priced by the same section 5a band.

== Result
*K1 n23: the candidate beats the strongest existing IC arm at all three weights* (0.784, 0.799, 0.832 against `lazy-c4`, every replicate upper end below 1, beyond A/A). It is not a win over rho: 0.883 / 1.105 / 2.262 of `rho.signed_frobenius_strong` at w = 0 / 0.1 / 1, unresolved inside the band. *K1 n29: it regresses* 5.30 / 5.62 / 6.31 against `lazy-c1`.

#figure(image("figures/fig1-ratio.svg", width: 100%), caption: [R = cost(kf3-c1g-sr) / cost(strictest lazy). Squares: pooled 95 % CI; dots: the three replicates; grey: A/A (a point, counts are deterministic). Log axis.])

== K1 n23

#figure(table(columns: 4, align: (left, right, right, right), [arm], [S(w=0)], [S(w=0.1)], [S(w=1)],
[rho-strong], [0.886], [0.950], [1.533],
[rho-strong-aa], [0.886], [0.950], [1.533],
[rho-neg], [1.182], [1.282], [2.185],
[k1-l9], [43.823], [52.783], [133.419],
[lazy-c3], [1.152], [1.508], [4.711],
[lazy-c4], [0.997], [1.314], [4.166],
[kf3-c1g-sr], [0.782], [1.050], [3.467],
[kf3-c1g-w], [1.199], [1.467], [3.882],
), caption: [Mean S per arm (band-priced).])

#figure(table(columns: 5, align: (left, left, right, right, left), [w], [strictest], [R pooled [95 % CI]], [kf3-c1g-sr / rho-strong], [H1],
[0], [lazy-c4], [0.784 [0.747, 0.821]], [0.883 [0.852, 0.913]], [holds],
[0.1], [lazy-c4], [0.799 [0.762, 0.837]], [1.105 [1.067, 1.143]], [holds],
[1], [lazy-c4], [0.832 [0.795, 0.869]], [2.262 [2.171, 2.357]], [holds],
), caption: [H1 and distance to rho-strong.])

== K1 n29

#figure(table(columns: 4, align: (left, right, right, right), [arm], [S(w=0)], [S(w=0.1)], [S(w=1)],
[rho-strong], [4.567], [4.685], [5.745],
[rho-strong-aa], [4.567], [4.685], [5.745],
[rho-neg], [1.892], [2.007], [3.045],
[k1-l10], [3189.471], [3510.404], [6398.802],
[lazy-c1], [1.569], [2.063], [6.507],
[lazy-c2], [2.373], [3.249], [11.133],
[kf3-c1g-sr], [8.322], [11.597], [41.073],
[kf3-c1g-w], [11.096], [14.371], [43.848],
), caption: [Mean S per arm (band-priced).])

#figure(table(columns: 5, align: (left, left, right, right, left), [w], [strictest], [R pooled [95 % CI]], [kf3-c1g-sr / rho-strong], [H1],
[0], [lazy-c1], [5.303 [5.226, 5.384]], [1.822 [1.808, 1.836]], [regression],
[0.1], [lazy-c1], [5.622 [5.540, 5.704]], [2.475 [2.453, 2.497]], [regression],
[1], [lazy-c1], [6.312 [6.223, 6.403]], [7.149 [7.019, 7.279]], [regression],
), caption: [H1 and distance to rho-strong.])

#figure(image("figures/fig2-S.svg", width: 100%), caption: [Mean S per arm and weight (log axis).])

== Limits
Two tiny curves, configurations tuned on disjoint sizing seeds, no exponent claim, no leaderboard row (no `VALIDATION.json`). `*_full` arms errored on n23 (no price row); the n29 `*_full` read is the same direction (5.355 [5.273, 5.439]). Local route, L0. See `README.md` for all deviations.
