#!/usr/bin/env bash
# run_cells_r2a.sh -- P1 round 2, card CARD-p1-r2-a: R2c (arm F) and R2a (arm T), orchestration only;
# every number comes from p1tool and msolve.
#
# Environment (both required):
#   P1TOOL     absolute path of the built p1tool binary
#   RUNS_ROOT  existing repository-relative directory that will hold run-r2a-20261108T053528Z/
# Run from the repository root.
#
# Protocol: research/split_prime_ecdlp_20261107/followups/p1_rr_dickson/PROTOCOL-R2.md (with its Addendum A).
# Pattern: calibration/run_cells.sh (round 1). Seeds are round 1's admissible seeds per cell, from
# runs/run-20261107T214621Z/cells.tsv, written literally in JOBS below (no seed scan in this driver).
#   R2c = system f at D24, D31, K0 x (m,t) = (3,3), (3,4), (3,5), two seeds each (18 jobs), run first.
#   R2a = system t at D24 m3 t5, D31 m3 t5, K0 m3 t5, D24 m5 t3, two seeds each (8 jobs), run second.
# Curves: D24 = T24 (dickson, p 14680063, a 3607823, b 6826282); D31 = T31 (dickson, p 2046820351,
# a 289962109, b 1299273082); K0 = p0 (kummer, p 786433, a 658649, b 132295).
# msolve limits (R2a, R2c): VLIMIT=6291456 (KiB), TLIMIT=3600 (s). A job that reaches the memory cap
# or the time limit is a recorded result (parse status), never a stop. An emit or parse failure is a
# HARD_ERROR (exit 1). One msolve job at a time.
set -u

: "${P1TOOL:?P1TOOL not set}"
: "${RUNS_ROOT:?RUNS_ROOT not set}"
HERE=$(cd "$(dirname "$0")" && pwd)
RUNNER="$HERE/msolve_cell.sh"
SRC="$HERE/../p1tool/src/main.rs"
VL=6291456
TL=3600

[ -d "$RUNS_ROOT" ] || exit 2
RUN="$RUNS_ROOT/run-r2a-20261108T053528Z"
mkdir "$RUN" || exit 2
echo "RUN_DIR=$RUN"

hard_error() {
  echo "HARD_ERROR: $*" | tee -a "$RUN/ERROR.txt"
  exit 1
}

# provenance (recorded, never fatal)
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
} > "$RUN/provenance.txt" 2>&1
cat "$RUN/provenance.txt"

printf 'job\tpart\tcell\tsystem\tkind\tp\ta\tb\tm\tt\tseed\tms_sha256\tlog\tparse_json\n' > "$RUN/jobs.tsv"
: > "$RUN/parsed.jsonl"

# one line per job: part|cell|system|kind|p|a|b|m|t|seed ; F jobs first, then T jobs
JOBS=(
  "R2c|D24_m3_t3|f|dickson|14680063|3607823|6826282|3|3|2"
  "R2c|D24_m3_t3|f|dickson|14680063|3607823|6826282|3|3|5"
  "R2c|D31_m3_t3|f|dickson|2046820351|289962109|1299273082|3|3|2"
  "R2c|D31_m3_t3|f|dickson|2046820351|289962109|1299273082|3|3|4"
  "R2c|K0_m3_t3|f|kummer|786433|658649|132295|3|3|2"
  "R2c|K0_m3_t3|f|kummer|786433|658649|132295|3|3|3"
  "R2c|D24_m3_t4|f|dickson|14680063|3607823|6826282|3|4|1"
  "R2c|D24_m3_t4|f|dickson|14680063|3607823|6826282|3|4|2"
  "R2c|D31_m3_t4|f|dickson|2046820351|289962109|1299273082|3|4|1"
  "R2c|D31_m3_t4|f|dickson|2046820351|289962109|1299273082|3|4|2"
  "R2c|K0_m3_t4|f|kummer|786433|658649|132295|3|4|1"
  "R2c|K0_m3_t4|f|kummer|786433|658649|132295|3|4|2"
  "R2c|D24_m3_t5|f|dickson|14680063|3607823|6826282|3|5|1"
  "R2c|D24_m3_t5|f|dickson|14680063|3607823|6826282|3|5|2"
  "R2c|D31_m3_t5|f|dickson|2046820351|289962109|1299273082|3|5|1"
  "R2c|D31_m3_t5|f|dickson|2046820351|289962109|1299273082|3|5|2"
  "R2c|K0_m3_t5|f|kummer|786433|658649|132295|3|5|1"
  "R2c|K0_m3_t5|f|kummer|786433|658649|132295|3|5|2"
  "R2a|D24_m3_t5|t|dickson|14680063|3607823|6826282|3|5|1"
  "R2a|D24_m3_t5|t|dickson|14680063|3607823|6826282|3|5|2"
  "R2a|D31_m3_t5|t|dickson|2046820351|289962109|1299273082|3|5|1"
  "R2a|D31_m3_t5|t|dickson|2046820351|289962109|1299273082|3|5|2"
  "R2a|K0_m3_t5|t|kummer|786433|658649|132295|3|5|1"
  "R2a|K0_m3_t5|t|kummer|786433|658649|132295|3|5|2"
  "R2a|D24_m5_t3|t|dickson|14680063|3607823|6826282|5|3|2"
  "R2a|D24_m5_t3|t|dickson|14680063|3607823|6826282|5|3|5"
)

# emit SYS KIND P A B M T SEED BASE
emit() {
  nice -n 10 "$P1TOOL" emit --system "$1" --kind "$2" --p "$3" --a "$4" --b "$5" \
    --m "$6" --t "$7" --seed "$8" --out "$9.ms" --plant "$9.ms.plant.json" \
    > "$9.emit.json" 2> "$9.emit.err"
}

JOB=0
for line in "${JOBS[@]}"; do
  IFS='|' read -r part cell sys kind p a b m t s <<< "$line"
  dir="$RUN/$cell"
  mkdir -p "$dir" || hard_error "mkdir $dir"
  base="$dir/${sys}_s$s"
  JOB=$((JOB + 1))
  emit "$sys" "$kind" "$p" "$a" "$b" "$m" "$t" "$s" "$base"
  rc=$?
  [ "$rc" -eq 0 ] || hard_error "job $JOB $cell $sys seed $s emit exit $rc, $(tail -n 2 "$base.emit.err" | tr '\n' ' ')"
  VLIMIT=$VL TLIMIT=$TL bash "$RUNNER" "$base.ms" "$base.log" || hard_error "msolve_cell.sh exit $? on $base"
  nice -n 10 "$P1TOOL" parse "$base.log" > "$base.parse.json" || hard_error "parse exit $? on $base.log"
  msha=$(sha256sum "$base.ms" | cut -d' ' -f1)
  printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\n' \
    "$JOB" "$part" "$cell" "$sys" "$kind" "$p" "$a" "$b" "$m" "$t" "$s" "$msha" "$base.log" "$base.parse.json" >> "$RUN/jobs.tsv"
  cat "$base.parse.json" >> "$RUN/parsed.jsonl"
  echo "job $JOB $part $cell $sys seed $s $(grep -o '"status":"[^"]*"' "$base.parse.json") $(tail -n 1 "$base.log")"
done

echo "RUN_COMPLETE run_dir=$RUN jobs=$JOB"
for f in provenance.txt jobs.tsv parsed.jsonl; do
  sha256sum "$RUN/$f"
done
exit 0
