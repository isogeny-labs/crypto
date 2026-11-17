#!/bin/sh
# usage: run2a.sh P SEED EXH NCTL
cd "$(dirname "$0")"
( ulimit -v 524288; { echo "P=$1;SEED=$2;EXH=$3;NCTL=$4;"; cat t2a.gp; } | gp -q -f )
