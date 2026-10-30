#!/bin/bash
# SUPPLEMENTARY (added after the main run, see DEVIATIONS.md item 11): arm a1c = A1 with the fruitless-cycle test over the last 8 points.
set -u
cd ${D:-/tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/twist_ecdlp/E5-kummer-multiplicative-walk}
JOBS=${JOBS:-2}
[ -z "${REPLICATE:-}" ] && { sha256sum lib_a1c.gp run_cell_a1c.gp run_supp.sh > FROZEN_SUPP_SHA256.txt; date -u > FROZEN_SUPP_AT.txt; }
cell() { ARM=$1 B=$2 INST=$3 TRIALS=$4 timeout ${CELL_TIMEOUT:-3000} gp -q -f run_cell_a1c.gp < /dev/null > raw/cell_$1_b$2_i$3.txt 2> raw/cell_$1_b$2_i$3.err; echo "$1 b$2 i$3 rc=$?" >> out/SUPP_RC.txt; }
export -f cell
: > out/SUPP_RC.txt
for spec in "40 12" "36 24" "32 40" "28 60" "24 80" "20 80"; do set -- $spec
  printf "a1c $1 1 $2\na1c $1 2 $2\n"
done | xargs -P $JOBS -L 1 bash -c 'cell $0 $1 $2 $3'
echo supp-done >> out/SUPP_RC.txt
