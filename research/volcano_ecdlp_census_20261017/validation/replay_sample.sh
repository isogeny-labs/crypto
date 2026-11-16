#!/usr/bin/env bash
# Independent replay of a stratified sample of committed Phase B sessions.
#
# Validator-written (not part of the producer's pipeline). For each sampled
# session: check the committed archive's SHA-256 against runs/progress.jsonl,
# extract it to a scratch directory, run `ecbench verify --replay-all`
# (falling back to `--replay 50` if --replay-all exceeds 600 s), compare the
# receipt's file hashes with the committed Phase B audit receipt, then delete
# the extracted copy. One session at a time (free disk is small).
#
# Usage: replay_sample.sh STUDY_DIR ECBENCH SCRATCH SAMPLE_FILE OUT_JSONL
set -u
STUDY=$1; ECB=$2; SCR=$3; SAMPLE=$4; OUT=$5
mkdir -p "$SCR/replay" "$SCR/receipts"
while read -r spec; do
  [ -z "$spec" ] && continue
  arc="$STUDY/sessions/$spec.tar.xz"
  want=$(jq -r --arg s "$spec" 'select(.status=="done" and .spec==$s) | .sha256_tar_xz' "$STUDY/runs/progress.jsonl")
  got=$(sha256sum "$arc" | awk '{print $1}')
  dst="$SCR/replay/$spec"
  rm -rf "$dst"; mkdir -p "$dst"
  t0=$(date +%s)
  tar -xJf "$arc" -C "$dst"
  t1=$(date +%s)
  mode="replay-all"; rcpt="$SCR/receipts/$spec.replay-all.json"
  timeout 600 "$ECB" verify --dir "$dst/$spec" --replay-all --out "$rcpt" >/dev/null 2>"$SCR/receipts/$spec.replay-all.stderr"
  rc=$?
  t2=$(date +%s)
  fallback_reason=null
  if [ "$rc" -eq 124 ]; then
    fallback_reason="\"replay-all exceeded 600 s (killed at $((t2-t1)) s)\""
    rm -f "$rcpt"
    mode="replay-50"; rcpt="$SCR/receipts/$spec.replay-50.json"
    "$ECB" verify --dir "$dst/$spec" --replay 50 --out "$rcpt" >/dev/null 2>"$SCR/receipts/$spec.replay-50.stderr"
    rc=$?
  fi
  t3=$(date +%s)
  rsha=$( [ -f "$rcpt" ] && sha256sum "$rcpt" | awk '{print $1}' || echo null)
  # compare file hashes with the committed Phase B audit receipt
  files_match=$(jq -n --slurpfile a "$rcpt" --slurpfile b "$STUDY/sessions/$spec.audit.json" '$a[0].files == $b[0].files and $a[0].session_id == $b[0].session_id')
  jq -c -n --arg spec "$spec" --arg want "$want" --arg got "$got" --arg mode "$mode" \
     --argjson rc "$rc" --argjson fb "$fallback_reason" --arg rsha "$rsha" \
     --argjson tx $((t1-t0)) --argjson tv $((t3-t1)) --argjson fm "$files_match" \
     --slurpfile r "$rcpt" '
    ($r[0]) as $r |
    {spec:$spec, archive_sha256:$got, progress_sha256:$want, archive_sha256_match:($got==$want),
     mode:$mode, fallback_reason:$fb, verify_rc:$rc, extract_s:$tx, verify_s:$tv,
     session_id:$r.session_id, session_status:$r.session_status, audit_ok:$r.ok,
     problems:$r.problems, checks_all_true:([$r.checks[]]|all), checks:$r.checks,
     records:$r.records, verified_records:$r.verified_records,
     replays:($r.replays|length), replays_reproduced:([$r.replays[]|select(.reproduced)]|length),
     replay_details:([$r.replays[].detail]|group_by(.)|map({(.[0]):length})|add),
     auditor_binary_sha256:$r.auditor_binary_sha256, auditor_env_class_id:$r.auditor_env_class_id,
     auditor_hardware_class_id:$r.auditor_hardware_class_id,
     files_match_phase_b_audit:$fm, receipt_sha256:$rsha}' >> "$OUT"
  rm -rf "$dst"
  xz -9 -f "$rcpt"
done < "$SAMPLE"
echo "replay sample finished" >> "$OUT.done"
