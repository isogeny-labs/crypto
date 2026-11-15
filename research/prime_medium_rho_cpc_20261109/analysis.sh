#!/usr/bin/env bash
# PROTOCOL.md sections 4-6, after the five sessions are fetched into sessions/.
# Gates G1-G4, the primary M (bound fit without --tier), the decision table,
# and the secondaries. ecbench and jq only. Run from the repository root.
#   ECBENCH=path/to/ecbench analysis.sh
set -euo pipefail
E=${ECBENCH:-./target/release/ecbench}
D=research/prime_medium_rho_cpc_20261109
S=$D/sessions
A=$D/analysis
mkdir -p "$A"
declare -A EXPECTED=([PC]=2016 [M1]=1344 [M2]=448 [M3]=112 [M4]=112)

# G1 audit (pod audit as admitted) and G4 completeness.
for j in PC M1 M2 M3 M4; do
  jq -e --argjson n "${EXPECTED[$j]}" '.ok==true and .session_status=="complete" and .records==$n
    and (.problems|length)==0 and all(.replays[]; .reproduced==true)' "$S/$j.pod/audit.json" >/dev/null \
    || { echo "G1/G4 FAIL: $j audit" >&2; echo '{"G1":false}' > "$A/gates.json"; exit 61; }
  # Independent local re-audit (every run replayed), as the parents did.
  "$E" verify --dir "$S/$j" --replay-all --out "$S/$j.audit.json"
  jq -e '.ok==true and (.problems|length)==0 and all(.replays[]; .reproduced==true)' "$S/$j.audit.json" >/dev/null \
    || { echo "G1 FAIL: $j local re-audit" >&2; echo '{"G1":false}' > "$A/gates.json"; exit 62; }
done

# G2 A/A: identical counts run for run.
aa() { # DIR A B
  jq -s --arg a "$2" --arg b "$3" '
    [ .[] | select(.warmup|not) | select(.arm==$a or .arm==$b) ]
    | group_by([.workload.workload_id, .round])
    | map({a: (map(select(.arm==$a))[0].cost.total_gae), b: (map(select(.arm==$b))[0].cost.total_gae)})
    | {pairs: length, identical: (map(select(.a==.b)) | length)}' "$1/records.jsonl"
}
g2=$(jq -n --argjson pc "$(aa $S/PC bsgs-neg bsgs-neg-aa)" \
  --argjson m "$(for j in M1 M2 M3 M4; do aa $S/$j rho-neg rho-neg-aa; done | jq -s '{pairs: (map(.pairs)|add), identical: (map(.identical)|add)}')" \
  '{pc: $pc, m: $m, pass: ($pc.pairs > 0 and $pc.pairs == $pc.identical and $m.pairs > 0 and $m.pairs == $m.identical)}')

# G3c (a) closed forms, (b) detection by ecbench compare.
"$D/gate_pc.sh" "$S/PC/records.jsonl" > "$A/gate_pc.json"
"$E" compare --dir "$S/PC" --a bsgs-neg --b bsgs-tb --save > "$A/compare-pc.txt"
pc_cmp=$(ls "$S/PC/comparisons/"bsgs-neg__bsgs-tb*.json | head -1)
g3=$(jq -n --slurpfile g "$A/gate_pc.json" --slurpfile c "$pc_cmp" '
  {exact: $g[0].exact, records: $g[0].records, expected_records: 1728,
   ratio: $c[0].ops.ratio_b_over_a, ci95: $c[0].ops.ci95,
   pass: ($g[0].exact == 1728 and $g[0].records == 1728 and ($c[0].ops.ci95[0] > 1.2))}')

jq -n --argjson g2 "$g2" --argjson g3 "$g3" '{G1: true, G4: true, G2: $g2, G3c: $g3,
  all_pass: ($g2.pass and $g3.pass)}' > "$A/gates.json"

# Primary: M from the bound fit over M1-M4, without --tier, at w = 0, 0.1 and 1.
dirs=""; for j in M1 M2 M3 M4; do dirs="$dirs --dir $S/$j --audit $S/$j.audit.json"; done
for w in 0 0.1 1; do
  case $w in 0) sfx=; wa=;; *) sfx="-w$w"; wa="--weight $w";; esac
  # shellcheck disable=SC2086
  "$E" bound fit $dirs --arm rho-neg --label "rho.negation, prime medium tier, CPC 2026-11-09$sfx" $wa \
    --root . --out "$A/bound-rho-neg$sfx.json" > /dev/null
  "$E" bound check --root . --record "$A/bound-rho-neg$sfx.json"
done
jq --slurpfile g "$A/gates.json" '.constant.ratio_to_floor as $m |
  {M: $m.value, ci95: $m.ci95,
   verdict: (if ($g[0].all_pass|not) then "inconclusive"
     elif ($m.value <= 1.10 and $m.ci95[1] < 1.30) then "setup_excess"
     elif ($m.value >= 1.30 and $m.ci95[0] > 1.10) then "walk_shortfall"
     elif ($m.ci95[0] > 1.10 and $m.value < 1.30) then "intermediate_excess"
     else "unresolved" end),
   within_1_10_strict: ($m.ci95[1] <= 1.10)}' "$A/bound-rho-neg.json" > "$A/verdict.json"

# Secondaries.
for j in M1 M2 M3 M4; do "$E" compare --dir "$S/$j" --a rho-neg --b rho-plain --save > "$A/compare-$j-plain.txt"; done
"$E" bound fit $dirs --arm rho-plain --label "rho.plain, prime medium tier, CPC 2026-11-09" --root . --out "$A/bound-rho-plain.json" > /dev/null
P=research/prime_medium_tier_rho_20261107/sessions
pdirs="$dirs"; for j in J1 J2 J3 J4 J5 J6; do pdirs="$pdirs --dir $P/$j --audit $P/$j.audit.json"; done
# shellcheck disable=SC2086
"$E" bound fit $pdirs --arm rho-neg --label "rho.negation, prime medium tier, parents + CPC pooled" \
  --root . --out "$A/bound-rho-neg-pooled.json" > /dev/null
cat "$A/gates.json" "$A/verdict.json"
