#!/bin/bash
# Q-a / Q-b1 exhaustive cells. Usage: ./run_cells.sh BIN [OUTDIR]
B=$1; O=${2:-raw}; cd "$(dirname "$0")"
for na in "11 0" "11 1" "13 0" "19 0"; do set -- $na
  timeout 3000 prlimit --as=2000000000 $B cells $1 $2 $O/cells_n$1_a$2.json 2> $O/cells_n$1_a$2.log; echo "n=$1 a=$2 exit=$?"; done
