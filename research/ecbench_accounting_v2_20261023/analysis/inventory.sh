#!/bin/bash
# Session inventory: id, status, records, binary, commit recorded, levels earned, audit receipt sha256.
cd "$(dirname "$0")/.."
printf "session\tsession_id\tstatus\trecords\tverified\tbinary_sha256_12\tcommit_12\tlevels\treplay_receipt_sha256\n"
for d in sessions/*/ archive/; do
  n=$(basename $d); [ -f $d/session.json ] || continue
  sj=$d/session.json
  lv=$(jq -r -s '[.[]|.isolation.level]|group_by(.)|map("\(.[0])x\(length)")|join(" ")' $d/records.jsonl)
  ver=$(jq -s '[.[]|select(.outcome.status=="verified")]|length' $d/records.jsonl)
  rc=$(grep -h '^receipt sha256' audits/$n.verify.txt 2>/dev/null | awk '{print $3}')
  printf "%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\n" $n $(jq -r .session_id $sj) $(jq -r .status $sj) $(jq -r .records_written $sj) $ver $(jq -r .binary_sha256 $sj | cut -c1-12) $(jq -r .git_commit $sj | cut -c1-12) "$lv" "$rc"
done
