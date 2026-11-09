# Single large prime on the genus-5 cover (2026-10-27)

Report: [`REPORT.pdf`](REPORT.pdf) (source `REPORT.typ`). Protocols, frozen before their data:
[`PROTOCOL.md`](PROTOCOL.md) (v1, amendments 1–2) and [`PROTOCOL_v2.md`](PROTOCOL_v2.md).

- v1 (120 runs, q = 2003–12007): lp:0.5 against the merged iota baseline. P1 falsified (3/5 at
  q = 8009). Every win came from baseline instances that needed several linear-algebra rounds.
- v2 (80 runs, q = 4001–12007): the precheck (no Wiedemann solve while a column is empty) is
  faster on 8/8 multi-round instances (2.0–5.1×); Q1's one-round clause failed on one instance
  (0.92 vs an A/A floor of 0.941). Against the fixed baseline, lp:0.5 is slower at q ≤ 8009
  (Q2 holds) and faster at q = 12007 on 5/5 seeds, median 1.08. That q = 12007 result is an
  exploratory diagnostic from an unisolated VM, not an established crossover.
- Every run solved the DLP and passed all correctness checks. Rho is faster at every q.
- Not done: validation gate (so no benchmarks or leaderboard row); double large primes.

Tables: `results/summary.md`, `results/summary_v2.md`, computed by `humbert summarize-lp .` (Rust,
rejects duplicate records); `plot_lp.py` only draws figures from its CSVs. Reproductions go into a
new directory: `run_campaign.sh <new-dir>`, `run_v2.sh <new-dir>`, then `humbert summarize-lp <new-dir>`.
