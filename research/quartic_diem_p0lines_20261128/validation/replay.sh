#!/bin/sh
# Validator replay of research/quartic_diem_p0lines_20261128 with the frozen binary and
# the frozen commands, unchanged (adapted from quartic_diem_nofb_20261126/validation/replay.sh).
#
#   SNAP: `git archive c271f320e40 research/quartic_diem_p0lines_20261128 | tar -x -C SNAP`
#   OUT : validator scratch directory (never the study's raw/)
#
# The frozen scripts write `--out raw/run.jsonl` and `--out raw/run131101.jsonl`
# relative to the working directory and call the binary at the path they hard-code
# (B=.../scratchpad/bin/genus_ladder.p0).  binary.sha256 names the file without its
# directory and carries a second free-text line, so `sha256sum -c` cannot read it; this
# script checks the hard-coded path against the hash on its first line instead.
# Both scripts run verbatim (checked against frozen.sha256) with OUT as the working
# directory, run_main.sh (2 threads) then run_131101.sh (1 thread), never concurrently.
set -e
SNAP=$1
OUT=$2
ST="$SNAP/research/quartic_diem_p0lines_20261128"
B=/tmp/claude-0/-home-user/c9b2fe43-c085-5dfa-9cde-b0311430c5d1/scratchpad/bin/genus_ladder.p0
WANT=$(head -n1 "$ST/binary.sha256" | cut -d' ' -f1)
( cd "$ST" && sha256sum -c frozen.sha256 )
grep -q "^B=$B\$" "$ST/run_main.sh"
grep -q "^B=$B\$" "$ST/run_131101.sh"
test "$(sha256sum "$B" | cut -d' ' -f1)" = "$WANT"
mkdir -p "$OUT/raw"
cd "$OUT"
test ! -e raw/run.jsonl && test ! -e raw/run131101.jsonl
date -u +%FT%TZ > raw/main_start.txt
sh "$ST/run_main.sh" > raw/main_stdout.txt 2>&1
date -u +%FT%TZ > raw/main_end.txt
sh "$ST/run_131101.sh" > raw/r131101_stdout.txt 2>&1
date -u +%FT%TZ > raw/r131101_end.txt
test "$(sha256sum "$B" | cut -d' ' -f1)" = "$WANT"
sha256sum raw/run.jsonl raw/run131101.jsonl
echo REPLAY_DONE
