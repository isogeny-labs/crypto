#!/usr/bin/env bash
# Runs on one RunPod CPU pod (image rust:1.97.0-bookworm). One pod per seed.
# PROTOCOL.md + PROTOCOL_AMENDMENT_1.md govern; this file only executes them.
# Inputs (pod env): SEED, SRC_URL, SRC_SHA256, DEADLINE_EPOCH, and presigned
# S3 PUT URLs PUT_GRID PUT_AA PUT_WIDE PUT_RUNS PUT_HOST PUT_LOG PUT_DONE.
set -uo pipefail
W=${WORK:-/work}
# Smoke-test overrides only (unset on the pods): KMIN KMAX SECOND_K BIN_OVERRIDE.
KMIN=${KMIN:-12}; KMAX=${KMAX:-20}; SECOND_K=${SECOND_K:-16}; SD=$((1 << SECOND_K))
if [ -e $W/raw/runs.jsonl ]; then RESTARTED=1; else RESTARTED=0; fi
mkdir -p $W/raw; cd $W
LOG=$W/raw/pod.log
exec > >(tee -a "$LOG") 2>&1
up() { curl -sS -f -X PUT --retry 5 --retry-delay 5 -T "$1" "$2" >/dev/null || echo "upload failed: $1"; }
sync_all() {
  for f in grid aa wide_coeffs runs; do touch raw/$f.jsonl; done
  up raw/grid.jsonl "$PUT_GRID"; up raw/aa.jsonl "$PUT_AA"; up raw/wide_coeffs.jsonl "$PUT_WIDE"
  up raw/runs.jsonl "$PUT_RUNS"; up "$LOG" "$PUT_LOG"
}
finish() {  # $1 = status
  echo "{\"status\":\"$1\",\"seed\":$SEED,\"end_utc\":\"$(date -u +%FT%TZ)\"}" > raw/done.json
  sync_all; up raw/done.json "$PUT_DONE"
  # Pods carry a pod-scoped key; remove self so billing stops. Fallback: idle.
  if [ -n "${RUNPOD_POD_ID:-}" ] && [ -n "${RUNPOD_API_KEY:-}" ]; then
    curl -sS https://api.runpod.io/graphql -H "Authorization: Bearer $RUNPOD_API_KEY" -H 'content-type: application/json' \
      -d "{\"query\":\"mutation { podTerminate(input: {podId: \\\"$RUNPOD_POD_ID\\\"}) }\"}" || true
  fi
  [ -n "${BIN_OVERRIDE:-}" ] && exit 0
  sleep infinity
}
echo "start $(date -u +%FT%TZ) seed=$SEED"
# A container restart must not overwrite the uploaded data with a fresh run.
[ $RESTARTED -eq 1 ] && finish container_restarted
curl -sS -f --retry 5 -o src.tar.gz "$SRC_URL" || finish src_download_failed
echo "$SRC_SHA256  src.tar.gz" | sha256sum -c - || finish src_sha_mismatch
mkdir src && tar -xzf src.tar.gz -C src && cd src
{
  echo "{\"seed\":$SEED,\"src_sha256\":\"$SRC_SHA256\",\"pod_id\":\"${RUNPOD_POD_ID:-}\",\"start_utc\":\"$(date -u +%FT%TZ)\","
  echo "\"nproc\":$(nproc),\"rustc\":\"$(rustc --version)\",\"kernel\":\"$(uname -r)\","
  echo "\"cpu_model\":\"$(grep -m1 'model name' /proc/cpuinfo | cut -d: -f2 | xargs)\","
  echo "\"mem_total_kb\":$(awk '/MemTotal/{print $2}' /proc/meminfo),"
  echo "\"cgroup_cpu_max\":\"$(cat /sys/fs/cgroup/cpu.max 2>/dev/null)\",\"cgroup_mem_max\":\"$(cat /sys/fs/cgroup/memory.max 2>/dev/null)\","
} > $W/raw/host.part
if [ -n "${BIN_OVERRIDE:-}" ]; then B=$BIN_OVERRIDE; else
  CARGO_INCREMENTAL=0 cargo build --release --example la_engine_bench || finish build_failed
  B=$PWD/target/release/examples/la_engine_bench
fi
echo "\"binary_sha256\":\"$(sha256sum $B | cut -d' ' -f1)\",\"build_done_utc\":\"$(date -u +%FT%TZ)\"}" >> $W/raw/host.part
mv $W/raw/host.part $W/raw/host.json; up $W/raw/host.json "$PUT_HOST"
cd $W

declare -A STOPPED
N=0
# run_one <outfile> <phase> <arm> <D> <seed> <bits> <word|-> [extra...]
run_one() {
  local out=$1 phase=$2 arm=$3 D=$4 s=$5 mb=$6 word=$7; shift 7
  local W2=(); [ "$word" = u128 ] && W2=(--word u128)
  local attempt rc t0 t1 ok
  for attempt in 1 2; do
    if [ "$(date +%s)" -ge "$DEADLINE_EPOCH" ]; then finish deadline_reached; fi
    N=$((N+1)); t0=$(date +%s.%N)
    RAYON_NUM_THREADS=4 timeout 3600 $B ours --arm "$arm" --cols "$D" --weight 3 \
      --seed "$s" --modulus-bits "$mb" "${W2[@]}" "$@" > rec.json 2> err.txt; rc=$?
    t1=$(date +%s.%N)
    ok=false
    if [ $rc -eq 0 ] && [ "$(wc -l < rec.json)" -eq 1 ] && grep -q '"verified":true' rec.json; then ok=true; fi
    if [ $rc -eq 0 ] && [ -s rec.json ]; then cat rec.json >> raw/$out.jsonl; else
      echo "{\"failed\":true,\"arm\":\"$arm\",\"cols\":$D,\"seed\":$s,\"modulus_bits\":$mb,\"word\":\"$word\",\"exit\":$rc}" >> raw/$out.jsonl; fi
    echo "{\"n\":$N,\"phase\":\"$phase\",\"file\":\"$out\",\"arm\":\"$arm\",\"cols\":$D,\"seed\":$s,\"modulus_bits\":$mb,\"word\":\"$word\",\"extra\":\"$*\",\"attempt\":$attempt,\"exit\":$rc,\"valid\":$ok,\"outer_wall_s\":$(awk "BEGIN{printf \"%.3f\", $t1 - $t0}"),\"end_utc\":\"$(date -u +%FT%TZ)\",\"stderr_tail\":\"$(tail -c 200 err.txt | tr -d '\"\\\n' )\"}" >> raw/runs.jsonl
    echo "[$N] $phase $arm D=$D s=$s B=$mb $word rc=$rc valid=$ok $(awk "BEGIN{printf \"%.3f\", $t1 - $t0}")s"
    sync_all
    if [ $rc -eq 124 ] || [ $rc -eq 137 ]; then return 2; fi   # timeout / OOM: no repeat
    if $ok; then return 0; fi
  done
  return 1
}
aa() {  # A/A: full-bw at U192@129 and u64@61, D=2^16, seed 1
  run_one aa "aa-$1" full-bw $SD 1 129 -
  run_one aa "aa-$1" full-bw $SD 1 61 u64
}
ARMS=("full-bw 61 u64" "full-bl 61 u64" "full-bw 61 u128" "full-bl 61 u128"
      "full-bw 89 -" "full-bl 89 -" "full-bw 127 -" "full-bl 127 -"
      "full-bw 129 -" "full-bl 129 -" "full-bw 190 -" "full-bl 190 -"
      "full-bw 255 -" "full-bl 255 -")
aa before
s=$SEED
for k in $(seq $KMIN $KMAX); do D=$((1 << k))
  r=$(( (k + s) % 14 ))
  for i in $(seq 0 13); do
    set -- ${ARMS[$(( (i + r) % 14 ))]}
    key="$1/$2/$3"
    if [ -n "${STOPPED[$key]:-}" ]; then
      echo "{\"n\":null,\"phase\":\"grid\",\"arm\":\"$1\",\"cols\":$D,\"seed\":$s,\"modulus_bits\":$2,\"word\":\"$3\",\"skipped\":\"stop rule after ${STOPPED[$key]}\"}" >> raw/runs.jsonl
      continue
    fi
    run_one grid grid "$1" "$D" "$s" "$2" "$3"; rc=$?
    [ $rc -eq 2 ] && STOPPED[$key]="D=$D"
  done
  if [ $k -eq $SECOND_K ]; then
    aa mid
    for i in $(seq 0 13); do   # secondary: --coeffs wide at D=2^16, this seed
      set -- ${ARMS[$(( (i + r) % 14 ))]}
      run_one wide_coeffs wide "$1" $SD "$s" "$2" "$3" --coeffs wide
    done
  fi
done
aa after
finish complete
