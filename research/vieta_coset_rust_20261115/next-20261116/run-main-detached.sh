#!/usr/bin/env bash
set -u
cd /workspace/crypto || exit 2
out=research/vieta_coset_rust_20261115/next-20261116/run
[[ ! -e "$out/main-supervisor.json" && ! -e "$out/main" ]] || exit 2
started=$(date -u +%Y-%m-%dT%H:%M:%SZ)
python3 tools/benchrec.py run benchmarks/records/20261116T1938Z-vieta-half-gcd-rational.json python3 tools/isolated_bench.py run --cpus 4 --out "$out/main-isolation.jsonl" -- timeout --signal=TERM --kill-after=5s 3600 bash "$out/bin/run-panel.sh" main "$out/main" "$out/bin/qualifier" > "$out/main-supervisor.log" 2>&1
code=$?
finished=$(date -u +%Y-%m-%dT%H:%M:%SZ)
printf '{"schema":"vieta-next-detached-supervisor/v1","started":"%s","finished":"%s","exit":%s,"no_retry":true}\n' "$started" "$finished" "$code" > "$out/main-supervisor.json"
exit "$code"
