# Multi-target IC v4: IC vs standard multi-target rho (2026-11-12)

Study: [`research/mt_fieldops_v4_20261112/`](../../research/mt_fieldops_v4_20261112/) (protocol, results, data,
independent validation). Figures `ratios.svg`, `ratios.png`; report `REPORT.pdf`.

No ranking against rho is established. Under the frozen rule the IC (cheaper of ic5 and ic5 with retention) is
1.04-1.16x (h = 0), 1.03-1.08x (h = 0.1) and 1.20-1.76x (h = 1) the cheapest of a Kuhn-Struik retention rho and a
distinguished-point rho at I = 10M; with rho's per-step canonicalisations priced at h (section 5a) every decision is
unranked (IC 0.89-0.94x at h = 1). Distance to a paired BSGS: 1.14-1.49x. 3960 runs on 33 fresh curves, every
target verified, every row replayed exactly by the validator. Gate: does not pass (validator status
insufficient_evidence, strongest_available false), so this directory has no row in `benchmarks/README.md`.
[pcc:unverified]
