#!/usr/bin/env bash
# Continue on verified smaller cofactors without restarting the curve budget.
set -euo pipefail
here=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
record=$1
out=$2
b1=$3
left=$4
mkdir -p "$out"
[[ -z $(find "$out" -mindepth 1 -maxdepth 1 -print -quit) ]] || exit 1
cp "$record" "$out/input-partition.json"
current="$out/input-partition.json"
round=0
while ((left > 0)) && jq -e '.status == "partial"' "$current" >/dev/null; do
  round=$((round+1))
  attempt="$out/round-$round"
  value=$(jq -r '.residuals[0][0]' "$current")
  bash "$here/run_ecm.sh" "$value" "$attempt" "$b1" "$left"
  used=$(awk '/^Using B1=/ {n++} END {print n+0}' "$attempt/stdout.txt")
  ((used > 0 && used <= left)) || { echo 'Invalid ECM curve accounting' >&2; exit 1; }
  left=$((left-used))
  [[ -s $attempt/candidate-factors.txt ]] || break
  snapshot="$attempt/certified"
  mkdir -p "$snapshot/certs-final"
  jq -r --arg out "$snapshot" '"ID=\""+.id+"\"; N="+.value+"; BASE_PARTS=["+([(.factors+.residuals)[]|.[0] as $p|range(.[1])|$p]|join(","))+"]; OUT=\""+$out+"\";"' "$current" > "$snapshot/input.gp"
  jq -Rs -r '"EXTRA_FACTORS=["+(split("\n")|map(select(test("^[0-9]+$")))|join(","))+"];"' "$attempt/candidate-factors.txt" >> "$snapshot/input.gp"
  CERTIFY_INPUT="$snapshot/input.gp" "${GP_BIN:-gp}" -q -f "$here/certify_parts.gp" > "$snapshot/result.json" 2> "$snapshot/stderr.txt"
  jq -e '.product_verified and .known_factors_prime and .certificates_replay_valid' "$snapshot/result.json" >/dev/null
  current="$snapshot/result.json"
done
cp "$current" "$out/result.json"
jq -n --argjson b1 "$b1" --argjson remaining "$left" --argjson rounds "$round" \
  '{b1:$b1,unused_curve_allocation:$remaining,cofactor_attempts:$rounds}' > "$out/accounting.json"
echo "Continuation finished: $(jq -r '.id + ": " + .status' "$current")"
