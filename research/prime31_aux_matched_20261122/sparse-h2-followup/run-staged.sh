#!/usr/bin/env bash
# A staged selection is an additional study gate; the existing runner retains
# its publication, checkpoint, budget, process and source-identity requirements.
set -euo pipefail
export LC_ALL=C
if [[ $# != 3 || "$1" != run ]]; then
  echo 'usage: run-staged.sh run ABSOLUTE_CELL.json ABSOLUTE_NEW_OUTPUT_DIR' >&2
  exit 64
fi
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
root=$(git -C "$here" rev-parse --show-toplevel)
cell=$(realpath "$2")
output=$3
if [[ "$output" != /* || -e "$output" || -L "$output" ]]; then
  echo 'Output must be a new absolute path; existing receipts cannot be reused.' >&2
  exit 64
fi
: "${PRIME31_SELECTION:?path to the published admitted stage selection}"
selection=$(realpath "$PRIME31_SELECTION")
[[ "$cell" == "$root/"* && "$selection" == "$root/"* && -f "$selection" ]] || exit 65
relative=${selection#"$root/"}
git -C "$root" ls-files --error-unmatch -- "$relative" >/dev/null
selected_hash=$(sha256sum "$selection" | awk '{print $1}')
[[ "$selected_hash" == "$(git -C "$root" cat-file blob "HEAD:$relative" | sha256sum | awk '{print $1}')" ]] || exit 65
cell_hash=$(sha256sum "$cell" | awk '{print $1}')
stage=$(jq -er 'select(.schema=="prime31-isogeny-choice.cell/v2") | .stage' "$cell")
id=$(jq -er '.cell_id' "$cell")
jq -e --arg stage "$stage" --arg id "$id" --arg hash "$cell_hash" '
  .schema=="prime31-isogeny-choice.selection/v1" and
  .status=="admitted" and .stage==$stage and .retries==0 and
  (.aggregate_lane_ceiling_usd>0 and .aggregate_lane_ceiling_usd<=5) and
  (.canonical_protocol_sha256 | test("^[0-9a-f]{64}$")) and
  (.cells | length)>0 and
  ([.cells[] | select(.cell_id==$id and .input_sha256==$hash)] | length)==1 and
  (if .stage=="complete_recovery" then
     (.pilot_receipt_sha256 | test("^[0-9a-f]{64}$")) else true end)
' "$selection" >/dev/null
deadline=$(date -u -d "$(jq -er '.original_absolute_deadline_utc' "$selection")" +%s)
now=$(date -u +%s)
(( now + 120 <= deadline )) || { echo 'Original deadline cannot fit this cell.' >&2; exit 65; }
retain_selection() {
  local code=$?
  trap - EXIT
  if [[ -d "$output" && -f "$output/SHA256SUMS" ]]; then
    cp "$selection" "$output/stage-selection.json"
    (cd "$output" && sha256sum stage-selection.json) >> "$output/SHA256SUMS"
  fi
  exit "$code"
}
trap retain_selection EXIT
bash "$here/../isogeny-choice/run-choice.sh" run "$cell" "$output"
