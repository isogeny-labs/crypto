#!/bin/sh
# Builds report.pdf from README.md and the two figures. Thin orchestration only:
#   sh build.sh
# Needs pandoc (>= 3.1, typst writer) and typst; figures need python3 + matplotlib.
set -e
here=$(cd "$(dirname "$0")" && pwd)
cd "$here/figures" && python3 -I plot.py && cd "$here"
pandoc README.md -f gfm -t typst -o .body.typ
{
  cat <<'TYP'
#set page(paper: "a4", margin: 1.6cm)
#set text(size: 8.6pt)
#set par(justify: false)
#show table: set text(size: 6.6pt)
#set table(stroke: 0.4pt + gray, inset: 3pt)
#show raw.where(block: false): r => text(font: "DejaVu Sans Mono", size: 7.2pt, r.text.replace("_", "_\u{200b}").replace("/", "/\u{200b}"))
#show heading.where(level: 1): set text(size: 13pt)
#show heading.where(level: 2): set text(size: 11pt)
#show heading.where(level: 3): set text(size: 9.5pt)
TYP
  cat .body.typ
  cat <<'TYP'

= Figures

#figure(image("figures/degree_vs_ell.svg", width: 100%), caption: [Refutation degree against factor-base dimension. Left: the repository's three instruments on the chained S3 system (plain XL, MutantXL closure, msolve F4), n <= 15, four draws per cell; hollow labels mark lower bounds. Right: published Magma F4 maximum step degrees, the same quantity as the closure, all measured at overdetermination Delta = n - m ell of 13 to 76. Sources in figures/plot.py.])

#figure(image("figures/arity_curve.svg", width: 85%), caption: [Cost against arity at n = 131 (ECC2K-130). Free-oracle floor and per-target budget from RESEARCH_ECC2K130_DECOMPOSITION.md section 5.3; the what-if rows hold a degree-4 closure, w = 2.2 and an 8-bit column-touch to rho-step conversion, and are derived, not measured (section 1.3). The shaded region has no algebraic degree measurement in any repository.])
TYP
} > report.typ
rm -f .body.typ
typst compile report.typ report.pdf
echo "built report.pdf"
