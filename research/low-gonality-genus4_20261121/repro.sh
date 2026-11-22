#!/bin/sh
# Rebuild and reproduce the smallest cell (p = 251, curve 0, no rho) and, in full, the ladder. Run from the repository root.
set -e
export CARGO_TARGET_DIR=/home/user/crypto/target
cargo build --release -j2 --bin genus4_pencil_ic_lp
cargo test --release -j2 --bin genus4_pencil_ic_lp
GIT=$(git rev-parse HEAD)
BIN=target/release/genus4_pencil_ic_lp
D=research/low-gonality-genus4_20261121
# smallest cell (about 1.5 min, IC arms only):
$BIN dlp 251 0 20261121 0 /tmp/g4lp_smoke_251_0.json "$GIT"
# full study (hours; the rho runs at p = 2039 take about 35 min each):
#   BIN=$BIN GIT=$GIT xargs -P3 -L1 $D/run_cell.sh < $D/jobs.txt ; BIN=$BIN GIT=$GIT $D/run_yield.sh
python3 $D/analyze.py && python3 $D/make_report.py && python3 $D/plot.py && python3 $D/md2typ.py
cd $D && /tmp/report-venv/bin/python -c "import typst; typst.compile('REPORT.typ', output='REPORT.pdf')"
