#!/usr/bin/env bash
# Driver for PROTOCOL.md. Strictly sequential, resumable: skips cells whose json or .fail exists.
# shell + jq. Usage: run_cells.sh   (reads cells.list next to it)
set -u
BIN=/home/user/frozen-bin/curve25519_family_scale
WANT=d66b85b0dcb60c2d1b551a2e3ce19de4cc76e6897ce359eafce1bf579606ca90
D=$(cd "$(dirname "$0")" && pwd); mkdir -p "$D/cells" "$D/stderr"
got=$(sha256sum "$BIN" | cut -d' ' -f1)
[ "$got" = "$WANT" ] || { echo "binary hash mismatch: $got" >&2; exit 2; }
while read -r stage k t G seed tr; do
  id="$stage-k$k-t$t-g$G-tr$tr-s$seed"
  [ -e "$D/cells/$id.json" ] || [ -e "$D/cells/$id.fail" ] && continue
  tmp=$(mktemp); s=$(date +%s); up=$(uptime | sed 's/.*load average: //')
  ( ulimit -v 3500000; timeout 1000 "$BIN" f4cost "$k" isogeny "$t" "$G" 100 "$seed" 900 2 full 0 - "$tr" ) >"$tmp" 2>"$D/stderr/$id.err"
  rc=$?; e=$(( $(date +%s) - s ))
  if [ $rc -eq 0 ] && jq -e . "$tmp" >/dev/null 2>&1; then
    jq -c --arg id "$id" --arg stage "$stage" --argjson seed "$seed" --arg load "$up" --argjson el "$e" --arg bin "$got" \
      '. + {id:$id,stage:$stage,seed:$seed,load_at_start:$load,elapsed_s:$el,binary_sha256:$bin}' "$tmp" >"$D/cells/$id.json"
    echo "ok $id rc=0 elapsed_s=$e load=$up" >>"$D/status.txt"
  else
    echo "failed rc=$rc elapsed_s=$e load=$up" >"$D/cells/$id.fail"
    echo "failed $id rc=$rc elapsed_s=$e load=$up" >>"$D/status.txt"
  fi
  rm -f "$tmp"
done < "$D/cells.list"
echo "driver finished $(date -u +%FT%TZ)" >>"$D/status.txt"
