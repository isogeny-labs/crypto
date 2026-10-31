# Prime-field k-sum table against the mitm incumbent, same session (2026-10-29)

These are copies of the visual record of
[`research/prime_ksum_vs_incumbent_20261029/`](../../research/prime_ksum_vs_incumbent_20261029/README.md),
which is the source of truth: `report.pdf`, and each graph as SVG with a PNG
render (`ratio`, `rho_multiple`), produced by that directory's `plot.mjs`.

**Headline, as the report states it.**
- **Instances.** The incumbent's (`prime_base_oracle_tuning_20261029`) four holdout curves, 48 targets and measurement seed. `inc` and `rho-neg` reproduce the incumbent's recorded means exactly.
- **Result.** The k = 5 sum table costs 0.70×, 0.45×, 0.33× and 0.22× the mitm incumbent at 20, 24, 28 and 32 bits (w = 0). It is below 1 at every band weight, and every prediction lies inside its interval.
- **Primary test (24–32 bits): holds.**
- **Track.** Generic decomposition (bounded by √r; cannot beat rho). This is a ranking of two generic recipes on the same instances.
- **Distance remaining.** Still 5.4–7.3× rho and 6.6–7.4× BSGS on these curves.

**No row in the results table yet.** The validation gate
([`docs/VALIDATION_GATE.md`](../../docs/VALIDATION_GATE.md)) admits a row only with an
independent-host replay certificate and a validator report. This study has one host (3,456 verified runs, all 2,304 replays identical).
