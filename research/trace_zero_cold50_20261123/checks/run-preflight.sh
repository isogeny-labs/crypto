#!/usr/bin/env bash
set -euo pipefail
cd /workspace/crypto
trace_root=research/trace_zero_cold50_20261123
trace_binary="$trace_root/bin/elimination"
export TRACE_BASELINE_EXE="$PWD/$trace_root/bin/baseline"
export TRACE_INCUMBENT_EXE="$PWD/$trace_root/bin/incumbent"
export VALGRIND=/workspace/scratch/profile-tools/root/usr/bin/valgrind
export VALGRIND_LIB=/workspace/scratch/profile-tools/root/usr/libexec/valgrind
export RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1
unset TRACE_DISABLE_TALLY TRACE_NO_PHASE_DUMPS
trace_start=$(date -u +%s)
printf '%s\n' "$trace_start" > "$trace_root/start.epoch"
date -u +%Y-%m-%dT%H:%M:%SZ > "$trace_root/start.txt"
set +e
taskset -c 0 bash -c 'ulimit -v 2097152; exec timeout 600 "$1" calibrate "$2" "$3"' trace-calibration "$trace_binary" research/trace_zero_projected_20261115/inputs /workspace/scratch/trace-cold50-calibration > "$trace_root/calibration.json" 2> "$trace_root/calibration.stderr"
trace_exit=$?
set -e
printf '%s\n' "$trace_exit" > "$trace_root/calibration.exit"
if [ "$trace_exit" -ne 0 ];then exit "$trace_exit";fi
trace_remaining=$((600-$(date -u +%s)+trace_start))
set +e
taskset -c 0 bash -c 'ulimit -v 2097152; exec timeout "$1" "$2" pilot "$3" "$4"' trace-pilot "$trace_remaining" "$trace_binary" research/trace_zero_projected_20261115/inputs /workspace/scratch/trace-cold50-pilot > "$trace_root/preflight.json" 2> "$trace_root/preflight.stderr"
trace_exit=$?
set -e
printf '%s\n' "$trace_exit" > "$trace_root/preflight.exit"
if [ "$trace_exit" -ne 0 ];then exit "$trace_exit";fi
python3 - <<'PY'
import json,time,hashlib
from pathlib import Path
p=Path('research/trace_zero_cold50_20261123');pilot=json.loads((p/'preflight.json').read_text());rows=pilot['rows'];assert len(rows)==16
assert json.loads((p/'calibration.json').read_text())['passes']
for r in rows:
 assert r['result']['valid'],r
 if 'distinct-planted' in r['instance']:assert r['result']['outcome']=='Found' and r['result']['replay'],r
q=json.loads((p/'protocol.json').read_text())
for arm in q['arms']:
 name='baseline' if arm['id'].startswith('baseline') else arm['id']
 assert arm['instrument']['binary_sha256']==hashlib.sha256((p/'bin'/name).read_bytes()).hexdigest()
cost=0
for n in [83,131]:
 for arm in q['arms']:
  values=[r['engineering_elapsed_seconds'] for r in rows if r['n']==n and r['arm']==arm['id']];assert len(values)==2;cost+=15*max(values)
fixed=time.time()-int((p/'start.epoch').read_text());q['sample_size']['preflight_runtime']={'fixed_seconds':fixed,'per_sample_seconds':cost/120,'threads':1,'estimated_total_seconds':fixed+cost,'measured_on':'16-call engineering pilot, per field/arm maximum positive/uniform elapsed*15; fixed includes three-binary partition calibration. Resource planning only.'}
(p/'protocol.json').write_text(json.dumps(q,indent=2)+'\n')
(p/'preflight-gate.json').write_text(json.dumps({'calls':16,'validity':'passed','positive_detection':'passed','phase_calibration':'passed','estimated_total_seconds':fixed+cost,'fits600':fixed+cost<600},indent=2)+'\n')
if fixed+cost>=600:raise SystemExit('Retain infeasible attempt; no silent sample reduction')
with (p/'checks/analysis-smoke-rows.jsonl').open('x') as f:
 for r in rows:
  row={'instance':r['instance'],'arm':r['arm'],'replicate':0,'result':r['result']};row.update(r['metrics']);f.write(json.dumps(row)+'\n')
PY
"$trace_binary" analyze "$trace_root/checks/analysis-smoke-rows.jsonl" > "$trace_root/checks/analysis-smoke.json"
