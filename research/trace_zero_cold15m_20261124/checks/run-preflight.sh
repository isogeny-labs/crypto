#!/usr/bin/env bash
set -euo pipefail
cd /workspace/crypto
export TRACE_STUDY=research/trace_zero_cold15m_20261124
trace_binary="$TRACE_STUDY/bin/elimination-wire"
export TRACE_BASELINE_EXE="$PWD/$TRACE_STUDY/bin/baseline" TRACE_INCUMBENT_EXE="$PWD/$TRACE_STUDY/bin/incumbent"
export VALGRIND=/workspace/scratch/profile-tools/root/usr/bin/valgrind VALGRIND_LIB=/workspace/scratch/profile-tools/root/usr/libexec/valgrind RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1
date -u +%s > "$TRACE_STUDY/start.epoch"
trace_start=$(cat "$TRACE_STUDY/start.epoch")
trace_remaining=$((600-$(date -u +%s)+trace_start))
taskset -c 0 bash -c 'ulimit -v 2097152; exec timeout "$1" "$2" calibrate "$3" "$4"' trace-cal "$trace_remaining" "$trace_binary" research/trace_zero_projected_20261115/inputs /workspace/scratch/cold15m-calibration > "$TRACE_STUDY/calibration.json" 2> "$TRACE_STUDY/calibration.stderr"
trace_remaining=$((600-$(date -u +%s)+trace_start))
taskset -c 0 bash -c 'ulimit -v 2097152; exec timeout "$1" "$2" pilot "$3" "$4"' trace-pilot "$trace_remaining" "$trace_binary" research/trace_zero_projected_20261115/inputs /workspace/scratch/cold15m-pilot > "$TRACE_STUDY/preflight.json" 2> "$TRACE_STUDY/preflight.stderr"
python3 - <<'PY'
from pathlib import Path
import os,json,hashlib,time
p=Path(os.environ['TRACE_STUDY']);pilot=json.loads((p/'preflight.json').read_text());rows=pilot['rows'];assert len(rows)==16
assert json.loads((p/'calibration.json').read_text())['passes']
for r in rows:
 assert r['result']['valid']
 if 'distinct-planted' in r['instance']:assert r['result']['outcome']=='Found' and r['result']['replay']
q=json.loads((p/'protocol.json').read_text())
for a in q['arms']:
 name='baseline' if a['id'].startswith('baseline') else a['id'];assert a['instrument']['binary_sha256']==hashlib.sha256((p/'bin/elimination-wire' if name=='elimination' else p/'bin'/name).read_bytes()).hexdigest()
cost=0
for n in [83,131]:
 for a in q['arms']:cost+=15*max(r['engineering_elapsed_seconds'] for r in rows if r['n']==n and r['arm']==a['id'])
fixed=time.time()-int((p/'start.epoch').read_text());q['sample_size']['preflight_runtime']={'fixed_seconds':fixed,'per_sample_seconds':cost/120,'threads':1,'estimated_total_seconds':fixed+cost,'measured_on':'Corrected16-call pilot, per-field/arm maximum*15; original epoch includes invalid attempt and all correction/build costs.'}
(p/'protocol.json').write_text(json.dumps(q,indent=2)+'\n');(p/'preflight-gate.json').write_text(json.dumps({'calls':16,'calibration_passes':True,'validity_passes':True,'positive_passes':True,'estimated_total_seconds':fixed+cost,'fits_original600':fixed+cost<600},indent=2)+'\n');assert fixed+cost<600,'retain original deadline, no downscale or reset'
PY
