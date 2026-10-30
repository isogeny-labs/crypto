#!/usr/bin/env bash
# Stages of the fall-degree data request (protocols FD-BINARY-CLOSURE-EXT and
# FD-F4-STEP-DEGREE, frozen in protocols/*/protocol.json). One process per
# instance, JSON lines into runs/<stage>/*.jsonl, a host record per stage.
#
#   run.sh BIN host
#   run.sh BIN calibrate     # every recorded m=2 instance, closure D=3..4 (positive controls + A/A against lfdclose2's record)
#   run.sh BIN closure-d4    # new draws: R n=41..51 x3, P n=37,39,47,49,51 x2; planted controls n=33..51
#   run.sh BIN closure-d5    # the 8 certified instances at n=33..37, closure at D=5 only
#   run.sh BIN f4            # F4 step degree: every recorded instance + P n=34..48 x2
#
# BIN is target/release/examples/fd_m2_degrees. Seeds per cell are read from
# seeds.tsv (written at freeze from the protocol hash); JOBS bounds the
# number of concurrent instances (default: nproc for D<=4, 1 for D=5).
set -euo pipefail
export F4_F2_MAX_ROWS=${F4_F2_MAX_ROWS:-2000000000} F4_F2_MAX_COLS=${F4_F2_MAX_COLS:-2000000000}
here=$(cd "$(dirname "$0")" && pwd)
bin=$1
stage=$2
out="$here/runs/$stage"
mkdir -p "$out"
rec="$here/inputs/instances_m2_recorded.json"
seed_of() { awk -v c="$1" '$1 == c { print $2 }' "$here/seeds.tsv"; }
gnu_time() { /usr/bin/time -v "$@"; }

host() {
  {
    echo "date=$(date -u +%FT%TZ)"
    echo "runner=${RUNNER_NAME:-local} run=${GITHUB_RUN_ID:-}/${GITHUB_RUN_ATTEMPT:-} commit=${GITHUB_SHA:-$(git -C "$here" rev-parse HEAD)}"
    echo "binary_sha256=$(sha256sum "$bin" | cut -d' ' -f1)"
    echo "nproc=$(nproc)"
    grep -m1 'model name' /proc/cpuinfo || true
    echo "memory_max=$(cat /sys/fs/cgroup/memory.max 2>/dev/null || echo na)"
    uname -r
    curl -s -m 2 http://169.254.169.254/latest/meta-data/instance-type 2>/dev/null && echo || true
  } > "$out/host.txt"
}

one() { # name args...
  local name=$1; shift
  [ -s "$out/$name.jsonl" ] && return 0          # resume: finished instances are kept
  gnu_time "$bin" "$@" > "$out/$name.part" 2> "$out/$name.err" && mv "$out/$name.part" "$out/$name.jsonl"
}
export -f one gnu_time
export bin out

names() { grep -o '"name": *"[A-Z][0-9]*[a-z]"' "$rec" | sed 's/.*"\([A-Z][0-9]*[a-z]\)"/\1/'; }

case "$stage" in
  host) host ;;
  calibrate)
    host
    names | xargs -P "${JOBS:-4}" -I{} bash -c 'one "$1" --instances "'"$rec"'" --names "$1" --closure --d-min 3 --d-max 4 --max-cols 400000' _ {}
    for nm in R21a R25a R29a R33a; do   # A/A: same closure on 4 threads
      RAYON_NUM_THREADS=4 one "aa-$nm" --instances "$rec" --names "$nm" --closure --d-min 3 --d-max 4 --max-cols 400000
    done
    ;;
  closure-d4)
    host
    {
      for n in 41 43 45 47 49 51; do echo "R${n} --draw $n:R:3 --seed $(seed_of "R$n")"; done
      for n in 37 39 47 49 51; do echo "P${n} --draw $n:P:2 --seed $(seed_of "P$n")"; done
      for n in 33 35 37 39 41 43 45 47 49 51; do echo "sat${n} --draw $n:R:1 --seed $(seed_of "sat$n") --planted"; done
    } > "$out/plan.txt"
    xargs -P "${JOBS:-2}" -L1 bash -c 'one "$1" "${@:2}" --closure --d-min 3 --d-max 4 --max-cols 400000' _ < "$out/plan.txt"
    ;;
  closure-d5)
    host
    for nm in T33a T33b R35a R35b U35a U35b R37a R37b; do
      JOBS=1 one "$nm" --instances "$rec" --names "$nm" --closure --d-min 5 --d-max 5 --max-cols 600000
    done
    ;;
  f4)
    host
    names | xargs -P "${JOBS:-4}" -I{} bash -c 'one "$1" --instances "'"$rec"'" --names "$1" --f4 --f4-budget 21600' _ {}
    for n in $(seq 34 48); do
      one "P${n}_f4" --draw "$n:P:2" --seed "$(seed_of "f4-P$n")" --f4 --f4-budget 86400
    done
    ;;
  *) echo "unknown stage $stage" >&2; exit 2 ;;
esac
