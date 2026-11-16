# Prime-field index calculus with a k-sum table (2026-10-28)

Copies of the visual record of
[`research/prime_ksum_table_20261028/`](../../research/prime_ksum_table_20261028/README.md),
which is the source of truth: `report.pdf`, and each graph as SVG with a 150 dpi PNG
render (`s_vs_bits`, `cost_law`), produced by that directory's `plot.mjs`.

Headline (as the report states it): on the frontier campaign's six `a = −3` prime curves
and test targets, a table of every signed 4-sum of a smaller base lowers the prime-field
index-calculus `S` from 83.7 to 11.6 at 40 bits (7.2×; 9.4× at `w = 1`), below the pair
table at every size and band weight, fitted exponent 0.521 [0.506, 0.538] against 0.620.
Class: engineering. Still 13.1× rho and 10.4× BSGS at 40 bits; the method is generic and
its limit is BSGS.

**No row in the results table yet.** The validation gate
([`docs/VALIDATION_GATE.md`](../../docs/VALIDATION_GATE.md)) admits a row only with an
independent-host replay certificate and a validator report; this study has one host
(replays by `ecbench verify --replay-all`, all six sessions identical).
