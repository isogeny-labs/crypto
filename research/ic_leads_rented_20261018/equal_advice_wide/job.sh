#!/usr/bin/env bash
# Pod job for the frozen equal-advice session (research/equal_advice_wide_20261018/).
# Plumbing only: cgroup-aware preflight, host facts, build, then exactly the
# frozen run.sh. It never edits PROTOCOL.md or run.sh.
#   job.sh <OUT>            (boot.sh: job.sh /workspace/out, cwd /workspace/src)
# Environment (all optional; the defaults are the pod run):
#   RUNGS            rungs for run.sh (default "n73 n83"; local test: n41smoke)
#   MIN_CGROUP_GIB   refuse below this memory limit (default 56 = 48 + build headroom)
#   THREADS          default nproc
#   CARGO_TARGET_DIR default target
#   OUT_ROOT_BASE    parent of the UTC-stamped OUT_ROOT
#                    (default research/equal_advice_wide_20261018/runs)
set -u
OUT=${1:?usage: job.sh <OUT dir>}
mkdir -p "$OUT"
OUT=$(cd "$OUT" && pwd)
ROOT=$(cd "$(dirname "$0")/../../.." && pwd)
cd "$ROOT" || exit 1
# shellcheck disable=SC1091
[ -f "$HOME/.cargo/env" ] && . "$HOME/.cargo/env"

RUNGS=${RUNGS:-"n73 n83"}
MIN_CGROUP_GIB=${MIN_CGROUP_GIB:-56}
TARGET_DIR=${CARGO_TARGET_DIR:-target}
OUT_ROOT_BASE=${OUT_ROOT_BASE:-research/equal_advice_wide_20261018/runs}
THREADS=${THREADS:-$(nproc)}
STAMP=$(date -u +%Y%m%dT%H%M%SZ)
OUT_ROOT=$OUT_ROOT_BASE/$STAMP

status() { echo "$(date -u +%FT%TZ) $1" >>"$OUT/STATUS"; }

# ---- host facts and cgroup limit -------------------------------------------
memtotal_kib=$(awk '/^MemTotal:/ {print $2}' /proc/meminfo)
memtotal_bytes=$((memtotal_kib * 1024))
limit_src=none
limit_raw=max
for f in /sys/fs/cgroup/memory.max /sys/fs/cgroup/memory/memory.limit_in_bytes; do
  if [ -r "$f" ]; then limit_src=$f; limit_raw=$(cat "$f"); break; fi
done
# "max" (v2) or a number at or above MemTotal (v1 reports ~2^63 when unlimited)
# means no container limit: fall back to MemTotal and say so.
basis="cgroup"
case "$limit_raw" in
  ''|max) limit_bytes=$memtotal_bytes; basis="MemTotal (no cgroup limit: $limit_src=$limit_raw)" ;;
  *[!0-9]*) limit_bytes=$memtotal_bytes; basis="MemTotal (unparsable cgroup value)" ;;
  *) if [ "$limit_raw" -ge "$memtotal_bytes" ]; then
       limit_bytes=$memtotal_bytes; basis="MemTotal (cgroup limit $limit_raw >= MemTotal)"
     else limit_bytes=$limit_raw; fi ;;
esac
[ "$limit_src" = none ] && basis="MemTotal (no cgroup memory file)"
cpu_max=$(cat /sys/fs/cgroup/cpu.max 2>/dev/null || echo "n/a")
limit_gib=$((limit_bytes / 1073741824))

{
  echo "date: $(date -u +%FT%TZ)"
  echo "commit: $(git rev-parse HEAD 2>/dev/null)"
  echo "rustc: $(rustc --version 2>&1)"
  echo "kernel: $(uname -r)"
  echo "nproc: $(nproc)"
  echo "threads: $THREADS"
  echo "cgroup memory file: $limit_src raw=$limit_raw"
  echo "cgroup cpu.max: $cpu_max"
  echo "effective memory limit bytes: $limit_bytes ($limit_gib GiB), basis: $basis"
  echo "min required GiB: $MIN_CGROUP_GIB"
  grep MemTotal /proc/meminfo
  echo "rungs: $RUNGS"
  df -Pk "$OUT" "$ROOT" | sed 's/^/df: /'
  lscpu
} >"$OUT/ea-host.txt" 2>&1

status EA_PREFLIGHT
if [ "$limit_gib" -lt "$MIN_CGROUP_GIB" ]; then
  printf '{"status":"REJECTED_CGROUP","limit_bytes":%s,"limit_gib":%s,"limit_source":"%s","basis":"%s","min_gib":%s,"memtotal_kib":%s,"rungs":"%s"}\n' \
    "$limit_bytes" "$limit_gib" "$limit_src" "$basis" "$MIN_CGROUP_GIB" "$memtotal_kib" "$RUNGS" >"$OUT/ea-rejected.json"
  status REJECTED_CGROUP
  status "EA_FAILED rc=2"
  exit 2
fi
case "$cpu_max" in
  max*|n/a) ;;
  *) q=${cpu_max% *}; p=${cpu_max#* }
     if [ "$p" -gt 0 ] && [ "$q" -lt $((THREADS * p)) ]; then
       echo "WARNING: cpu.max $cpu_max allows fewer CPUs than THREADS=$THREADS" >>"$OUT/ea-host.txt"
       status "WARN_CPU_QUOTA $cpu_max"
     fi ;;
esac
status "EA_PREFLIGHT_OK limit=${limit_gib}GiB basis=${basis%% *}"

# ---- build ------------------------------------------------------------------
export CARGO_TARGET_DIR=$TARGET_DIR
status EA_BUILD
EXAMPLES=(--example ic_equal_advice_wide --example ic_equal_advice_claims --example koblitz_orbit_dlp_fast)
case " $RUNGS " in *" n41smoke "*) EXAMPLES+=(--example koblitz_orbit_dlp_fast_online) ;; esac
if ! cargo build --release "${EXAMPLES[@]}" >"$OUT/ea-build.log" 2>&1; then
  status EA_BUILD_FAILED
  status "EA_FAILED rc=3"
  exit 3
fi
# ecbench runs the shipped claim checker inside run.sh when present. Optional.
if cargo build --release --bin ecbench >"$OUT/ea-build-ecbench.log" 2>&1; then
  status EA_BUILD_ECBENCH_OK
else
  status EA_BUILD_ECBENCH_FAILED_CONTINUING
fi
status EA_BUILD_DONE

# ---- frozen session ----------------------------------------------------------
mkdir -p "$OUT_ROOT"
case "$OUT_ROOT" in /*) ln -sfn "$OUT_ROOT" "$OUT/runs" ;; *) ln -sfn "$ROOT/$OUT_ROOT" "$OUT/runs" ;; esac
status "EA_SESSION_START rungs=[$RUNGS] threads=$THREADS out_root=$OUT_ROOT"
# shellcheck disable=SC2086  # RUNGS is a word list on purpose
THREADS=$THREADS OUT_ROOT=$OUT_ROOT \
  research/equal_advice_wide_20261018/run.sh $RUNGS >"$OUT/ea-run.stdout" 2>"$OUT/ea-run.stderr" &
rp=$!
# Mirror run.sh's own phase lines ("[ts] rung n73: preflight", "start BL arm",
# "IC process 1", ...) into STATUS while it runs.
( tail -n +1 -F --pid="$rp" "$OUT/ea-run.stderr" 2>/dev/null | grep --line-buffered '^\[' |
    while IFS= read -r l; do echo "$(date -u +%FT%TZ) EA_PHASE ${l#*] }" >>"$OUT/STATUS"; done ) &
mon=$!
wait "$rp"
rc=$?
wait "$mon" 2>/dev/null
missing=0
for r in $RUNGS; do
  if [ -f "$OUT_ROOT/$r/assembled/decision.json" ]; then
    status "EA_DECISION $r $(tr -d '\n ' <"$OUT_ROOT/$r/assembled/decision.json" | grep -o '"rung_verdict":"[^"]*"' | head -1)"
  else
    missing=1
  fi
done
# The frozen run.sh's EXIT trap runs `kill -- -$$` on its own process group,
# which includes the leader itself, so a successful run.sh exits 143 (SIGTERM).
# Treat 143 as success only when every rung wrote its decision.json.
if [ "$rc" -eq 143 ] && [ "$missing" -eq 0 ]; then
  status "EA_NOTE run.sh rc=143 (self-kill by its EXIT trap) with all decision.json present: treated as success"
  rc=0
fi
if [ "$rc" -eq 0 ] && [ "$missing" -eq 1 ]; then rc=4; fi
if [ "$rc" -eq 0 ]; then status EA_DONE; else status "EA_FAILED rc=$rc"; fi
exit "$rc"
