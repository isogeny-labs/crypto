# The k = 5 sum table on Table B's own prime curves (2026-11-15)

Goal (owner): lower the counted cost of the index-calculus incumbent on a row of tables A-C by a verified, measured margin.
Result: `ksum:m=5` costs **0.69 / 0.67 / 0.60×** (20 bits), 0.60 / 0.57 / 0.50× (22 bits, secondary) and **0.50 / 0.48 / 0.43×** (24 bits)
of the strictest pair-table incumbent at w = 0 / 0.1 / 1 GAE per unpriced native operation, in the same session and on the same
curves, targets and seeds, with all 95% interval upper ends below 0.74 and replicate noise at most 6.2%. Rho stays ahead
(5.6-6.9× under the §5a band; the distance depends on the pricing reading). `[pcc:unverified]`, counted cost at L0.

- [`report.pdf`](report.pdf) (source `report.typ`, graph `l1-ratios.svg`), [`PROTOCOL.md`](PROTOCOL.md) (registered before any data), [`results.md`](results.md).
- Evidence: `sessions/` (9; `records.jsonl` in the artifact store), `replay-certificate.json` (second host), `validator/` (independent recompute, status pass), `band/` (§5a), `BINARY.sha256`, `data/` (pod run log and verify output).
- Leaderboard: section "Table B prime rows re-measured with the k = 5 sum table" in `docs/ic/LEADERBOARD.md`; benchmarks entry `benchmarks/20261115-tableb-ksum/`; record `benchmarks/records/20261115T0349Z-tableb-ksum-l1.json`.
- Reproduce: `node make_specs.mjs`; `pod/build.sh`, `pod/run.sh` on a pod with rustc 1.100.0; `replay-local.sh <ecbench>`; `node analysis/evaluate.mjs <ecbench> sessions <out>`; `node analysis/summary.mjs`; `node analysis/plot.mjs`.
