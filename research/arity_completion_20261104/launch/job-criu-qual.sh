#!/usr/bin/env bash
# CRIU/EBS command-mode qualification payload (not evidence for any study).
# Runs the prime-tower positive-control cell ctl-m3-n15-kummer (recorded D = 7,
# about 9 minutes and 1.2 GB on EPYC 9R45) once, with the frozen command, so a
# worker loss can be induced mid-cell and the restored output compared with the
# recorded rows in /opt/study/resume/prime/cells/ctl-m3-n15-kummer.jsonl.
source /opt/study/launch/job-lib.sh
preamble
C="$OUT/cells"; mkdir -p "$C"
log "qualification cell ctl-m3-n15-kummer start"
prlimit --as=8589934592 timeout 7500 /usr/bin/time -v /usr/local/bin/pkm_tower_pilot --engine tower --p 2013265921 \
  --controls tower --planted 0 --random 2 --ladder-t none --kinds kummer --m 3 --t-min 5 --max-t-m3 5 \
  --budget 7200 --max-nnz 2000000000 --trace --out "$C/ctl-m3-n15-kummer.jsonl" 2> "$C/ctl-m3-n15-kummer.time"
rc=$?
log "qualification cell exit $rc"
python3 - "$C/ctl-m3-n15-kummer.jsonl" /opt/study/resume/prime/cells/ctl-m3-n15-kummer.jsonl > "$OUT/qualification-compare.txt" <<'PY'
import json, sys
drop = {"ms", "wall_s", "wall_seconds"}
def rows(p):
    out = []
    for ln in open(p).read().splitlines():
        r = json.loads(ln)
        if isinstance(r, dict) and r.get("m") is not None:
            out.append({k: v for k, v in r.items() if k not in drop})
    return out
a, b = rows(sys.argv[1]), rows(sys.argv[2])
same = a == b and len(a) > 0
print("rows", len(a), "recorded", len(b), "identical_except_timing", same)
for i, (x, y) in enumerate(zip(a, b)):
    diff = sorted(k for k in set(x) | set(y) if x.get(k) != y.get(k))
    if diff: print("row", i, "differs in", diff)
sys.exit(0 if same else 1)
PY
cmp=$?
log "qualification compare exit $cmp"
[ "$rc" -eq 0 ] && [ "$cmp" -eq 0 ]
