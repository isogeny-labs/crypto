#!/usr/bin/env bash
set -uo pipefail
cd /workspace/crypto-trace
trace_panel=$1
trace_source=$(git rev-parse HEAD)
trace_root=research/trace_zero_comparisons_20261115
trace_run="$trace_root/$trace_panel"
trace_binary="$trace_root/native/target/release/trace-zero-comparisons"
trace_start=$(date -u +%s)
echo "$trace_start" > "$trace_run/start.epoch"
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$trace_run/start.txt"
taskset -c 0 bash -c 'ulimit -v 2097152; export OMP_NUM_THREADS=1 RAYON_NUM_THREADS=1; exec timeout 600 "$1" preflight "$2" "$3"' trace-pilot "$trace_binary" "$trace_panel" "$trace_root/workload.json" > "$trace_run/preflight.json" 2> "$trace_run/preflight.stderr"
trace_pilot_exit=$?
echo "$trace_pilot_exit" > "$trace_run/preflight.exit"
if [ "$trace_pilot_exit" -ne 0 ]; then exit "$trace_pilot_exit"; fi
python3 - "$trace_panel" "$trace_source" <<'PY'
import json,sys,hashlib
from pathlib import Path
root=Path('research/trace_zero_comparisons_20261115');p=root/sys.argv[1]
rows=json.loads((p/'preflight.json').read_text())['rows']
for r in rows:
 assert r['result']['valid'],r
 if r['n']==5: assert r['result']['outcome']=='Found' and r['result']['replay'] and r['result']['mapped_replay'],r
q=json.loads((p/'protocol.json').read_text())
assert len(rows)==5*len(q['arms'])
binary_hash=hashlib.sha256((root/'native/target/release/trace-zero-comparisons').read_bytes()).hexdigest()
for arm in q['arms']:
 assert arm['instrument']['binary_sha256']==binary_hash
 arm['instrument']['commit']=sys.argv[2]
elapsed=[r['engineering_elapsed_seconds'] for r in rows]
q['sample_size']['preflight_runtime']={'fixed_seconds':sum(elapsed),'per_sample_seconds':max(elapsed),'threads':1,'estimated_total_seconds':sum(elapsed)+q['extensions']['planned_rows']*max(elapsed),'measured_on':'One CPU local native engineering pilot, sum retained as fixed preflight work,max call as conservative per-row planning bound; no timing comparison.'}
(p/'protocol.json').write_text(json.dumps(q,indent=2)+'\n')
(p/'preflight-gate.json').write_text(json.dumps({'rows':len(rows),'independent_validity':'passed','n5_positive_detection':'passed','engineering_elapsed_seconds':sum(elapsed)},indent=2)+'\n')
PY
if [ "$?" -ne 0 ]; then echo 'PILOT GATE FAILED; no primary rows' >&2; exit 3; fi
python3 /workspace/crypto-autoresearcher/tools/experiment_protocol.py freeze "$trace_run/protocol.json" --commit "$trace_source" --filed-before-affected-data > "$trace_run/freeze-receipt.json"
if [ "$?" -ne 0 ]; then exit 4; fi
git add --sparse "$trace_run"
git commit -m "Freeze $trace_panel comparison after engineering pilot"
if [ "$?" -ne 0 ]; then exit 5; fi
git push origin codex/ecc2k-trace-homomorphism
if [ "$?" -ne 0 ]; then exit 6; fi
trace_remaining=$((600-$(date -u +%s)+trace_start))
if [ "$trace_remaining" -le 0 ]; then echo 'Freeze exhausted total budget; no primary run' >&2; exit 7; fi
taskset -c 0 bash -c 'ulimit -v 2097152; export OMP_NUM_THREADS=1 RAYON_NUM_THREADS=1; exec timeout "$1" "$2" run "$3" "$4" "$5"' trace-run "$trace_remaining" "$trace_binary" "$trace_panel" "$trace_root/workload.json" "$trace_run/rows.jsonl" > "$trace_run/run.stdout" 2> "$trace_run/run.stderr"
trace_exit=$?
echo "$trace_exit" > "$trace_run/run.exit"
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$trace_run/end.txt"
exit "$trace_exit"
