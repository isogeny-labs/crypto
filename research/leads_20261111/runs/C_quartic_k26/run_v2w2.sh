#!/usr/bin/env bash
# Packed-relation variant bin/v2w2 (src/, DRel 32 bytes, u32 adjacency offsets): identity checks
# against bin/v2w and the committed v2, then k = 25 under a hard 3 GiB RSS cap (kill above it).
set -u
D=$(cd "$(dirname "$0")" && pwd)
cd "$D"
P=raw/v2w2; mkdir -p $P/id_default $P/id_compact $P/k25
for q in 4051 16363 65479 262111; do
  t=ic_${q}_1_D
  [ -s $P/id_default/v2w_$t.res.json ] || python3 runjob.py $P/id_default/v2w_$t 1200 ./bin/v2w2 ic $q 1 D 3.0 2.0 4 4 >/dev/null
  cp -n raw/controls/v2_orig_$t.jsonl $P/id_default/
done
python3 compare.py $P/id_default > $P/id_default/COMPARE.txt
export V2W_COMPACT=1
for u in "8388571 1" "16777183 2"; do
  set -- $u; t=ic_$1_$2_D
  timeout 3000 bash -c "until [ -s raw/wide/c_$t.res.json ]; do sleep 10; done"
  [ -s $P/id_compact/v2w_$t.res.json ] || python3 runjob.py $P/id_compact/v2w_$t 1200 ./bin/v2w2 ic $1 $2 D 3.0 2.0 4 4 >/dev/null
  cp -n raw/wide/c_$t.jsonl $P/id_compact/v2_orig_$t.jsonl
done
python3 compare.py $P/id_compact > $P/id_compact/COMPARE.txt
export RSS_CAP_KIB=3145728
for s in 1 2 3; do
  t=ic_33554383_${s}_D
  [ -s $P/k25/c_$t.res.json ] || python3 runjob.py $P/k25/c_$t 1200 ./bin/v2w2 ic 33554383 $s D 3.0 2.0 4 4 >/dev/null
  if grep -q '"killed_rss_cap": true\|"killed_timeout": true' $P/k25/c_$t.res.json; then break; fi
done
for c in 1 2 3; do
  [ -s $P/k25/calib_33554383_$c.res.json ] || python3 runjob.py $P/k25/calib_33554383_$c 1200 ./bin/v2w2 calib 33554383 $c >/dev/null
done
echo "v2w2 done $(date -u +%FT%TZ)" > raw/V2W2_DONE
