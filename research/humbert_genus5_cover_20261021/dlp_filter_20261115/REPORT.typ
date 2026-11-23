#set document(title: "Filtered double-large-prime matrices on the genus-5 cover")
#set page(paper: "a4", margin: 1.8cm, numbering: "1")
#set text(size: 9.5pt)
#set par(justify: true)
#set heading(numbering: "1.")

#align(center)[
  #text(size: 15pt, weight: "bold")[Filtered double-large-prime matrices on the genus-5 cover:\ the fill-in goes, the lead over single large primes shrinks with $q$]\
  #v(2pt)
  Study `research/humbert_genus5_cover_20261021/dlp_filter_20261115` · 2026-11-15 · follows `dlp_20261113` (PR \#4065)
]

#v(6pt)
*Question.* `dlp_20261113` found that double large primes (dlp) halve the relation candidates
against single large primes (lp), but that fully eliminating the large primes makes the reduced
rows about twice as heavy (28 vs 13.5 non-zeros per row). Here the `dlp:0.5` relation set is held
fixed and only the matrix given to Wiedemann changes. `dlpm`: raw rows, large primes kept as extra
columns, singletons removed. `dlpf`: `dlpm` plus greedy merges. A large prime of degree 2–12 is
eliminated through its lightest row only while (columns − 1)(nnz + Δ) < columns · nnz.

*Scope and standing.* Toy fields only. 4 values of $q$ × 5 seeds × 6 arms = 120 runs under
`PROTOCOL.md`, frozen before data. The same instance is used per $(q, "seed")$, arm order rotates
per seed, and all runs use one frozen binary (`bin/humbert-v4`, sha256 prefix `5924fd1b`). The
host is a 4-vCPU shared VM with no isolation (L0); isolab-eks was not reachable (no AWS identity).
Every wall-time number is *exploratory* and an *unverified assertion* [pcc:unverified].
The candidate, matrix and mulmod counts are host-independent. No row goes to the IC leaderboard:
there is no replay certificate, validator report or `VALIDATION.json`.

*Correctness.* All 120 runs solved the DLP and passed the pushforward ($8P$), $T(P) != 0$, [pcc:unverified]
iota-pair and $X_infinity = 4c$ checks (`humbert summarize-filter`).

*Interruption.* The runner hit the session's 2-hour background limit after 117 of 120 runs. The
last instance ($q = 24001$, seed 5) is missing arms dlp50, dlpm50 and dlpf50, in protocol order
$k = 3, 4, 5$. They were run afterwards with the same binary (hash checked) and the same command,
and are marked `"resumed": true` in `v4.jsonl`. Their counted fields agree with the C0 checks.
Their wall times come from a later process and could carry a host-state difference.

= Control C0 (counted): exact

- 45/45 `pre`, `lp50` and `dlp50` runs at $q <= 16001$ reproduce their `dlp_20261113` records
  (rel_candidates, la_mulmods, x_recovered).
- dlpm50, dlpf50 and dlpf50_aa have exactly dlp50's relation candidates on all 20 instances.
- dlpf50_aa has counted fields identical to dlpf50 on all 20 instances.

= Predictions against the data

#figure(image("figures/filter_mulmods.svg", width: 76%), caption: [Counted Wiedemann mulmods, same instance (`results/instances_v4.csv`).])

- *F1 (counted): holds.* dlpf50 does fewer LA mulmods than dlp50 on 20/20 instances. The ratio is [pcc:unverified]
  0.719–0.773, with medians 0.758, 0.754, 0.761, 0.750 by $q$.
- *F2 (counted): holds.* dlpf50 does fewer LA mulmods than dlpm50 on 20/20 instances (ratio
  0.668–0.688). Keeping every large prime as a column (dlpm) is _worse_ than full elimination
  (dlp): 1.46e9 vs 1.32e9 at $q = 8009$. Its rows are light (10 per row), but it has 1.73× the
  columns.
- *F3 (wall time): holds, barely at the top.* dlpf50 beats lp50 on 5/5, 5/5 and 4/5 seeds at
  $q = 12007$, 16001 and 24001. The median paired lp/dlpf ratio falls steadily: 1.31, 1.26, 1.12,
  1.02 at $q = 8009, 12007, 16001, 24001$.
- *F4 (wall time): holds; at $q = 24001$ the margin is inside the noise floor.* Median IC /
  expected-steps rho: dlpf 2.38 vs best other 2.82 ($q = 12007$); 1.47 vs 1.65 (16001); 1.21 vs
  1.23 (24001, lp50). At 24001 the 1.6 % margin is smaller than the A/A spread there (dlpf/dlpf_aa
  0.962–1.002). The protocol's verdict is "holds"; read it as "not separated from lp at 24001".
- *F5 (exploratory).* The log–log slope of median $t_"ic"$ from $q = 8009$ to 24001 is 1.44 for
  lp50, 1.67 for dlpf50, 1.74 for dlp50, 1.76 for dlpm50 and 1.90 for `pre`. On these slopes
  dlpf's lead over lp ends near the top of the grid.

#figure(image("figures/filter_ratios.svg", width: 76%), caption: [Paired wall-time ratios (L0). Above 1, dlpf is faster. Black: the A/A arm.])

= Where the time goes

#let ph = csv("results/phases_v4.csv").slice(1)
#figure(
  text(size: 7.5pt, table(
    columns: 11, align: right,
    [*q*], [*arm*], [*t_rel*], [*t_reduce*], [*t_filter*], [*t_LA*], [*t_ic*], [*candidates*], [*LA cols*], [*LA nnz*], [*mulmods*],
    ..ph.map(r => r.slice(0, 6) + r.slice(7, 12)).flatten()
  )),
  caption: [Phase medians per arm (`results/phases_v4.csv`; seconds, L0).],
)

Matrix shape at $q = 24001$ (medians, counted):

#text(size: 8.5pt, table(
  columns: 5, align: (left, right, right, right, right),
  [*arm*], [*columns*], [*nnz*], [*nnz / row*], [*mulmods*],
  [lp50], [5975], [80 289], [13.4], [5.46e9],
  [dlp50], [5975], [172 160], [28.4], [12.09e9],
  [dlpm50], [10 424], [104 248], [10.0], [13.04e9],
  [dlpf50], [6765], [109 678], [16.0], [8.90e9],
))

The mechanism is now clear [pcc:unverified]. The filter keeps 822 of the large primes as columns
(median), merges 3669 and prunes the rest. That brings the rows down to 16 non-zeros (from 28),
at 13 % more columns than lp. *dlpf still does 1.63× lp's LA mulmods at every $q$* (median
1.608–1.627, flat). dlpf wins on wall time only through collection: it needs half the relation
candidates, and its $t_"rel"$ is 27.4 s against lp's 56.1 s at $q = 24001$. Its LA is 63.2 s
against 38.9 s. Collection scales more slowly than LA (LA columns × nnz grows as about $q^2$ here),
so the LA penalty overtakes the collection saving as $q$ grows. That is exactly what F3's ratio
trend shows.

#figure(image("figures/filter_matrix.svg", width: 100%), caption: [Matrix dimension, weight and counted Wiedemann work per arm (medians).])

= Distance remaining to rho

#figure(image("figures/filter_gap_to_rho.svg", width: 72%), caption: [Median IC time / expected-steps rho, same instance. Every arm stays above 1 at every $q$.])

Rho is still faster at every $q$ for every arm. The best median is dlpf50 at 1.21× rho [pcc:unverified]
($q = 24001$), with lp50 at 1.23×. Under §5a, IC does no group operations, so the comparison is
on paired wall time only, and the 0 / 0.1 / 1 GAE band does not change the order.

= Leads this opens

+ *Excess, then clique removal.* Every arm stops at columns + 5 rows, so the filter has no surplus
  to spend. NFS filtering collects 5–20 % excess and deletes the heaviest large-prime cliques.
  Collection is dlp's cheap phase (half of lp's $t_"rel"$), so buying excess with it to cut LA
  weight is the natural trade. Prediction to freeze next: at $q = 24001$ an excess of 10 % with
  clique removal brings dlpf's mulmods below lp's.
+ *A non-greedy merge rule.* The greedy rule stops while 822 large primes are still columns. A
  Markowitz-style rule with a cost-model target (structured Gaussian elimination to a fixed
  density) is the standard next step.
+ *A block/dense hybrid.* With about 6.8 k columns at 16 non-zeros per row, the matrix is small
  enough for a structured-Gauss-to-dense switch to compete with Wiedemann. That needs a counted
  dense cost.

= Reproduce

```
cargo build --release && cargo test --release          # 7 tests
BIN=dlp_filter_20261115/bin/humbert-v4 BIN_SHA256_PREFIX=5924fd1b ./dlp_filter_20261115/run_v4.sh <new-dir>
target/release/humbert summarize-filter dlp_filter_20261115/results <dir>/results/v4.jsonl \
  dlp_20261113/campaign_a/results/v3.jsonl dlp_20261113/campaign_b/results/v3.jsonl
python3 dlp_filter_20261115/plot_filter.py
```
