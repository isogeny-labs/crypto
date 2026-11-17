# Partial-run report

Raw evidence and the separately labelled post-interruption analysis were pushed
as `2ef5d57e0e01` before this report was rendered. Both PDF pages were inspected.
The report retains all33 completed recoveries, one interrupted attempt and two
unstarted cells. It does not turn the incomplete panel into a completed run.

Render from the repository root using Typst:

```python
import typst
typst.compile(
    'research/explicit_relations_fullcache_20261101/reports/run-20261101T0412Z/report.typ',
    output='research/explicit_relations_fullcache_20261101/reports/run-20261101T0412Z/report.pdf',
    root='.',
)
```

`presentation-custody.json` binds the benchmark PDF/SVG copies to their sources.
Timing formulas and their post-interruption status are documented in
`../../retained-analysis-v2/analysis-custody.json`; original frozen artifacts
remain unchanged.
