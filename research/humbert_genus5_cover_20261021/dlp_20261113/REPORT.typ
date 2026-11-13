#set document(title: "Double large primes on the genus-5 cover")
#set page(paper: "a4", margin: 1.8cm, numbering: "1")
#set text(size: 9.5pt)
#set par(justify: true)
#set heading(numbering: "1.")

#align(center)[
  #text(size: 15pt, weight: "bold")[Double large primes on the genus-5 cover:\ half the candidates, twice the linear algebra]\
  #v(2pt)
  Study `research/humbert_genus5_cover_20261021/dlp_20261113` · 2026-11-13 · follows `lp_20261027` (PR \#2710)
]

#v(6pt)
*Scope.* Toy fields only ($q$ prime, $4001 <= q <= 16001$), the iota-folded index calculus on the
descended genus-5 curve $C$ (PR \#2580). One campaign under `PROTOCOL.md`, frozen before data.
Grid: 4 values of $q$ × 5 seeds × 5 arms = 100 runs, same instance per $(q, "seed")$ in every arm,
arm order rotated per seed, one frozen binary (`bin/humbert-v3`, sha256 prefix `a8ff53874ff869a9`).
Rho runs once per instance; every arm is compared with that instance's expected-steps rho
$sqrt(pi r slash 2)$ × measured s/step.

*Host and standing.* 4-vCPU shared cloud VM, no isolation (L0); isolab-eks (AGENTS.md §9a) was
not reachable from the session. Every wall-time number below is *exploratory* and an
*unverified assertion*; no row goes to the IC leaderboard (no replay certificate, validator report
or `VALIDATION.json`). The candidate and mulmod counts are host-independent.

*Arms.* `pre`: `iota` with the linear-algebra precheck (the PROTOCOL_v2 baseline). `pre_aa`: the
same again in its own process (A/A). `lp50`: single large prime, small base of half the x-values.
`dlp50`, `dlp35`: double large prime with small bases of 50 % and 35 %. A residual with at most
two points outside the small base is reduced against stored pivots, one per large prime
(`cur·b − pivot·a` cancels that large prime); a row that reaches zero large primes is a full
relation; a row left with one or two unpivoted large primes becomes the pivot for one of them;
rows with more are dropped. Large-prime logs follow from the pivots after the linear algebra.

*Correctness.* All 100 runs solved the DLP and passed the pushforward ($8P$), $T(P) != 0$,
iota-pair and $X_infinity = 4c$ checks (`humbert summarize-dlp`: "all correctness gates pass: true").

= Predictions against the data

#let s = csv("results/summary_v3.csv").slice(1)
#figure(
  text(size: 8pt, table(
    columns: 7, align: right,
    [*q*], [*A/A median [min, max]*], [*pre/dlp50 median [min, max]*], [*dlp50 faster*], [*lp50/dlp50 median*], [*cand dlp50/lp50 max*], [*IC/rho-exp median: pre, lp50, dlp50, dlp35*],
    ..s.map(r => (r.at(0), [#r.at(2) [#r.at(3), #r.at(4)]], [#r.at(5) [#r.at(6), #r.at(7)]], [#r.at(8)/#r.at(1)], r.at(9), r.at(10), [#r.at(11), #r.at(12), #r.at(13), #r.at(14)])).flatten()
  )),
  caption: [Paired ratios on the same instance, 5 seeds per $q$ (`results/summary_v3.csv`, from `campaign_a` and `campaign_b`). Above 1, dlp50 is faster.],
)

- *R1 (counted): holds.* dlp50 needs fewer relation candidates than lp50 on all 20 instances; the
  ratio is 0.472–0.513, flat in $q$.
- *R2 (wall time): holds.* dlp50 is faster than `pre` on 5/5 seeds at each of $q = 8009$, 12007 and
  16001 (medians 1.15, 1.15, 1.41). The A/A spread is within ±5 % except one instance at 8.2 %.
- *R3: holds.* Median IC / expected-steps rho for dlp50 is below `pre` at each $q >= 8009$:
  2.18 vs 2.34, 2.85 vs 3.15, 1.65 vs 2.28. Rho is still faster at every $q$ for every arm; that
  is the distance remaining.
- *R4 (exploratory): no exponent gain.* The log–log slope of median $t_"ic"$ from 4001 to 16001 is
  1.68 for dlp50 and 1.77 for `pre`. Both are far above the $q^(1.6)$ that GTTD predicts
  asymptotically for genus 5.

#figure(image("figures/dlp_ratios.svg", width: 78%), caption: [Paired wall-time ratios; points are instances, lines are medians.])

= The unpredicted result: dlp's advantage over lp vanishes with $q$

The median of lp50/dlp50 falls from 1.71 at $q = 4001$ to 1.24, 1.08 and 0.97. At $q = 16001$
dlp50 is no faster than single large primes, even though it still needs half the candidates.
The counted phase split explains it:

#let ph = csv("results/phases_v3.csv").slice(1).filter(r => r.at(1) != "pre_aa")
#figure(
  text(size: 8pt, table(
    columns: 8, align: right,
    [*q*], [*arm*], [*t_rel (s)*], [*t_LA (s)*], [*t_descent (s)*], [*candidates*], [*LA mulmods*], [*unknowns*],
    ..ph.map(r => (r.at(0), r.at(1), r.at(2), r.at(3), r.at(4), r.at(5), r.at(6), r.at(7))).flatten()
  )),
  caption: [Medians over the 5 instances per $q$ (`results/phases_v3.csv`).],
)

At every $q$, dlp50 and lp50 solve the same number of unknowns. dlp50 needs 0.50× the candidates
but does 1.21×, 2.12×, 2.11× and 2.13× the LA mulmods. Wiedemann work at a fixed dimension is
proportional to the matrix weight, so dlp's reduced rows are about twice as heavy. Each reduction
step adds a pivot's whole support to the row, and chains of pivots compound that fill-in.
Relation collection grows more slowly in $q$ than the linear algebra does (`pre`: $t_"rel"$ 2.6 → 14 s,
$t_"LA"$ 3.0 → 57 s). The candidate saving is therefore paid for in the phase that dominates at
larger $q$. dlp35 shrinks the matrix (fewer unknowns), not its density, and is the fastest arm at
$q = 16001$ by median IC/rho (1.59). The margin over lp50 and dlp50 is inside the instance spread.

#figure(image("figures/dlp_phases.svg", width: 88%), caption: [Counted cost per phase: candidates (left) and LA mulmods (right), log–log.])
#figure(image("figures/dlp_gap_to_rho.svg", width: 70%), caption: [Median IC time over expected-steps rho; 1 is parity.])

= What this means for the lead

- Against the fixed baseline (`pre`), double large primes are a measured improvement at every $q$
  tested (R2, R3). This is an unverified, unisolated (L0) assertion, not a leaderboard row.
- Against single large primes, they are not, from $q approx 16000$ on. The best arm per $q$ at the
  top of the grid is any of lp50, dlp50 or dlp35, all within noise of each other.
- The exponent did not move at toy scale (R4). The binding constraint is linear-algebra density,
  not the relation yield.
- *Next step (proposed, not run):* a weight-bounded reduction. Drop or defer any reduced row whose
  support exceeds a bound $w$, or cap the pivot-chain length. Trade back some of the 2× candidate
  saving for the 2× LA weight. Prediction to freeze: at $q = 16001$, LA mulmods drop to within 1.2×
  of lp50 while candidates stay at most 0.75× lp50.

*Labelling caveat.* The `dlp_cycles` field counts every full row emitted by the reducer, including
rows that had no large prime from the start; it is not a count of graph cycles. No analysis uses it.

*Reproduce.* `BIN=dlp_20261113/bin/humbert-v3 BIN_SHA256_PREFIX=a8ff53874ff869a9 PRIMES="…" ./run_v3.sh <new-dir>`;
`humbert summarize-dlp dlp_20261113/results campaign_a/results/v3.jsonl campaign_b/results/v3.jsonl`;
`python3 plot_dlp.py`. Bench record: `benchmarks/records/20261113T1559Z-humbert-genus5-dlp.json`.
