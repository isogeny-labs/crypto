#!/bin/bash
# Native codec/plan development only: never invokes a solver or starts a session.
set -eu
cli=$1
tests=$2
out=$3
test ! -e "$out"
mkdir -p "$out"
"$tests" --list > "$out/tests.list" 2> "$out/list.stderr"
test "$(rg -c ': test$' "$out/tests.list")" -eq 8
"$tests" --test-threads=1 --nocapture > "$out/tests.stdout" 2> "$out/tests.stderr"
test ! -s "$out/tests.stderr"
rg -q '^test result: ok\. 8 passed; 0 failed;' "$out/tests.stdout"
study=research/prime_conductor_gap80_20261031/paired-schedule-20261118
"$cli" plan --spec "$study/legacy-cli-spec.json" --doc > "$out/legacy.json" 2> "$out/legacy.stderr"
cmp "$out/legacy.json" "$study/proof/v4-legacy.json"
"$cli" plan --spec "$study/paired-cli-spec.json" --doc > "$out/paired.json" 2> "$out/paired.stderr"
cmp "$out/paired.json" "$study/proof/v4-paired.json"
mkdir "$out/full-plan"
cp "$study/paired-wide-spec.json" "$out/full-plan/spec.json"
"$cli" plan --spec "$out/full-plan/spec.json" --doc > "$out/full-plan/plan.json" 2> "$out/full.stderr"
python3 - "$out" <<'PY'
import json,pathlib,sys
out=pathlib.Path(sys.argv[1]);p=json.loads((out/'full-plan/plan.json').read_text())
assert len(p['workloads'])==192 and len(p['executions'])==4032
curves=list(dict.fromkeys(w['curve']['slug'] for w in p['workloads']))
assert len(curves)==6
with (out/'selections.tsv').open('w') as f:
    for label,a,b in [('field202-image',0,1),('field202-coordinate',0,2),('field96-image',3,4),('field96-coordinate',3,5)]:
        f.write('\t'.join((label,curves[a],curves[b]))+'\n')
PY
while IFS=$'\t' read -r label source destination; do
  "$cli" endpoint-pairs --dir "$out/full-plan" --source-curve "$source" --destination-curve "$destination" --arm rho-counted --plan-only > "$out/$label.json" 2> "$out/$label.stderr"
done < "$out/selections.tsv"
"$cli" endpoint-pairs --help > "$out/help.stdout" 2> "$out/help.stderr"
python3 - "$out" <<'PY'
import json,pathlib,sys
out=pathlib.Path(sys.argv[1]);summaries=[]
for label in ['field202-image','field202-coordinate','field96-image','field96-coordinate']:
    v=json.loads((out/(label+'.json')).read_text());pairs=v['pairs']
    assert v['schema']=='ecbench.endpoint_pairs/v1' and v['scope']=='structural_join_only'
    assert len(pairs)==224 and len({p['target_index'] for p in pairs})==32
    assert sum(p['warmup'] for p in pairs)==32
    assert not v['result_admitted'] and v['observations_in_input']==v['observations_selected']==0
    assert v['conductor_gap'] is v['cost_ratio'] is v['wall_ratio'] is None
    assert all(p['source_status']==p['destination_status']=='missing' for p in pairs)
    assert all(p['source']['workload_id']!=p['destination']['workload_id'] for p in pairs)
    assert all(p['source']['execution']['algorithm_seed']==p['destination']['execution']['algorithm_seed'] for p in pairs)
    summaries.append({'selection':label,'planned_pairs':len(pairs),'targets':32,'warmup_pairs':32,'observed_records':0})
assert all(not p.read_bytes() for p in out.glob('*.stderr'))
print(json.dumps({'scope':'offline native codec and plan qualification','focused_tests_passed':8,'legacy_plan_byte_identical':True,'paired_plan_byte_identical':True,'workloads':192,'executions':4032,'selections':summaries,'structural_join_implemented':True,'statistical_comparison_implemented':False,'benchmark_session_executed':False,'runtime_measurements':None,'conductor_crossing_established':False}))
PY
