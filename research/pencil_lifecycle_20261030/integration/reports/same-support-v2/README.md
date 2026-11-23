# Same-support mathematical component report v2

Frozen additive report of producer and independent machine checks plus separate source-fidelity review. No changes to the earlier report or original receipts.

- report.pdf / report.tex / classifier-outcome.tex: three-page inspected report and editable source.
- dependencies.svg / dependencies.pdf: editable dependency graph and embedded vector rendering.
- claims.json: exact named claims, premises, machine checks, source-review scope and next weighted-coordinate/source bridge.
- evidence-manifest.json / evidence.sha256: hashes of copied evidence. evidence/ retains source, producer checks, independent fresh-directory checks, prior failures and copy receipts. Large dependency mirrors and compiled objects remain in their original locations; receipts identify their hashes.

Both successful theorem readbacks report only propext, Classical.choice and Quot.sound. The independent replay rebuilt both helpers and both candidates in a fresh directory before a separate readback; its hashed toolchain/dependency mirror is reused, not a separate kernel rechecker. No formal ledger promotion is performed.

All three final PDF pages were inspected visually after correcting an SVG connector fill; the final LaTeX render has no overfull boxes. Renderer commands: rsvg-convert -f pdf -o dependencies.pdf dependencies.svg; pdflatex -interaction=nonstopmode -halt-on-error report.tex (twice); pdftoppm -r85 -png report.pdf page. Report generation ran no research-code builds, trials, cloud operations or measurements. Root owns later integration/delivery.
