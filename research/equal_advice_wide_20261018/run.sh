#!/usr/bin/env bash
# Co-measured equal-advice session: compact-orbit index calculus (IC) and
# Bernstein-Lange (BL) preprocessing rho at equal advice bytes, one process
# group, one resource envelope. Thin orchestration only: every measurement,
# replay, claim and statistic is native (Rust). Frozen by PROTOCOL.md.
#
#   research/equal_advice_wide_20261018/run.sh [rung ...]
#
# rung: n73 n83 (the frozen session) or n41smoke (harness smoke test, not a
# result). Default: n73 n83, in that order, each into
# $OUT_ROOT/<rung>/ (default research/equal_advice_wide_20261018/runs/<UTC>/).
#
# Environment (all optional):
#   OUT_ROOT     output root
#   CPUSET       taskset list for every process (default: all online CPUs)
#   THREADS      setup threads for the BL table walks and the IC rank stage
#                (default: CPUs in CPUSET). Online intervals are single-thread.
#   TARGETS      public targets per rung (default 64; n41smoke 16)
#   MEM_GIB      address-space cap per process (defaults per rung below)
#   CARGO_TARGET_DIR  cargo target directory (default ./target)
#   ISOLATION_LEVEL   the isolation level the host EARNED for both arms
#                (L0 default; set L2/L3 only with an isolab receipt, which
#                this script does not produce)
set -euo pipefail

if [ -z "${EA_IN_GROUP:-}" ]; then
  # Become the leader of a fresh process group / session: every arm below is
  # a member of this one group.
  exec env EA_IN_GROUP=1 setsid --wait "$0" "$@"
fi
trap 'trap - EXIT TERM INT; kill -- -$$ 2>/dev/null || true' EXIT TERM INT

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT"
STAMP="$(date -u +%Y%m%dT%H%MZ)"
OUT_ROOT="${OUT_ROOT:-$ROOT/research/equal_advice_wide_20261018/runs/$STAMP}"
TARGET_DIR="${CARGO_TARGET_DIR:-$ROOT/target}"
NCPU="$(nproc)"
CPUSET="${CPUSET:-0-$((NCPU - 1))}"
cpuset_count() { # number of CPUs in a taskset list such as 0-3,8,10-11
  local n=0 part
  IFS=',' read -ra parts <<<"$1"
  for part in "${parts[@]}"; do
    if [[ "$part" == *-* ]]; then n=$((n + ${part#*-} - ${part%-*} + 1)); else n=$((n + 1)); fi
  done
  echo "$n"
}
THREADS="${THREADS:-$(cpuset_count "$CPUSET")}"
ISOLATION_LEVEL="${ISOLATION_LEVEL:-L0}"
TARGET_LABEL="equal-advice-wide-20261018-v1"
BL_SEED="v1"

# ---- frozen per-rung constants (PROTOCOL.md "Rungs") -----------------------
rung_config() {
  case "$1" in
  n73)
    N=73; A=0; K=600; IC_EXAMPLE=koblitz_orbit_dlp_fast; IC_SEED=7
    IC_ENV=(); IC_STATES=26280000
    ADVICE_A=2781357248     # item-9 formula bytes of K=600, n=73 (IC-favourable)
    ADVICE_B=5630296064     # wait4 peak RSS of the frozen n=73 K=600 IC run
    MEM_GIB_DEFAULT=16; MIN_RAM_GIB=24
    CANDIDATE=experiments/koblitz-single-target-n73-20261003/candidate-manifest.json
    CURVE_ID=EC1N73Ckb1h25afb0fafbdf; EXPECTED_BASE_HASH=""
    TARGETS_DEFAULT=64 ;;
  n83)
    N=83; A=1; K=3240; IC_EXAMPLE=koblitz_orbit_dlp_fast; IC_SEED=29
    IC_ENV=(KIC_PACKED_ROOT_TABLE=1 KIC_PACKED_ROOT_TABLE_DENSE=0 KIC_PACKED_ROOT_TABLE_LOAD75=0
      KIC_STATE28=0 KIC_STATE26=0 KIC_ROOT_ONLY22=0 KIC_ROOT_ONLY21=1 KIC_INDEXED_ROOT5=1
      KIC_ROOT_BLOOM=0 KIC_SYMMETRIC_S3=1 KIC_INDEXED5_LOAD75=0 KIC_INDEXED5_FIXED30=1
      KIC_SPARSE_CANONICAL=0 KIC_TARGET_THREADS=1)
    IC_STATES=435652020
    ADVICE_A=14517401540    # 21*435652020 + 5*2^30: Lean-checked K3240 root21 payload
    ADVICE_B=14954881024    # wait4 peak RSS of the K3240 root21 cloud run
    MEM_GIB_DEFAULT=32; MIN_RAM_GIB=48
    CANDIDATE=research/n83_factor_base_panel_results_20261014/candidate_K3240_root21.json
    CURVE_ID=EC1N83Ckb1a1haacb69ee9f37
    EXPECTED_BASE_HASH=54847e372e3d68a2509bb16227aef70e09ceb9948d9e73149b6abc21355a2c0e
    TARGETS_DEFAULT=64 ;;
  n41smoke)
    N=41; A=0; K=100; IC_EXAMPLE=koblitz_orbit_dlp_fast_online; IC_SEED=7
    IC_ENV=(); IC_STATES=410000
    ADVICE_A=43689632; ADVICE_B=60000000
    MEM_GIB_DEFAULT=4; MIN_RAM_GIB=6
    CANDIDATE=""; CURVE_ID=""; EXPECTED_BASE_HASH=""
    TARGETS_DEFAULT=16 ;;
  *) echo "unknown rung $1 (n73 | n83 | n41smoke)" >&2; exit 2 ;;
  esac
}

json_escape() { printf '%s' "$1" | sed 's/\\/\\\\/g; s/"/\\"/g'; }

run_rung() {
  local rung="$1"
  rung_config "$rung"
  local OUT="$OUT_ROOT/$rung"
  local TARGET_COUNT="${TARGETS:-$TARGETS_DEFAULT}"
  local MEM_GIB="${MEM_GIB:-$MEM_GIB_DEFAULT}"
  local AS_BYTES=$((MEM_GIB * 1024 * 1024 * 1024))
  mkdir -p "$OUT/barrier" "$OUT/sidecar"
  local B="$TARGET_DIR/release/examples"
  local BL_BIN="$B/ic_equal_advice_wide" IC_BIN="$B/$IC_EXAMPLE" CLAIMS_BIN="$B/ic_equal_advice_claims"
  # GNU time records each arm's maximum resident set when it is installed;
  # the BL summary line carries VmHWM either way.
  local WRAP=(taskset -c "$CPUSET" prlimit --as="$AS_BYTES" nice -n 0)
  local SESSION_ID="${rung}-$STAMP-pg$$"

  echo "[$(date -u +%FT%TZ)] rung $rung: preflight" >&2
  local ram_kib; ram_kib="$(awk '/MemTotal/ {print $2}' /proc/meminfo)"
  local free_kib; free_kib="$(df -Pk "$OUT" | awk 'NR==2 {print $4}')"
  if [ "$ram_kib" -lt $((MIN_RAM_GIB * 1024 * 1024)) ] || [ "$THREADS" -lt 1 ] || [ "$free_kib" -lt "${MIN_DISK_KIB:-2097152}" ]; then
    printf '{"status":"REJECTED_HOST_CAPACITY","rung":"%s","mem_total_kib":%s,"min_ram_gib":%s,"threads":%s,"disk_free_kib":%s}\n' \
      "$rung" "$ram_kib" "$MIN_RAM_GIB" "$THREADS" "$free_kib" | tee "$OUT/preflight.json" >&2
    return 2
  fi
  for tool in taskset prlimit setsid nice; do
    command -v "$tool" >/dev/null || { echo "missing $tool" >&2; return 2; }
  done
  printf '{"status":"PASS_CAPACITY_PREFLIGHT","rung":"%s","mem_total_kib":%s,"min_ram_gib":%s,"threads":%s,"cpuset":"%s"}\n' \
    "$rung" "$ram_kib" "$MIN_RAM_GIB" "$THREADS" "$CPUSET" >"$OUT/preflight.json"

  echo "[$(date -u +%FT%TZ)] build" >&2
  CARGO_TARGET_DIR="$TARGET_DIR" cargo build --release \
    --example ic_equal_advice_wide --example ic_equal_advice_claims --example "$IC_EXAMPLE" \
    2>"$OUT/build.stderr"

  # Targets: one file for both arms; scalars go to the sidecar, which no
  # producer reads and the claims tool does not read (it replays recovered
  # scalars on the generic curve instead).
  "$BL_BIN" targets "$N" "$A" "$TARGET_COUNT" "$TARGET_LABEL" "$OUT/points.txt" "$OUT/sidecar/scalars.txt"

  local COMMON_ENV=(RAYON_NUM_THREADS=1)
  local pg; pg="$(ps -o pgid= -p $$ | tr -d ' ')"

  echo "[$(date -u +%FT%TZ)] start BL arm (table walks use $THREADS threads, then idle at the barrier)" >&2
  env "${COMMON_ENV[@]}" BL_THREADS="$THREADS" BL_PASSES=2 BL_BARRIER_DIR="$OUT/barrier" \
    BL_PREFIX_BYTES="$ADVICE_A" BL_NULL_TARGETS=8 BL_SEED="$BL_SEED" BL_PROGRESS_S=300 \
    "${WRAP[@]}" bash -c 'f=$1; shift; if [ -x /usr/bin/time ]; then exec /usr/bin/time -v -o "$f" "$@"; else exec "$@"; fi' _ "$OUT/bl.time" \
    "$BL_BIN" bl "$N" "$A" "$ADVICE_B" "$OUT/points.txt" "$OUT/bl.jsonl" \
    >"$OUT/bl.stdout" 2>"$OUT/bl.stderr" &
  local bl_pid=$!
  while [ ! -e "$OUT/barrier/bl_ready" ]; do
    kill -0 "$bl_pid" 2>/dev/null || { echo "BL arm died before its table was ready" >&2; return 1; }
    sleep 15
  done
  echo "[$(date -u +%FT%TZ)] BL table ready" >&2

  run_ic() { # run_ic <k>
    echo "[$(date -u +%FT%TZ)] IC process $1 (setup threads $THREADS, online single thread)" >&2
    env "${COMMON_ENV[@]}" "${IC_ENV[@]}" KIC_RANK_THREADS="$THREADS" \
      "${WRAP[@]}" bash -c 'f=$1; shift; if [ -x /usr/bin/time ]; then exec /usr/bin/time -v -o "$f" "$@"; else exec "$@"; fi' _ "$OUT/ic_run$1.time" \
      "$IC_BIN" "construct:$N:$A:$K" "$OUT/points.txt" "$IC_SEED" "$OUT/ic_run$1.jsonl" \
      >"$OUT/ic_run$1.stdout" 2>"$OUT/ic_run$1.stderr"
  }
  run_bl_pass() { # run_bl_pass <p>
    echo "[$(date -u +%FT%TZ)] BL online pass $1" >&2
    touch "$OUT/barrier/go_$1"
    while [ ! -e "$OUT/barrier/done_$1" ]; do
      kill -0 "$bl_pid" 2>/dev/null || { echo "BL arm died in pass $1" >&2; return 1; }
      sleep 5
    done
  }
  # ABBA: IC1, BL pass 1, BL pass 2, IC2. The idle arm sleeps (the BL
  # process polls its barrier every 500 ms) with its advice resident.
  run_ic 1
  run_bl_pass 1
  run_bl_pass 2
  run_ic 2
  wait "$bl_pid"

  local envelope
  envelope="$(printf '{"session_id":"%s","process_group_id":%s,"cpuset":"%s","online_threads":1,"setup_threads":%s,"address_space_cap_bytes":%s,"nice":0,"kernel":"%s","cpu_model":"%s","mem_total_kib":%s,"isolation_receipt":null}' \
    "$SESSION_ID" "$pg" "$CPUSET" "$THREADS" "$AS_BYTES" "$(uname -r)" \
    "$(json_escape "$(awk -F': ' '/model name/ {print $2; exit}' /proc/cpuinfo)")" "$ram_kib")"
  local bin_sha_ic bin_sha_bl
  bin_sha_ic="$(sha256sum "$IC_BIN" | cut -d' ' -f1)"
  bin_sha_bl="$(sha256sum "$BL_BIN" | cut -d' ' -f1)"
  local candidate_json="null"
  [ -n "$CANDIDATE" ] && candidate_json="\"$ROOT/$CANDIDATE\""
  local expected_json="null"
  [ -n "$EXPECTED_BASE_HASH" ] && expected_json="\"$EXPECTED_BASE_HASH\""
  cat >"$OUT/session.json" <<JSON
{
  "schema": "ic_equal_advice_session/v1",
  "session_id": "$SESSION_ID", "rung": "$rung", "n": $N, "a": $A, "K": $K, "ic_states": $IC_STATES,
  "curve_id": "$CURVE_ID", "targets": "$OUT/points.txt", "target_label": "$TARGET_LABEL",
  "input_law": "ic_equal_advice.public_hash_target",
  "ic_rank_seed": $IC_SEED, "bl_seed": "$BL_SEED", "bl_online_threads": 1,
  "candidate_manifest": $candidate_json, "expected_base_hash": $expected_json,
  "ic_binary_sha256": "$bin_sha_ic", "bl_binary_sha256": "$bin_sha_bl",
  "runs": [
    {"run": 1, "ic_jsonl": "$OUT/ic_run1.jsonl", "bl_jsonl": "$OUT/bl.jsonl", "bl_pass": 1},
    {"run": 2, "ic_jsonl": "$OUT/ic_run2.jsonl", "bl_jsonl": "$OUT/bl.jsonl", "bl_pass": 2}
  ],
  "levels": [
    {"level": 0, "tier": "B", "bl_bytes": $ADVICE_B},
    {"level": 1, "tier": "A", "bl_bytes": $ADVICE_A}
  ],
  "resource_envelope": $envelope,
  "isolation": {"required": "L2", "ic": "$ISOLATION_LEVEL", "rho": "$ISOLATION_LEVEL"},
  "co_measured": {"same_process_group": true, "process_group_id": $pg,
                  "order": ["IC#1", "BL#1", "BL#2", "IC#2"], "bl_process_resident_throughout": true},
  "host": {"id": "$(hostname)"},
  "independent_validation": false, "independent_replay_pointer": null,
  "a_a_tolerance": 0.25,
  "claim_boundary": "Equal-advice diagnostic on public synthetic Koblitz points: BL preprocessing rho with Frobenius folding against compact-orbit IC at the IC table's byte budget. Not a deployed-curve claim, not an asymptotic claim, not end-to-end S; precomputation of both arms is reported apart from the online intervals."
}
JSON
  echo "[$(date -u +%FT%TZ)] assemble, replay, check" >&2
  "$CLAIMS_BIN" "$OUT/session.json" "$OUT/assembled" | tee "$OUT/assembled.stdout"
  if [ -x "$TARGET_DIR/release/ecbench" ]; then
    # The shipped checker, on every claim (exit 1 = problems listed; expected
    # until independent validation and a qualifying isolation receipt exist).
    : >"$OUT/assembled/ecbench_claim_check.tsv"
    for claim in "$OUT"/assembled/claims/*.claim.json; do
      rc=0; "$TARGET_DIR/release/ecbench" claim check --report "$claim" >/dev/null 2>&1 || rc=$?
      printf '%s\t%s\n' "$(basename "$claim")" "$rc" >>"$OUT/assembled/ecbench_claim_check.tsv"
    done
  fi
  echo "[$(date -u +%FT%TZ)] rung $rung done: $OUT/assembled/decision.json" >&2
}

RUNGS=("$@")
[ "${#RUNGS[@]}" -gt 0 ] || RUNGS=(n73 n83)
mkdir -p "$OUT_ROOT"
for rung in "${RUNGS[@]}"; do
  run_rung "$rung"
done
