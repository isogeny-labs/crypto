#!/bin/sh
# usage: run3.sh "6,7" SEED NCTL NT
cd "$(dirname "$0")"
( ulimit -v 524288; { echo "TT=[$1];SEED=$2;NCTL=$3;NT=$4;"; cat t3.gp; } | gp -q -f 2>&1 | grep -v Warning )
