#!/bin/bash
set -eu
S=/tmp/claude-0/-home-user/829a5db7-e7b3-5b24-827a-595408da4b3a/scratchpad/validator
REPO=${1:-/home/user/crypto}
V=$REPO/research/weil_descent_angles_20261018/validation
cd $S
CELLS=$(python3 -I -c "import re;s=open('compare_replay.py').read();print(' '.join(re.findall(r'^    \(\"([A-Za-z0-9_]+)\", \"', s, re.M)))")
DUMPS=""; for c in $CELLS; do [ -f replay/$c/dump.jsonl ] && DUMPS="$DUMPS replay/$c/dump.jsonl"; done
./verify_dump $DUMPS > verify_all.txt || { echo "VERIFIER REPORTED FAILURES"; }
python3 -I compare_replay.py pinned replay selection.json compare_out.json > compare_out.txt
mkdir -p $V/replay
for c in $CELLS; do
  [ -d replay/$c ] || continue
  mkdir -p $V/replay/$c
  for f in routes.csv control.csv hsweep.csv memsweep.csv glue.csv; do [ -f replay/$c/$f ] && cp replay/$c/$f $V/replay/$c/$f; done
  gzip -n -9 -c replay/$c/dump.jsonl > $V/replay/$c/dump.jsonl.gz
done
mkdir -p $V/replay/H2__gluecheck && cp replay/H2__gluecheck/glue_checks.txt $V/replay/H2__gluecheck/glue_checks.txt
cp nonce.txt $V/replay/nonce.txt
cp selection.json $V/replay/selection.json
cp replay/replay_log.txt $V/replay/replay_log.txt
cp compare_out.json $V/replay/replay-comparison.json
cp verify_all.txt $V/replay/verify_dump.out.txt
cp recompute_out.json $V/replay/recompute.out.json
cp headlines_out.json $V/replay/headlines.out.json
cp compare_replay.py $V/compare_replay.py
cp assemble.py $V/assemble.py
cp finalize.sh $V/finalize.sh
python3 -I assemble.py $REPO $S
