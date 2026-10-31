# Matched affine-query run report

Both PDF pages were visually inspected after rendering. The plot includes all
12 executions and identifies the formula and seed of each point. The report
uses the frozen analyzer outputs in ../../results-v1 and cites raw commit
c79bd38363f9de485087a0e206032cf5938dbdae, pushed before rendering.

Rebuild the presentation (no experiments):

```sh
rustc -O render.rs -o /tmp/ecbench-affine-report-render
/tmp/ecbench-affine-report-render ../../results-v1/rows.csv scaling.svg
python3 -c "import typst; typst.compile('report.typ', output='report.pdf')"
```

The Rust renderer only places recorded values on the SVG. Report tables are
transcribed from rows.csv and the frozen ratios.csv; no fitted analysis is added.
