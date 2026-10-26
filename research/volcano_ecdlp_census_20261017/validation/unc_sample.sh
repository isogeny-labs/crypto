#!/usr/bin/env bash
# For each sampled session: extract the committed archive (after checking its
# SHA-256 against runs/progress.jsonl), re-derive total_gae, S and the
# uncharged count of every measured record from records.jsonl
# (unc_from_records.jq), compare with the committed extract and uncharged
# CSVs (unc_compare.sh), delete the extracted copy.
# Usage: unc_sample.sh STUDY SCRATCH SAMPLE_FILE OUT_JSONL
set -u
STUDY=$1; SCR=$2; SAMPLE=$3; OUT=$4; HERE=$(dirname "$0")
mkdir -p "$SCR/unc"
while read -r spec; do
  [ -z "$spec" ] && continue
  want=$(jq -r --arg s "$spec" 'select(.status=="done" and .spec==$s) | .sha256_tar_xz' "$STUDY/runs/progress.jsonl")
  got=$(sha256sum "$STUDY/sessions/$spec.tar.xz" | awk '{print $1}')
  if [ "$want" != "$got" ]; then echo "{\"spec\":\"$spec\",\"archive_sha256_match\":false}" >> "$OUT"; continue; fi
  rm -rf "$SCR/unc/$spec"; mkdir -p "$SCR/unc/$spec"
  tar -xJf "$STUDY/sessions/$spec.tar.xz" -C "$SCR/unc/$spec"
  "$HERE/unc_compare.sh" "$STUDY" "$spec" "$SCR/unc/$spec/$spec/records.jsonl" | jq -c '. + {archive_sha256_match: true}' >> "$OUT"
  rm -rf "$SCR/unc/$spec"
done < "$SAMPLE"
