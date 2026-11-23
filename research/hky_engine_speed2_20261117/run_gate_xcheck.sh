#!/bin/bash
# Instrument cross-check: v1 (hky2) == v2 (hky3) at the degrees the definitional reference cannot
# reach in reasonable time here (m6/m7 c7). v1 itself passed the definitional gate on the smaller
# cells, so agreement here is transitive evidence. Also re-runs the standard sets the reference can do.
set -u
HERE=$(cd "$(dirname "$0")" && pwd)
V1=${V1:-$HERE/../hky_engine_speed_20261117/bin/pilot_candidate}
V2=${V2:-$HERE/bin/pilot_v2}
EQ=${EQ:-$HERE/bin/hky_engine_equality}
OUT=$1; shift
mkdir -p "$OUT"; P1=2013265921
common=(--ladder-t none --budget 3600 --hky-max-cols 2000000 --hky-max-entries 14000000000)
xset() { local name=$1; shift
  "$V1" "${common[@]}" "$@" --engine hky2 --out "$OUT/$name.v1.jsonl" 2>/dev/null
  "$V2" "${common[@]}" "$@" --engine hky3 --out "$OUT/$name.v2.jsonl" 2>/dev/null
  printf '%-22s ' "$name"; "$EQ" compare "$OUT/$name.v1.jsonl" "$OUT/$name.v2.jsonl" | tail -1
}
xset m6-tower-t2-c7  --p $P1 --planted 0 --random 1 --hky-cmax 7 --kinds kummer --m 6 --controls tower --t-min 2 --max-t 2
xset m7-tower-t2-c7  --p $P1 --planted 0 --random 1 --hky-cmax 7 --kinds kummer --m 7 --controls tower --t-min 2 --max-t 2
xset m4-tower-t3-c7  --p $P1 --planted 0 --random 1 --hky-cmax 7 --kinds kummer --m 4 --controls tower --t-min 3 --max-t 3
xset m5-tower-t3-c7  --p $P1 --planted 0 --random 1 --hky-cmax 7 --kinds kummer --m 5 --controls tower --t-min 3 --max-t 3
