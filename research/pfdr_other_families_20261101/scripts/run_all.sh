#!/usr/bin/env bash
# PROTOCOL.md "Execution". Usage: scripts/run_all.sh   (writes under results/)
set -euo pipefail
cd "$(dirname "$0")/.."
I=implementation; R=results; mkdir -p $R
python3 -I $I/run.py build --out $R/build
python3 -I $I/run.py selftest --out $R/selftest --build $R/build
python3 -I $I/run.py pilot --build $R/build --out $R/pilot
python3 -I $I/run.py census --build $R/build --design $R/pilot/design.json --block main --out $R/main
ST=$(find $R/selftest -name '*.json' | head -1)
python3 -I $I/run.py analyze --design $R/pilot/design.json --main $R/main/main-jobs.jsonl --selftest "$ST" --out $R/analysis-main.json
if python3 -c "import json,sys; sys.exit(0 if json.load(open('$R/flags.json'))['alive_cells'] else 1)" 2>/dev/null; then
  python3 -I $I/run.py census --build $R/build --design $R/pilot/design.json --block rep --flags $R/flags.json --out $R/rep
  python3 -I $I/run.py analyze --design $R/pilot/design.json --main $R/main/main-jobs.jsonl --rep $R/rep/rep-jobs.jsonl --selftest "$ST" --out $R/analysis-final.json
fi
