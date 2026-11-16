#set document(title: "One-solve Lanczos on the genus-5 cover")
#set page(paper: "a4", margin: 1.8cm, numbering: "1")
#set text(size: 9.5pt)
#set par(justify: true)
#set heading(numbering: "1.")

#align(center)[
  #text(size: 15pt, weight: "bold")[One-solve Lanczos on the genus-5 cover:\ the linear algebra drops to 0.19×, excess + clique removal does not pay]\
  #v(2pt)
  Study `research/humbert_genus5_cover_20261021/la_solver_20261116` · 2026-11-16 · follows `dlp_filter_20261115` (PR \#4065)
]

#v(6pt)
*Question.* In `dlp_filter_20261115` the filtered double-large-prime arm (`dlpf`) was the best
index-calculus arm, but its linear algebra (LA) was 63 of 96 s at $q = 24001$, and its lead over
single large primes (`lp`) was shrinking with $q$. Three levers on the LA phase were tested, each
isolated in its own arm, against the incumbent `dlpf:0.5` with two Wiedemann solves (`w2`):
`w1` (one Wiedemann solve plus the end-to-end check $x P = Q$), `l1` (one scalar Lanczos solve,
every mulmod counted), and `dlpx` (10 % excess relations, then clique removal).

*Scope and standing.* Toy fields only. 3 values of $q$ × 5 seeds × 8 arms = 120 runs under
`PROTOCOL.md`, frozen before data, with one dated addendum about launch splitting written before
any analysis. Same instance per $(q, "seed")$, arm order rotated per seed, one frozen binary
(`bin/humbert-v5`, sha256 prefix `6fc61135`). The host is a 4-vCPU shared VM, no isolation (L0);
isolab-eks was not reachable (no AWS identity). Every wall-time number is *exploratory* and an
*unverified assertion* [pcc:unverified]. Candidate, matrix and mulmod counts are
host-independent. No row goes to the IC leaderboard: there is no replay certificate, validator
report or `VALIDATION.json`.

*Correctness.* All 120 runs solved the DLP and passed every gate (pushforward $8P$, [pcc:unverified]
$T(P) != 0$, iota pairs, $X_infinity = 4c$, and the end-to-end check $x P = Q$, which does not use
the planted answer).

*Interruption.* The $q = 24001$ launch hit the 2 h background limit after 34 of 40 runs. The
other six (seed 5, protocol positions 2–7) ran afterwards with the same binary and order and carry
`"resumed": true`. $q = 32003$ ran in three launches (seed 1; seeds 2–3; seeds 4–5). The cause of
the slow launch is the `dlpx` pathology in §4.

= Control C0 (counted): exact

- 20/20 `lp50_w2` and `dlpf50_w2` runs at $q <= 24001$ reproduce their `v4` records
  (rel_candidates, la_mulmods, x_recovered).
- Every solver variant of an arm saw exactly the same relation candidates on every instance.
- `dlpx50_l1_aa` has counted fields identical to `dlpx50_l1` on all 15 instances.

= Predictions against the data

#figure(image("figures/solver_mulmods.svg", width: 76%), caption: [Counted LA mulmods, same instance (`results/instances_v5.csv`).])

- *S1 (counted): holds.* w1/w2 mulmods = 0.500 on 15/15 instances (exactly one of two solves).
- *S2 (counted): holds.* l1/w1 = 0.401–0.407 for dlpf (≤ 0.50), and lp l1/w2 = 0.201–0.213 [pcc:unverified]
  (≤ 0.25), with Lanczos charged for every dot product and update and Wiedemann not.
- *S3 (safety): holds.* 90 one-solve runs (w1, l1), 0 end-to-end or solve failures. By the rule
  of three the failure rate is below 3.3 % (95 %). The unimplemented repair path (one more solve
  plus descent) therefore adds at most 3.3 % of a solve in expectation.
- *S4 (wall time): holds.* Median t_LA dlpf l1/w2 is 0.192, 0.194, 0.192 at $q = 16001$, 24001,
  32003 (bound 0.35). The wall-time ratio tracks the counted ratio 0.203.
- *X1 (counted): holds, weakly.* dlpx/dlpf mulmods (both l1) is 0.890–0.974, median 0.962–0.971.
- *X2 (wall time): falsified.* dlpx is faster than dlpf (l1) on 1/5 seeds at $q = 24001$ and
  0/5 at 32003. On the instances without the pathology below, dlpx is 1–5 % slower: the 10 %
  extra relations cost more collection time than the 3 % of LA it saves.
- *G1 (wall time): holds.* With l1, dlpf is faster than lp on 5/5 seeds at every $q$, with the
  paired lp/dlpf median at 1.59, 1.54, 1.51. Under w2 this lead had fallen to 1.02 at 24001.
- *G2 (wall time): holds.* The best l1 median IC / expected-steps rho is 0.55 at $q = 24001$ and
  0.36 at 32003 (dlpf50_l1). Against rho with the negation map ($sqrt(pi r slash 4)$ steps) it is
  0.78 and 0.51. See §5 for what this does and does not mean.
- *F5 (exploratory).* Log–log slope of median $t_"ic"$ from 16001 to 32003: lp50_l1 1.29,
  dlpf50_l1 1.36, dlpx50_l1 1.39, lp50_w2 1.50, dlpf50_w1 1.63, pre_l1 1.73, dlpf50_w2 1.75.

#figure(image("figures/solver_ratios.svg", width: 76%), caption: [Paired wall-time ratios (L0). Black: the A/A arm (0.979–1.068).])

= Against the incumbent

The previous best arm [pcc:unverified], `dlpf50_w2`, becomes `dlpf50_l1` with nothing else changed. The relation
set is identical (C0), the counted LA work is 0.203× (median), and the paired wall-time ratio
dlpf_w2 / dlpf_l1 is [pcc:unverified]

#align(center, table(
  columns: 4, align: right,
  [*q*], [*median*], [*min*], [*max*],
  [16001], [1.89], [1.84], [1.96],
  [24001], [2.16], [2.09], [2.16],
  [32003], [2.42], [2.37], [2.53],
))

The gain grows with $q$ because LA was the growing share of the old arm's time. The lower bound
of every per-q range is far outside the A/A spread (0.98–1.07).

#let ph = csv("results/phases_v5.csv").slice(1)
#figure(
  text(size: 7.5pt, table(
    columns: 11, align: right,
    [*q*], [*arm*], [*t_rel*], [*t_filter*], [*t_LA*], [*t_ic*], [*candidates*], [*LA cols*], [*LA nnz*], [*mulmods*], [*cliques*],
    ..ph.map(r => r.slice(0, 5) + r.slice(6, 12)).flatten()
  )),
  caption: [Phase medians per arm (`results/phases_v5.csv`; seconds, L0).],
)

#figure(image("figures/solver_phases.svg", width: 100%), caption: [Median phase times. After Lanczos, relation collection is 62 % of dlpf's time at $q = 32003$.])

= The dlpx pathology: clique removal and the precheck feed each other

At $q = 24001$, seed 5, both dlpx processes (A/A identical) needed 17 087 precheck extensions,
collected 91 985 relations for 5950 unknowns and spent 2368 s in the filter ($t_"ic"$ 2637 s,
50.9× rho). The loop: removing a tree of large-prime-linked rows can leave a factor-base column
with no relation; the empty-column precheck then raises the target by twice the empty count; the
new surplus is removed again as cliques, which empties columns again. Milder instances of the
same loop: 42 and 29 extensions at 24001 (seeds 4, 3) and 198 at 32003 (seed 4). This is a
defect of this `dlpx` implementation, not of clique removal in general. A tree whose removal
would empty a column should not be removed. X2 is falsified on the unaffected instances as well,
so the fix would not change the verdict at 10 % excess.

= Distance remaining to rho

#figure(image("figures/solver_gap_to_rho.svg", width: 72%), caption: [Median IC time / expected-steps rho, same instance.])

dlpf50_l1 is below this repository's rho on these toy instances: 0.77, 0.55, 0.36 (no negation
map) or 1.09, 0.78, 0.51 (with) at $q = 16001$, 24001, 32003 [pcc:unverified]. Read it narrowly:

- It is the expected crossover of a known attack. The genus-5 cover gives about $q^(1.4)$ here
  against rho's $sqrt(r) approx q^2$. It is not a break of any curve, and not a new exponent.
- The rho reference is this crate's single-threaded scalar rho (about 555 ns per step,
  6 $FF_(q^4)$ multiplications), timed in the same process. A faster rho moves the crossover [pcc:unverified].
- $r$ varies by up to 24× between instances at the same $q$, so the per-instance ratio varies
  (0.22–1.19 at 24001) while the IC time hardly does.
- §5a: IC does no group operations, so arms are compared on paired wall time only. The
  0 / 0.1 / 1 GAE band does not change the order.

= Leads this opens

+ *Relation collection is now the bottleneck* (38 of 62 s for dlpf at 32003). Cheaper
  decomposition tests or sieving in the cover's coordinates are the next lever. LA is 34 %.
+ *Block Lanczos / block Wiedemann* would parallelise the remaining LA over the 4 cores. On a
  single core, scalar Lanczos is already close to the $n$-iteration floor.
+ *Fix the dlpx loop* (never remove a tree that empties a column) and re-test excess at 2–5 %.
  It cannot gain more than the 3 % of LA that X1 shows, so it is low priority.
+ *Validation for the leaderboard:* an isolated (L2) rerun with a replay certificate is needed
  before `dlpf50_l1` can replace `dlpf50_w2` as a leaderboard row.

= Reproduce

```
cargo build --release && cargo test --release          # 7 tests
BIN=la_solver_20261116/bin/humbert-v5 BIN_SHA256_PREFIX=6fc61135 PRIMES=16001 ./la_solver_20261116/run_v5.sh <dir>/q16001
BIN=... PRIMES=32003 SEEDS="1 2" K_FROM=0 ./la_solver_20261116/run_v5_part.sh <dir>/q32003   # split launches
target/release/humbert summarize-solver la_solver_20261116/results <dir>/q*/results/v5.jsonl \
  dlp_filter_20261115/campaign/results/v4.jsonl
python3 la_solver_20261116/plot_solver.py
```
