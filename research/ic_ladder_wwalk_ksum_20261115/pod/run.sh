#!/bin/bash
# Run the whole ladder panel on a plain Linux CPU box (no Docker): train -> select -> test per size, the k = 32
# positive control, replays, analysis, and SHA256SUMS over one output directory.
#   pod/run.sh <study dir (this directory, copied to the pod)> [workdir=/work]
# Needs: workdir/bin/ecbench (pod/build.sh), node >= 18, taskset (optional), jq (optional).  Resumable: a finished
# session (session.json present) is skipped.  Environment: SIZES="36 40 44 48" (default), VERIFY48=60 (sampled replay
# count at 48 bits; replay-all there costs another full run), TOY=1 (the local smoke test; specs-toy/, never evidence).
set -u
STUDY=$(cd "${1:?study dir}" && pwd); WORK=${2:-/work}; BIN=$WORK/bin/ecbench
SIZES=${SIZES:-"36 40 44 48"}; VERIFY48=${VERIFY48:-60}
TOY=${TOY:-0}; export TOY
if [ "$TOY" = 1 ]; then SPECS=$STUDY/specs-toy; PARAMS=$STUDY/params-toy; OUT=${OUT:-$WORK/out-toy}; else SPECS=$STUDY/specs; PARAMS=$STUDY/params; OUT=${OUT:-$WORK/out}; fi
export RAYON_NUM_THREADS=1 SIZES
mkdir -p "$OUT"/{train,test,control,logs}; cd "$STUDY" || exit 1
[ -x "$BIN" ] || { echo "no $BIN: run pod/build.sh first"; exit 1; }
LOG=$OUT/runlog.tsv
{ echo "start $(date -u +%FT%TZ) load $(cut -d' ' -f1-3 /proc/loadavg) nproc $(nproc) mem_kib $(awk '/MemTotal/{print $2}' /proc/meminfo)"; sha256sum "$BIN"; lscpu | head -20; } >> "$OUT/host.txt" 2>&1
cp "$WORK/bin/rustc.txt" "$WORK/bin/BINARY.sha256" "$OUT/" 2>/dev/null || true

# Amendment A1 (2026-11-15, before any data): lane i is pinned to the i-th CPU this process may use (Cpus_allowed_list),
# not to CPU id i; container CPU ids need not start at 0 (RunPod attempt 1 failed every session with taskset EINVAL).
ALLOWED=($(awk '/Cpus_allowed_list/{print $2}' /proc/self/status | tr ',' '\n' | while IFS=- read -r a b; do seq "$a" "${b:-$a}"; done))
pin() { local i=$1; shift; local c=${ALLOWED[$i]:-}; if [ -n "$c" ] && command -v taskset >/dev/null; then taskset -c "$c" "$@"; else "$@"; fi; }
ecrun() { # ecrun CPU SPEC OUTDIR   (skips finished sessions)
  local cpu=$1 spec=$2 dir=$3 s e rc
  [ -f "$dir/session.json" ] && return 0; rm -rf "$dir"
  s=$(date +%s); pin "$cpu" "$BIN" run --spec "$spec" --out "$dir" --cpus inherit --allow-busy --lock "/tmp/crypto-bench-ladder-cpu$cpu.lock" --wait > "$dir.log" 2>&1; rc=$?; e=$(date +%s)
  printf '%s\trc=%s\t%ss\tload=%s\n' "$(basename "$dir")" "$rc" $((e - s)) "$(cut -d' ' -f1 /proc/loadavg)" >> "$LOG"
  return $rc
}
size_job() { # size_job CPU BITS : train, select, test, verify
  local cpu=$1 b=$2 rc
  SIZES=$b node make_specs.mjs train >/dev/null || return 1
  ecrun "$cpu" "$SPECS/train/T$b.json" "$OUT/train/T$b" || { echo "T$b failed"; return 1; }
  SIZES=$b node select.mjs "$BIN" "$OUT/train" "$PARAMS" >> "$OUT/logs/select.txt" 2>&1 || { echo "select $b failed"; return 1; }
  SIZES=$b node make_specs.mjs test >/dev/null || return 1
  ecrun "$cpu" "$SPECS/test/P$b.json" "$OUT/test/P$b" || { echo "P$b failed"; return 1; }
  if [ "$b" -ge 48 ] && [ "$TOY" != 1 ]; then pin "$cpu" "$BIN" verify --dir "$OUT/test/P$b" --replay "$VERIFY48" --exit-code --out "$OUT/test/P$b.verify.json" > /dev/null 2>&1
  else pin "$cpu" "$BIN" verify --dir "$OUT/test/P$b" --replay-all --exit-code --out "$OUT/test/P$b.verify.json" > /dev/null 2>&1; fi
  echo "P$b verify rc=$?" >> "$OUT/verify.txt"
}
control_job() { # the autolab k = 32 curve, positive control for the wwalk recipe
  local cpu=$1; node make_specs.mjs control >/dev/null || return 1
  ecrun "$cpu" "$SPECS/control/PC32.json" "$OUT/control/PC32" || return 1
  pin "$cpu" "$BIN" verify --dir "$OUT/control/PC32" --replay-all --exit-code --out "$OUT/control/PC32.verify.json" > /dev/null 2>&1
  echo "PC32 verify rc=$?" >> "$OUT/verify.txt"
}

# Lanes, one CPU each: the largest size alone; the rest queued behind each other on as many CPUs as there are.
NC=$(nproc); biggest=$(echo $SIZES | tr ' ' '\n' | sort -n | tail -1); rest=$(echo $SIZES | tr ' ' '\n' | sort -nr | grep -vx "$biggest" | tr '\n' ' ')
lane() { local cpu=$1; shift; for j in "$@"; do case $j in control) control_job "$cpu";; *) size_job "$cpu" "$j";; esac; done; }
jobs_rest="$rest control"
if [ "$NC" -ge 3 ]; then
  set -- $jobs_rest; first=$1; shift
  lane 0 "$biggest" & lane 1 "$first" & lane 2 "$@" & wait
elif [ "$NC" -eq 2 ]; then lane 0 "$biggest" & lane 1 $jobs_rest & wait
else lane 0 $jobs_rest "$biggest"; fi
echo "sessions done $(date -u +%FT%TZ)" >> "$LOG"

# Analysis and the digest list.
node analysis/evaluate.mjs "$BIN" "$OUT" "$OUT/analysis" > "$OUT/logs/evaluate.txt" 2>&1
node analysis/memory.mjs "$OUT" > "$OUT/analysis/memory.txt" 2>&1 || true
cp -r "$PARAMS" "$OUT/params" 2>/dev/null; cp -r "$SPECS" "$OUT/specs" 2>/dev/null
(cd "$OUT" && find . -type f ! -name SHA256SUMS -print0 | sort -z | xargs -0 sha256sum > SHA256SUMS)
echo "done $(date -u +%FT%TZ); outputs and SHA256SUMS in $OUT" | tee -a "$LOG"
