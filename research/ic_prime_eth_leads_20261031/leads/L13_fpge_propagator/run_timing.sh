#!/bin/bash
cd "$(dirname "$0")"
B=$1; S=$2; SD=$3
nice -n 10 ./l13 inst --b $B --s $S --seed $SD --np 6 --nu 6 --cap 200000 --out raw/b${B}_s${S}_seed${SD}.jsonl --tmp tmp 2> raw/b${B}_s${S}_seed${SD}.err
