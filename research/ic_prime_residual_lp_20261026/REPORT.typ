#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set par(justify: true)
#show table: set text(size: 8pt)

= The residual collector (large-prime limit) on prime curves

2026-10-26 · `research/ic_prime_residual_lp_20261026` · protocol frozen in `c77297bda`/`4aafbbffa` · six ecbench
sessions, 1,536 records, all verified, audits OK · reference `rho.negation` · lead `LEAD-ICP-residual-partial-relations`.

== Question

Does crediting residuals lower the counted cost of the prime `ic.pipeline` baseline? Crediting means storing every residual of a partial
decomposition and turning collisions into relations: `ic_framework::residual`, the `|D| = r` limit of the
large-prime variation. If it does, is the gain generic? The collector ran at base sizes 4–64 and at the plan's `N₀`,
on six registered prime curves (20.6–32 bits), against the `ic plan` recipe and rho.

== Result

#table(columns: 8, align: (left, right, right, right, left, left, left, left),
  [*log#sub[2] r*], [*S plan*], [*S res4*], [*S rho*], [*res4/plan w=0*], [*w=0.1*], [*w=1*], [*res4/rho*],
  [20.59], [10.59], [4.51], [1.19], [0.426 [0.379, 0.479]], [1.119 [0.983, 1.274]], [3.886 [3.338, 4.525]], [3.78],
  [21.19], [10.51], [4.32], [0.87], [0.411 [0.333, 0.507]], [1.104 [0.873, 1.397]], [3.871 [2.972, 5.043]], [4.94],
  [24.00], [13.15], [3.95], [0.98], [0.300 [0.279, 0.324]], [0.868 [0.801, 0.940]], [3.067 [2.806, 3.351]], [4.05],
  [25.35], [14.36], [4.18], [0.92], [0.291 [0.251, 0.339]], [0.866 [0.740, 1.013]], [3.105 [2.600, 3.709]], [4.54],
  [28.00], [20.40], [3.72], [0.93], [0.182 [0.160, 0.207]], [0.550 [0.482, 0.628]], [1.949 [1.706, 2.228]], [4.01],
  [32.00], [29.40], [3.51], [0.89], [0.119 [0.104, 0.137]], [0.367 [0.318, 0.424]], [1.330 [1.137, 1.555]], [3.95],
)

*Counted (w = 0), the 4-point collector is 2.3–8.4× cheaper than the plan; at 1 GAE per native operation it is 1.33–3.89×
dearer.* The order reverses inside the AGENTS.md §5a band at every size, so the two are not ranked. Exponents (slope of
ln GAE on ln r): plan 0.634, collector 0.45–0.48 at every fixed base, rho 0.480. The collector has rho's exponent at about 4×
rho's counted constant.

#figure(image("figures/band_reversal.svg", width: 82%), caption: [res4 against the plan at w = 0, 0.1 and 1, paired.])

*Curve-free null.* The same collector and seeds on `Z/r` with a random-bijection key cost the same. The curve-free/curve
ratio of mean group operations is 0.964–1.029 over p21, p24 and p28 × B = 4, 16 and 64, and all nine 95% intervals contain 1.

#figure(image("figures/curve_free_null.svg", width: 66%), caption: [Curve-free / curve group operations per solve.])

== Decision

All six predictions held, with P1 holding only in the counted unit.

*Classification.* The result is generic collision search, band-dominated by the plan, so it moves no leaderboard
row and no planner rule.

*Proposed atlas disposition.* `closed_known`: residual crediting on prime fields is the birthday bound. The w = 1
ratio falls with size and would cross 1 near log#sub[2] r ≈ 35 if the trend held; that is an extrapolation, and both arms
would still be at least 4× rho.

*Deviations.* No resume checkpoint could be uploaded (no token); the owner said to proceed, and every input and output is
committed and pushed. The binary was built at `b561ebca8`, after the freeze; the change does not touch these arms.
