#!/usr/bin/env bash
# Amendment 1 driver: stop each tower arm once its t = 4 cell is complete (6 rows), then run the nulls.
cd "$(git rev-parse --show-toplevel)"
D=research/cover_image_fb_r3_m3_20261112
done4() { python3 -c "
import json,sys
n=sum(1 for l in open('$D/runs/m3-$1.jsonl') if (r:=json.loads(l)).get('control')=='tower' and r.get('t')==4)
sys.exit(0 if n>=6 else 1)" 2>/dev/null; }
while pgrep -f "$D/run.sh" >/dev/null; do
  for a in cubic quart generic3 quad; do
    if done4 $a; then
      pid=$(pgrep -f "runs/m3-$a.jsonl")
      [ -n "$pid" ] && kill $pid && echo "$(date -u +%FT%TZ) stopped m3-$a after t=4 (Amendment 1)" >> $D/runs/progress.txt
    fi
  done
  sleep 10
done
"$D/run_nulls.sh"
