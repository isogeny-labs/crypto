# perfindex validation (2026-10-24)

Evidence for the `perfindex` tool of PR aburan28/crypto#2478
(`examples/perfindex`, `docs/perf/PERFORMANCE_INDEX.md`).

| path | holds |
|:--|:--|
| `null-136x5/` | the null run: a binary against a byte-different copy of itself, 136 quick kernels, 5 rounds, one thread, pinned to one CPU. `plan.json`, `raw.jsonl` (2040 invocations, as the harness printed them), `compare.json`, `cost.json`, `SHA256SUMS`. Re-summarised from the original raw rows with `perfindex reanalyze` (see `plan.json` `reanalysis`) |
| `reports/run-20261024T0208Z/` | run report (`report.typ` source, `report.pdf`), the two figures as SVG and PNG, and `plot_null.py`, which only plots recorded values |

Check and replay:

```
cd null-136x5 && sha256sum -c SHA256SUMS
cargo run --release --example perfindex -- replay .
```

Copies of the report and figures are in `benchmarks/20261024-perfindex-null-validation/`
(not listed in `benchmarks/README.md`: the validation gate there needs a matched rho
reference, and this run measures the benchmarking tool, not an algorithm's speed).
