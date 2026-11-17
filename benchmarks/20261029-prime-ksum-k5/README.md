# Prime-field index calculus with a 5-sum table (2026-10-29)

These are copies of the visual record of
[`research/prime_ksum_k5_20261028/`](../../research/prime_ksum_k5_20261028/README.md),
which is the source of truth: `report.pdf`, and each graph as SVG with a PNG
render (`s_vs_bits`, `ratio`), produced by that directory's `plot.mjs`.

**Headline, as the report states it.** The panel is the frontier campaign's six `a = −3` prime curves plus new 44- and 48-bit rungs, with 16 targets × 2 rounds and 1,920 verified runs, all replay-all audited.
- A table of every signed 5-sum of a smaller base lowers the prime-field index-calculus `S` below the k = 4 table at every size from 24 to 48 bits and every band weight.
- At 40 bits: 8.51 against 11.56, which is 0.74× (0.71× at `w = 1`).
- At 48 bits: 0.67×.
- Medium-tier exponent: 0.547 [0.543, 0.554] against 0.565 [0.560, 0.571].
- Track: generic decomposition (bounded by √r; cannot beat rho); class engineering within that track. Not ranked against the Table B incumbent (different curves).
- Still 9.7× rho and 7.6× BSGS at 40 bits, and 11.0× rho at 48 bits. The method is generic and its limit is BSGS.

**No row in the results table yet.** The validation gate
([`docs/VALIDATION_GATE.md`](../../docs/VALIDATION_GATE.md)) admits a row only with an
independent-host replay certificate and a validator report. This study has one host, with replays by `ecbench verify --replay-all`: all eight sessions identical.
