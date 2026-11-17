#!/usr/bin/env bash
# Reproduce the large-prime feasibility study (L6 of #2216).  2 threads, no wall-time claims.
#   run.sh            from the repository root, after a release build:
#   CARGO_BUILD_JOBS=4 cargo build --release --example jv_cover --example jv_lp_model
set -euo pipefail
D=research/jv_cover_large_prime_20261018; RAW=$D/raw
BIN=target/release/examples/jv_cover; MODEL=target/release/examples/jv_lp_model
mkdir -p "$RAW/ref"
# 1. real decompositions (opt-in JV_LP_LOG=1; the dlp path itself is unchanged).  Slow: p=251 is ~10 min/seed.
$D/collect.sh "$BIN" "$RAW"
# 2. recorded inputs (frozen copies of experiments/30_*, and #2182's post-#1717 dlp/ccov records)
#    are in $RAW/ref; see README "Inputs".
# 3. measured vs modelled yields; Monte-Carlo solutions-needed curves; the cost model
$MODEL yields --out "$RAW/yields.json" $RAW/dlp_p101_s*.log $RAW/dlp_p251_s*.log
$MODEL curves --out "$RAW/curves.json" > "$RAW/curves.log" 2>&1
$MODEL cost --curves "$RAW/curves.json" --ccov-old "$RAW/ref/exp30_ccov_pre1717.json" \
  --dlp "$RAW/ref/pr2182_dlp_new.json" $RAW/ref/exp*dlp*.json $RAW/dlp_p*_s*.json --out "$RAW/cost.json" > "$RAW/cost.stdout"
# 4. graphs (draws recorded/derived numbers only)
/home/ec2-user/.venvs/report/bin/python $D/plot.py "$RAW" $D/figs
# 5. report
/home/ec2-user/.venvs/report/bin/python -c "import typst; typst.compile('$D/REPORT.typ', output='$D/REPORT.pdf')"
