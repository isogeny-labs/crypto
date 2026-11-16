#!/bin/bash
# DEEPDIVE.md -> deepdive.typ -> deepdive.pdf (pandoc + typst wheel in /tmp/report-venv; research-visuals "Rendering")
cd "$(dirname "$0")" || exit 1
printf '%s\n' '#set page(paper: "a4", margin: 1.6cm, footer: context align(center, text(size: 7pt, fill: gray)[EXPLORATORY, POST HOC deep dive: no verdict is changed, no speedup is claimed. Page #counter(page).display()]))' '#set text(size: 9pt)' '#set table(stroke: 0.4pt + gray, inset: 3pt)' '#show table: set text(size: 7pt)' '#show image: set image(width: 100%)' > deepdive.typ
pandoc DEEPDIVE.md --wrap=none -f markdown+pipe_tables+raw_attribute -t typst >> deepdive.typ
/tmp/report-venv/bin/python -c "import typst; typst.compile('deepdive.typ', output='deepdive.pdf')"
