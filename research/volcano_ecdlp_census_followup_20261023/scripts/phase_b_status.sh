#!/bin/bash
# Prints one JSON line about the Phase B campaign and exits 0. Reads only; changes nothing.
# {"sessions_done":N,"driver_alive":bool,"guard_fired":bool,"last_spec":"...","disk_free_mb":N,"last_audit_ok":bool}
#   sessions_done  distinct specs with a progress line whose status is "done"
#   driver_alive   runs/driver.pid names a live process whose command line contains run_phase_b.sh
#   guard_fired    the last exit token in runs/phase_b.log is GUARD_* or BINARY_SHA_MISMATCH, and the driver is not alive
#   last_spec      spec of the last session line in progress.jsonl ("" when none)
#   last_audit_ok  audit_ok of that line (false when none)
STUDY=${STUDY:-/home/user/crypto/research/volcano_ecdlp_census_followup_20261023}
cd "$STUDY" || exit 2
PROG=runs/progress.jsonl
LOG=runs/phase_b.log
prog() { [ -f $PROG ] && jq -R -c 'try fromjson catch empty' $PROG; }
alive=false
if [ -f runs/driver.pid ]; then
  pid=$(cat runs/driver.pid 2>/dev/null)
  case "$pid" in ''|*[!0-9]*) ;; *)
    if kill -0 "$pid" 2>/dev/null && tr '\0' ' ' < /proc/$pid/cmdline 2>/dev/null | grep -q run_phase_b.sh; then alive=true; fi;;
  esac
fi
guard=false
if [ "$alive" = false ] && [ -f $LOG ]; then
  tok=$(grep -E '^(GUARD_[A-Z]+|BINARY_SHA_MISMATCH|ALLDONE|STOPPED|DRIVER_LOCKED)' $LOG | tail -1)
  case "$tok" in GUARD_*|BINARY_SHA_MISMATCH*) guard=true;; esac
fi
free=$(df -Pm "$STUDY" | awk 'NR==2{print $4}')
prog | jq -s -c --argjson alive $alive --argjson guard $guard --argjson free "$free" '
  [.[]|select(.spec!=null and .status!=null)] as $s
  | {sessions_done:([$s[]|select(.status=="done")|.spec]|unique|length),
     driver_alive:$alive, guard_fired:$guard,
     last_spec:(($s|last|.spec) // ""),
     disk_free_mb:$free,
     last_audit_ok:((($s|last|.audit_ok) // false) == true)}'
