#!/usr/bin/env bash
# One explicitly chosen development unit request, never an automatic sweep.
# No retry, no timing interpretation, no EKS/benchmark qualification.
set -euo pipefail
[[ $# == 2 ]] || { echo 'Usage: submit_unit.sh PUBLIC_MAGMA_INPUT FRESH_OUTPUT_DIR' >&2; exit 2; }
[[ -f "$1" && ! -e "$2" ]] || { echo 'Input must exist and output must be fresh' >&2; exit 2; }
input_bytes=$(wc -c < "$1")
(( input_bytes <= 51200 )) || { echo 'Input exceeds advertised service cap' >&2; exit 2; }
mkdir -p "$2"
cp "$1" "$2/input.m"
jq -n --rawfile code "$1" '{code:$code}' > "$2/request.json"
sha256sum "$2/input.m" "$2/request.json" > "$2/request.sha256"
date -u +%Y-%m-%dT%H:%M:%SZ > "$2/submitted_at.txt"
set +e
curl --silent --show-error --max-time 25 --connect-timeout 10 \
  --header 'Content-Type: application/json' \
  --data-binary "@$2/request.json" \
  --dump-header "$2/response.headers" \
  --output "$2/response.json" \
  --write-out '%{http_code}\n' \
  https://calculator.magma-maths.org/execute > "$2/http-status.txt" 2> "$2/curl.stderr"
transport_exit=$?
set -e
printf '%s\n' "$transport_exit" > "$2/transport-exit.txt"
(( transport_exit == 0 )) || { echo 'Transport failed; request is not retried' >&2; exit 1; }
[[ $(cat "$2/http-status.txt") == 200 ]] || { echo 'HTTP rejection; request is not retried' >&2; exit 1; }
jq -e '.success == true and .exit_code == 0 and .truncated == false and .error == null and ((.warnings // []) | length) == 0' \
  "$2/response.json" > "$2/response-admission.txt"
jq -er '.stdout' "$2/response.json" > "$2/stdout.log"
echo 'CAS unit returned without reported error; exact native correctness checks still required.'
