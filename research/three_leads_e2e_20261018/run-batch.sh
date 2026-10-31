#!/usr/bin/env bash
set -euo pipefail
# Run from the repository root. COUNTS ONLY: independent sessions may overlap;
# L0 wall measurements are not evidence for a timing or fastest-tool claim.
study=research/three_leads_e2e_20261018
mode=${1:?genus, controls, decomposition or replay}
mkdir -p "$study/sessions" "$study/run-logs" "$study/replay-receipts"
case "$mode" in
 genus) printf '%s\n' "$study"/genus-fixtures-v1/genus-{16,20,24,28,32,36,40}.json ;;
 genus-v2) printf '%s\n' "$study"/genus-fixtures-v2/genus-{16,20,24,28,32,36,40}.json ;;
 order-genus) printf '%s\n' "$study"/genus-order-fixtures-v1/order-genus-{16,20,24,28,32,36,40}.json ;;
 controls) printf '%s\n' "$study"/genus-fixtures-v1/genus-*-independent.json "$study"/genus-fixtures-v1/genus-*-unfiltered.json ;;
 controls-v2) printf '%s\n' "$study"/genus-fixtures-v2/genus-*-independent.json "$study"/genus-fixtures-v2/genus-*-unfiltered.json ;;
 order-controls) printf '%s\n' "$study"/genus-order-fixtures-v1/order-genus-*-independent.json "$study"/genus-order-fixtures-v1/order-genus-*-unfiltered.json ;;
 decomposition) printf '%s\n' "$study"/coverage-fixtures-v2/binary-*.json "$study"/coverage-fixtures-v2/odd-*.json "$study"/coverage-fixtures-v2/pkm_*.json ;;
 decomposition-v2) printf '%s\n' "$study"/decomposition-fixtures-v2/*.json ;;
 trace-filter) printf '%s\n' "$study"/trace-filter-fixtures-v1/*.json ;;
 replay) find "$study/sessions" -name session.json -print | sort ;;
 replay-complete) find "$study/sessions" -name session.json -print | sort | while IFS= read -r file; do name=${file%/session.json}; name=${name##*/}; if [[ ! -e "$study/replay-receipts/$name.json" ]] && jq -e '.status=="complete"' "$file" > /dev/null; then printf '%s\n' "$file"; fi; done ;;
 *) exit 2 ;;
esac | xargs -P "${JOBS:-4}" -r -I{} bash -c '
set -euo pipefail
study=research/three_leads_e2e_20261018
input=$1
if [[ $2 == replay* ]]; then
 dir=${input%/session.json}; name=${dir##*/}
 "${STUDY_BINARY:-$study/scratch/ecbench-frozen}" verify --dir "$dir" --replay-all > "$study/replay-receipts/$name.json" 2> "$study/run-logs/$name-replay.stderr"
else
 name=${input##*/}; name=${name%.json}
 if [[ $2 == *-v2 ]]; then name="$name-v2"; fi
 if [[ $2 == trace-filter ]]; then name="$name-filtered"; fi
 "${STUDY_BINARY:-$study/scratch/ecbench-frozen}" run --spec "$input" --out "$study/sessions/$name" --cpus none --allow-busy --settle 0 --lock "/tmp/crypto-three-leads-$name.lock" --quiet > "$study/run-logs/$name.log" 2>&1
fi
' _ '{}' "$mode"
