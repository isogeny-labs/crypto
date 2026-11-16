#!/usr/bin/env bash
# Metadata parser plus exact native GMP/OpenSSL relation replay; no CAS calls.
set -euo pipefail
[[ $# == 4 ]] || { echo 'Usage: replay_batch.sh ADAPTER DEPTH ORIGINAL_STDOUT FRESH_OUTPUT_DIR' >&2; exit 2; }
adapter=$1
depth=$2
raw_stdout=$3
result_dir=$4
[[ -x "$adapter" && -f "$raw_stdout" && ! -e "$result_dir" ]] || { echo 'Invalid input or nonfresh output' >&2; exit 2; }
[[ $depth == 1 || $depth == 2 ]] || { echo 'Only registered complete unit depths1/2 admitted' >&2; exit 2; }
mkdir -p "$result_dir"
awk '
  /^BEGIN_ARM / {
    if (inside || ($2!="dense" && $2!="tower" && $2!="paired" && $2!="oriented") || seen[$2]++) exit 2;
    inside=$2; count++; next
  }
  /^END_ARM / { if (inside!=$2) exit 2; inside=""; next }
  !inside && $0 !~ /^[[:space:]]*$/ { exit 2 }
  END { if (inside || count!=4) exit 2 }
' "$raw_stdout"
for arm in dense tower paired oriented; do
  awk -v arm="$arm" '
    $0 == "BEGIN_ARM " arm { if (seen || inside) exit 2; seen=1; inside=1; next }
    $0 == "END_ARM " arm { if (!inside) exit 2; inside=0; ended=1; next }
    inside { print }
    END { if (!seen || !ended || inside) exit 2 }
  ' "$raw_stdout" > "$result_dir/$arm.log"
  "$adapter" replay "$depth" 2 planted "$result_dir/$arm.log" --producer-unverified > "$result_dir/$arm-replay.json"
  expected_tuples=2
  [[ $depth == 2 ]] && expected_tuples=1
  expected_solutions=$expected_tuples
  [[ $arm == paired ]] && expected_solutions=$((4 * expected_tuples))
  jq -e --argjson tuples "$expected_tuples" --argjson solutions "$expected_solutions" \
    '.unique_ordered_tuples == $tuples and .replayed_cas_solutions == $solutions' \
    "$result_dir/$arm-replay.json" > "$result_dir/$arm-set-check.txt"
done
sha256sum "$raw_stdout" > "$result_dir/original-stdout.sha256"
echo 'All registered complete counts and exact point replays match (producer assertion).'
