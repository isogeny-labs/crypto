#!/bin/bash
# run.sh <n> <K> <norm> <ntargets> <sym 0|1> [rank_seed]
set -u
W=$(cd "$(dirname "$0")" && pwd); B=$W/../bin/kodf
n=$1 K=$2 N=$3 T=$4 SYM=$5 SEED=${6:-1}
tag=n${n}_K${K}_N${N}_T${T}_s${SYM}_r${SEED}
head -n $T $W/targets/n${n}_32.txt > $W/runs/$tag.targets
/usr/bin/time -p env KIC_NORM=$N KIC_PAIR_SYMMETRY=$SYM $B construct:$n:0:$K $W/runs/$tag.targets $SEED $W/runs/$tag.jsonl > $W/runs/$tag.stdout 2> $W/runs/$tag.stderr
echo "$tag exit $?"
