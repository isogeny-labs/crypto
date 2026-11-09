# Native five-curve construction and counted-cost report

This report covers native-v17/v19 software qualification, native-v18/v19 certificate mutations, the retained construction/refusal attempts, and the prepublished structural-v2 counted scan. It does not report admitted cryptographic timing. The fixed five-curve inputs and source preceded the scan; source/input/binary checks and all 90 rows remain in qualification/structural-v2/.

`plot-data.json` is an unchanged copy of structural-v2/group-count-minima.json. `chart.typ` renders its quantitative figure as `counts.svg`; `report.typ` draws the same figure and tables in the two-page PDF. The direct/fused minima use equal weight for doubles and additions for display only. Negations, map formula costs, excluded backend/setup costs and full timing obligations remain explicit.

Render with Typst 0.13.1: `typst compile report.typ report.pdf`; `typst compile --format svg chart.typ counts.svg`; inspect pages using `typst compile --format png --ppi 110 report.typ 'page-{p}.png'`. The first standalone chart exceeded its one-page canvas; chart-v1.typ and its failure log are retained. The expanded chart canvas renders one SVG. The original report render’s unavailable-font warning remains in render-v1.log; the corrected explicit Libertinus Serif render is warning-free. Inspection receipt and PDF/figure/source hashes are retained; preview PNGs were inspected in /tmp and are reproducible rather than committed.

Full derivation and exact commands are in ../../CONSTRUCTION.md. The measurement gate remains open in ../../../../problems/cm-marked-scalar-measurement-admission.md. No study status is promoted by generating this report.
