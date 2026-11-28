#!/bin/sh
# Local native development validation, never a timing or parameter-search driver.
set -eu
repo=$1
candidate=$2
baseline=$3
checker=$4
out=$5
cd "$repo"
study=research/prime_conductor_gap80_20261031/prepared-norm-20261118
inputs=research/prime_conductor_gap80_20261031/evaluator-20261118/inputs
test ! -e "$out"
mkdir -p "$out"
# Reuse passed controls from the identical frozen candidate, not a rerun.
cp "$study/results/attempt-a/primitive.json" "$out/primitive.json"
cp "$study/results/attempt-a/default-fixtures.jsonl" "$out/default-fixtures.jsonl"
cmp "$out/default-fixtures.jsonl" research/prime_conductor_gap80_20261031/window-audit-20261118/results/default-fixtures.jsonl
run_mode() {
 label=$1; input=$2; window=$3; block=$4; shift 4
 # Optional token is solely a fixed internal flag, never user-provided text.
 reference=--pari-reference
 case "$label" in blocked17-*|full64-*|explicit18-*) reference=;; esac
 "$baseline" --evaluate "$window" "$block" $reference "$@" < "$input" > "$out/$label-baseline.json" 2> "$out/$label-baseline.stderr"
 "$candidate" --evaluate "$window" "$block" $reference "$@" < "$input" > "$out/$label-default.json" 2> "$out/$label-default.stderr"
 cmp "$out/$label-baseline.json" "$out/$label-default.json"
 "$candidate" --evaluate "$window" "$block" $reference "$@" --prepared-norm < "$input" > "$out/$label-prepared.json" 2> "$out/$label-prepared.stderr"
 test ! -s "$out/$label-baseline.stderr"
 test ! -s "$out/$label-default.stderr"
 test ! -s "$out/$label-prepared.stderr"
}
run_case() {
 label=$1; input=$2; window=$3; block=$4; shift 4
 run_mode "$label-affine" "$input" "$window" "$block" "$@"
 run_mode "$label-batch7" "$input" "$window" "$block" "$@" --projective-batch 7
 run_mode "$label-batch64" "$input" "$window" "$block" "$@" --projective-batch 64
}
run_case blocked17 "$inputs/control10009.txt" 64 17
run_case full64 "$inputs/control10009.txt" 64 64
run_case explicit18 "$inputs/control10009.txt" 200 200 --baby-window 18
run_case small1 "$inputs/control19.txt" 4 1
run_case small-explicit "$inputs/control19.txt" 4 4 --baby-window 1
run_case empty-leftover "$inputs/control5.txt" 2 2
run_case pilot-five "$study/control-pilot-five.txt" 8 4
sh research/prime_conductor_gap80_20261031/evaluator-20261118/check-rejections.sh "$candidate" "$out/default-rejections"
PREPARED_BINARY="$candidate" sh research/prime_conductor_gap80_20261031/evaluator-20261118/check-rejections.sh "$repo/$study/prepared-wrapper.sh" "$out/prepared-rejections"
for invalid in duplicate unknown; do
 case $invalid in duplicate) set -- --prepared-norm --prepared-norm;; unknown) set -- --prepared-norm=yes;; esac
 if "$candidate" --evaluate 4 1 "$@" < "$inputs/control19.txt" > "$out/$invalid.stdout" 2> "$out/$invalid.stderr"; then code=0; else code=$?; fi
 printf '%s\n' "$code" > "$out/$invalid.exit"
 test "$code" -eq 2
 test ! -s "$out/$invalid.stdout"
 rg -F 'unknown/duplicate evaluation option' "$out/$invalid.stderr" > /dev/null
done
python3 - "$out" <<'PY'
# Serialized receipt comparisons only; mathematical checks ran in native C++/PARI.
import json,pathlib,sys
out=pathlib.Path(sys.argv[1]);primitive=json.loads((out/'primitive.json').read_text())
assert primitive['fields']==3 and primitive['includes_202_and_96_bit_fields']
assert primitive['positive_prepared_evaluations']>0 and primitive['collision_controls']>0 and primitive['invalid_modulus_controls']==6
rows=[]
for path in sorted(out.glob('*-prepared.json')):
 d=json.loads(path.read_text());old=json.loads(path.with_name(path.name.replace('-prepared.json','-baseline.json')).read_text())
 backend=d.pop('norm_backend');assert d==old
 assert backend['name']=='prepared_flint_v1' and backend['text_conversions']==0
 assert backend['modulus_preparations']==d['work_counts']['baby_products']
 assert backend['evaluations']==d['work_counts']['lognorm_calls']
 if d['ell']=='10009':
  oracle=json.loads((out.parent/'attempt-a/blocked17-affine-baseline.json').read_text())
  assert oracle['pari_direct_reference']
  for key in ('p','source','ell','kernel','codomain','kernel_pairs_covered','images'):
   assert d[key]==oracle[key]
  assert not d['pari_direct_reference']
 else:
  assert d['pari_direct_reference']
 assert d['conductor_gap'] is None and d['runtime_measurements'] is None
 rows.append({'case':path.stem,'p':d['p'],'image_records':len(d['images']),'preparations':backend['modulus_preparations'],'evaluations':backend['evaluations']})
assert len(rows)==21 and len({r['p'] for r in rows})==4
rejects=sum(len((out/name/'rejections.jsonl').read_text().splitlines()) for name in ('default-rejections','prepared-rejections'))+2
assert rejects==18
print(json.dumps({'scope':'native development qualification only','primitive':primitive,'prepared_pipeline_replays':len(rows),'point_input_evaluations':sum(r['image_records'] for r in rows),'pipeline_fields':4,'default_bytes_unchanged':True,'reference_policy':'degree-10009 matches completed attempt-a PARI oracle; other fixed cases use fresh PARI','legacy_prepared_pari_equal':True,'negative_cli_controls':rejects,'rows':rows,'benchmark_session_executed':False,'full_80bit_crossing_complete':False,'runtime_measurements':None}))
PY
