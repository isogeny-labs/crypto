#!/bin/bash
# Copy the run's PDF, SVG graphs and 150-dpi PNG renders into benchmarks/<date>-<slug>/
# (AGENTS.md "Performance results also land in benchmarks/"). The study directory stays the source of truth.
set -euo pipefail
cd "$(dirname "$0")/../.."
RUN=research/isogeny-class-rho/runs/run-20261018T0857Z
DEST=benchmarks/20261018-isogeny-class-rho
mkdir -p $DEST
cp research/isogeny-class-rho/report.pdf $DEST/report.pdf
for c in P1 P0 B1 K; do
  [ -d $RUN/$c ] || continue
  mkdir -p $DEST/$c
  for f in $RUN/$c/*.svg $RUN/$c/*.png; do [ -f "$f" ] && cp "$f" $DEST/$c/; done
done
cp research/isogeny-class-rho/PROTOCOL.md $DEST/PROTOCOL.md
du -sh $DEST
