#!/usr/bin/env bash
# Amendment 15: validate completion and the registered draw-0 identity gate.
set -euo pipefail
out=${1:?output directory required}
reference=${2:?reference JSONL required}
fail() {
  jq -n --arg reason "$1" '{schema:"e12b_completion/v1",passed:false,reason:$reason}' >"$out/completion_check.json"
  printf 'completion_failed: %s\n' "$1" >>"$out/status"
  exit 1
}
grep -qx 'gate_passed' "$out/status" || fail 'original gates missing'
grep -qx 'done' "$out/status" || fail 'original wrapper incomplete'
grep -qx '3:24:7 exit=0' "$out/cells_status" || fail 'cell did not exit successfully'
jq -e -s 'length == 4 and (map(.draw) == [0,1,2,3]) and
  all(.[]; .schema == "hky_par_newcell/v1" and .m == 3 and .n == 24 and
    .ell == 7 and .seed == 20261012 and .solutions == 0)' \
  "$out/E_3-24-7.jsonl" >/dev/null || fail 'four complete matching draws required'
jq -e -s 'length == 1 and .[0].draw == 0' "$reference" >/dev/null || fail 'invalid reference'
canonical='walk(if type == "object" then del(.secs) else . end)'
jq -S "$canonical" "$reference" >"$out/gate4_reference.canonical.json"
head -1 "$out/E_3-24-7.jsonl" | jq -S "$canonical" >"$out/gate4_actual.canonical.json"
diff -u "$out/gate4_reference.canonical.json" "$out/gate4_actual.canonical.json" \
  >"$out/gate4.diff" || fail 'draw 0 differs on a non-timing field'
jq -n '{schema:"e12b_completion/v1",passed:true,draws:4,gate4:"equal_non_timing_fields",scientific_reading:null}' \
  >"$out/completion_check.json"
printf 'completion_checks_passed\n' >>"$out/status"
