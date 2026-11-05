#!/usr/bin/env bash
# Load ecdb/sage/generate.sage output into the deployed database: every
# finished unit's JSONL is split into chunks of whole lines (about 12 MB, so a
# gzipped chunk stays far below the 6 MB Lambda payload limit after base64),
# and each chunk is sent to the admin function as
#   {"action": "import_pack", "name": "generated", "data": <base64 gzip>}.
# The importer re-verifies every record and is idempotent, so a rerun only
# adds what is new.
#
#   AWS_REGION=us-west-2 ecdb/deploy/load-generated.sh /path/to/generator/out [set ...]
set -euo pipefail
out=${1:?usage: load-generated.sh OUT_DIR [set ...]}; shift || true
sets=("$@"); [ ${#sets[@]} -gt 0 ] || sets=(census-fp census-f2m crypto-prime pairing genus2)
aws=${AWS:-aws}; name=${ECDB_NAME:-ecdb}
work=$(mktemp -d); trap 'rm -rf "$work"' EXIT
log() { printf '%s %s\n' "$(date -u +%H:%M:%S)" "$*" >&2; }

for set in "${sets[@]}"; do
  dir="$out/$set"; [ -d "$dir" ] || { log "$set: no output yet"; continue; }
  # only units whose .done marker exists (a .jsonl without one is still being written)
  : > "$work/$set.jsonl"
  for done in "$dir"/*.done; do
    [ -e "$done" ] || continue
    cat "${done%.done}.jsonl" >> "$work/$set.jsonl"
  done
  n=$(wc -l < "$work/$set.jsonl"); log "$set: $n records"
  [ "$n" -gt 0 ] || continue
  split -C 12M -d -a 4 "$work/$set.jsonl" "$work/$set-part-"
  for part in "$work/$set-part-"*; do
    gzip -9 -n "$part"
    ev="$work/event.json"
    printf '{"action":"import_pack","name":"generated","data":"%s"}' "$(base64 -w0 "$part.gz")" > "$ev"
    resp="$work/resp.json"
    $aws lambda invoke --function-name "$name-admin" --cli-read-timeout 900 --payload "fileb://$ev" "$resp" \
      --query FunctionError --output text >&2
    log "$(basename "$part"): $(head -c 600 "$resp")"
    rm -f "$part.gz"
  done
  rm -f "$work/$set.jsonl"
done
