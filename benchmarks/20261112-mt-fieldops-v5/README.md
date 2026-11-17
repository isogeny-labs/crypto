# Multi-target IC v5: IC vs multi-target rho with native work priced (2026-11-12)

Study: [`research/mt_fieldops_v5_20261112/`](../../research/mt_fieldops_v5_20261112/) (protocol, results, data,
independent validation). Figures `ratios.svg`, `ratios.png`; report `REPORT.pdf`.

With every arm's hash-table and native work priced at h (section 5a), the cheapest of four multi-target IC variants
against the cheapest of ten runs of this harness's multi-target rho is unranked in all 27 (cell, I/M) decisions
because the order flips with h: at I = 10M 1.06-1.16x at h = 0, 1.00-1.05x at h = 0.1, 0.71-0.80x at h = 1. The
h = 1 advantage rests on charging distinguished-point rho two native operations per point. Distance to a paired
BSGS: 1.11-1.47x. 3366 runs on 33 fresh curves, every target verified, every row replayed exactly by the
validator. Claim PCC-20261112-22e5d2. [pcc:unverified]
