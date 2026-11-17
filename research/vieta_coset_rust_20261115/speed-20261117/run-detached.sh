#!/usr/bin/env bash
set -u -o noclobber
cd /workspace/crypto || exit 2
panel=${1:?preflight or main}
record=${2:?open benchmark record}
out=research/vieta_coset_rust_20261115/speed-20261117/run
case "$panel" in preflight) cap=1200 ;; main) cap=7200 ;; *) exit 2 ;; esac
[[ ! -e "$out/$panel-supervisor.json" && ! -e "$out/$panel-supervisor.log" && ! -e "$out/$panel" ]] || exit 2
started=$(date -u +%Y-%m-%dT%H:%M:%SZ)
python3 tools/benchrec.py run "$record" python3 tools/isolated_bench.py run --cpus 4 --out "$out/$panel-isolation.jsonl" -- timeout --signal=TERM --kill-after=5s "$cap" bash "$out/bin/run-panel.sh" "$panel" "$out/$panel" "$out/bin/qualifier" > "$out/$panel-supervisor.log" 2>&1
code=$?
finished=$(date -u +%Y-%m-%dT%H:%M:%SZ)
printf '{"schema":"vieta-speed-supervisor/v1","panel":"%s","started":"%s","finished":"%s","exit":%s,"no_retry":true}\n' "$panel" "$started" "$finished" "$code" > "$out/$panel-supervisor.json"
exit "$code"
