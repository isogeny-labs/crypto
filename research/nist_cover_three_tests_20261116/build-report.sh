#!/bin/sh
set -eu
cd "$(dirname "$0")"
python3 render_figures.py
sed -e 's@figures/kernels.svg@figures/kernels.pdf@g' -e 's@figures/route.svg@figures/route.pdf@g' -e 's@figures/recovery.svg@figures/recovery.pdf@g' README.md |
  pandoc --from markdown+tex_math_dollars --pdf-engine=pdflatex -V geometry:margin=0.7in -V fontsize=10pt -V colorlinks=true --resource-path=. -o REPORT.pdf
sed 's@figures/geometry.svg@figures/geometry.pdf@g' GEOMETRY.md |
  pandoc - GEOMETRY_MODEL.md --from markdown+tex_math_dollars --pdf-engine=pdflatex -V geometry:margin=0.7in -V fontsize=10pt -V colorlinks=true --resource-path=. -o GEOMETRY.pdf
dot -Tsvg figures/kernel-pairs.dot -o figures/kernel-pairs.svg
dot -Tpdf figures/kernel-pairs.dot -o figures/kernel-pairs.pdf
sed 's@figures/kernel-pairs.svg@figures/kernel-pairs.pdf@g' SYMMETRY.md |
  pandoc --from markdown+tex_math_dollars --pdf-engine=pdflatex -V geometry:margin=0.7in -V fontsize=10pt -V colorlinks=true --resource-path=. -o SYMMETRY.pdf
dot -Tsvg figures/kummer.dot -o figures/kummer.svg
dot -Tpdf figures/kummer.dot -o figures/kummer.pdf
sed 's@figures/kummer.svg@figures/kummer.pdf@g' KUMMER.md |
  pandoc --from markdown+tex_math_dollars --pdf-engine=pdflatex -V geometry:margin=0.7in -V fontsize=10pt -V colorlinks=true --resource-path=. -o KUMMER.pdf
dot -Tsvg figures/backmap.dot -o figures/backmap.svg
dot -Tpdf figures/backmap.dot -o figures/backmap.pdf
sed 's@figures/backmap.svg@figures/backmap.pdf@g' BACKMAP.md |
  pandoc --from markdown+tex_math_dollars --pdf-engine=pdflatex -V geometry:margin=0.7in -V fontsize=10pt -V colorlinks=true --resource-path=. -o BACKMAP.pdf
pandoc FORMAL_RECONSTRUCTION.md --from markdown+tex_math_dollars --pdf-engine=pdflatex -V geometry:margin=0.7in -V fontsize=10pt -V colorlinks=true --resource-path=. -o FORMAL_RECONSTRUCTION.pdf
dot -Tsvg figures/polarized-quotient.dot -o figures/polarized-quotient.svg
dot -Tpdf figures/polarized-quotient.dot -o figures/polarized-quotient.pdf
sed 's@figures/polarized-quotient.svg@figures/polarized-quotient.pdf@g' POLARIZED_QUOTIENT.md |
  pandoc --from markdown+tex_math_dollars --pdf-engine=pdflatex -V geometry:margin=0.7in -V fontsize=10pt -V colorlinks=true --resource-path=. -o POLARIZED_QUOTIENT.pdf
