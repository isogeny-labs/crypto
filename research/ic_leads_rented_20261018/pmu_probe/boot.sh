#!/usr/bin/env bash
# Boot script of the PMU probe pod (item 5). Plumbing only: run probe.py,
# serve its JSON on :8000 for the rest of the pod's short life. The launcher's
# lifetime cap (POD_MAX_SECONDS, at most 900) ends the container; the
# launching session terminates the pod as soon as it has the file.
set -u
OUT=/workspace/out; mkdir -p "$OUT"
BRANCH=${BRANCH:?BRANCH}
curl -sSfL "https://raw.githubusercontent.com/aburan28/crypto/$BRANCH/research/ic_leads_rented_20261018/pmu_probe/probe.py" -o /root/probe.py
python3 /root/probe.py > "$OUT/pmu.json" 2> "$OUT/probe.err"; echo "exit $?" > "$OUT/probe.exit"
{ uname -a; cat /proc/cpuinfo | grep -m1 'model name'; ls /sys/bus/event_source/devices; } > "$OUT/host.txt" 2>&1
cd "$OUT" && exec python3 -m http.server 8000
