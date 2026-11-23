#!/usr/bin/env bash
# Frozen analysis for protocol version 6 (IC-LINE-LAYOUT-E2E-FP28-V6-20261106).
#
# usage: analyze-v6.sh ECBENCH OUT_DIR [--counted COUNTED_SESSION] SESSION1 SESSION2
#
# ECBENCH          the ecbench binary extracted from the pinned image; its sha256 is
#                  checked against the pinned value below (exit 2 on a mismatch).
# OUT_DIR          written: per-session replay and compare files, verdict.json and,
#                  with --counted, counted-*.json and counted-band.txt.
# SESSION1/2       the `session/` directories of the two scientific wall jobs. The job's
#                  own `control-comparison.json` (written by run-ecbench next to `session/`,
#                  i.e. `dirname SESSION/control-comparison.json`) is read for the A/A gate.
# COUNTED_SESSION  the `session/` directory of the counted companion job (optional).
#
# Preconditions that are NOT checked here and are read by the director's card
# before this script runs (PROTOCOL-V6.md, "Reading order"): the SHA256SUMS check,
# job-exit.json exit_code 0 from one pod for each wall job, input-spec.json equal
# to the frozen spec and build-info.json equal to the frozen build.
#
# Roles in the frozen specs are structural only (decision section 12.2): incumbent=baseline,
# incumbent-aa=control, candidate=candidate, pc-fb2x=reference. This script reads arms by
# NAME, never by role.
#
# Exit status: 0 whenever the analysis ran to the end, whatever the verdict (read
# verdict.json); 2 for a usage error or an ecbench binary that is not the pinned one.
set -euo pipefail

PINNED_ECBENCH_SHA256=73362eed4165d6db20fe1d398baa8fabefa2497aff6e78dab9ed8f31af31dd01
# Native counters (keys of phases[].native or phases[].work_native) that layout=lines is
# allowed to change, as a JSON array of strings. Filled from pre-freeze check F4 before the
# freeze; the empty list means no native counter may differ.
ALLOWED_NATIVE='[]'
WALL_RECORDS=576
WALL_PAIRS=128
CELLS_PER_ARM=144
SEED_CANDIDATE=2026110661
SEED_AA=2026110662
SEED_POSITIVE=2026110663
RESAMPLES=4000

usage() { echo "usage: analyze-v6.sh ECBENCH OUT_DIR [--counted COUNTED_SESSION] SESSION1 SESSION2" >&2; exit 2; }
[[ $# -ge 4 ]] || usage
ecbench=$1
out=$2
shift 2
counted=""
if [[ ${1:-} == --counted ]]; then
  [[ $# -ge 2 ]] || usage
  counted=$2
  shift 2
fi
[[ $# -eq 2 ]] || usage
[[ -f $ecbench && -x $ecbench ]] || { echo "ecbench is not an executable file: $ecbench" >&2; exit 2; }
[[ $(sha256sum "$ecbench" | cut -d' ' -f1) == "$PINNED_ECBENCH_SHA256" ]] \
  || { echo "ecbench sha256 is not $PINNED_ECBENCH_SHA256" >&2; exit 2; }
for s in "$@" ${counted:+"$counted"}; do
  [[ -f $s/records.jsonl && -f $s/session.json ]] || { echo "not a session directory: $s" >&2; exit 2; }
done
mkdir -p "$out"

# Projection of one record onto the fields that must agree between the incumbent and the
# candidate: recovered logarithm, total_gae, the unpriced-operation counts that `ecbench
# band` prices, and per phase adds, doubles, scalar_mults, gae and the native counters
# other than the allowed ones.
PROJ='def strip($allowed): with_entries(select(.key as $k | ($allowed | index($k)) | not));
def proj($allowed): {
  recovered: .outcome.recovered,
  total_gae: .cost.total_gae,
  unpriced: .cost.unpriced,
  phases: [.phases[] | {name, adds, doubles, scalar_mults, gae,
    native: ((.native // {}) | strip($allowed)),
    work_native: ((.work_native // {}) | strip($allowed))}]
};
def cell: [.workload.workload_id, .round, .warmup];'

# 1 when the candidate and incumbent records agree cell by cell (warm-ups included), else 0.
record_parity() {
  jq -s -e --argjson allowed "$ALLOWED_NATIVE" --argjson cells "$CELLS_PER_ARM" "$PROJ"'
    [ .[] | select(.arm == "incumbent" or .arm == "candidate") ]
    | group_by(cell)
    | (length == $cells)
      and all(.[]; length == 2 and ((map(.arm) | sort) == ["candidate", "incumbent"])
                   and ((map(proj($allowed)) | .[0]) == (map(proj($allowed)) | .[1])))
  ' "$1/records.jsonl" >/dev/null && echo 1 || echo 0
}

# 1 when the counted session carries, for the incumbent and the candidate, the same
# projected fields as the wall session in every (arm, workload, round, warm-up) cell.
cross_session_parity() {
  jq -n -e --slurpfile w "$1/records.jsonl" --slurpfile c "$2/records.jsonl" \
    --argjson allowed "$ALLOWED_NATIVE" --argjson cells "$CELLS_PER_ARM" "$PROJ"'
    def keyed(rs): [ rs[] | select(.arm == "incumbent" or .arm == "candidate")
                     | {key: ([.arm] + cell | map(tostring) | join("|")), value: proj($allowed)} ]
                   | from_entries;
    (keyed($w) ) as $a | (keyed($c)) as $b
    | (($a | length) == 2 * $cells) and (($a | keys) == ($b | keys)) and ($a == $b)
  ' >/dev/null && echo 1 || echo 0
}

run_compare() { # run_compare SESSION A B SEED OUTFILE
  "$ecbench" compare --dir "$1" --a "$2" --b "$3" --resamples "$RESAMPLES" --seed "$4" --json > "$5" 2> "$5.stderr" || true
  jq -e . "$5" >/dev/null 2>&1 || echo '{}' > "$5"
}
gate() { # gate FILE 'jq program' -> 1 or 0
  jq -e "$2" "$1" >/dev/null 2>&1 && echo 1 || echo 0
}

sessions_json='[]'
n=0
for session in "$@"; do
  n=$((n + 1))
  prefix="$out/session-$n"
  verify_rc=0
  "$ecbench" verify --dir "$session" --replay-all --exit-code --out "$prefix-replay.json" >/dev/null 2>&1 || verify_rc=$?
  [[ -f "$prefix-replay.json" ]] || echo '{}' > "$prefix-replay.json"
  run_compare "$session" incumbent candidate "$SEED_CANDIDATE" "$prefix-candidate.json"
  # Direct incumbent vs incumbent-aa is a self-A/A: its aa_* fields may be null, so this
  # file is descriptive only and no gate reads it (decision section 12.2).
  run_compare "$session" incumbent incumbent-aa "$SEED_AA" "$prefix-aa-self-descriptive.json"
  # The launcher's own admission comparison (pc-fb2x against incumbent-aa), written by
  # run-ecbench into the job output directory next to session/.
  control_src="$(dirname "${session%/}")/control-comparison.json"
  if [[ -f $control_src ]] && jq -e . "$control_src" >/dev/null 2>&1; then
    cp "$control_src" "$prefix-control-comparison.json"
  else
    echo '{}' > "$prefix-control-comparison.json"
  fi
  run_compare "$session" incumbent pc-fb2x "$SEED_POSITIVE" "$prefix-positive.json"

  g_replay=0
  if [[ $verify_rc == 0 ]]; then
    g_replay=$(jq -e '.ok == true and .regraded == true and .records_bound == true and .records > 0
      and .verified_records == .records and (.replays | length) > 0
      and all(.replays[]; .reproduced == true)' "$prefix-replay.json" >/dev/null 2>&1 && echo 1 || echo 0)
    [[ $(jq -s 'length' "$session/records.jsonl") == "$WALL_RECORDS" ]] || g_replay=0
  fi
  g_ops=$(gate "$prefix-candidate.json" '
    .ops.status == "ok" and .ops.same_seeds == true and .ops.unpaired_runs == 0
    and .ops.ratio_b_over_a == 1 and .ops.ci95 == [1, 1]')
  g_records=$(record_parity "$session")
  g_admission=$(jq -e --argjson pairs "$WALL_PAIRS" '
    .wall.status == "admitted" and .wall.pairs_below_level == 0 and .wall.pairs == $pairs' \
    "$prefix-candidate.json" >/dev/null 2>&1 && echo 1 || echo 0)
  # A/A gate (decision section 12.2): wall.aa_ci95 of the --a incumbent --b candidate comparison
  # (which must show aa_arms = {incumbent, incumbent-aa} and self_aa false) and wall.aa_ci95 of
  # control-comparison.json must each be non-null, contain 1 and lie inside [0.9, 1.1].
  g_aa=$(jq -s -e '
    def aa_ok: .wall.aa_ci95 != null
      and .wall.aa_ci95[0] <= 1 and .wall.aa_ci95[1] >= 1
      and .wall.aa_ci95[0] >= 0.9 and .wall.aa_ci95[1] <= 1.1;
    (.[0] | .wall.self_aa == false and ((.wall.aa_arms // []) | sort) == ["incumbent", "incumbent-aa"] and aa_ok)
    and (.[1] | aa_ok)' \
    "$prefix-candidate.json" "$prefix-control-comparison.json" >/dev/null 2>&1 && echo 1 || echo 0)
  g_positive=$(gate "$prefix-positive.json" '
    .ops.status == "ok" and .ops.same_seeds == true and .ops.ratio_b_over_a > 1
    and .wall.status == "admitted" and .wall.total_wall_ratio > 1 and .wall.total_wall_ci95[0] > 1')
  g_below=$(gate "$prefix-candidate.json" '
    .wall.total_wall_ratio < 1 and .wall.total_wall_ci95[1] < 1
    and .wall.corrected_ratio < 1 and .wall.corrected_ci95[1] < 1')
  g_noise=$(gate "$prefix-candidate.json" '.wall.outside_noise == true')

  # Verdict of this session. A failed measurement gate is invalid_measurement; with all
  # gates passed, an effect wholly below 1 and outside the A/A noise is supported_within_scope;
  # an effect inside the A/A noise is within_control_distribution; any other valid result
  # (outside the noise but an interval not wholly below 1) is inconclusive.
  if [[ $g_replay == 1 && $g_ops == 1 && $g_records == 1 && $g_admission == 1 && $g_aa == 1 && $g_positive == 1 ]]; then
    if [[ $g_below == 1 && $g_noise == 1 ]]; then v=supported_within_scope
    elif [[ $g_noise == 0 ]]; then v=within_control_distribution
    else v=inconclusive; fi
  else
    v=invalid_measurement
  fi
  sessions_json=$(jq -n --argjson acc "$sessions_json" --argjson n "$n" --arg session "$session" --arg v "$v" \
    --argjson replay "$g_replay" --argjson ops "$g_ops" --argjson records "$g_records" \
    --argjson admission "$g_admission" --argjson aa "$g_aa" --argjson positive "$g_positive" \
    --argjson below "$g_below" --argjson noise "$g_noise" --argjson verify_rc "$verify_rc" \
    '$acc + [{session: $n, dir: $session, verify_exit: $verify_rc, verdict: $v,
              gates: {replay_and_audit: ($replay == 1), counted_parity_ops: ($ops == 1),
                      counted_parity_records: ($records == 1), admission: ($admission == 1),
                      aa_inside_0_9_1_1: ($aa == 1), positive_control_detected: ($positive == 1),
                      total_and_corrected_ci_below_1: ($below == 1), outside_noise: ($noise == 1)}}]')
done

overall=$(jq -n -r --argjson s "$sessions_json" '
  if all($s[]; .verdict == "supported_within_scope") then "supported_within_scope"
  elif any($s[]; .verdict == "invalid_measurement") then "invalid_measurement"
  elif any($s[]; .verdict == "inconclusive") then "inconclusive"
  else "within_control_distribution" end')

counted_json=null
if [[ -n $counted ]]; then
  cverify_rc=0
  "$ecbench" verify --dir "$counted" --replay-all --exit-code --out "$out/counted-replay.json" >/dev/null 2>&1 || cverify_rc=$?
  [[ -f "$out/counted-replay.json" ]] || echo '{}' > "$out/counted-replay.json"
  c_audit=0
  if [[ $cverify_rc == 0 ]]; then
    c_audit=$(jq -e '.ok == true and .regraded == true and .records_bound == true and .records > 0
      and .verified_records == .records and (.replays | length) > 0
      and all(.replays[]; .reproduced == true)' "$out/counted-replay.json" >/dev/null 2>&1 && echo 1 || echo 0)
    [[ $(jq -s 'length' "$counted/records.jsonl") == "$WALL_RECORDS" ]] || c_audit=0
  fi
  c_invalid=$(jq -s -e '[.[] | select(.outcome.status != "verified" or .outcome.matches_target != true)] | length == 0' \
    "$counted/records.jsonl" >/dev/null 2>&1 && echo 1 || echo 0)
  c_cross=1
  for s in "$@"; do
    [[ $(cross_session_parity "$s" "$counted") == 1 ]] || c_cross=0
  done
  for w in 0 0.1 1; do
    # The bootstrap seed is the ecbench default: no counted seed is frozen.
    for pair in "incumbent candidate" "rho-neg incumbent" "rho-neg candidate" "rho-neg rho-neg-aa"; do
      read -r ca cb <<< "$pair"
      "$ecbench" compare --dir "$counted" --a "$ca" --b "$cb" --weight "$w" --resamples "$RESAMPLES" --json \
        > "$out/counted-compare-${ca}__${cb}-w$w.json" 2> "$out/counted-compare-${ca}__${cb}-w$w.stderr" || true
    done
  done
  "$ecbench" band --dir "$counted" --weights 0,0.1,1 --ranks --json "$out/counted-band.json" > "$out/counted-band.txt" 2>&1 || true
  c_ops=$(jq -s -e 'all(.[]; .ops.ratio_b_over_a == 1)' \
    "$out/counted-compare-incumbent__candidate-w0.json" \
    "$out/counted-compare-incumbent__candidate-w0.1.json" \
    "$out/counted-compare-incumbent__candidate-w1.json" >/dev/null 2>&1 && echo 1 || echo 0)
  counted_json=$(jq -n --arg dir "$counted" --argjson verify_rc "$cverify_rc" --argjson audit "$c_audit" \
    --argjson invalid "$c_invalid" --argjson cross "$c_cross" --argjson ops "$c_ops" \
    '{dir: $dir, verify_exit: $verify_rc,
      acceptance: {audit_and_replay: ($audit == 1), invalid_outcomes_empty: ($invalid == 1),
                   incumbent_candidate_records_equal_wall_sessions: ($cross == 1),
                   incumbent_candidate_ops_ratio_1_at_w0_0_1_1: ($ops == 1)},
      note: "inputs to the validation gate only; no wall field of this session is read or reported"}')
fi

jq -n --arg verdict "$overall" --argjson sessions "$sessions_json" --argjson counted "$counted_json" \
  --arg sha "$PINNED_ECBENCH_SHA256" --argjson allowed "$ALLOWED_NATIVE" \
  '{schema: "ic.line-layout.analysis/v6", protocol_id: "IC-LINE-LAYOUT-E2E-FP28-V6-20261106",
    ecbench_sha256: $sha, allowed_native_counters: $allowed,
    verdict: $verdict, sessions: $sessions, counted: $counted,
    rule: "both independent L2 wall sessions must pass replay and audit, exact counted parity (ops and record level), admission, the A/A interval aa_ci95 of the incumbent-vs-candidate comparison (aa_arms incumbent and incumbent-aa, self_aa false) and of control-comparison.json each non-null, containing 1 and inside [0.9,1.1], the pc-fb2x positive control and wholly-below-1 total and corrected intervals outside the A/A noise; per-session verdict precedence: invalid_measurement, supported_within_scope, within_control_distribution, inconclusive; overall: all supported, else invalid_measurement, else inconclusive, else within_control_distribution"}' \
  > "$out/verdict.json"
echo "$overall"
