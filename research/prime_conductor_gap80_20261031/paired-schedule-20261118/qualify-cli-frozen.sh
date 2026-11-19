#!/bin/sh
# Native ecbench plans only; Python below validates serialized metadata.
set -eu
repo=$1
binary=$2
out=$3
cd "$repo"
study=research/prime_conductor_gap80_20261031/paired-schedule-20261118
test ! -e "$out"
mkdir -p "$out"
timeout 60 "$binary" plan --spec "$study/legacy-cli-spec.json" --doc > "$out/legacy.json" 2> "$out/legacy.stderr"
test ! -s "$out/legacy.stderr"
cmp "$out/legacy.json" "$study/results/legacy-a/plan.json"
timeout 60 "$binary" plan --spec "$study/paired-cli-spec.json" --doc > "$out/paired.json" 2> "$out/paired.stderr"
test ! -s "$out/paired.stderr"
timeout 60 "$binary" plan --spec "$study/paired-wide-spec.json" --json > "$out/full.json" 2> "$out/full.stderr"
test ! -s "$out/full.stderr"
for rejection in odd public single ic; do
  if timeout 60 "$binary" plan --spec "$study/negative-$rejection.json" --doc > "$out/$rejection.stdout" 2> "$out/$rejection.stderr"; then rejection_status=0; else rejection_status=$?; fi
  printf '%s\n' "$rejection_status" > "$out/$rejection.exit"
  test "$rejection_status" -eq 2
  test ! -s "$out/$rejection.stdout"
done
python3 - "$study" "$out" <<'PY'
import collections,json,pathlib,sys
study,out=map(pathlib.Path,sys.argv[1:]);d=json.loads((out/'paired.json').read_text())
source=json.loads((study/'paired-cli-spec.json').read_text())
vectors={tuple(row[:4]):row[4] for row in json.loads((study/'seed-vectors.json').read_text())['rows']}
assert d['schema']=='ecbench.plan/v1'
assert len(d['workloads'])==12 and len(d['executions'])==252
assert d['spec_id']!=json.loads((out/'legacy.json').read_text())['spec_id']
groups=collections.defaultdict(list)
keys=[]
for seq,e in enumerate(d['executions']):
    assert e['seq']==seq
    w=d['workloads'][e['workload']]
    assert not w['curve']['registered']
    phase=e['round'] if e['warmup'] else e['round']-source['measurement']['warmup']
    assert e['algorithm_seed']==vectors[(w['curve']['r'],w['target_index'],e['warmup'],phase)]
    groups[(e['round'],w['curve']['r'],w['target_index'])].append(e)
    keys.append((e['round'],w['target_index'],w['curve']['r']))
assert keys==sorted(keys)
for (round,_,_),events in groups.items():
    phase=round if round<source['measurement']['warmup'] else round-source['measurement']['warmup']
    curves=[source['workloads']['curves'].index(d['workloads'][e['workload']]['curve_spec']) for e in events]
    unique=sorted(set(curves),reverse=bool(phase%2));arms=list(range(len(d['arms'])))
    if phase%2: arms.reverse()
    assert curves==[c for c in unique for _ in arms]
    assert [e['arm'] for e in events]==arms*len(unique)
    assert len({e['algorithm_seed'] for e in events})==1
    assert len({d['workloads'][e['workload']]['planted'] for e in events})==1
assert 'even' in (out/'odd.stderr').read_text()
assert 'planted_shared' in (out/'public.stderr').read_text()
assert 'at least two' in (out/'single.stderr').read_text()
assert 'no prime_mont4 adapter' in (out/'ic.stderr').read_text()
full=json.loads((out/'full.json').read_text())
assert len(full['workloads'])==192 and full['executions']==4032
assert all(not w['curve']['registered'] for w in full['workloads'])
print(json.dumps({'legacy_plan_byte_identical':True,'paired_workloads':12,'paired_executions':252,'same_order_index_groups':len(groups),'full_paired_workloads':192,'full_paired_executions':4032,'negative_admissions':4,'all_workloads_unregistered':True,'benchmark_session_executed':False,'runtime_measurements':None}))
PY
