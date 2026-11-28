#!/usr/bin/env bash
set -euo pipefail
cd /workspace/crypto-trace
trace_root=research/trace_zero_fast_20261115
trace_input=research/trace_zero_linearized_20261115/workload.json
trace_binary="$trace_root/native/target/release/trace-zero-fast"
trace_source=$(git rev-parse HEAD)
export PATH=/workspace/scratch/trace-profile-tools/root/usr/bin:$PATH
export LD_LIBRARY_PATH=/workspace/scratch/trace-profile-tools/root/usr/lib/x86_64-linux-gnu
export VALGRIND_LIB=/workspace/scratch/trace-profile-tools/root/usr/libexec/valgrind
export VALGRIND=/workspace/scratch/trace-profile-tools/root/usr/bin/valgrind
export RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1
trace_start=$(date -u +%s)
printf '%s\n' "$trace_start" > "$trace_root/start.epoch"
date -u +%Y-%m-%dT%H:%M:%SZ > "$trace_root/start.txt"
set +e
taskset -c 0 bash -c 'ulimit -v 2097152; exec timeout 600 "$1" pilot "$2" "$3"' trace-pilot "$trace_binary" "$trace_input" /workspace/scratch/trace-fast-pilot > "$trace_root/preflight.json" 2> "$trace_root/preflight.stderr"
trace_exit=$?
set -e
printf '%s\n' "$trace_exit" > "$trace_root/preflight.exit"
if [ "$trace_exit" -ne 0 ];then exit "$trace_exit";fi
python3 - "$trace_source" <<'PY'
import json,sys,time,hashlib
from pathlib import Path
p=Path('research/trace_zero_fast_20261115');pilot=json.loads((p/'preflight.json').read_text());rows=pilot['rows'];assert len(rows)==12
q=json.loads((p/'protocol.json').read_text());binary=hashlib.sha256((p/'native/target/release/trace-zero-fast').read_bytes()).hexdigest()
for r in rows:
 assert r['result']['valid'],r
 if 'distinct-planted' in r['instance']:assert r['result']['outcome']=='Found' and r['result']['replay'],r
for arm in q['arms']:
 assert arm['instrument']['binary_sha256']==binary;arm['instrument']['commit']=sys.argv[1]
cost=0
for n in [83,131]:
 for arm in q['arms']:
  selected=[r['engineering_elapsed_seconds'] for r in rows if r['n']==n and r['arm']==arm['id']]
  assert len(selected)==2;cost+=15*max(selected)
fixed=time.time()-int((p/'start.epoch').read_text())
q['sample_size']['preflight_runtime']={'fixed_seconds':fixed,'per_sample_seconds':cost/90,'threads':1,'estimated_total_seconds':fixed+cost,'measured_on':'12-call CPU0/2GiB engineering pilot including both largest bases; fixed field/arm max of positive/uniform times15 calls. Planning only, not timing evidence.'}
(p/'protocol.json').write_text(json.dumps(q,indent=2)+'\n')
(p/'preflight-gate.json').write_text(json.dumps({'calls':12,'validity':'passed','positive_detection':'passed','estimated_total_seconds':fixed+cost,'fits600':fixed+cost<600},indent=2)+'\n')
if fixed+cost>=600:raise SystemExit('Infeasible estimate: preserve attempt, do not reduce primary sample')
with (p/'checks/analysis-smoke-rows.jsonl').open('x') as f:
 for r in rows:
  row={'instance':r['instance'],'arm':r['arm'],'result':r['result']};row.update(r['result']['metrics']);f.write(json.dumps(row)+'\n')
PY
"$trace_binary" analyze "$trace_root/checks/analysis-smoke-rows.jsonl" > "$trace_root/checks/analysis-smoke.json"
python3 /workspace/crypto-autoresearcher/tools/experiment_protocol.py freeze "$trace_root/protocol.json" --commit "$trace_source" --filed-before-affected-data > "$trace_root/freeze-receipt.json"
git add --sparse "$trace_root"
git commit -m 'Freeze cold-query instruction comparison after largest-class pilot'
if ! git push origin codex/ecc2k-trace-homomorphism;then
 git push origin HEAD:refs/heads/codex/ecc2k-fast-run-20261115
fi
trace_remaining=$((600-$(date -u +%s)+trace_start))
if [ "$trace_remaining" -le 0 ];then exit 8;fi
set +e
taskset -c 0 bash -c 'ulimit -v 2097152; exec timeout "$1" "$2" run "$3" "$4" "$5"' trace-run "$trace_remaining" "$trace_binary" "$trace_input" /workspace/scratch/trace-fast-primary "$trace_root/rows.jsonl" > "$trace_root/run.stdout" 2> "$trace_root/run.stderr"
trace_exit=$?
set -e
printf '%s\n' "$trace_exit" > "$trace_root/run.exit"
date -u +%Y-%m-%dT%H:%M:%SZ > "$trace_root/end.txt"
exit "$trace_exit"
