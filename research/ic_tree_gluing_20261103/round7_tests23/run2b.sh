#!/bin/sh
# usage: run2b.sh P SEED K NS NB VER
cd "$(dirname "$0")"
( ulimit -v 524288; { echo "P=$1;SEED=$2;K=$3;NS=$4;NB=$5;VER=$6;"; cat t2b.gp; } | gp -q -f 2>&1 | grep -v Warning )
