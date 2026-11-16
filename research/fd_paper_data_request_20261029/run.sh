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
# After a stage, `python3 <crypto-autoresearcher>/tools/experiment_protocol.py from-rows
# runs/<stage>/executions.jsonl --protocol protocols/<p>/protocol.json --out
# runs/<stage>/export.json --export-id fd-<stage>-<run>` writes the unified export
# (the protocol tool lives in crypto-autoresearcher and is not copied here), then
# `check-export --verify-files`.
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

# One execution row per attempt in runs/<stage>/executions.jsonl, the input of
# `experiment_protocol.py from-rows` (experiment.export/v1): arm, instance,
# seed, status (succeeded | failed | interrupted), wall seconds, exit code.
row() { # name status wall exit
  local arm=closureA seed
  case "$stage:$1" in
    calibrate:aa-*) arm=closureA-aa ;;
    calibrate:*) arm=pos-known-refuting ;;
    closure-d4:sat*) arm=neg-satisfiable ;;
    f4:*) arm=f4 ;;
  esac
  seed=$(awk -v c="${SEED_CELL:-$1}" '$1 == c { print $2 }' "$here/seeds.tsv")
  printf '{"run_id":"%s/%s/%s","arm":"%s","instance_id":"%s","seed":%s,"replicate":0,"status":"%s","wall_seconds":%s,"exit_code":%s,"tier":"none","verification":"not_run","runner":"%s","commit":"%s"}\n' \
    "$stage" "$1" "${GITHUB_RUN_ID:-local}-${GITHUB_RUN_ATTEMPT:-0}" "$arm" "$1" "${seed:-null}" "$2" "$3" "$4" \
    "${RUNNER_NAME:-local}" "${GITHUB_SHA:-unknown}" >> "$out/executions.jsonl"
}
one() { # name args...
  local name=$1; shift
  [ -s "$out/$name.jsonl" ] && return 0          # resume: finished instances are kept
  local t0=$SECONDS rc=0
  trap 'row "$name" interrupted $((SECONDS - t0)) 143; exit 143' TERM INT
  gnu_time "$bin" "$@" > "$out/$name.part" 2> "$out/$name.err" || rc=$?
  trap - TERM INT
  if [ "$rc" -eq 0 ]; then
    mv "$out/$name.part" "$out/$name.jsonl"
    row "$name" succeeded $((SECONDS - t0)) 0
  else
    row "$name" failed $((SECONDS - t0)) "$rc"
  fi
  return "$rc"
}
export -f one row gnu_time
export bin out here stage

names() { grep -o '"name": *"[A-Z][0-9]*[a-z]"' "$rec" | sed 's/.*"\([A-Z][0-9]*[a-z]\)"/\1/'; }

case "$stage" in
  host) host ;;
  calibrate)
    host
    names | xargs -P "${JOBS:-4}" -I{} bash -c 'one "$1" --instances "'"$rec"'" --names "$1" --closure --d-min 3 --d-max 4 --max-cols 400000' _ {} || true
    for nm in R21a R25a R29a R33a; do   # A/A: same closure on 4 threads
      RAYON_NUM_THREADS=4 one "aa-$nm" --instances "$rec" --names "$nm" --closure --d-min 3 --d-max 4 --max-cols 400000 || true
    done
    ;;
  closure-d4)
    host
    {
      for n in 41 43 45 47 49 51; do echo "R${n} --draw $n:R:3 --seed $(seed_of "R$n")"; done
      for n in 37 39 47 49 51; do echo "P${n} --draw $n:P:2 --seed $(seed_of "P$n")"; done
      for n in 33 35 37 39 41 43 45 47 49 51; do echo "sat${n} --draw $n:R:1 --seed $(seed_of "sat$n") --planted"; done
    } > "$out/plan.txt"
    xargs -P "${JOBS:-2}" -L1 bash -c 'one "$1" "${@:2}" --closure --d-min 3 --d-max 4 --max-cols 400000' _ < "$out/plan.txt" || true
    ;;
  closure-d5)
    host
    for nm in T33a T33b R35a R35b U35a U35b R37a R37b; do
      JOBS=1 one "$nm" --instances "$rec" --names "$nm" --closure --d-min 5 --d-max 5 --max-cols 600000 || true
    done
    ;;
  f4)
    host
    names | xargs -P "${JOBS:-4}" -I{} bash -c 'one "$1" --instances "'"$rec"'" --names "$1" --f4 --f4-budget 21600' _ {} || true
    for n in $(seq 34 48); do
      SEED_CELL="f4-P$n" one "P${n}_f4" --draw "$n:P:2" --seed "$(seed_of "f4-P$n")" --f4 --f4-budget 86400 || true
    done
    ;;
  *) echo "unknown stage $stage" >&2; exit 2 ;;
esac
