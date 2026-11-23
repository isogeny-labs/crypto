#!/usr/bin/env bash
# run_cells.sh -- P1 claim-run driver (orchestration only; every number comes from p1tool and msolve).
#
# Environment (both required):
#   P1TOOL     absolute path of the built p1tool binary
#   RUNS_ROOT  repository-relative directory that will hold run-<UTC start>/
# Run from the repository root.
#
# Protocol: research/split_prime_ecdlp_20261107/followups/p1_rr_dickson/PROTOCOL.md
# (Addendum A, commands per cell; amendment 1 curves, amendment 2 Kummer prime-order curve,
# amendment 3 seed skip rule).
#
# Seed rule (amendment 3): seeds are the integers 1, 2, ..., 50 in order; a cell takes the
# first two admissible seeds; every skipped seed is recorded with its reason in skips.tsv;
# a cell with no admissible seed in 1..50 is recorded "not_admissible" and nothing is imputed.
# A seed is skipped only when `p1tool emit` panics with "base set too small" or
# "no admissible planted instance found"; any other emit failure stops the run (exit 1).
# Admissibility is a property of (prime, m, t, seed): RR is emitted first, T on the same seed.
#
# Job order: cells (m, t) = (3,3), (3,4), (3,5), (5,3); inside a cell the configurations
# D24 (Dickson T24), D31 (Dickson T31), K0 (Kummer p0, prime-order curve), each with system rr
# then system t, each system on the first admissible seed then the second. (5,3) runs on D24 only.
# One msolve job at a time. msolve limits: VLIMIT=1572864 (KiB), TLIMIT=1200 (s), as in the protocol.
set -u

: "${P1TOOL:?P1TOOL not set}"
: "${RUNS_ROOT:?RUNS_ROOT not set}"
HERE=$(cd "$(dirname "$0")" && pwd)
RUNNER="$HERE/msolve_cell.sh"

mkdir -p "$RUNS_ROOT" || exit 2
RUN="$RUNS_ROOT/run-$(date -u +%Y%m%dT%H%M%SZ)"
mkdir "$RUN" || exit 2
echo "RUN_DIR=$RUN"

hard_error() {
  echo "HARD_ERROR: $*" | tee -a "$RUN/ERROR.txt"
  exit 1
}

# provenance
{
  echo "run_start_utc=$(date -u +%Y-%m-%dT%H:%M:%SZ)"
  echo "git_head=$(git rev-parse HEAD)"
  echo "p1tool_path=$P1TOOL"
  sha256sum "$P1TOOL"
  echo "addendum_A_development_binary_sha256=4ba83d9a5ab271ddbec38ea0e263d97457dab73997cfcc80f6abb33b98f8250f"
  sha256sum /usr/bin/msolve
  echo "VLIMIT=1572864 TLIMIT=1200"
  echo "nproc=$(nproc)"
  uname -a
} > "$RUN/provenance.txt" 2>&1
cat "$RUN/provenance.txt"

printf 'cell\tseed\temit_exit\treason\n' > "$RUN/skips.tsv"
printf 'cell\tadmissible_seeds\tskipped_seeds\tstatus\n' > "$RUN/cells.tsv"
printf 'job\tcell\tsystem\tkind\tp\ta\tb\tm\tt\tseed\tms_sha256\tlog\tparse_json\n' > "$RUN/jobs.tsv"
: > "$RUN/parsed.jsonl"

# emit SYS KIND P A B M T SEED BASE
emit() {
  nice -n 10 "$P1TOOL" emit --system "$1" --kind "$2" --p "$3" --a "$4" --b "$5" \
    --m "$6" --t "$7" --seed "$8" --out "$9.ms" --plant "$9.ms.plant.json" \
    > "$9.emit.json" 2> "$9.emit.err"
}

JOB=0
SEEDS=()
SKIPPED=""

# scan CELL KIND P A B M T : fills SEEDS (first two admissible) and SKIPPED
scan() {
  local cell=$1 kind=$2 p=$3 a=$4 b=$5 m=$6 t=$7 s rc reason dir
  dir="$RUN/$cell"
  mkdir -p "$dir" || hard_error "mkdir $dir"
  SEEDS=()
  SKIPPED=""
  for ((s = 1; s <= 50; s++)); do
    emit rr "$kind" "$p" "$a" "$b" "$m" "$t" "$s" "$dir/rr_s$s"
    rc=$?
    if [ "$rc" -ne 0 ]; then
      reason=$(grep -A1 'panicked at' "$dir/rr_s$s.emit.err" | tail -n 1)
      case "$reason" in
        "base set too small"* | "no admissible planted instance found"*)
          printf '%s\t%s\t%s\t%s\n' "$cell" "$s" "$rc" "$reason" >> "$RUN/skips.tsv"
          SKIPPED="${SKIPPED:+$SKIPPED,}$s"
          continue
          ;;
        *)
          hard_error "$cell seed $s rr emit exit $rc, reason: $reason"
          ;;
      esac
    fi
    emit t "$kind" "$p" "$a" "$b" "$m" "$t" "$s" "$dir/t_s$s"
    rc=$?
    [ "$rc" -eq 0 ] || hard_error "$cell seed $s t emit exit $rc after rr emit succeeded"
    SEEDS+=("$s")
    [ "${#SEEDS[@]}" -eq 2 ] && break
  done
}

# job CELL SYS KIND P A B M T SEED
job() {
  local cell=$1 sys=$2 kind=$3 p=$4 a=$5 b=$6 m=$7 t=$8 s=$9 base msha
  base="$RUN/$cell/${sys}_s$s"
  JOB=$((JOB + 1))
  VLIMIT=1572864 TLIMIT=1200 bash "$RUNNER" "$base.ms" "$base.log" || hard_error "msolve_cell.sh exit $? on $base"
  nice -n 10 "$P1TOOL" parse "$base.log" > "$base.parse.json" || hard_error "parse exit $? on $base.log"
  msha=$(sha256sum "$base.ms" | cut -d' ' -f1)
  printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\n' \
    "$JOB" "$cell" "$sys" "$kind" "$p" "$a" "$b" "$m" "$t" "$s" "$msha" "$base.log" "$base.parse.json" >> "$RUN/jobs.tsv"
  cat "$base.parse.json" >> "$RUN/parsed.jsonl"
  echo "job $JOB $cell $sys seed $s $(grep -o '"status":"[^"]*"' "$base.parse.json") $(tail -n 1 "$base.log")"
}

for mt in "3 3" "3 4" "3 5" "5 3"; do
  set -- $mt
  m=$1
  t=$2
  for cfg in D24 D31 K0; do
    if [ "$m" = 5 ] && [ "$cfg" != D24 ]; then
      continue
    fi
    case "$cfg" in
      D24) kind=dickson; p=14680063; a=3607823; b=6826282 ;;
      D31) kind=dickson; p=2046820351; a=289962109; b=1299273082 ;;
      K0) kind=kummer; p=786433; a=658649; b=132295 ;;
    esac
    cell="${cfg}_m${m}_t${t}"
    scan "$cell" "$kind" "$p" "$a" "$b" "$m" "$t"
    if [ "${#SEEDS[@]}" -eq 0 ]; then
      status=not_admissible
    elif [ "${#SEEDS[@]}" -eq 1 ]; then
      status=one_admissible_seed
    else
      status=two_admissible_seeds
    fi
    printf '%s\t%s\t%s\t%s\n' "$cell" "${SEEDS[*]:-}" "${SKIPPED:-}" "$status" >> "$RUN/cells.tsv"
    echo "cell $cell admissible_seeds=[${SEEDS[*]:-}] skipped_seeds=[${SKIPPED:-}] $status"
    for sys in rr t; do
      for s in "${SEEDS[@]:-}"; do
        [ -n "$s" ] || continue
        job "$cell" "$sys" "$kind" "$p" "$a" "$b" "$m" "$t" "$s"
      done
    done
  done
done

echo "RUN_COMPLETE run_dir=$RUN"
for f in provenance.txt cells.tsv skips.tsv jobs.tsv parsed.jsonl; do
  sha256sum "$RUN/$f"
done
exit 0
