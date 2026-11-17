#!/usr/bin/env bash
# Independent validator for the root-finder round (docs/VALIDATION_GATE.md).
#
# Recomputes the headline numbers straight from the sessions' records.jsonl
# with jq alone (no code shared with examples/pkm_rootfind_analyze.rs) and
# compares them with the analyser's ANALYSIS.json. Writes the validator
# report and the replay certificate (from the committed replay-all audits)
# into the benchmarks entry.
#
# Usage: research/pkm_tower_rootfind_20261018/validate.sh OUT_DIR
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
D=research/pkm_tower_rootfind_20261018
OUT=${1:?OUT_DIR}
mkdir -p "$OUT"
sha() { sha256sum "$1" | cut -d' ' -f1; }

# median of a JSON array of numbers
MEDIAN='def median: sort | if length == 0 then null elif length % 2 == 1 then .[length/2|floor] else (.[length/2 - 1] + .[length/2]) / 2 end;'

headline='[]'
raw='[]'
for s in P5 P6 P7 P8; do
  R=$D/sessions/$s/records.jsonl
  # The gate resolves paths relative to the evidence directory.
  raw=$(jq -c --arg p "$(realpath --relative-to="$OUT" "$R")" --arg h "$(sha "$R")" '. + [{path: $p, sha256: $h}]' <<<"$raw")
  recomputed=$(jq -s "$MEDIAN"'
    map(select(.warmup | not) | select(.outcome.status == "verified"))
    | (map(select(.arm == "f4")) | map(.cost.s) | median) as $f4
    | (map(select(.arm == "rootfind-subtract")) | map(.cost.s) | median) as $rs
    | (map(select(.arm == "rho-negation")) | map(.cost.s) | median) as $rho
    | (group_by(.workload.workload_id)
       | map((map(select(.arm == "rootfind-subtract"))[0].cost.s) / (map(select(.arm == "f4"))[0].cost.s))
       | median) as $ratio
    | {f4_s: $f4, rootfind_subtract_s: $rs, rho_s: $rho, ratio: $ratio}' "$R")
  claimed=$(jq --arg s "$s" '.sessions[$s].cells[0]
    | {f4_s: .arms.f4.s.median,
       rootfind_subtract_s: .arms["rootfind-subtract"].s.median,
       rho_s: .arms["rho-negation"].s.median,
       ratio: .questions.q1_s_root_over_groebner["rootfind-subtract/f4"].s_ratio.median}' $D/ANALYSIS.json)
  headline=$(jq -c --arg s "$s" --argjson c "$claimed" --argjson r "$recomputed" '. + [
      {name: ($s + " median S, tower F4"), claimed: $c.f4_s, recomputed: $r.f4_s},
      {name: ($s + " median S, rootfind-subtract"), claimed: $c.rootfind_subtract_s, recomputed: $r.rootfind_subtract_s},
      {name: ($s + " median S, rho-negation"), claimed: $c.rho_s, recomputed: $r.rho_s},
      {name: ($s + " median per-target S ratio rootfind-subtract / F4"), claimed: $c.ratio, recomputed: $r.ratio}
    ]' <<<"$headline")
done
status=$(jq -r 'if all(.[]; (.claimed - .recomputed | fabs) <= 1e-9 * ((.claimed | fabs) + 1)) then "pass" else "fail" end' <<<"$headline")
jq -n --arg st "$status" --argjson h "$headline" --argjson r "$raw" '{
  status: $st,
  independent: true,
  method: "jq recompute from records.jsonl (research/pkm_tower_rootfind_20261018/validate.sh); shares no code with the analyser",
  raw_data: $r,
  headline: $h
}' > "$OUT/validator-report.json"

# Replay certificate from the committed `ecbench verify --replay-all` audits.
runs='[]'
for s in P5 P6 P7 P8; do
  A=$D/audits/$s-replay-all.json
  jq -e '.ok == true and (.problems | length) == 0' "$A" >/dev/null
  runs=$(jq -c --slurpfile a "$A" --arg s "$s" --arg ap "$(realpath --relative-to="$OUT" "$A")" --arg ah "$(sha "$A")" \
    '. + [$a[0].replays[] | {session: $s, seq, record_id, reproduced, audit: $ap, audit_sha256: $ah}]' <<<"$runs")
done
jq -n --argjson r "$runs" '{
  status: (if all($r[]; .reproduced) then "verified" else "fail" end),
  independent: true,
  scalars_verified: true,
  how: "ecbench verify --replay-all re-executed every measured run in a fresh child process from the frozen spec and inputs, required identical counts, and re-verified each recovered scalar against its public target",
  runs: $r
}' > "$OUT/replay-certificate.json"
echo "validator: $status; replays: $(jq '.runs | length' "$OUT/replay-certificate.json")"
