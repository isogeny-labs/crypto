#!/usr/bin/env bash
# Thin orchestration only. Every science subprocess has a separate busy lease.
set -euo pipefail
# An outer busy wrapper would deadlock the per-phase reservations below.
zero_parent=$PPID
for zero_depth in {1..16}; do
  test "$zero_parent" -gt 1 || break
  test -r "/proc/$zero_parent/cmdline" || break
  mapfile -d '' -t zero_parent_args < "/proc/$zero_parent/cmdline"
  for ((zero_arg=0; zero_arg+1<${#zero_parent_args[@]}; zero_arg++)); do
    if [[ ${zero_parent_args[zero_arg]} == */isolated_bench.py && ${zero_parent_args[zero_arg+1]} == busy ]]; then
      printf 'run-panel.sh manages per-phase reservations; invoke without an outer busy wrapper.\n' >&2
      exit 64
    fi
  done
  zero_parent=$(ps -o ppid= -p "$zero_parent" 2>/dev/null | tr -d ' ') || break
done
if test "${1:-}" != --frozen; then
  zero_out=${1:?fresh output directory}
  test ! -e "$zero_out"
  mkdir -p "$zero_out/metadata" "$zero_out/cases" "$zero_out/inputs"
  zero_out=$(cd "$zero_out" && pwd)
  cp "$0" "$zero_out/metadata/executed-script.sh"
  exec bash "$zero_out/metadata/executed-script.sh" --frozen "$zero_out" "${@:2}"
fi
shift
zero_out=$1
zero_public=$(cd "${2:?public input directory}" && pwd)
zero_reference=$(cd "${3:?immutable five-zero reference directory}" && pwd)
zero_point_summary=${4:?immutable independent point-replay summary}
zero_gate=$(cd "${5:?successful native gate directory}" && pwd)
zero_repo=$(git rev-parse --show-toplevel)
zero_study=research/twist_index_calculus_20261020/zero_fiber_collector
export RAYON_NUM_THREADS=1
ulimit -c 0
test "$(cat "$zero_gate/metadata/exit")" = 0
sha256sum -c "$zero_gate/metadata/sources.sha256" > "$zero_out/metadata/gate-sources-unchanged-before.txt"
cp "$zero_gate/bin/study" "$zero_out/runner"
chmod 500 "$zero_out/runner"
sha256sum "$zero_out/runner" > "$zero_out/metadata/runner.sha256"
cp "$zero_gate/metadata/sources.sha256" "$zero_out/metadata/compiled-sources.sha256"
cp "$zero_study/PROTOCOL.md" "$zero_out/metadata/frozen-protocol.md"
git rev-parse HEAD > "$zero_out/metadata/source-commit.txt"
date -u +%FT%TZ > "$zero_out/metadata/started-utc.txt"
sha256sum "$zero_point_summary" > "$zero_out/metadata/required-independent-replay.sha256"
test "$(cut -d ' ' -f 1 "$zero_out/metadata/required-independent-replay.sha256")" = 8b92c608edbb446095ceffb539e995da3f8dbae3e6ee95dd980b87b308a0989d
printf '%s\n' 'one science worker; RAYON_NUM_THREADS=1; 8589934592-byte address-space cap; 1200-second outer watchdog; 1180-second native cooperative deadline; shared migrated host, no timing admission' > "$zero_out/metadata/resource-envelope.txt"
for zero_p in 37 73 97 157 229 241 337; do
  for zero_seed in 0 1 2; do
    zero_id="p$zero_p-seed$zero_seed"
    cp "$zero_public/$zero_id.json" "$zero_out/inputs/$zero_id.json"
    for zero_arm in phi-l0-active double-l0-active; do
      sha256sum "$zero_reference/cases/$zero_id/$zero_arm.json" >> "$zero_out/metadata/reference-files.sha256"
    done
    sha256sum "$zero_reference/cases/$zero_id/summary.json" >> "$zero_out/metadata/reference-files.sha256"
  done
done
sha256sum "$zero_out"/inputs/*.json > "$zero_out/metadata/public-inputs.sha256"
zero_phase() {
  local zero_tag=$1 zero_rc
  shift
  printf '%q ' "$@" > "$zero_out/metadata/$zero_tag.command.txt"
  date -u +%FT%TZ > "$zero_out/metadata/$zero_tag.started-utc.txt"
  if "$@" > "$zero_out/metadata/$zero_tag.stdout" 2> "$zero_out/metadata/$zero_tag.stderr"; then
    zero_rc=0
  else
    zero_rc=$?
  fi
  printf '%s\n' "$zero_rc" > "$zero_out/metadata/$zero_tag.exit"
  date -u +%FT%TZ > "$zero_out/metadata/$zero_tag.finished-utc.txt"
  printf '%s exited %s\n' "$zero_tag" "$zero_rc"
  return "$zero_rc"
}
for zero_p in 37 73 97 157 229 241 337; do
  for zero_seed in 0 1 2; do
    zero_id="p$zero_p-seed$zero_seed"
    zero_input="$zero_out/inputs/$zero_id.json"
    zero_sha=$(sha256sum "$zero_input" | cut -d ' ' -f 1)
    for zero_arm in phi-l0-active double-l0-active; do
      zero_case="$zero_out/cases/$zero_id/$zero_arm"
      zero_tag="$zero_id-$zero_arm"
      mkdir -p "$zero_case"
      zero_phase "$zero_tag-collect" python3 "$zero_repo/tools/isolated_bench.py" busy -- \
        /usr/bin/time -v -o "$zero_out/metadata/$zero_tag-collect.time.txt" \
        prlimit --as=8589934592 -- timeout --signal=TERM --kill-after=10s 1200s \
        "$zero_out/runner" --zero-five-polynomial "$zero_input" "$zero_sha" "$zero_arm" "$zero_case/first" || true
      if test -f "$zero_case/first/record.json"; then
        zero_phase "$zero_tag-audit" python3 "$zero_repo/tools/isolated_bench.py" busy -- \
          /usr/bin/time -v -o "$zero_out/metadata/$zero_tag-audit.time.txt" \
          prlimit --as=8589934592 -- timeout --signal=TERM --kill-after=10s 1200s \
          "$zero_out/runner" --zero-five-polynomial-audit "$zero_input" "$zero_sha" "$zero_case/first" \
          "$zero_reference/cases/$zero_id" "$zero_case/audit" || true
      else
        printf 'No candidate record exists; no complete set or zero yield inferred.\n' > "$zero_out/metadata/$zero_tag-audit.not-attempted.txt"
      fi
    done
  done
done
zero_terminal=0
zero_phase summarize python3 "$zero_repo/tools/isolated_bench.py" busy -- \
  prlimit --as=8589934592 -- timeout --signal=TERM --kill-after=10s 1200s \
  "$zero_out/runner" --zero-five-polynomial-summary "$zero_out" "$zero_out/summary.json" || zero_terminal=$?
sha256sum -c "$zero_out/metadata/public-inputs.sha256" > "$zero_out/metadata/public-inputs-unchanged.txt"
sha256sum -c "$zero_out/metadata/reference-files.sha256" > "$zero_out/metadata/references-unchanged.txt"
sha256sum -c "$zero_out/metadata/compiled-sources.sha256" > "$zero_out/metadata/compiled-sources-unchanged.txt"
date -u +%FT%TZ > "$zero_out/metadata/finished-utc.txt"
printf '%s\n' "$zero_terminal" > "$zero_out/metadata/terminal-exit"
printf 'Panel terminal; consult native summary status. No admitted timing or target-recovery claim.\n' > "$zero_out/terminal.txt"
exit "$zero_terminal"
