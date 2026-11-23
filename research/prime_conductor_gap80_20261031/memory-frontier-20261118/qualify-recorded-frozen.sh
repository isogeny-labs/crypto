#!/bin/sh
# Join native integer model output to already retained native pipeline metadata.
set -eu
repo=$1
audit=$2
out=$3
cd "$repo"
study=research/prime_conductor_gap80_20261031
records="$study/prepared-norm-20261118/results/attempt-b"
test ! -e "$out"
mkdir -p "$out"
for label in blocked17 full64 explicit18 small1 small-explicit empty-leftover pilot-five; do
 case $label in
 blocked17) set -- 10009 17;; full64) set -- 10009 64;; explicit18) set -- 10009 200 18;;
 small1) set -- 19 1;; small-explicit) set -- 19 4 1;; empty-leftover) set -- 5 2;; pilot-five) set -- 5 4;; esac
 degree=$1;block=$2;shift 2
 "$audit" "$degree" "$block" 137438953472 "$@" > "$out/$label.json" 2> "$out/$label.stderr"
 test ! -s "$out/$label.stderr"
done
python3 - "$out" "$records" <<'PY'
import json,pathlib,sys
out,records=map(pathlib.Path,sys.argv[1:]);rows=[]
for p in sorted(out.glob('*.json')):
 d=json.loads(p.read_text());old=json.loads((records/(p.stem+'-affine-prepared.json')).read_text());w=old['work_counts']
 assert d['ell']==old['ell']
 assert d['B']==str(old['baby_count']) and d['G']==str(old['giant_count'])
 assert d['n']==old['kernel_pairs_covered']
 assert d['baby_blocks']==w['baby_products'] and d['baby_blocks']==w['baby_scalar_starts']
 assert d['rectangles']==w['rectangles'] and d['rectangles']==w['giant_scalar_starts']
 if d['L']=='0':assert w['leftover_scalar_starts']=='0' and w['leftover_recurrence_additions']=='0'
 rows.append({'case':p.stem,'ell':d['ell'],'B':d['B'],'G':d['G'],'rectangles':d['rectangles'],'existing_native_metadata_equal':True})
assert len(rows)==7
print(json.dumps({'scope':'serialized metadata equality, native arithmetic only','recorded_schedule_controls':rows,'passed':True,'runtime_measurements':None}))
PY
