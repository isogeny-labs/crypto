# Quartic-extension genus-five cover — toy reproduction (in progress)

Status: complete. Results report: [`REPORT.pdf`](REPORT.pdf) (source `REPORT.typ`,
rendered with Typst from the files in `results/` and `figures/`); tables in
[`results/summary.md`](results/summary.md); benchmark copy in
[`benchmarks/20261026-humbert-genus5-cover/`](../../benchmarks/20261026-humbert-genus5-cover/).
The attack sweep covers 90 runs (q = 61–8009, 3 seeds, full and iota arms): 90/90 logs
recovered, every invariant check passed, and matched rho is faster at every q. [pcc:unverified]

Toy fields only (q prime, 17 ≤ q ≤ 8009; subgroup orders r < 2^62).
Native Rust in `src/`, no dependencies. `plots.py` and `summarize.py` only lay out
recorded values.

## Reproduce

    cargo test --release            # 4 regression tests
    ./run_sweep.sh classify         # results/classify.jsonl
    ./run_sweep.sh verify           # results/verify.jsonl
    PRIMES="61 101 ... 8009" ./run_sweep.sh attack   # results/attack.jsonl
    python3 summarize.py            # Markdown tables of the recorded rows
    python3 plots.py                # figures/*.svg, *.png

## Records

- `results/verify.jsonl` — L-polynomial of the explicit model vs L_E(T^4) L_D(T)
  (q = 17, 19, 23; both twists; 2 seeds).
- `results/classify.jsonl` — binary-pattern classifier, d ∈ {2,3,4}, source genus 1–3.
- `results/attack.jsonl` — end-to-end index calculus on C vs matched Pollard rho.
- `results/attack_v1_superseded.*` — first sweep, kept as a record; it contains one
  wrong log (q = 1511) from a linear-algebra bug (unused factor-base columns),
  fixed before the current sweep.
- `results/host.txt` — host (4 vCPU shared cloud VM, no isolation).

## Double large primes (2026-11-13, `dlp_20261113/`)

Frozen protocol `dlp_20261113/PROTOCOL.md`; 100 runs (q = 4001–16001, 5 seeds, 5 arms); all
correctness gates pass; report `dlp_20261113/REPORT.pdf`. Unisolated L0 wall time, unverified
assertion, not a leaderboard row.

- R1 (counted) holds: dlp:0.5 needs 0.47–0.51× the relation candidates of lp:0.5 on all 20 instances. [pcc:unverified]
- R2 and R3 hold against iota + precheck: dlp:0.5 faster on 5/5 seeds at q = 8009, 12007 and 16001.
  Median IC / expected-steps rho is 2.18, 2.85 and 1.65, against 2.34, 3.15 and 2.28 for the baseline.
- Not predicted: dlp:0.5's lead over lp:0.5 falls from 1.71× to 0.97× (q = 4001 → 16001). Its reduced
  rows are about twice as heavy (2.1× LA mulmods at the same number of unknowns), and the linear
  algebra dominates at larger q.
- R4 (exploratory): slope of median t_ic is 1.68 (dlp:0.5) against 1.77 (baseline); no exponent gain.
  Rho is faster at every q.
- Next step (proposed): weight-bounded reduction (cap the support of reduced rows or the pivot-chain length).

## Filtered double-large-prime matrices (2026-11-15, `dlp_filter_20261115/`)

Frozen protocol `dlp_filter_20261115/PROTOCOL.md`; 120 runs (q = 8009–24001, 5 seeds, 6 arms); all
correctness gates pass; C0 control exact (45/45 replays of `dlp_20261113`, identical candidates and
A/A counted fields); report `dlp_filter_20261115/REPORT.pdf`. Unisolated L0 wall time, unverified
assertion [pcc:unverified], not a leaderboard row.

- F1, F2 (counted) hold: keeping large primes as columns and merging greedily (dlpf) does 0.72–0.77× [pcc:unverified]
  the LA mulmods of full elimination and 0.67–0.69× of no merging, on all 20 instances.
- F3, F4 hold: dlpf beats lp:0.5 on wall time 5/5, 5/5, 4/5 at q = 12007, 16001, 24001, and has the
  lowest median IC / expected-steps rho (2.38, 1.47, 1.21). At q = 24001 the margin over lp (1.23) is
  inside the A/A spread.
- Not solved: dlpf's LA is still 1.61–1.63× lp's at every q (16 vs 13.4 non-zeros per row, 13% more
  columns). It wins on collection only, and its lead over lp falls 1.31 → 1.02 (slopes 1.67 vs 1.44).
  Rho is faster at every q.
- Next step (proposed): collect 10% excess and remove heavy large-prime cliques; or a non-greedy
  (Markowitz) merge rule.
- The runner hit the session's 2 h background limit at 117/120 rows; the last instance's three missing
  arms ran afterwards with the same binary and are tagged `resumed: true`.

## One-solve Lanczos and excess + clique removal (2026-11-16, `la_solver_20261116/`)

Frozen protocol `la_solver_20261116/PROTOCOL.md` (with a launch-splitting addendum written before
analysis); 120 runs (q = 16001, 24001, 32003; 5 seeds; 8 arms); all correctness gates, including the
end-to-end check x·P = Q, pass; C0 control exact (20/20 replays of `v4`, identical candidates across
solvers, identical dlpx A/A counted fields); report `la_solver_20261116/REPORT.pdf`. Unisolated L0
wall time, unverified assertion [pcc:unverified], not a leaderboard row.

- S1–S4 hold [pcc:unverified]: one Wiedemann solve is exactly 0.500× two; scalar Lanczos is 0.40× one
  Wiedemann solve, counting every Lanczos mulmod. The incumbent dlpf:0.5 with Lanczos needs 0.203×
  the counted LA, and t_LA is 0.19×. Paired t_ic of old (w2) over new (l1) is 1.89×, 2.16× and 2.42×
  at q = 16001, 24001 and 32003. There were 0 failures in 90 one-solve runs.
- G1 holds: with l1, dlpf is faster than lp on 15/15 instances (median 1.51–1.59×).
- G2 holds: the best median IC / expected-steps rho is 0.55 (q = 24001) and 0.36 (32003); against
  negation-map rho it is 0.78 and 0.51. This is the expected toy crossover of the cover attack
  (slope 1.36 vs rho's about 2) against this crate's scalar rho, not a break.
- X1 holds weakly (dlpx 0.96× dlpf's mulmods); X2 is falsified (dlpx is faster on 1/5 and 0/5 seeds).
  Found a feedback loop: clique removal empties factor-base columns, the precheck then raises the target,
  and so on (17 087 extensions and 2637 s at q = 24001 seed 5).
- Next step (proposed): relation collection is now 62% of dlpf's time; then block Lanczos on 4 cores;
  then an L2 rerun with a replay certificate so dlpf50_l1 can reach the leaderboard.

## Three levers on dlpf50_l1 (2026-11-16, `levers_20261116/`)

Faster relation arithmetic (`HUMBERT_REL_FAST`), threaded Krylov iterations (`HUMBERT_LA_THREADS`)
and safe clique removal (`HUMBERT_CLIQUE_SAFE`) were built and independently verified in workflow
`wf_ff8cffdd-dad`, then merged behind flags. The campaign was 135 runs under a protocol frozen
before data: all solved, and the counted fields are identical to the flag-free incumbent on all
75 flagged runs. Paired median t_ic comb/base was 0.184, 0.182 and 0.170 at q = 16001, 24001 and
32003. The relation phase ran at 0.042× and the LA phase at 0.29× [pcc:unverified] (L0). With
relations cheap, lp ties dlpf, and excess with safe clique removal gives no measurable gain.
Report: `levers_20261116/REPORT.pdf`.

## Next leads and a matched rho (2026-11-20, `next_leads_20261120/`)

Faster setup (`HUMBERT_SETUP_FAST`), a factor-base re-balance and a matched multi-thread rho
(`HUMBERT_RHO_*`: distinguished points, 64 batched affine walks per thread, Barrett K4, negation map)
were built and independently verified in workflow `wf_51f8d293-995`. The campaign was 150 runs under
a protocol frozen before data. Setup is 0.06× and counted identical; dlpf at alpha 0.3 is 0.83× of
alpha 0.5 at q = 32003. The matched rho runs at about 14× the scalar rho's steps per second.
Against it, the best IC arm takes 1.68, 1.19 and 0.77 of rho's expected time at q = 16001, 24001
and 32003 [pcc:unverified] (L0). The earlier "below rho" ratios of 0.04–0.36 were measured against
a weak scalar rho and are superseded. Report: `next_leads_20261120/REPORT.pdf`.

## Slope test against a SIMD matched rho (2026-11-20, `slope_20261120/`)

The crate now accepts q < 2^16. Two further flags were verified in workflow `wf_6b058b58-659`:
the SIMD matched rho (`HUMBERT_RHO_SIMD`) and Montgomery mod-r linear algebra
(`HUMBERT_LA_FASTMOD`). The campaign was 160 runs over q = 16001–65521; all solved, with counted
fields identical. Against the SIMD rho, which runs at 4.5× the v7 rho's steps per second, the
best IC arm takes 3.89, 1.97, 2.00 and 1.37 of rho's expected time. The slopes are 1.21 for IC
and 2.00 for rho, which puts the extrapolated crossover near q ≈ 92 000 [pcc:unverified] (L0).
Report: `slope_20261120/REPORT.pdf`.

## Crossover test at q < 2^17 (2026-11-21, `crossover_20261121/`)

The crate now accepts q < 2^17. The campaign was 150 runs over q = 65521–131071, all solved. The
crossover extrapolated from `slope_20261120` (q ≈ 92 000) is falsified: the best IC arm's
per-instance ratio to the SIMD rho's expected time rises from 1.21 to 2.00 [pcc:unverified]. Two
causes are measured: LA is quadratic in q at fixed alpha, and the r < 2^62 rule makes r ≪ q^4,
which flattens rho. Report: `crossover_20261121/REPORT.pdf`. A lab-wide audit of rho references
followed: `research/rho_reference_audit_20261121`.
