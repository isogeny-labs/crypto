#!/usr/bin/env bash
set -uo pipefail
cd /workspace/crypto-trace
trace_root=research/trace_zero_linearized_20261115
trace_binary="$trace_root/native/target/release/trace-zero-linearized"
trace_source=$(git rev-parse HEAD)
python3 - <<'PY'
import json
from pathlib import Path
p=Path('research/trace_zero_linearized_20261115/protocol.json');q=json.loads(p.read_text());q['extensions']['runtime_planning']='Before pilot: n5/7/9 use sole field representative; n83/131 smallest pilot dimension>=case dimension (4->5,6->8,10->12). Sum per-arm costs of all650 planned rows, plus actual pilot/setup elapsed. No samples or methods omitted.';p.write_text(json.dumps(q,indent=2)+'\n')
PY
trace_start=$(date -u +%s)
echo "$trace_start" > "$trace_root/start.epoch"
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$trace_root/start.txt"
taskset -c 0 bash -c 'ulimit -v 2097152; export RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1; exec timeout 600 "$1" preflight "$2"' trace-pilot "$trace_binary" "$trace_root/workload.json" > "$trace_root/preflight.json" 2> "$trace_root/preflight.stderr"
trace_exit=$?;echo "$trace_exit" > "$trace_root/preflight.exit"
if [ "$trace_exit" -ne 0 ];then exit "$trace_exit";fi
python3 - "$trace_source" <<'PY'
import json,sys,time,hashlib
from pathlib import Path
p=Path('research/trace_zero_linearized_20261115');rows=json.loads((p/'preflight.json').read_text())['rows'];assert len(rows)==45
for r in rows:
 assert r['result']['valid'],r
 if r['n']==5:assert r['result']['outcome']=='Found' and r['result']['replay'],r
q=json.loads((p/'protocol.json').read_text());binary=hashlib.sha256((p/'native/target/release/trace-zero-linearized').read_bytes()).hexdigest()
for a in q['arms']:
 assert a['instrument']['binary_sha256']==binary;a['instrument']['commit']=sys.argv[1]
cases=json.loads((p/'workload.json').read_text())['cases'];cost=0
for c in cases:
 for a in q['arms']:
  rs=[r for r in rows if r['n']==c['n'] and r['arm']==a['id']]
  if c['n']>=83:rs=[r for r in rs if r['dimension']>=c['dimension']]
  selected=min(rs,key=lambda r:r['dimension']);cost+=2*selected['engineering_elapsed_seconds']
fixed=time.time()-int((p/'start.epoch').read_text())
q['sample_size']['preflight_runtime']={'fixed_seconds':fixed,'per_sample_seconds':cost/650,'threads':1,'estimated_total_seconds':fixed+cost,'measured_on':'Stratified45-call CPU0 engineering pilot, fixed field/ceiling-dimension map; fixed includes known-answer pilot setup. Planning only, no performance claim.'}
(p/'protocol.json').write_text(json.dumps(q,indent=2)+'\n')
(p/'preflight-gate.json').write_text(json.dumps({'rows':45,'validity':'passed','n5_detection':'passed','estimated_total_seconds':fixed+cost,'fits600':fixed+cost<600},indent=2)+'\n')
if fixed+cost>=600:raise SystemExit('Runtime estimate infeasible: preserve all rows and amend before primary; no downscaling')
with (p/'checks/analysis-smoke-rows.jsonl').open('x') as f:
 for r in rows:
  z=r['result'];f.write(json.dumps({'arm':r['arm'],'instance':f"pilot-n{r['n']}-d{r['dimension']}",'n':r['n'],'dimension':r['dimension'],'kind':'pilot','replicate':0,'result':z,'conflicts':z['conflicts'],'peak_variables':z['peak_variables'],'linear_columns':z['linear_columns'],'group_ops':z['charged_group_add_double_count']})+'\n')
PY
if [ "$?" -ne 0 ];then exit 3;fi
"$trace_binary" analyze "$trace_root/checks/analysis-smoke-rows.jsonl" > "$trace_root/checks/analysis-smoke.json"
if [ "$?" -ne 0 ];then exit 4;fi
python3 /workspace/crypto-autoresearcher/tools/experiment_protocol.py freeze "$trace_root/protocol.json" --commit "$trace_source" --filed-before-affected-data > "$trace_root/freeze-receipt.json"
if [ "$?" -ne 0 ];then exit 5;fi
git add --sparse "$trace_root"
git commit -m 'Freeze linearized scale panel after largest-class pilot'
if [ "$?" -ne 0 ];then exit 6;fi
git push origin codex/ecc2k-trace-homomorphism
if [ "$?" -ne 0 ];then git push origin HEAD:refs/heads/codex/ecc2k-linearized-run-20261115; if [ "$?" -ne 0 ];then exit 7;fi;fi
trace_remaining=$((600-$(date -u +%s)+trace_start))
if [ "$trace_remaining" -le 0 ];then exit 8;fi
taskset -c 0 bash -c 'ulimit -v 2097152; export OMP_NUM_THREADS=1 RAYON_NUM_THREADS=1; exec timeout "$1" "$2" run "$3" "$4"' trace-run "$trace_remaining" "$trace_binary" "$trace_root/workload.json" "$trace_root/rows.jsonl" > "$trace_root/run.stdout" 2> "$trace_root/run.stderr"
trace_exit=$?;echo "$trace_exit" > "$trace_root/run.exit";date -u '+%Y-%m-%dT%H:%M:%SZ' > "$trace_root/end.txt";exit "$trace_exit"
