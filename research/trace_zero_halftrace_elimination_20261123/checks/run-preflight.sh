#!/usr/bin/env bash
set -euo pipefail
cd /workspace/crypto
trace_root=research/trace_zero_halftrace_elimination_20261123
trace_binary="$trace_root/bin/elimination"
export TRACE_BASELINE_EXE="$PWD/$trace_root/bin/baseline"
trace_source=$(git rev-parse HEAD)
export VALGRIND=/workspace/scratch/profile-tools/root/usr/bin/valgrind
export VALGRIND_LIB=/workspace/scratch/profile-tools/root/usr/libexec/valgrind
export RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1
unset TRACE_DISABLE_TALLY TRACE_NO_PHASE_DUMPS
trace_start=$(date -u +%s)
printf '%s\n' "$trace_start" > "$trace_root/start.epoch"
date -u +%Y-%m-%dT%H:%M:%SZ > "$trace_root/start.txt"
set +e
taskset -c 0 bash -c 'ulimit -v 2097152; exec timeout 600 "$1" calibrate "$2" "$3"' trace-calibration "$trace_binary" research/trace_zero_projected_20261115/inputs /workspace/scratch/trace-halftrace-calibration-final > "$trace_root/calibration.json" 2> "$trace_root/calibration.stderr"
trace_exit=$?
set -e
printf '%s\n' "$trace_exit" > "$trace_root/calibration.exit"
if [ "$trace_exit" -ne 0 ];then exit "$trace_exit";fi
trace_remaining=$((600-$(date -u +%s)+trace_start))
set +e
taskset -c 0 bash -c 'ulimit -v 2097152; exec timeout "$1" "$2" pilot "$3" "$4"' trace-pilot "$trace_remaining" "$trace_binary" research/trace_zero_projected_20261115/inputs /workspace/scratch/trace-halftrace-pilot > "$trace_root/preflight.json" 2> "$trace_root/preflight.stderr"
trace_exit=$?
set -e
printf '%s\n' "$trace_exit" > "$trace_root/preflight.exit"
if [ "$trace_exit" -ne 0 ];then exit "$trace_exit";fi
python3 - "$trace_source" <<'PY'
import json,sys,time,hashlib
from pathlib import Path
p=Path('research/trace_zero_halftrace_elimination_20261123');pilot=json.loads((p/'preflight.json').read_text());rows=pilot['rows'];assert len(rows)==12
assert json.loads((p/'calibration.json').read_text())['passes']
q=json.loads((p/'protocol.json').read_text());binary=hashlib.sha256((p/'bin/elimination').read_bytes()).hexdigest()
for r in rows:
 assert r['result']['valid'],r
 if 'distinct-planted' in r['instance']:assert r['result']['outcome']=='Found' and r['result']['replay'],r
for arm in q['arms']:
 expected=hashlib.sha256((p/'bin'/('baseline' if arm['id'].startswith('baseline') else 'elimination')).read_bytes()).hexdigest();assert arm['instrument']['binary_sha256']==expected;arm['instrument']['commit']=sys.argv[1]
cost=0
for n in [83,131]:
 for arm in q['arms']:
  values=[r['engineering_elapsed_seconds'] for r in rows if r['n']==n and r['arm']==arm['id']];assert len(values)==2;cost+=15*max(values)
fixed=time.time()-int((p/'start.epoch').read_text())
q['sample_size']['preflight_runtime']={'fixed_seconds':fixed,'per_sample_seconds':cost/90,'threads':1,'estimated_total_seconds':fixed+cost,'measured_on':'12-call CPU0/2GiB engineering pilot; per field/arm max of positive/uniform times15 primary calls; fixed includes partition calibration/pilot. Resource planning only.'}
(p/'protocol.json').write_text(json.dumps(q,indent=2)+'\n')
(p/'preflight-gate.json').write_text(json.dumps({'calls':12,'validity':'passed','positive_detection':'passed','phase_calibration':'passed','estimated_total_seconds':fixed+cost,'fits600':fixed+cost<600},indent=2)+'\n')
if fixed+cost>=600:raise SystemExit('Estimate infeasible: preserve attempt; no silent sample reduction')
with (p/'checks/analysis-smoke-rows.jsonl').open('x') as f:
 for r in rows:
  row={'instance':r['instance'],'arm':r['arm'],'replicate':0,'result':r['result']};row.update(r['metrics']);f.write(json.dumps(row)+'\n')
PY
"$trace_binary" analyze "$trace_root/checks/analysis-smoke-rows.jsonl" > "$trace_root/checks/analysis-smoke.json"
