# Data dictionary

| Path | Schema or columns | Meaning |
| --- | --- | --- |
| `results/raw/fb*-bits*.jsonl` | `factor-base-graph-replication-record/v1` | One candidate solve and paired rho solve for each public target; 35 files × 180 rows. Stored through `.artifacts.json`. |
| `results/raw/*.time` | `elapsed_s`, `exit` | Diagnostic process receipt for one arm/size file. |
| `results/targets.jsonl` | `factor-base-graph-replication-target/v1` | Representative-price projection of every arm/target record. Stored through `.artifacts.json`. |
| `results/price-grid.csv` | 13 named columns | Aggregate candidate and rho cost for 5 arms × 8 slices × 36 pricing cells. Stored through `.artifacts.json`. |
| `results/graph-data.csv` | 7 named columns | Native-analyzer aggregate values used by the left graph panel. |
| `results/original-vs-replication.csv` | 9 named columns | Native-analyzer original/replication comparison used by the right graph panel. |
| `results/summary.json` | `factor-base-graph-replication-analysis/v1` | Counts, representative prices, 36-cell bands and win/loss/tie totals by arm and slice. |
| `results/validator-report.json` | `factor-base-graph-replication-validator/v1` | Independent derivation, source-answer and aggregate recomputation result. |
| `results/manifest.json` | `factor-base-graph-replication-manifest/v1` | Revision, commands, environment and SHA-256/byte inventory. |
| `factor-base-replication.svg`, `.png` | SVG, PNG | Graph rendered only from the two native aggregate CSV files. |
| `REPORT.md`, `REPORT.tex`, `REPORT.pdf` | Markdown, LaTeX, PDF | Reviewer-facing report in source and rendered forms. |

`candidate_GAE` and `rho_GAE` are complete cohort sums in source-addition
equivalents. Ratios are `candidate_GAE / rho_GAE`. `processed_divisors` and
`rejected_large_large` are native counters, not priced wall time.
