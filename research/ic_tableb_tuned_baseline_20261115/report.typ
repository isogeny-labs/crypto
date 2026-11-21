#set page(margin: 1.6cm, paper: "a4")
#set text(size: 9pt)
= Table B incumbent baselines: recorded vs tuned (2026-11-15) \[pcc:unverified\]
Evidence note, no new runs. Counted GAE per sqrt(r), w = 0 shown; w = 0.1 and 1 in README.md. Sources: `research/ic_tableb_ksum_20261115/summary.json`, `research/ic_tableb_binary_ksum_20261115/summary.json`, `docs/ic/runs/ic-boundary-ledger-round5-2026-09-22.json`. Validation: VAL-20261115-8c7c3c (binary); prime validator report `research/ic_tableb_ksum_20261115/validator/validator-report.json`.

*Not an improvement.* Re-tuning the incumbent is an accounting/baseline correction, not an index-calculus improvement (CLAUDE.md: relabelling and accounting changes are not improvements). It shows that Table B's recorded IC/rho gaps for the binary rows overstate the distance for a tuned pair table. Proposal for the owner: re-baseline the binary rows in a future preregistered Table B round.

#figure(image("baseline-gap.png", width: 88%))

#table(columns: 9, align: center, inset: 3pt, stroke: 0.4pt + gray,
[row], [rec. S_IC / S_rho], [inc-tb S], [best other incumbent S], [inc-tb / best], [ksum5 S], [rho-neg S], [rec. IC/rho], [best / rho-neg],
[fp20], [10.72 / 1.29], [10.29], [inc-sel: 10.36], [0.99], [7.11], [1.27], [8.3], [8.2],
[fp22], [12.77 / 1.22], [11.15], [inc-hi: 11.83], [0.94], [6.65], [1.17], [10.5], [10.1],
[fp24], [13.97 / 1.04], [12.82], [inc-w0: 12.74], [1.01], [6.38], [1.09], [13.5], [11.7],
[f2m15], [26.31 / 2.01], [16.27], [inc-n18: 11.84], [1.37], [13.47], [1.97], [13.1], [6.0],
[f2m18], [21.30 / 2.01], [20.82], [inc-n21: 10.85], [1.92], [11.26], [1.88], [10.6], [5.8],
[f2m21], [24.39 / 1.36], [22.15], [inc-n62: 10.28], [2.15], [7.01], [1.26], [18.0], [8.2],
[f2m24], [27.24 / 1.18], [22.60], [inc-n67: 10.99], [2.06], [6.99], [1.21], [23.0], [9.1],
[f2m27], [36.93 / 0.97], [47.91], [inc-n150: 13.95], [3.44], [6.41], [1.07], [38.2], [13.0],
)

Prime rows: inc-tb within 0.94-1.04x of the best other incumbent at every weight, so no overstatement. Binary rows: inc-tb is 1.37-3.44x the tuned incumbent at w = 0 (1.47-3.86x at w = 1). Recorded S_IC/S_rho are another harness and are never divided into in-session S. ksum5/rho-neg stays 5.6-6.9 at every row and weight: rho remains ahead.
