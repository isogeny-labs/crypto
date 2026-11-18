# SAT CLI acceptance descriptive report

Derived from the retained run `/var/tmp/sat-cli-custody-acceptance-20261103-v1`, protocol version 2. The protocol ID is the available experiment identifier; no separate EXP/HYP/GOAL ledger ID is supplied in these receipts. This component supports the active ECDLP index-calculus training dataset objective.

`report.typ` is the report source; `report.pdf` is its inspected two-page rendering. `patterns.svg` plots recorded pattern counts for two cells. `data.json` retains source hashes, actual outcomes and exact command receipts. `render.py` reproduces the extraction, graph and PDF using the retained evidence root, matplotlib, Typst and PyMuPDF. This script performs no native execution.

Both PDF pages were visually inspected at rendered resolution: graph labels, caption/run ID, table values, provenance hashes and narrative are legible without clipping. `pdf-text.txt` is the extracted PDF text; `page-1.png` and `page-2.png` are the inspected previews.

All new artifacts live outside the sealed run. Publish the immutable run records first, then this report on the same branch. No S3 reports-prefix upload or repository mutation was performed by the report author.
