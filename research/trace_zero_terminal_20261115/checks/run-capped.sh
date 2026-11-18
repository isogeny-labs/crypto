#!/usr/bin/env bash
set -uo pipefail
cd /workspace/crypto-trace
trace_root=research/trace_zero_terminal_20261115
trace_binary="$trace_root/native/target/release/trace-zero-terminal"
trace_source=$(git rev-parse HEAD)
trace_start=$(date -u +%s)
echo "$trace_start" > "$trace_root/start.epoch"
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$trace_root/start.txt"
taskset -c 0 bash -c 'ulimit -v 2097152; export RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1; exec timeout 600 "$1" preflight "$2"' trace-pilot "$trace_binary" "$trace_root/workload.json" > "$trace_root/preflight.json" 2> "$trace_root/preflight.stderr"
trace_exit=$?
echo "$trace_exit" > "$trace_root/preflight.exit"
if [ "$trace_exit" -ne 0 ]; then exit "$trace_exit"; fi
python3 - "$trace_source" <<'PY'
import json,sys,hashlib
from pathlib import Path
p=Path('research/trace_zero_terminal_20261115');rows=json.loads((p/'preflight.json').read_text())['rows']
assert len(rows)==35
for r in rows:
 assert r['result']['valid'],r
 if r['n']==5:assert r['result']['outcome']=='Found' and r['result']['replay'],r
q=json.loads((p/'protocol.json').read_text());binary=hashlib.sha256((p/'native/target/release/trace-zero-terminal').read_bytes()).hexdigest()
for a in q['arms']:
 assert a['instrument']['binary_sha256']==binary;a['instrument']['commit']=sys.argv[1]
cost=[r['engineering_elapsed_seconds'] for r in rows]
q['sample_size']['preflight_runtime']={'fixed_seconds':sum(cost),'per_sample_seconds':max(cost),'threads':1,'estimated_total_seconds':sum(cost)+686*max(cost),'measured_on':'35-call CPU0 engineering pilot. Max call planning bound, no timing-performance evidence;600second outer cap unchanged.'}
q['measurement']['record']=[f for f in q['measurement']['record'] if f not in ['preprocessing_group_ops','through_solver_group_ops']]
(p/'protocol.json').write_text(json.dumps(q,indent=2)+'\n')
(p/'preflight-gate.json').write_text(json.dumps({'rows':35,'validity':'passed','n5_detection':'passed'},indent=2)+'\n')
with (p/'checks/analysis-smoke-rows.jsonl').open('x') as f:
 for r in rows:
  result=r['result'];f.write(json.dumps({'arm':r['arm'],'instance':'pilot-n'+str(r['n']),'n':r['n'],'dimension':0,'kind':'pilot','replicate':0,'result':result,'conflicts':result['conflicts'],'peak_variables':result['peak_variables'],'group_ops':result['charged_group_add_double_count']})+'\n')
PY
if [ "$?" -ne 0 ]; then echo 'Pilot gate failed; preserve all data, no primary rows' >&2;exit 3;fi
"$trace_binary" analyze "$trace_root/checks/analysis-smoke-rows.jsonl" > "$trace_root/checks/analysis-smoke.json"
if [ "$?" -ne 0 ]; then exit 4;fi
python3 /workspace/crypto-autoresearcher/tools/experiment_protocol.py freeze "$trace_root/protocol.json" --commit "$trace_source" --filed-before-affected-data > "$trace_root/freeze-receipt.json"
if [ "$?" -ne 0 ];then exit 5;fi
git add --sparse "$trace_root"
git commit -m 'Freeze terminal-domain panel after positive pilot and analysis smoke'
if [ "$?" -ne 0 ];then exit 6;fi
git push origin codex/ecc2k-trace-homomorphism
if [ "$?" -ne 0 ];then
 git push origin HEAD:refs/heads/codex/ecc2k-terminal-run-20261115
 if [ "$?" -ne 0 ];then exit 7;fi
fi
trace_remaining=$((600-$(date -u +%s)+trace_start))
if [ "$trace_remaining" -le 0 ];then exit 8;fi
taskset -c 0 bash -c 'ulimit -v 2097152; export OMP_NUM_THREADS=1 RAYON_NUM_THREADS=1; exec timeout "$1" "$2" run "$3" "$4"' trace-run "$trace_remaining" "$trace_binary" "$trace_root/workload.json" "$trace_root/rows.jsonl" > "$trace_root/run.stdout" 2> "$trace_root/run.stderr"
trace_exit=$?
echo "$trace_exit" > "$trace_root/run.exit"
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$trace_root/end.txt"
exit "$trace_exit"
