#!/usr/bin/env bash
# Control (lead step 2). The committed v2 (built unmodified from research/quartic_n4v2_20261020/v2.rs
# as bin/v2_orig) and the wide fork bin/v2w run the same units; every non-timing field must be
# identical. v2w runs twice: default and V2W_COMPACT=1 (memory mode). Sequential, single-threaded.
# Then the builder control: v2w regenerates the committed instance list (k = 8..22, 63 instances).
set -u
D=$(cd "$(dirname "$0")" && pwd)
cd "$D"
O=raw/controls      # v2_orig outputs (reused from the preliminary pass; same binary, same sha256)
F=raw/controls_final; C=raw/controls_compact
mkdir -p $O $F $C
units() {
  for q in 4051 8191 16363 32719 65479 131071 262111 524287 1048507 2097091; do echo "$q 1 D"; done
  for s in 1 2 3; do echo "4194271 $s D"; done
  for q in 4051 8191 16363 32719 65479; do echo "$q 1 S"; done
}
units | while read q s a; do
  t=ic_${q}_${s}_${a}
  [ -s $O/v2_orig_$t.res.json ] || python3 runjob.py $O/v2_orig_$t 1200 ./bin/v2_orig ic $q $s $a 3.0 2.0 4 4 >/dev/null
  [ -s $F/v2w_$t.res.json ] || python3 runjob.py $F/v2w_$t 1200 ./bin/v2w ic $q $s $a 3.0 2.0 4 4 >/dev/null
  [ -s $C/v2w_$t.res.json ] || V2W_COMPACT=1 python3 runjob.py $C/v2w_$t 1200 ./bin/v2w ic $q $s $a 3.0 2.0 4 4 >/dev/null
  cp -n $O/v2_orig_$t.jsonl $F/; cp -n $O/v2_orig_$t.jsonl $C/
done
python3 compare.py $F > $F/COMPARE.txt
python3 compare.py $C > $C/COMPARE.txt
# builder control
[ -s raw/instances_v2w.res.json ] || python3 runjob.py raw/instances_v2w 1800 ./bin/v2w instances 223:5 499:5 991:5 2011:5 4051:5 8191:5 16363:5 32719:5 65479:5 131071:3 262111:3 524287:3 1048507:3 2097091:3 4194271:3 >/dev/null
echo "controls done $(date -u +%FT%TZ)" > raw/CONTROLS_DONE
