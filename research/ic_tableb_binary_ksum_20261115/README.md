# The k-sum table on Table B's random-binary curves (2026-11-15) `[pcc:unverified]`

Binary twin of `research/ic_tableb_ksum_20261115`. Counted cost at L0 (generic decomposition track; rho and BSGS stay ahead).
Result: `ksum:m=5` costs 0.68 / 0.66 / 0.59x (n21), 0.64 / 0.63 / 0.61x (n24) and 0.46 / 0.44 / 0.39x (n27) of the strictest tuned pair-table
incumbent at w = 0 / 0.1 / 1 GAE per unpriced native operation, in the same session; L1 holds on all three primary rows (pooled and per
replicate, above the replicate noise). On the secondary rows n15 and n18 `ksum5` is not below the incumbent (1.14 / 1.04 at w = 0);
`ksum4` is (0.95 / 0.90). Rho stays ahead by 5.3-7.5x. Table B's own recipe base (`inc-tb`) is 1.4-3.4x worse than the tuned incumbent.

- `PROTOCOL.md` (registered in commit 77bc214198 before any study data; hashes in `REGISTRATION.sha256`), `report.pdf` (source `report.typ`, graph `l1-ratios.svg`/`.png`), `results.md`, `results.json`, `summary.json`.
- Evidence: `sessions/B<bits>-r<rep>/` (15 sessions; `records.jsonl.gz`, gunzip before `ecbench` reads them), `data/` (run log, `ecbench verify --replay-all` outputs, all rc 0), `band/` (§5a, `--ranks`), `pilot/` (disclosed sizing pilots), `BINARY.sha256`.
- Reproduce: `node make_specs.mjs`; `run_all.sh` (repo root; `ecbench` built from commit 2476332d22); `node analysis/evaluate.mjs <ecbench> sessions <outdir>`; `node analysis/summary.mjs`; `node analysis/plot.mjs`.
- Not met: validation gate (no second-host replay, no independent validator, no `VALIDATION.json`); no leaderboard or `benchmarks/README.md` row.
