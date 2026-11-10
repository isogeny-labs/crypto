#set page(paper: "a4", margin: 1.6cm)
#set text(size: 9.5pt)
#set heading(numbering: none)

= Minors / guess-and-determine ECDLP vs matched negation rho: panel relaunch C

Study `research/minors_guess_determine_20261107` · run `minors-gd-panel-20261108-c`, pod uid `b8617fcd-e2e5-4235-90b0-b201e5bb3857` · 2026-11-08 · every number is *\[pcc:unverified\]*. The numbers come from `panel/analysis.json`, the frozen `minors_gd_analyze.py` output and the primary analysis. An independent re-derivation agreed on all 1017 compared quantities, with bootstrap bounds equal to Monte-Carlo precision. Full report: `RESULTS.md`.

== Verdict (PROTOCOL.md section 8, mechanical)

*CLOSED for the reconstruction*: the six arms `mgd.all2`, `mgd.gesc`, `mgd.apm2`, `gd`, `gd_dp1` and `gd_half`, in prime fields at 20-40 bits.

- Every arm has upper95(e_all) $>=$ 0.5. The values run from 0.815 to 1.065, and every lower bound is $>=$ 0.766.
- Every arm costs more than rho at every size: the minimum ratio is 4.37.
- No real-lead flag is raised.
- The ordering holds at inversion weights w = 1.5 bits(p), 3 and 10, and at unpriced weights $lambda$ = 0, 0.1 and 1, so the result is not unordered.

This is not an ecbench session, and it is not a leaderboard row.

== Distance remaining to matched rho (cheapest minors arm per size, paired bootstrap 95%)

#table(columns: 7, align: center, stroke: 0.4pt,
  [*bits*], [20], [24], [28], [32], [36], [40],
  [rho cost (mults)], [1.09e5], [1.91e5], [3.67e5], [9.95e5], [3.48e6], [1.29e7],
  [cheapest arm], [`mgd.gesc`], [`mgd.gesc`], [`mgd.apm2`], [`mgd.apm2`], [`mgd.apm2`], [`mgd.apm2`],
  [its cost (mults)], [4.78e5], [1.17e6], [7.10e6], [8.15e7], [1.27e9], [2.00e10],
  [ratio to rho], [4.37], [6.11], [19.3], [81.9], [365], [1552],
  [95%], [4.28-4.46], [5.75-6.51], [17.2-21.7], [71.8-93.1], [309-431], [1049-2721],
)

== Exponents (slope of log2 cost per bit; bootstrap 95%)

#table(columns: 4, align: center, stroke: 0.4pt,
  [*arm*], [*e_all (primary)*], [*e_top3 (32-40)*], [*e_arm - e_rho, all sizes*],
  [`rho`], [0.346 \[0.342, 0.349\]], [0.461 \[0.447, 0.476\]], [-],
  [`mgd.all2`], [0.890 \[0.853, 0.968\]], [0.969 \[0.842, 1.249\]], [\[0.507, 0.622\]],
  [`mgd.gesc`], [0.823 \[0.798, 0.861\]], [1.031 \[0.942, 1.170\]], [\[0.451, 0.517\]],
  [`mgd.apm2`], [0.787 \[0.766, 0.816\]], [0.992 \[0.919, 1.093\]], [\[0.420, 0.471\]],
  [`gd`], [0.835 \[0.786, 0.922\]], [n/a (\<3 sizes)], [\[0.440, 0.577\]],
  [`gd_dp1`], [0.979 \[0.919, 1.065\]], [n/a (\<3 sizes)], [\[0.574, 0.720\]],
  [`gd_half`], [0.930 \[0.838, 1.005\]], [n/a (\<3 sizes)], [\[0.493, 0.659\]],
)

#page(flipped: true)[#figure(image("minors_gd_fig1.svg", width: 100%),
  caption: [Values from `analysis.json` of run `minors-gd-panel-20261108-c`, with 200 matched instances per size. (a) Cost per solve, 95% intervals; open markers are zero-success lower bounds. (b) Ratio to matched rho, with the A/A ratio. (c) Exponents against the 0.5 decision threshold.])]

== Controls

- *Positive.*
  - Development self-test: all arms listed recover k ("SELFTEST PASS"). `gd_dp1` and `gd_half` are not in the self-test list.
  - In the panel, all 6956 successes are correct.
- *Negative (a).* The verifier rejects k+1 (development).
- *Negative (b).* Curve vs uniform zero-minor counts agree at 12-16 bits (development only).
- *Negative (c).* `bad_candidates` = 0 in all 9600 records.
- *A/A.* rho_aa/rho has every 95% interval containing 1, with half-width 2.6-8.0%.

== Execution record

- *Launch 1* (14 vCPU) was never scheduled: 14180m exceeds the 14000m allocatable. It was deleted with no data.
- *Relaunch B* (12 vCPU) was lost to Spot (exit 137, node shutdown). It is not admitted.
- *Relaunch C*, the single allowed relaunch, ran 12:46:21-16:45:14Z, exit 0 `Completed`.
  - The exit comes from the EKS audit log (audit id `c22e0d20-...`), because the Pod object was deleted with its node.
  - JOBS 14 to 12 changes only the shard parallelism: the measured quantities are exact counts.
  - `SHA256SUMS` verifies.
  - The protocol, run_panel and analyze hashes equal the frozen values.
- Wall time (about 47.8 vCPU-h) is informational. Total charged work is 6.69e12 units against the 7e12 the protocol estimated.

== Caveats

- This is a reconstruction and not the authors' code. It runs in prime fields, without `apm3`.
- The rho in PROTOCOL section 2 text (R = 64, 32 walkers) is stale. The frozen `rho.rs` has R = 32, W = 16.
- The censored MLE assumes a constant hazard. `gd*` runs a single attempt per run from 28/32 bits up.
- Sparse successes affect the fits: `gd_half` has 1 success and `gd_dp1` 3 at 36 bits, which makes some bootstrap draws drop a size.
- There is no top3 exponent for the `gd` family.
- The protocol's "per-attempt estimate in the README" check is UNRESOLVED as written: the README has none.
- The README's "five orders for gd at 40 bits" is UNRESOLVED: only a lower bound (> 1.04e4) is available.
- There is no replay certificate, validator report or ecbench cross-check.
