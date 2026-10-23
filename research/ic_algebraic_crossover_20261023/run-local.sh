#!/usr/bin/env bash
# Thin orchestration only. All algebra, counters and verification are native.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
study=research/ic_algebraic_crossover_20261023
mode=${1:?selftest, structural, or guess}
run=${2:?fresh output directory}
test ! -e "$run"
mkdir -p "$run"
bin=target/release/examples/ic_algebraic_followup
test -x "$bin"
timer=${IC_TIMER:-/usr/bin/time}
if test ! -x "$timer"; then timer=/workspace/.tools/usr/bin/time; fi
test -x "$timer"
"$timer" --version > "$run/timer-version.txt"
git rev-parse HEAD > "$run/source-commit.txt"
sha256sum "$bin" examples/ic_algebraic_followup.rs "$study/PROTOCOL.md" \
  "$study/inputs/guess-ell8.jsonl" "$study/run-local.sh" > "$run/hashes.txt"
sha256sum "$timer" >> "$run/hashes.txt"
if test -f examples/ic_algebraic_followup/exact.rs; then
  sha256sum examples/ic_algebraic_followup/*.rs >> "$run/hashes.txt"
fi
uname -a > "$run/host.txt"
lscpu >> "$run/host.txt"
cat /sys/fs/cgroup/memory.max > "$run/cgroup-memory-max.txt"
date -u +%FT%TZ > "$run/started.txt"
seqno=0
run_one() {
  local cell="$run/cell-$(printf '%04d' "$seqno")" rc
  seqno=$((seqno + 1))
  mkdir "$cell"
  jq -n --args '{argv:$ARGS.positional,timeout_seconds:900,address_space_bytes:12884901888,isolation:"L0"}' \
    -- "$bin" "$@" > "$cell/command.json"
  date -u +%FT%TZ > "$cell/started.txt"
  set +e
  "$timer" -v -o "$cell/rusage.txt" \
    timeout --kill-after=15s 900s prlimit --as=12884901888 -- "$bin" "$@" \
    > "$cell/rows.jsonl" 2> "$cell/stderr.txt"
  rc=$?
  set -e
  jq -n --argjson rc "$rc" --arg ended "$(date -u +%FT%TZ)" \
    '{exit_code:$rc,ended:$ended,status:(if $rc==0 then "finished" elif $rc==124 then "timeout" elif $rc==137 then "killed_cause_unconfirmed" else "failed" end)}' \
    > "$cell/status.json"
  printf '%s %s\n' "$cell" "$rc"
}
case "$mode" in
  selftest) run_one selftest ;;
  single) run_one "${@:3}" ;;
  verify) run_one verify-fixtures --input "${3:?input file}" ;;
  fixtures)
    for cell in 3:25:9 4:17:6 4:18:6 3:34:8 3:36:8 4:38:6 6:10:3 5:15:3 10:10:2; do
      IFS=: read -r m n ell <<< "$cell"
      for family in prefix geometric geomtraceu random; do
        run_one fixtures --m "$m" --n "$n" --ell "$ell" --family "$family" --seed 20261023 --draws 32
      done
    done ;;
  structural)
    for n in 13 19 23 31; do
      for ell in 2 3 4; do
        for family in prefix geometric geomtraceu random; do
          for rep in 0 1; do
            run_one profile --n "$n" --ell "$ell" --family "$family" --rep "$rep" \
              --seed 20261023 --targets 4 --degree 4 --max-cols 30000
          done
        done
      done
    done ;;
  guess)
    k=${3:?guess count 0..4}
    case "$k" in
      0) vars=none; cap=140000 ;;
      1) vars=7; cap=140000 ;;
      2) vars=7,15; cap=140000 ;;
      3) vars=7,15,23; cap=140000 ;;
      4) vars=7,15,23,24; cap=130000 ;;
      *) exit 2 ;;
    esac
    for draw in 0 3 5 6; do
      for ((branch=0; branch<(1<<k); branch++)); do
        run_one guess --input "$study/inputs/guess-ell8.jsonl" --draw "$draw" \
          --vars "$vars" --degree 4 --max-cols "$cap" \
          --first-branch "$branch" --end-branch "$((branch+1))"
      done
    done ;;
  *) exit 2 ;;
esac
date -u +%FT%TZ > "$run/ended.txt"
