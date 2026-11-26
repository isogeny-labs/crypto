#!/bin/sh
# Validator replay of research/quartic_diem_nofb_20261126 with the frozen binary and the
# frozen commands, unchanged.
#
#   SNAP: `git archive cc7e5c5488a research/quartic_diem_nofb_20261126 | tar -x -C SNAP`
#   OUT : validator scratch directory (never the study's raw/)
#
# The frozen scripts write `--out raw/run.jsonl` and `--out raw/run131101.jsonl`
# relative to the working directory, and call the binary at the path they hard-code
# (B=.../scratchpad/bin/genus_ladder.nofb).  So this script checks the scripts against
# frozen.sha256 and the binary against binary.sha256, then runs both scripts verbatim
# with OUT as the working directory: the output lands in OUT/raw/, never in the study.
# Order and thread counts are the frozen ones: run_main.sh (2 threads), then
# run_131101.sh (1 thread), never concurrently (at most 2 threads).
set -e
SNAP=$1
OUT=$2
ST="$SNAP/research/quartic_diem_nofb_20261126"
( cd "$ST" && sha256sum -c frozen.sha256 && sha256sum -c binary.sha256 )
mkdir -p "$OUT/raw"
cd "$OUT"
test ! -e raw/run.jsonl && test ! -e raw/run131101.jsonl
date -u +%FT%TZ > raw/main_start.txt
sh "$ST/run_main.sh" > raw/main_stdout.txt 2>&1
date -u +%FT%TZ > raw/main_end.txt
sh "$ST/run_131101.sh" > raw/r131101_stdout.txt 2>&1
date -u +%FT%TZ > raw/r131101_end.txt
( cd "$ST" && sha256sum -c binary.sha256 )
sha256sum raw/run.jsonl raw/run131101.jsonl
echo REPLAY_DONE
