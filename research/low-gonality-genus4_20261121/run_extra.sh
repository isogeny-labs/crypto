#!/bin/sh
cd "$(dirname "$0")"
for j in "251 0" "251 1" "251 2" "509 0" "509 1" "509 2"; do set -- $j; ./run_cell.sh $1 $2 3; done
