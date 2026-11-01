#!/usr/bin/env bash
set -euo pipefail
cd research/factor_base_heldout_20261025
{
  printf '%s\n' .gitignore Cargo.toml Cargo.lock PROTOCOL.md DATA_DICTIONARY.md REPORT.md REPORT.tex REPORT.pdf \
    factor-base-heldout.svg factor-base-heldout.png evaluation-by-size.svg evaluation-by-size.png \
    render_graph.py render_report.py build_report.sh run_train.sh run_evaluation.sh make_manifest.sh
  find src/bin results reports -type f ! -name '*.log' ! -name '.gitignore' ! -name '.artifacts.json'
} | sort -u | xargs sha256sum > SHA256SUMS
