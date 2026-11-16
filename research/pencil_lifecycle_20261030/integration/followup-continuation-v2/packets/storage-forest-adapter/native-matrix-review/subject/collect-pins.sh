#!/usr/bin/env bash
set -euo pipefail
root=/var/tmp/pencil-kummer-matrix-binding-20261101
mkdir -p "$root/inputs/pins"
: > "$root/pins.jsonl"
while IFS=$'\t' read -r rel expected;do
 found=''
 for base in /home/ec2-user/crypto-pr2304-persist /var/tmp/crypto-ic-leaderboard-main-20261030 /home/ec2-user/crypto-pencil-signed-graph-20261029;do
  candidate="$base/$rel"
  if test -f "$candidate";then
   observed=$(sha256sum "$candidate");observed=${observed%% *}
   if test "$observed" = "$expected";then found="$candidate";break;fi
  fi
 done
 if test -n "$found";then
  dest="inputs/pins/$(basename "$rel")"
  cp --reflink=auto "$found" "$root/$dest"
  jq -nc --arg p "$rel" --arg l "$dest" --arg s "$found" '{path:$p,local:$l,recovered_from:$s}' >> "$root/pins.jsonl"
 else
  jq -nc --arg p "$rel" '{path:$p,local:null,recovered_from:null}' >> "$root/pins.jsonl"
 fi
done < <(jq -r '.source_digests[]|[.path,.sha256]|@tsv' "$root/inputs/summary.json")
jq -s '.' "$root/pins.jsonl" > "$root/pin-index.json"
