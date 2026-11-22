#!/usr/bin/env bash
# report.pdf from README.md + figure (pandoc + pdflatex; unicode mapped to ASCII/TeX).
set -eu
cd "$(dirname "$0")"
T=$(mktemp -d)
sed -e 's/≤/<=/g; s/≥/>=/g; s/²/^2/g; s/₃/3/g; s/×/x/g; s/∏/prod/g; s/∈/in/g; s/⊂/subset/g; s/μ/mu/g; s/−/-/g; s/…/.../g; s/→/->/g; s/–/-/g; s/—/--/g; s/±/+-/g; s/≈/~/g; s/·/*/g; s/≠/!=/g' README.md > $T/r.md
printf '\n\n## Figure\n\n![](fig_enc_control.pdf)\n' >> $T/r.md
pandoc $T/r.md --resource-path=. --pdf-engine=pdflatex -V geometry:margin=1.6cm -V fontsize=10pt -o report.pdf
rm -rf $T
