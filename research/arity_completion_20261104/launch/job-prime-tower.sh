#!/usr/bin/env bash
# Prime-tower m = 5 study (protocol prime_tower_m5 v2: frozen 1fd6ece55f, amendment AM1
# corrects the m = 2 N = 10 negative control to the recorded Kummer D = 4).
# Mode "main": the three positive-control cells, the controls verdict gate, then
# the four m = 5 cells at t = 2 and t = 3 (Kummer and isogeny), sequentially,
# each the frozen command of CARD-arity-prime-m5-controls / -t2-t3 with the
# output tree rooted at $OUT/cells/. Mode "t4": the host-memory, controls and
# t3-cap gates of CARD-arity-prime-m5-t4 against a previous attempt's cells
# (RESUME prefix), then the two t = 4 cells. A capped or memory-aborted cell
# (exit 124/134/137) is recorded as inconclusive_cap and does not stop the job
# (README decision 3); any other non-zero exit stops it.
# Usage: job-prime-tower.sh main|t4 [RESUME_S3_PREFIX]
source /opt/study/launch/job-lib.sh
preamble
resume_from "${2:-}"
MODE=${1:?mode main|t4}
P=/usr/local/bin/pkm_tower_pilot
C="$OUT/cells"; mkdir -p "$C"
pilot() { # name limit_bytes timeout_s budget_s args...
  local name=$1 lim=$2 to=$3 budget=$4; shift 4
  if [ -e "$C/$name.jsonl" ]; then log "cell $name exists (resumed); skipped"; return 0; fi
  log "cell $name start"
  prlimit --as="$lim" timeout "$to" /usr/bin/time -v "$P" --engine tower --p 2013265921 --controls tower \
    --planted 0 --random 2 --ladder-t none "$@" --budget "$budget" --max-nnz 2000000000 --trace \
    --out "$C/$name.jsonl" 2> "$C/$name.time"
  local rc=$?
  local class=error
  case "$rc" in 0) class=complete ;; 124|134|137) class=inconclusive_cap ;; esac
  printf '{"cell":"%s","exit":%s,"class":"%s"}\n' "$name" "$rc" "$class" >> "$OUT/cell-exits.jsonl"
  log "cell $name exit $rc"
  case "$rc" in 0|124|134|137) return 0 ;; *) return "$rc" ;; esac
}
controls_verdict() {
  python3 - "$C" <<'PY' > "$OUT/controls-verdict.txt"; echo $?
import json, sys
C = sys.argv[1]
want = [('ctl-m4-n8-kummer', 4, 8, 6), ('ctl-m3-n15-kummer', 3, 15, 7), ('ctl-m2-n10-kummer', 2, 10, 4)]
bad = 0
for cell, m, n, d in want:
    rows = []
    try:
        for ln in open(C + '/' + cell + '.jsonl').read().splitlines():
            r = json.loads(ln)
            if isinstance(r, dict) and r.get('m') == m and r.get('N') == n and r.get('kind') == 'kummer':
                rows.append(r)
    except OSError:
        pass
    ok = len(rows) != 0 and all((not r.get('timed_out')) and (not r.get('oversize')) and r.get('solving_degree_max') == d for r in rows)
    print('control', cell, 'rows', len(rows), 'recorded D', d, 'ok' if ok else 'MISMATCH')
    bad += 0 if ok else 1
sys.exit(1 if bad else 0)
PY
}
t3_cap_verdict() {
  python3 - "$C" <<'PY' > "$OUT/t3-cap-verdict.txt"; echo $?
import json, sys
C = sys.argv[1]
bad = 0
for kind in ('kummer', 'isogeny'):
    rows = []
    try:
        for ln in open(C + '/' + kind + '-m5-t3.jsonl').read().splitlines():
            r = json.loads(ln)
            if isinstance(r, dict) and r.get('m') == 5 and r.get('N') == 15:
                rows.append(r)
    except OSError:
        pass
    ok = len(rows) != 0 and all((not r.get('timed_out')) and (not r.get('oversize')) for r in rows)
    print('t3 cell', kind, 'rows', len(rows), 'completed within cap' if ok else 'EXCEEDED CAP OR MISSING')
    bad += 0 if ok else 1
sys.exit(1 if bad else 0)
PY
}
case "$MODE" in
  main)
    pilot ctl-m4-n8-kummer  8589934592  900 600  --kinds kummer --m 4 --t-min 2 --max-t-m4 2 || exit $?
    pilot ctl-m3-n15-kummer 8589934592 7500 7200 --kinds kummer --m 3 --t-min 5 --max-t-m3 5 || exit $?
    pilot ctl-m2-n10-kummer 8589934592  900 600  --kinds kummer --m 2 --t-min 5 --max-t 5    || exit $?
    v=$(controls_verdict); cat "$OUT/controls-verdict.txt" >&2
    if [ "$v" != 0 ]; then log "controls verdict failed; no m = 5 cell run"; exit 31; fi
    pilot kummer-m5-t2  8589934592 7500 7200 --kinds kummer  --m 5 --t-min 2 --max-t-m4 2 || exit $?
    pilot kummer-m5-t3  8589934592 7500 7200 --kinds kummer  --m 5 --t-min 3 --max-t-m4 3 || exit $?
    pilot isogeny-m5-t2 8589934592 7500 7200 --kinds isogeny --m 5 --t-min 2 --max-t-m4 2 || exit $?
    pilot isogeny-m5-t3 8589934592 7500 7200 --kinds isogeny --m 5 --t-min 3 --max-t-m4 3 || exit $?
    ;;
  t4)
    kb=$(awk '/MemTotal/ {print $2}' /proc/meminfo); echo "MemTotal kB $kb required 125829120" > "$OUT/host-memory.txt"
    [ "$kb" -ge 125829120 ] || { log "host memory below 120 GiB; t4 not run"; exit 32; }
    v=$(controls_verdict); [ "$v" = 0 ] || { log "controls verdict failed (resumed cells); t4 not run"; exit 31; }
    v=$(t3_cap_verdict); cat "$OUT/t3-cap-verdict.txt" >&2
    [ "$v" = 0 ] || { log "t3 capped or missing; t4 not run (protocol rule)"; exit 33; }
    pilot kummer-m5-t4  128849018880 86700 86400 --kinds kummer  --m 5 --t-min 4 --max-t-m4 4 || exit $?
    pilot isogeny-m5-t4 128849018880 86700 86400 --kinds isogeny --m 5 --t-min 4 --max-t-m4 4 || exit $?
    ;;
  *) log "unknown mode $MODE"; exit 2 ;;
esac
