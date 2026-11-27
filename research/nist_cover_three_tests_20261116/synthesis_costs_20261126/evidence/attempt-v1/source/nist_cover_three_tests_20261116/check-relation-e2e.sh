#!/bin/bash
set -eu
cd "$(dirname "$0")"
test "$#" -eq 1
mkdir "$1"
out=$(cd "$1" && pwd)
./check-relation-pipeline.sh "$out/relations"
./check-relation-transfer.sh "$out/relations" "$out/transfer"
wc -c "$out/relations/result.json" "$out/relations/checker-result.json" \
    "$out/transfer/result.json" "$out/transfer/checker-result.json" > "$out/output-bytes.txt"
