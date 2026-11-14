#!/bin/bash
# Pod-side: R04's scan probes on main (probes ported in this directory's
# commit), M1's two rows at 2^44.3, 2^44.5 and 2^47.2, four rounds, isolated
# on CPU 2, as ../scan-stages-v8-20261017/probe.sh ran them on v8.  Runs as
# UID 1000 in the study image; results sync to $ISOLAB_RESULTS_URI every 60 s.
set -u
OUT=/work/results ROOT=/work/repo S=/opt/scanp
mkdir -p "$OUT" "$ROOT"
LOG=$OUT/job.log
log() { printf '== %s %s\n' "$(date -u +%FT%TZ)" "$*" >> "$LOG"; }
sync_up() { aws s3 sync --only-show-errors --exclude '*.tmp' "$OUT" "$ISOLAB_RESULTS_URI" || log "sync failed"; }
finish() {
  local rc=$?; trap - EXIT
  [ -n "${SYNC_PID:-}" ] && kill "$SYNC_PID" 2>/dev/null
  printf '{"exit_code":%s,"job_name":"%s","pod_uid":"%s","node_name":"%s","finished_at":"%s"}\n' \
    "$rc" "${ISOLAB_JOB_ID:-}" "${POD_UID:-}" "${NODE_NAME:-}" "$(date -u +%FT%TZ)" > "$OUT/job-exit.json"
  (cd "$OUT" && find . -type f ! -name SHA256SUMS ! -name '*.tmp' -printf '%P\n' | LC_ALL=C sort | xargs -r sha256sum > SHA256SUMS)
  for i in 1 2 3 4 5; do sync_up && break; sleep 20; done
  exit "$rc"
}
trap finish EXIT; trap 'exit 143' TERM
log "job ${ISOLAB_JOB_ID:-?} pod ${POD_UID:-?} node ${NODE_NAME:-?} start"
tar -xf $S/tree.tar -C "$ROOT"
cp $S/build-info.json "$OUT/"; (cd $S/bin && sha256sum *) > "$OUT/binary.sha256"
cmp <(sort -k2 "$OUT/binary.sha256") <(sort -k2 $S/binaries.sha256) || { log "binaries differ from binaries.sha256"; exit 3; }
{ grep -m1 'model name' /proc/cpuinfo; nproc; cat /sys/fs/cgroup/cpuset.cpus.effective 2>/dev/null; uname -r; } > "$OUT/host.txt"
( while true; do sleep 60; sync_up; done ) & SYNC_PID=$!
P=$ROOT/research/ic_tool_program/suite/v1/params/S
SUITE=$ROOT/research/ic_tool_program/suite/v1/SUITE.json
BIN=$S/bin/ic-scanprobes
ROWS=("k0n53 M1-T01" "k0n53 M1-T02" "k1n59 M1-T01" "k1n59 M1-T02" "k0n61 M1-T01" "k0n61 M1-T02")
seed_of() { jq -r --arg id "$1-$2" '.rows[] | select(.id == $id) | .rho_seed' $SUITE; }
for round in 1 2 3 4; do
  for row in "${ROWS[@]}"; do
    set -- $row
    seed=$(seed_of $1 $2)
    d=$OUT/runs/$1/$2; mkdir -p $d
    [ -s $d/r$round.price.json ] && continue
    for try in 1 2 3 4 5 6 7 8 9 10; do
      RAYON_NUM_THREADS=1 $S/bin/isolated_bench run --wait --cpus 2 --out $d/r$round.isolation.jsonl \
        --label "scanp-main/$1/$2/r$round" -- \
        $BIN price --params $P/$1/$2.json --json --out $d/r$round.price.json --single-target --rho-seed $seed \
        > /dev/null 2>> "$LOG" && break
      log "REFUSED $1 $2 r$round try $try"; sleep 15
    done
    log "r$round $1/$2 scalar=$(jq -r '.certificates.ic.scalar' $d/r$round.price.json 2>/dev/null)"
  done
done
bash $S/stages.sh "$OUT/runs" > "$OUT/stages.tsv" 2>> "$LOG"
log "all done"
