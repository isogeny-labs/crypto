#!/usr/bin/env bash
# run_cells_r2b.sh -- P1 round 2, card CARD-p1-r2-b: R2b, T24 m5 t4, systems RR then T, orchestration
# only; every number comes from p1tool and msolve. RUN ALONE ON THE HOST (no other job, including no
# other round-2 card, may be running while this driver runs).
#
# Environment (both required):
#   P1TOOL     absolute path of the built p1tool binary
#   RUNS_ROOT  existing repository-relative directory that will hold run-r2b-20261108T053603Z/
# Run from the repository root.
#
# Protocol: research/split_prime_ecdlp_20261107/followups/p1_rr_dickson/PROTOCOL-R2.md (with its Addendum A).
# Pattern: calibration/run_cells.sh (round 1).
# Curve: D24 = T24 (dickson, p 14680063, a 3607823, b 6826282), m = 5, t = 4.
# Seed rule (round 1, amendment 3): seeds are the integers 1, 2, ..., 50 in order; R2b takes the FIRST
# admissible seed only; every skipped seed is recorded with its reason in skips.tsv; no admissible seed in
# 1..50 is recorded "not_admissible" in cells.tsv, no job runs and nothing is imputed.
# A seed is skipped only when `p1tool emit` panics with "base set too small" or
# "no admissible planted instance found"; any other emit failure stops the run (exit 1).
# Admissibility is a property of (prime, m, t, seed): RR is emitted first, T on the same seed.
# msolve limits (R2b): VLIMIT=13631488 (KiB), TLIMIT=10800 (s). A job that reaches the memory cap or the
# time limit is a recorded result (parse status), never a stop. An emit or parse failure is a HARD_ERROR
# (exit 1). Jobs: RR on the seed, then T on the seed, one msolve job at a time.
set -u

: "${P1TOOL:?P1TOOL not set}"
: "${RUNS_ROOT:?RUNS_ROOT not set}"
HERE=$(cd "$(dirname "$0")" && pwd)
RUNNER="$HERE/msolve_cell.sh"
SRC="$HERE/../p1tool/src/main.rs"
VL=13631488
TL=10800
KIND=dickson
P=14680063
A=3607823
B=6826282
M=5
T=4
CELL=D24_m5_t4

[ -d "$RUNS_ROOT" ] || exit 2
RUN="$RUNS_ROOT/run-r2b-20261108T053603Z"
mkdir "$RUN" || exit 2
echo "RUN_DIR=$RUN"

hard_error() {
  echo "HARD_ERROR: $*" | tee -a "$RUN/ERROR.txt"
  exit 1
}

# provenance (recorded, never fatal); the load and the msolve processes visible at start document "alone"
{
  echo "run_start_utc=$(date -u +%Y-%m-%dT%H:%M:%SZ)"
  echo "git_head=$(git rev-parse HEAD)"
  echo "p1tool_path=$P1TOOL"
  sha256sum "$P1TOOL"
  echo "pinned_main_rs_sha256=7e50f8c82e7bc981c180e402e90c20d94ca60d4cf73cd22aaee02c15efb84ed7"
  sha256sum "$SRC"
  sha256sum /usr/bin/msolve
  echo "VLIMIT=$VL TLIMIT=$TL"
  echo "nproc=$(nproc)"
  uname -a
  uptime
  free -k
  echo "msolve_processes_at_start:"
  pgrep -a msolve
  echo "end_msolve_processes_at_start"
} > "$RUN/provenance.txt" 2>&1
cat "$RUN/provenance.txt"

printf 'cell\tseed\temit_exit\treason\n' > "$RUN/skips.tsv"
printf 'cell\tadmissible_seed\tskipped_seeds\tstatus\n' > "$RUN/cells.tsv"
printf 'job\tcell\tsystem\tkind\tp\ta\tb\tm\tt\tseed\tms_sha256\tlog\tparse_json\n' > "$RUN/jobs.tsv"
: > "$RUN/parsed.jsonl"

# emit SYS SEED BASE
emit() {
  nice -n 10 "$P1TOOL" emit --system "$1" --kind "$KIND" --p "$P" --a "$A" --b "$B" \
    --m "$M" --t "$T" --seed "$2" --out "$3.ms" --plant "$3.ms.plant.json" \
    > "$3.emit.json" 2> "$3.emit.err"
}

dir="$RUN/$CELL"
mkdir "$dir" || hard_error "mkdir $dir"
SEED=""
SKIPPED=""
for ((s = 1; s <= 50; s++)); do
  emit rr "$s" "$dir/rr_s$s"
  rc=$?
  if [ "$rc" -ne 0 ]; then
    reason=$(grep -A1 'panicked at' "$dir/rr_s$s.emit.err" | tail -n 1)
    case "$reason" in
      "base set too small"* | "no admissible planted instance found"*)
        printf '%s\t%s\t%s\t%s\n' "$CELL" "$s" "$rc" "$reason" >> "$RUN/skips.tsv"
        SKIPPED="${SKIPPED:+$SKIPPED,}$s"
        continue
        ;;
      *)
        hard_error "$CELL seed $s rr emit exit $rc, reason: $reason"
        ;;
    esac
  fi
  emit t "$s" "$dir/t_s$s"
  rc=$?
  [ "$rc" -eq 0 ] || hard_error "$CELL seed $s t emit exit $rc after rr emit succeeded"
  SEED=$s
  break
done

if [ -z "$SEED" ]; then
  status=not_admissible
else
  status=one_admissible_seed
fi
printf '%s\t%s\t%s\t%s\n' "$CELL" "$SEED" "${SKIPPED:-}" "$status" >> "$RUN/cells.tsv"
echo "cell $CELL admissible_seed=[$SEED] skipped_seeds=[${SKIPPED:-}] $status"

# job SYS SEED
JOB=0
job() {
  local sys=$1 s=$2 base msha
  base="$dir/${sys}_s$s"
  JOB=$((JOB + 1))
  VLIMIT=$VL TLIMIT=$TL bash "$RUNNER" "$base.ms" "$base.log" || hard_error "msolve_cell.sh exit $? on $base"
  nice -n 10 "$P1TOOL" parse "$base.log" > "$base.parse.json" || hard_error "parse exit $? on $base.log"
  msha=$(sha256sum "$base.ms" | cut -d' ' -f1)
  printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\n' \
    "$JOB" "$CELL" "$sys" "$KIND" "$P" "$A" "$B" "$M" "$T" "$s" "$msha" "$base.log" "$base.parse.json" >> "$RUN/jobs.tsv"
  cat "$base.parse.json" >> "$RUN/parsed.jsonl"
  echo "job $JOB $CELL $sys seed $s $(grep -o '"status":"[^"]*"' "$base.parse.json") $(tail -n 1 "$base.log")"
}

if [ -n "$SEED" ]; then
  job rr "$SEED"
  job t "$SEED"
fi

echo "RUN_COMPLETE run_dir=$RUN jobs=$JOB"
for f in provenance.txt cells.tsv skips.tsv jobs.tsv parsed.jsonl; do
  sha256sum "$RUN/$f"
done
exit 0
