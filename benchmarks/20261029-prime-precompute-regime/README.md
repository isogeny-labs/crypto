# Prime-field precomputation regime: known-log k-sum table against preprocessing rho (2026-10-29)

These are copies of the visual record of
[`research/prime_precompute_regime_20261028/`](../../research/prime_precompute_regime_20261028/README.md),
which is the source of truth: `report.pdf`, and each graph as SVG with a PNG
render (`amortised`, `pt_plane`), produced by that directory's `plot.mjs`.

**Headline (control and accounting, not an improvement).**
- **Equal memory, 40 bits.** Preprocessing rho's online cost is 142–229× below the known-log k = 4 table's, and its precomputation is 333–339× dearer. The two cross at about 9.5–14.3 k targets.
- **Break-even against rho.** The known-log table's online `S` is 0.117 after a target-blind 5.0 √r, so it beats rho per target from about 6 targets on.
- **The equal-precomputation arm missed equal precomputation** (3.85× over), and its ranking is unresolved.
- No single-target cost falls below rho's.

**No row in the results table.** This is a control study with one host and no `VALIDATION.json`.
