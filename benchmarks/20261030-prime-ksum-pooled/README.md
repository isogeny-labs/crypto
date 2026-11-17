# Prime-field k-sum tables against a tuned mitm incumbent, 16 fresh curves (2026-10-30)

These are copies of the visual record of
[`research/prime_ksum_pooled_20261029/`](../../research/prime_ksum_pooled_20261029/README.md),
which is the source of truth: `report.pdf`, and each graph as SVG with a PNG
render (`ratio`, `rho_multiple`, `per_curve`), produced by that directory's `plot.mjs`.

**Headline, as the report states it.**
- **Panels.** 4 fresh prime-order curves per size at 20, 24, 28 and 32 bits (24 targets × 2 rounds each), all verified and replay-all audited. The incumbent was re-tuned on separate train curves by the same rule as the k-sum tables.
- **Result.** The k = 5 sum table costs 0.70×, 0.45×, 0.32× and 0.22× the tuned incumbent at 20 to 32 bits (w = 0). It is below the incumbent on all 16 curves and at every band weight. The registered primary test (24 to 32 bits, also against the w = 0-tuned incumbent) holds.
- **Exponent (toy tier).** 0.496 for k = 5 against 0.635 for the tuned incumbent.
- **Track.** Generic decomposition (bounded by √r; cannot beat rho). A ranking of generic recipes on the same instances.
- **Distance remaining.** 5.8–7.3× rho and 6.5–7.6× BSGS at w = 0.

**External comparison: not run. No external ECDLP implementation was run; the only references are this repository's matched rho and BSGS in the same session, so the report makes no claim to be the fastest known. Outstanding: the same targets through an external prime-field ECDLP tool (for example PARI/GP `elllog`) on the same host under the same conditions, and published rho timings at these sizes labelled "published, not reproduced". The owner's choice of reference tool is needed.**

**No row in the results table yet.** The validation gate
([`docs/VALIDATION_GATE.md`](../../docs/VALIDATION_GATE.md)) admits a row only with an
independent-host replay certificate and a validator report. This study has one host (13,824 verified runs, every replay identical).
