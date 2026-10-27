# Token efficiency metrics for the agent workflows (2026-10-20)

Tooling study: `tools/token_usage.py` (metrics and accounting rules in
[`docs/token-efficiency.md`](../../docs/token-efficiency.md)). The run
report below is the instrument applied to the session that built it. It is
descriptive and makes no comparison.

| run | what | report |
| --- | --- | --- |
| `run-20261020T2313Z` | one Claude Code session: Opus 5.5 main + one Haiku 5.5 subagent probe | [PDF](reports/run-20261020T2313Z/report.pdf), [record](reports/run-20261020T2313Z/record.json), [chart](reports/run-20261020T2313Z/cost.svg) |

Rebuild the PDF: `python -c "import typst; typst.compile('report.typ', output='report.pdf')"`
in the run directory (Typst from `pip install typst`).
