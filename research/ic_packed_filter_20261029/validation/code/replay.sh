#!/usr/bin/env bash
# Validator replay (TASK-20261029-654f73, J5). Own script.
# Copies a session directory to a scratch location, checks the copy against
# the session's SHA256SUMS, and runs `ecbench verify --replay-all` (every
# measured deterministic run). Warm-ups are not replayable by this binary
# (an attempt with --replay-seq on each warm-up seq was refused: "not
# replayable (a warm-up, ...)"), so the validator's own scalar check
# (scalars.py) covers them instead. The frozen directory is never written.
# usage: replay.sh <ecbench> <eks-results dir> <scratch out dir> <tag>
set -euo pipefail
ECB="$1"; SRC="$2"; OUT="$3"; TAG="$4"
want=21701be6c420c3507a81767ed20f759e230f1fe5ec002ef3f7bb63a9c0b64503
have=$(sha256sum "$ECB" | cut -d' ' -f1)
[ "$have" = "$want" ] || { echo "STOP: ecbench sha256 $have != $want"; exit 3; }
mkdir -p "$OUT/$TAG"
rm -rf "$OUT/$TAG/session"; cp -r "$SRC/session" "$OUT/$TAG/session"
# the copy must hash as the session's SHA256SUMS says
( cd "$SRC" && grep ' session/' SHA256SUMS ) > "$OUT/$TAG/session-sums.txt"
( cd "$OUT/$TAG" && sha256sum -c session-sums.txt ) > "$OUT/$TAG/copy-check.txt"
# warm-up sequence numbers, read from the seq and warmup keys only
python3 -I - "$OUT/$TAG/session/records.jsonl" > "$OUT/$TAG/warmup-seqs.txt" <<'PY'
import json, sys
for line in open(sys.argv[1]):
    r = json.loads(line)
    if r["warmup"]:
        print(r["seq"])
PY
start=$(date -u +%FT%TZ)
set +e
"$ECB" verify --dir "$OUT/$TAG/session" --replay-all --out "$OUT/$TAG/replay-receipt.json" > "$OUT/$TAG/verify.stdout" 2> "$OUT/$TAG/verify.stderr"
rc=$?
set -e
end=$(date -u +%FT%TZ)
printf '{"tag":"%s","exit":%d,"started":"%s","finished":"%s","ecbench_sha256":"%s","warmup_records_not_replayed":%d}\n' \
  "$TAG" "$rc" "$start" "$end" "$have" "$(wc -l < "$OUT/$TAG/warmup-seqs.txt")" > "$OUT/$TAG/replay-run.json"
# confirm the copy is still byte-identical after the replay
( cd "$OUT/$TAG" && sha256sum -c session-sums.txt ) > "$OUT/$TAG/copy-check-after.txt"
