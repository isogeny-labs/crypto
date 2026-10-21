#!/usr/bin/env bash
# Measurement driver for PROTOCOL_AMENDMENT_A1_perf.md: K = (instructions per
# dense-closure column) / (instructions per rho step) with perf_event_open
# interval counters, on a host with a hardware PMU and perf_event_paranoid <= 2.
#
#   research/k_conversion_20261018/perf/run_perf.sh [--dry]
#
# Environment:
#   PAR=N         jobs run at once (default 1: sequential, the reportable mode).
#                 Each concurrent job is pinned to its own CPU; PAR is recorded.
#   REPS=N        repeats of every job (default 3: the A/A control)
#   RUNS_ROOT=D   where runs/<host>-<UTC>/ goes (default: perf/runs; --dry: perf/test_local)
#   SKIP_DEG4=1   skip the degree-4 cells (degree 3 / rho / controls only)
#
# --dry runs a tiny subset with KPERF_SOFTWARE=1 (kernel task-clock and
# page-faults instead of hardware events): it tests the plumbing and its
# records are labelled "selftest":true; it is never a measurement.
#
# Writes <RUNS_ROOT>/<host>-<UTC>/{host.txt,raw.jsonl,null_whole.jsonl,parts/}; pipe the console output to a file with tee if wanted.
# Exit 0 measured, 3 the host cannot count (an "unsupported" record is written;
# nothing is estimated), 4 a job failed.
set -uo pipefail

DRY=0
[ "${1:-}" = "--dry" ] && DRY=1
HERE=$(cd "$(dirname "$0")" && pwd)
REPO=$(cd "$HERE/../../.." && pwd)
cd "$REPO"
BIN=${BIN:-$REPO/target/release/examples/k_conversion_perf}
PAR=${PAR:-1}
REPS=${REPS:-3}
export RAYON_NUM_THREADS=1
if [ "$DRY" = 1 ]; then
    export KPERF_SOFTWARE=1
    RUNS_ROOT=${RUNS_ROOT:-$HERE/test_local}
    REPS=${REPS_DRY:-2}
else
    RUNS_ROOT=${RUNS_ROOT:-$HERE/runs}
fi

[ -x "$BIN" ] || cargo build --release --example k_conversion_perf || exit 2
HOST=$(hostname -s 2>/dev/null | tr -c 'A-Za-z0-9\n' '-' || echo host)
STAMP=$(date -u +%Y%m%dT%H%M%SZ)
RUN=$RUNS_ROOT/${HOST}-${STAMP}
mkdir -p "$RUN/parts"
RAW=$RUN/raw.jsonl

je() { sed 's/\\/\\\\/g; s/"/\\"/g' | tr '\n\t' '  '; } # JSON-escape stdin to one line
cmd_or() { "$@" 2>&1 || echo "unavailable: $1"; }

# ---- host facts -------------------------------------------------------------
COMMIT=$(git rev-parse HEAD 2>/dev/null || echo unknown)
DIRTY=$(git status --porcelain 2>/dev/null | wc -l)
SHA=$(sha256sum "$BIN" | cut -d' ' -f1)
PARANOID=$(cat /proc/sys/kernel/perf_event_paranoid 2>/dev/null || echo unknown)
{
    echo "== date"; date -u
    echo "== uname"; uname -a
    echo "== lscpu"; cmd_or lscpu
    echo "== perf_event_paranoid"; echo "$PARANOID"
    echo "== event sources"; ls /sys/bus/event_source/devices 2>&1
    echo "== governor"; cat /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor 2>&1
    echo "== no_turbo"; cat /sys/devices/system/cpu/intel_pstate/no_turbo 2>&1
    echo "== meminfo"; grep -E 'MemTotal|MemAvailable|SwapTotal' /proc/meminfo
    echo "== rustc"; cmd_or rustc --version
    echo "== commit"; echo "$COMMIT dirty_files=$DIRTY"
    echo "== binary sha256"; echo "$SHA  $BIN"
    echo "== load"; cat /proc/loadavg
} > "$RUN/host.txt" 2>&1
host_rec() { # status
    printf '{"kind":"host","status":"%s","dry":%s,"host":"%s","utc":"%s","uname":"%s","cpu_model":"%s","logical_cpus":%s,"mem_total_kb":%s,"perf_event_paranoid":"%s","rustc":"%s","commit":"%s","dirty_files":%s,"binary_sha256":"%s","PAR":%s,"REPS":%s,"rayon_threads":1,"taskset":%s}\n' \
        "$1" "$DRY" "$HOST" "$STAMP" "$(uname -a | je)" \
        "$(grep -m1 'model name' /proc/cpuinfo | cut -d: -f2- | je)" "$(nproc)" \
        "$(awk '/MemTotal/{print $2}' /proc/meminfo)" "$PARANOID" "$(rustc --version 2>/dev/null | je)" \
        "$COMMIT" "$DIRTY" "$SHA" "$PAR" "$REPS" "$(command -v taskset >/dev/null && echo true || echo false)"
}

# ---- probe ------------------------------------------------------------------
PROBE=$("$BIN" probe); PRC=$?
echo "probe (exit $PRC): $PROBE"
if [ "$PRC" = 3 ]; then
    { host_rec unsupported; echo "$PROBE"; } > "$RAW"
    echo "run_perf: this host cannot count hardware events; nothing was measured (see $RAW)"
    exit 3
elif [ "$PRC" != 0 ]; then
    echo "run_perf: probe failed with exit $PRC"; exit 2
fi
[ "$DRY" = 1 ] || case "$PROBE" in *'"selftest":true'*) echo "run_perf: KPERF_SOFTWARE is set in the environment; unset it for a measurement"; exit 2;; esac
host_rec ok > "$RAW"

# ---- job list: kind|label|file|draw|degree  (rho: kind|label|field|seed|-) ----
D=research/dreg_closure_driver_20261016/draws
S=research/dreg_closure_scale_20261016/draws
cells=(
    "m3-16-6|$S/m3-16-6.jsonl|0 1 2 3"
    "m4-12-3|$S/m4-12-3.jsonl|1 2 3 4"
    "m4-11-5|$D/m4-11-5.jsonl|8 67 107 143"
    "m4-12-5|$D/m4-12-5.jsonl|24 26 47 52"
)
JOBS=()
if [ "$DRY" = 1 ]; then
    JOBS+=("rho|n61|n61|1|-" "closure|m3-16-6|$S/m3-16-6.jsonl|0|3")
else
    for s in 1 2 3; do JOBS+=("rho|n83|n83|$s|-" "rho|n61|n61|$s|-"); done
    for c in "${cells[@]}"; do
        IFS='|' read -r lab file draws <<< "$c"
        for d in $draws; do JOBS+=("closure|$lab|$file|$d|3"); done
    done
    if [ "${SKIP_DEG4:-0}" != 1 ]; then
        for c in "${cells[@]}"; do
            IFS='|' read -r lab file draws <<< "$c"
            for d in $draws; do JOBS+=("closure|$lab|$file|$d|4"); done
        done
    fi
fi

# ---- CPUs -------------------------------------------------------------------
CPUS=()
if command -v taskset >/dev/null; then
    LIST=$(taskset -cp $$ | sed 's/.*: //')
    for part in ${LIST//,/ }; do
        if [[ $part == *-* ]]; then for ((i=${part%-*}; i<=${part#*-}; i++)); do CPUS+=("$i"); done
        else CPUS+=("$part"); fi
    done
fi
echo "allowed CPUs: ${CPUS[*]:-none (no taskset)}; PAR=$PAR; ${#JOBS[@]} jobs x $REPS repeats"
if [ "$PAR" -gt 1 ] && [ "${#CPUS[@]}" -gt 0 ] && [ "$PAR" -gt "${#CPUS[@]}" ]; then
    echo "run_perf: PAR=$PAR exceeds ${#CPUS[@]} allowed CPUs"; exit 2
fi
# Sequential runs avoid CPU 0 (interrupt home) when there is a choice.
SLOT_CPU=()
for ((i=0; i<PAR; i++)); do
    if [ "${#CPUS[@]}" -gt 0 ]; then SLOT_CPU[i]=${CPUS[$(( (${#CPUS[@]} - 1 - i) % ${#CPUS[@]} ))]}; else SLOT_CPU[i]=none; fi
done

# ---- run one job ------------------------------------------------------------
run_job() { # idx rep slot spec
    local idx=$1 rep=$2 slot=$3 spec=$4 kind lab arg1 arg2 deg cpu pin=() out rc line tag
    IFS='|' read -r kind lab arg1 arg2 deg <<< "$spec"
    cpu=${SLOT_CPU[slot]}
    [ "$cpu" = none ] || pin=(taskset -c "$cpu")
    if [ "$kind" = closure ]; then tag="closure-$lab-d$arg2-deg$deg-r$rep"; args=(closure "$arg1" "$arg2" "$deg")
    else tag="rho-$lab-s$arg2-r$rep"; args=(rho "$arg1" "$arg2"); fi
    out=$("${pin[@]}" "$BIN" "${args[@]}" 2> "$RUN/parts/$tag.err"); rc=$?
    line=$(printf '%s\n' "$out" | grep '^{' | tail -1)
    if [ -z "$line" ]; then line='{"status":"failed"}'; fi
    printf '{"label":"%s","tag":"%s","rep":%s,"par":%s,"cpu":"%s","exit":%s,%s\n' \
        "$lab" "$tag" "$rep" "$PAR" "$cpu" "$rc" "${line#\{}" > "$RUN/parts/$(printf '%04d' "$idx")-$tag.json"
    echo "done $tag exit=$rc cpu=$cpu"
}

# ---- null control: whole process under perf stat ------------------------------
whole() { # label file draw degree
    local pf=(--repeat 1 --label "whole-$1-d$3-deg$4" --out "$RUN/null_whole.jsonl")
    [ "$DRY" = 1 ] && pf+=(--selftest)
    tools/perf_ipc.sh "${pf[@]}" -- env RAYON_NUM_THREADS=1 "$BIN" closure "$2" "$3" "$4" > /dev/null
    echo "null control $1 draw $3 degree $4: perf_ipc exit $?"
}

# ---- main loop: repeats outermost so slow drift shows in the A/A -------------
declare -a PIDS=()
FAIL=0
idx=0
for ((rep=1; rep<=REPS; rep++)); do
    for spec in "${JOBS[@]}"; do
        idx=$((idx+1))
        while :; do
            slot=-1
            for ((s=0; s<PAR; s++)); do
                if [ -z "${PIDS[s]:-}" ] || ! kill -0 "${PIDS[s]}" 2>/dev/null; then slot=$s; break; fi
            done
            [ "$slot" -ge 0 ] && break
            wait -n 2>/dev/null || true
        done
        if [ "$PAR" = 1 ]; then run_job "$idx" "$rep" 0 "$spec"
        else run_job "$idx" "$rep" "$slot" "$spec" & PIDS[slot]=$!; fi
    done
done
wait
cat "$RUN"/parts/*.json >> "$RAW"
grep -q '"exit":[1-9]' "$RAW" && FAIL=1

if [ "$DRY" = 1 ]; then
    whole m3-16-6 "$S/m3-16-6.jsonl" 0 3
else
    whole m3-16-6 "$S/m3-16-6.jsonl" 0 3
    [ "${SKIP_DEG4:-0}" = 1 ] || whole m3-16-6 "$S/m3-16-6.jsonl" 0 4
fi

echo "run_perf: wrote $RAW ($(wc -l < "$RAW") records); analyse with: python3 $HERE/analyze.py $RUN"
[ "$FAIL" = 0 ] || { echo "run_perf: some jobs failed (exit != 0), see $RAW"; exit 4; }
