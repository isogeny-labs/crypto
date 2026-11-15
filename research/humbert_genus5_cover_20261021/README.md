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
