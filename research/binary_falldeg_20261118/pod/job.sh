#!/usr/bin/env bash
# binary_falldeg_20261118 — pod job (POD_PLAN.md, Amendment A1 2026-11-18).
#
# Self-contained: needs bash + coreutils + /proc only (no python, no jq, no
# /usr/bin/time). Runs the frozen cells cheapest first, one binary process
# per cell, each under a wall budget and a memory guard; a guarded cell is
# stopped with SIGTERM/SIGKILL and recorded "censored_*" before the kernel
# OOM killer can fire. Resumable: a cell whose summary row already exists in
# $OUT/cells.jsonl is skipped.
#
# Usage (on the pod):
#   TOTAL_BUDGET_H=12 bash /workspace/code/job.sh   # base plan, in order
#   bash /workspace/code/job.sh dry-run    # smallest cell only, keep 1, -> $OUT/dryrun/
#   bash /workspace/code/job.sh plan       # print the cell order and exit
#
# Environment (all optional):
#   BIN              binary path            (default: <script dir>/binary_falldeg_arms)
#   EXPECT_SHA256    binary sha256          (default: the frozen build below)
#   OUT              output dir             (default: /workspace/out)
#   THREADS          RAYON_NUM_THREADS      (default: nproc)
#   TOTAL_BUDGET_H   whole-job wall cap, h  (default: 12)
#   TOTAL_BUDGET_S   same cap in seconds; overrides TOTAL_BUDGET_H (testing)
#   MEM_RESERVE_GB   headroom kept free     (default: 8)
#   SAMPLE_S         guard sample period, s (default: 5; progress line every 60 s)
#   DEG5=1           append the degree-5 opt-in P1x_3-21-7 (keep 1; ~30-90 h; A1.4)
#   FULL_P1=1        append the frozen cmax=5 P1 cells (keep 6), which
#                    Amendment A1 predicts cannot fit 256 GB / the budget
#   ONLY=<cell_id>   run just that cell (debugging)
#
# Outputs in $OUT:
#   host.txt                 host facts (nproc, free -g, lscpu, uptime, cgroup limit)
#   cells.jsonl              ONE summary row per finished/censored cell (schema binary_falldeg_cell/v1)
#   rows/<cell_id>.jsonl     the binary's per-(draw, arm) rows (binary_falldeg/v1)
#   rows/<cell_id>.err       the binary's stderr
#   progress.log             start/stop lines + a sample every 60 s (rss, hwm, memavail, load)
#   final_status             "done" | "budget_exhausted" | "interrupted" | "sha_mismatch"

set -uo pipefail

SCRIPT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
BIN=${BIN:-$SCRIPT_DIR/binary_falldeg_arms}
EXPECT_SHA256=${EXPECT_SHA256:-2ed36ccad8a7a3a903792d601704e7afa2eeb254954be3af4be12bb131ad0d54}
OUT=${OUT:-/workspace/out}
THREADS=${THREADS:-$(nproc)}
TOTAL_BUDGET_H=${TOTAL_BUDGET_H:-12}
MEM_RESERVE_GB=${MEM_RESERVE_GB:-8}
SAMPLE_S=${SAMPLE_S:-5}
MODE=${1:-full}
for v in THREADS MEM_RESERVE_GB SAMPLE_S TOTAL_BUDGET_H; do
  [[ ${!v} =~ ^[0-9]+$ ]] || { echo "job.sh: $v must be a whole number (got '${!v}')" >&2; exit 64; }
done

SEED=1262698074          # frozen master seed (pilot / P1 / P2 / A/A side a)
SEED_AA_B=1262698075     # A/A side b: a different master seed (POD_PLAN "A/A arm")

case "$MODE" in
  full) ;;
  dry-run|dryrun) MODE=dry-run; OUT=$OUT/dryrun ;;
  plan) ;;
  *) echo "usage: job.sh [full|dry-run|plan]" >&2; exit 64 ;;
esac

mkdir -p "$OUT/rows"
CELLS=$OUT/cells.jsonl
PROG=$OUT/progress.log
touch "$CELLS" "$PROG"

now() { date -u +%Y-%m-%dT%H:%M:%SZ; }
log() { printf '%s %s\n' "$(now)" "$*" | tee -a "$PROG" >&2; }
jstr() { # JSON-escape a string (backslash, quote, control chars -> space)
  local s=${1//\\/\\\\}; s=${s//\"/\\\"}; s=${s//$'\t'/ }; s=${s//$'\r'/ }; s=${s//$'\n'/ | }
  printf '"%s"' "$s"
}

# ───────────────────────── binary identity ─────────────────────────
if [[ ! -x $BIN ]]; then
  [[ -f $BIN ]] && chmod +x "$BIN" || { log "FATAL binary not found: $BIN"; echo sha_mismatch > "$OUT/final_status"; exit 2; }
fi
GOT_SHA=$(sha256sum "$BIN" | cut -d' ' -f1)
if [[ $GOT_SHA != "$EXPECT_SHA256" ]]; then
  log "FATAL sha256 mismatch: got $GOT_SHA expected $EXPECT_SHA256"
  echo sha_mismatch > "$OUT/final_status"
  exit 2
fi
log "binary $BIN sha256 $GOT_SHA OK mode=$MODE threads=$THREADS budget_h=$TOTAL_BUDGET_H"

# ───────────────────────── host facts ─────────────────────────
cg_limit_bytes() { # effective cgroup memory limit, or empty
  local v
  if [[ -r /sys/fs/cgroup/memory.max ]]; then v=$(cat /sys/fs/cgroup/memory.max)
  elif [[ -r /sys/fs/cgroup/memory/memory.limit_in_bytes ]]; then v=$(cat /sys/fs/cgroup/memory/memory.limit_in_bytes)
  fi
  [[ -n ${v:-} && $v != max && $v -lt 1152921504606846976 ]] && echo "$v"
}
MEMTOTAL_KB=$(awk '/^MemTotal:/{print $2}' /proc/meminfo)
CG=$(cg_limit_bytes || true)
LIMIT_KB=$MEMTOTAL_KB
if [[ -n $CG ]] && (( CG / 1024 < LIMIT_KB )); then LIMIT_KB=$(( CG / 1024 )); fi
GUARD_KB=$(( LIMIT_KB - MEM_RESERVE_GB * 1024 * 1024 ))
{
  echo "date_utc: $(now)"
  echo "hostname: $(hostname 2>/dev/null || cat /proc/sys/kernel/hostname)"
  echo "uname: $(uname -a)"
  echo "glibc: $(ldd --version 2>/dev/null | head -1)"
  echo "nproc: $(nproc)"
  echo "uptime: $(uptime)"
  echo "MemTotal_kB: $MEMTOTAL_KB"
  echo "cgroup_memory_limit_bytes: ${CG:-none}"
  echo "guard_rss_kB: $GUARD_KB (limit $LIMIT_KB kB minus reserve ${MEM_RESERVE_GB} GB)"
  echo "--- free -g"; free -g
  echo "--- lscpu (head)"; lscpu 2>/dev/null | head -20
  echo "--- binary"; echo "$BIN $GOT_SHA"
  echo "--- env"; echo "THREADS=$THREADS TOTAL_BUDGET_H=$TOTAL_BUDGET_H MEM_RESERVE_GB=$MEM_RESERVE_GB SAMPLE_S=$SAMPLE_S FULL_P1=${FULL_P1:-0} DEG5=${DEG5:-0}"
} > "$OUT/host.txt" 2>&1
cat "$OUT/host.txt" >&2

# ───────────────────────── cell plan ─────────────────────────
# id | stage | m:n:ell | arms | seed | keep | cmax | ffd_max | dres_max | budget_s | pred_wall_s | pred_mem_gb
# Order = cheapest first; predictions and bands are POD_PLAN.md Amendment A1
# (32 vCPU), written before any pod data. budget_s = per-cell cap (about the
# top of the band x 1.5-3); the whole job is also capped by TOTAL_BUDGET_H.
#   P2  frozen Ladder P2 (m=2, random vs window)
#   AA  frozen A/A arm: random only, two master seeds, keep 12
#   S   Amendment A1 degree-4 screen replacing frozen P1 at cmax=5
#   P1x Amendment A1 degree-5 opt-in (DEG5=1), keep 1
PLAN=(
  "P2_2-23-11|P2|2:23:11|random,subspace|$SEED|6|5|6|6|1800|120|0.1"
  "P2_2-25-12|P2|2:25:12|random,subspace|$SEED|6|5|6|6|3600|300|0.2"
  "AA2a_2-25-12|AA|2:25:12|random|$SEED|12|5|6|6|3600|300|0.2"
  "AA2b_2-25-12|AA|2:25:12|random|$SEED_AA_B|12|5|6|6|3600|300|0.2"
  "P2_2-29-14|P2|2:29:14|random,subspace|$SEED|6|5|6|6|9000|1500|0.4"
  "S1o_3-27-9|S|3:27:9|orbit,random,subspace|$SEED|6|4|6|6|900|30|0.1"
  "S1_3-21-7|S|3:21:7|orbit,random,subspace|$SEED|6|4|6|6|25200|5400|4"
  "AA1a_3-21-8|AA|3:21:8|random|$SEED|12|4|6|6|18000|3600|4"
  "AA1b_3-21-8|AA|3:21:8|random|$SEED_AA_B|12|4|6|6|18000|3600|4"
  "S1_3-25-7|S|3:25:7|random,subspace|$SEED|6|4|6|6|21600|4300|5"
  "S1_3-27-9|S|3:27:9|random,subspace|$SEED|6|4|6|6|21600|4300|14"
)
if [[ ${DEG5:-0} == 1 ]]; then
  # One draw, three arms, degree 5 (arms that fall at 4 stop there).
  PLAN+=("P1x_3-21-7|P1x|3:21:7|orbit,random,subspace|$SEED|1|5|6|6|324000|97000|160")
fi
if [[ ${FULL_P1:-0} == 1 ]]; then
  # Frozen P1 at cmax=5, keep 6. Predicted infeasible (A1.3 item 4, A1.4).
  PLAN+=(
    "P1_3-21-7|P1|3:21:7|orbit,random,subspace|$SEED|6|5|6|6|2000000|1700000|160"
    "P1_3-25-7|P1|3:25:7|random,subspace|$SEED|6|5|6|6|2000000|2600000|200"
    "P1_3-27-9|P1|3:27:9|orbit,random,subspace|$SEED|6|5|6|6|2000000|9000000|800"
  )
fi
if [[ $MODE == dry-run ]]; then
  # Smallest cell only, one kept draw, short budget.
  PLAN=("DRY_2-23-11|DRY|2:23:11|random,subspace|$SEED|1|5|6|6|900|20|0.1")
  TOTAL_BUDGET_H=1
fi
if [[ $MODE == plan ]]; then
  printf '%s\n' "${PLAN[@]}"
  exit 0
fi

JOB_T0=$(date +%s)
JOB_END=$(( JOB_T0 + ${TOTAL_BUDGET_S:-$(( ${TOTAL_BUDGET_H%.*} * 3600 ))} ))
CHILD=0
on_signal() {
  log "job.sh got signal; stopping cell pid=$CHILD (no summary row: the cell reruns on resume)"
  if (( CHILD > 0 )); then kill -TERM "$CHILD" 2>/dev/null; sleep 5; kill -KILL "$CHILD" 2>/dev/null; fi
  echo interrupted > "$OUT/final_status"
  exit 143
}
trap on_signal TERM INT HUP

status_kb() { awk -v k="$2:" '$1==k{print $2}' "/proc/$1/status" 2>/dev/null; }
memavail_kb() { awk '/^MemAvailable:/{print $2}' /proc/meminfo; }

run_cell() {
  local id=$1 stage=$2 cell=$3 arms=$4 seed=$5 keep=$6 cmax=$7 ffd=$8 dres=$9 budget=${10} pwall=${11} pmem=${12}
  [[ -n ${ONLY:-} && $ONLY != "$id" ]] && return 0
  # Resume: skip a cell whose latest summary row is final. A cell cut only by
  # the whole-job cap (censored_total_budget) or killed from outside
  # (killed_external) is rerun from scratch; its old row stays as history.
  local last; last=$(grep "\"cell_id\":\"$id\"" "$CELLS" | tail -1)
  if [[ -n $last && $last != *'"status":"censored_total_budget"'* && $last != *'"status":"killed_external"'* ]]; then
    log "skip $id (final summary row exists)"; return 0
  fi
  local now_s; now_s=$(date +%s)
  local remain=$(( JOB_END - now_s ))
  if (( remain < 120 )); then
    log "total budget exhausted before $id; not started (reruns on resume)"; return 3
  fi
  local eff_budget=$budget budget_src=cell
  if (( remain < budget )); then eff_budget=$remain; budget_src=total; fi

  local rows=$OUT/rows/$id.jsonl err=$OUT/rows/$id.err
  : > "$rows"; : > "$err"
  log "start $id stage=$stage cells=$cell arms=$arms seed=$seed keep=$keep cmax=$cmax budget_s=$eff_budget($budget_src) pred_wall_s=$pwall pred_mem_gb=$pmem"
  local t0; t0=$(date +%s)
  RAYON_NUM_THREADS=$THREADS "$BIN" run --cells "$cell" --seed "$seed" --keep "$keep" \
      --cmax "$cmax" --ffd-max "$ffd" --dres-max "$dres" --arms "$arms" --crosscheck \
      > "$rows" 2> "$err" &
  CHILD=$!
  local pid=$CHILD status="" hwm=0 rss=0 maxrss=0 minavail=999999999 last_prog=$t0 ma
  while kill -0 "$pid" 2>/dev/null; do
    sleep "$SAMPLE_S" & wait $!
    kill -0 "$pid" 2>/dev/null || break
    rss=$(status_kb "$pid" VmRSS); rss=${rss:-0}
    local h; h=$(status_kb "$pid" VmHWM); [[ -n $h ]] && hwm=$h
    (( rss > maxrss )) && maxrss=$rss
    ma=$(memavail_kb); (( ma < minavail )) && minavail=$ma
    local t; t=$(date +%s)
    if (( rss > GUARD_KB )) || (( ma < MEM_RESERVE_GB * 1024 * 1024 / 2 )); then
      status=censored_mem
      log "GUARD $id rss_kB=$rss guard_kB=$GUARD_KB memavail_kB=$ma -> SIGTERM"
    elif (( t - t0 > eff_budget )); then
      if [[ $budget_src == total ]]; then status=censored_total_budget; else status=censored_time; fi
      log "BUDGET $id wall_s=$(( t - t0 )) budget_s=$eff_budget -> SIGTERM"
    fi
    if [[ -n $status ]]; then
      kill -TERM "$pid" 2>/dev/null
      for _ in 1 2 3 4 5 6 7 8 9 10; do kill -0 "$pid" 2>/dev/null || break; sleep 1; done
      kill -KILL "$pid" 2>/dev/null
      break
    fi
    if (( t - last_prog >= 60 )); then
      last_prog=$t
      log "progress $id wall_s=$(( t - t0 )) rss_kB=$rss hwm_kB=$hwm memavail_kB=$ma load=$(cut -d' ' -f1-3 /proc/loadavg) rows=$(wc -l < "$rows")"
    fi
  done
  wait "$pid"; local rc=$?
  CHILD=0
  local t1; t1=$(date +%s)
  if [[ -z $status ]]; then
    if (( rc == 0 )); then status=ok
    elif (( rc == 137 )); then status=killed_external   # OOM killer or pod, not our guard
    else status=failed; fi
  fi
  local nrows; nrows=$(wc -l < "$rows")
  local kept_line; kept_line=$(grep -E '^cell |H-avail|skipping' "$err" | tail -3)
  printf '{"schema":"binary_falldeg_cell/v1","cell_id":"%s","stage":"%s","cells":"%s","arms":"%s","seed":%s,"keep":%s,"cmax":%s,"ffd_max":%s,"dres_max":%s,"status":"%s","exit_code":%s,"rows":%s,"wall_s":%s,"budget_s":%s,"budget_src":"%s","peak_rss_kB":%s,"max_sampled_rss_kB":%s,"min_memavail_kB":%s,"guard_rss_kB":%s,"pred_wall_s":%s,"pred_mem_gb":%s,"threads":%s,"binary_sha256":"%s","start_utc":"%s","end_utc":"%s","sample_s":%s,"stderr_tail":%s}\n' \
    "$id" "$stage" "$cell" "$arms" "$seed" "$keep" "$cmax" "$ffd" "$dres" "$status" "$rc" "$nrows" \
    "$(( t1 - t0 ))" "$eff_budget" "$budget_src" "$hwm" "$maxrss" "$minavail" "$GUARD_KB" "$pwall" "$pmem" \
    "$THREADS" "$GOT_SHA" "$(date -u -d "@$t0" +%Y-%m-%dT%H:%M:%SZ)" "$(date -u -d "@$t1" +%Y-%m-%dT%H:%M:%SZ)" "$SAMPLE_S" \
    "$(jstr "$kept_line")" >> "$CELLS"
  log "end $id status=$status rc=$rc rows=$nrows wall_s=$(( t1 - t0 )) peak_rss_kB=$hwm"
  [[ $status == censored_total_budget ]] && return 3
  return 0
}

final=done
for entry in "${PLAN[@]}"; do
  IFS='|' read -r id stage cell arms seed keep cmax ffd dres budget pwall pmem <<< "$entry"
  run_cell "$id" "$stage" "$cell" "$arms" "$seed" "$keep" "$cmax" "$ffd" "$dres" "$budget" "$pwall" "$pmem"
  if (( $? == 3 )); then final=budget_exhausted; break; fi
done
echo "$final" > "$OUT/final_status"
log "job finished: $final ($(wc -l < "$CELLS") cell rows in $CELLS)"
