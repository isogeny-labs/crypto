# Quartic-extension genus-five cover — toy reproduction (in progress)

Status: complete. Results report: [`REPORT.pdf`](REPORT.pdf) (source `REPORT.typ`,
rendered with Typst from the files in `results/` and `figures/`); tables in
[`results/summary.md`](results/summary.md); benchmark copy in
[`benchmarks/20261026-humbert-genus5-cover/`](../../benchmarks/20261026-humbert-genus5-cover/).
The attack sweep covers 90 runs (q = 61–8009, 3 seeds, full and iota arms): 90/90 logs
recovered, every invariant check passed, and matched rho is faster at every q.

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

- R1 (counted) holds: dlp:0.5 needs 0.47–0.51× the relation candidates of lp:0.5 on all 20 instances.
- R2 and R3 hold against iota + precheck: dlp:0.5 faster on 5/5 seeds at q = 8009, 12007 and 16001.
  Median IC / expected-steps rho is 2.18, 2.85 and 1.65, against 2.34, 3.15 and 2.28 for the baseline.
- Not predicted: dlp:0.5's lead over lp:0.5 falls from 1.71× to 0.97× (q = 4001 → 16001). Its reduced
  rows are about twice as heavy (2.1× LA mulmods at the same number of unknowns), and the linear
  algebra dominates at larger q.
- R4 (exploratory): slope of median t_ic is 1.68 (dlp:0.5) against 1.77 (baseline); no exponent gain.
  Rho is faster at every q.
- Next step (proposed): weight-bounded reduction (cap the support of reduced rows or the pivot-chain length).
