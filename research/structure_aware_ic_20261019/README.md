# Structure-aware index calculus: two constant-factor tests (2026-10-19)

Results report: [`report.pdf`](report.pdf) (source `report.typ`). Frozen protocol and four
amendments: [`PROTOCOL.md`](PROTOCOL.md). Graphs: `reports/` (SVG + PNG, from `plot_report.py`).
Sessions, audits, band reports: `sessions/`. Specs: `specs/`. Run script: `run_sessions.sh`.
Curve selector and analysis: `examples/structure_aware_curves.rs`.

| test | question | result | class |
|:--|:--|:--|:--|
| A: E11b, half base on three new a=1 cofactor-2 curves | is the half base below the default across the §5a band on ≥ 2 of 3 curves? | **Not falsified.** Frozen (default d9): MITM 0.282 [0.271, 0.299], 3 of 3 curves, but the baseline was mistuned. Tuned against tuned: 0.690, 0.653, 0.725 per curve (≈ 1.45× pooled), 3 of 3 across the band. F4 pair: 1.014 [0.942, 1.090], null. | engineering (constant) |
| B: j = 0 vs six-fold rho | is IC/rho better on j = 0 than on a random control of the same size? | **Falsified.** Q = 0.87–1.14 at 20–28 bits; at 32 bits 1.06 / 0.97 / 0.69 at w = 0 / 0.1 / 1 (order not stable across the band). IC gain 1.85–2.2× matches rho gain 1.67–1.92×. | null |

Distance remaining: tuned half-base MITM 8.8–12.4× its matched rho (Test A); tuned IC 8–35× its matched rho, rising with size (Test B).
Any gain here is a constant, not an exponent. Isolation: cloud VM, operation counts only, no wall-time claim.
No leaderboard row moves (no table A–C row improves), so the leaderboard was not regenerated.
