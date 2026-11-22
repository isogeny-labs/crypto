#!/bin/sh
# usage: run_cell.sh P CURVE RHO_TARGETS   (reads BIN and GIT from the environment)
cd "$(dirname "$0")"
"$BIN" dlp "$1" "$2" 20261121 "$3" "results/dlp_$1_$2.json" "$GIT" > "logs/dlp_$1_$2.log" 2>&1
