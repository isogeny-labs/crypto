#!/usr/bin/env bash
# Copy runs/large/* from the pod supervisor into runs/large/. Usage: POD_URL=... POD_TOKEN_FILE=... fetch_large.sh
set -euo pipefail
cd "$(dirname "$0")/.."
mkdir -p runs/large
T=$(cat "$POD_TOKEN_FILE")
for f in $(curl -sS -m 30 -H "Authorization: Bearer $T" "$POD_URL/list" | python3 -c "import json,sys; [print(x) for x in json.load(sys.stdin) if x.startswith('runs/large/')]"); do
  curl -sS -m 120 -H "Authorization: Bearer $T" "$POD_URL/file?path=$f" -o "$f"
done
ls runs/large | wc -l
