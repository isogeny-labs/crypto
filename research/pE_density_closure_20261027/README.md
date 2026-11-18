# p_E density closure note (lead 2)

2026-10-27. Closure note for the twist study's observation that per-target
m = 3 solution counts varied up to 3x across curves at fixed V while
mean / p_E^3 stayed constant. Analysis of recorded values only; no run.

- Report: [`REPORT.pdf`](REPORT.pdf) (source `REPORT.typ`).
- Tool: `src/main.rs` (std + serde_json). Reproduce:
  `cargo run --release -- ../twist_obstruction_20261026/reports/run-20261026T1456Z/rows.jsonl .`
  writes `closure.json`, `groups.tsv`; `python plot.py` lays out the figures.
- Result: yield per target = C(2 v_E + m, m) / #E (exact first moment);
  measured slope on p_E at m = 3 is 2.96 +- 0.06; at fixed |F| the effect is
  a factor-base-size relabelling worth at most one bit of dim V; at
  n = 131, l = 44 the heuristic gain from choosing b / class member is
  below 1e-4. Atlas disposition (evidence added, closure pending breakthrough review and the |F| check; not a closure) is in REPORT section "Atlas disposition".
- Input binding: sha256 of rows.jsonl is d8cab334273b5ca70c9b5fe3082c274f6cd871f0d2e1a42967e015a7f4c41189 (tracked file, HEAD fbccdb89e); this directory is untracked until committed. The reproduce command matches the checkpoint hook launcher regex, so a third party needs a checkpoint or CHECKPOINT_SKIP. Scope: the heuristic n = 131 conclusion is binary subspace bases only.
