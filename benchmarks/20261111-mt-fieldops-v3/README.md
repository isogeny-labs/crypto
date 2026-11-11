# Multi-target IC v3: k-term table (ic5) at 22-32 bits (2026-11-11)

Study: [`research/mt_fieldops_v3_20261111/`](../../research/mt_fieldops_v3_20261111/) (protocol, results, data,
independent validation). Figures: `ratios.svg`, `ratios.png`; report `REPORT.pdf`.

- **Validated:** ic5 is cheaper per target than ic4, the multi-target IC baseline, in all 36 (cell, I/M)
  decisions (0.47-0.99); class engineering. Distance remaining to a paired BSGS: 1.11-1.49 (dearer in all 36).
- **Against rho, not established:** 0.85-0.98 of this harness's anchored rho family, but dearer than the same
  family with Kuhn-Struik retention at h = 0 (1.07-1.34) and h = 0.1 (1.05-1.18); unranked in all 36.

Gate: `VALIDATION.json` does not pass (validator status `insufficient_evidence` for the vs-rho headline,
`strongest_available: false`), so the row renders unvalidated. 4488 runs on 44 fresh curves, every target verified,
every row replayed exactly by the validator. [pcc:unverified]
