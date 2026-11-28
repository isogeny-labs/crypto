set -euo pipefail
STD=sessions/c1-e2e-m5-std; TW=sessions/c1-e2e-m5-tw; E=./ecbench-freeze-234e619
for w in 0 0.1 1; do
  # C-tw-vs-std (decision): operations, cross-session, paired by (workload, round) seed.
  "$E" compare --dir "$STD" --a ic-std --b ic-tw-a --b-dir "$TW" --weight "$w" --save
  # C-gap-rho (distance remaining), each IC arm against both strong-rho arms.
  for ref in rho-strong-l1 rho-strong-l16; do
    "$E" compare --dir "$STD" --a "$ref" --b ic-std --weight "$w" --save
    "$E" compare --dir "$TW" --a "$ref" --b ic-tw-a --weight "$w" --save
  done
  "$E" compare --dir "$STD" --a rho-strong-l1 --b rho-strong-l16 --weight "$w" --save
  # C-aa-line (noise floor of the line draw).
  "$E" compare --dir "$TW" --a ic-tw-a --b ic-tw-b --weight "$w" --save
done
# Wall A/A per session (descriptive unless L2 and outside the A/A interval).
"$E" compare --dir "$TW" --a ic-tw-a --b ic-tw-a-aa --save
"$E" compare --dir "$STD" --a rho-strong-l1 --b rho-strong-l1-aa --save
"$E" table --dir "$STD"
"$E" table --dir "$TW"
