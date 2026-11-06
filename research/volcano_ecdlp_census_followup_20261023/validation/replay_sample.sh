#!/usr/bin/env bash
# Independent replay of a stratified sample of committed Phase B sessions
# (validator-written; adapted from the census validation/replay_sample.sh).
#
# For each sampled session (one at a time; free disk is small): stop if free
# disk is under 300 MB; check the binary sha256 and the committed archive's
# SHA-256 against runs/progress.jsonl; extract it to a scratch directory; run
# `ecbench verify --replay-all` (falling back to `--replay 50` only if
# --replay-all exceeds REPLAY_ALL_TIMEOUT s, default 1800); compare the
# receipt's file hashes and session id with the committed Phase B audit
# receipt; delete the extracted copy.
#
# Usage: replay_sample.sh STUDY_DIR ECBENCH SCRATCH SAMPLE_FILE OUT_JSONL
set -u
STUDY=$1; ECB=$2; SCR=$3; SAMPLE=$4; OUT=$5
TMO=${REPLAY_ALL_TIMEOUT:-1800}
WANT_BIN=ac4bc5b26f858a4d8a374208297157840af25fb1dc25ea1c19dd41360735fb35
mkdir -p "$SCR/replay" "$SCR/receipts"
while read -r sess; do
  [ -z "$sess" ] && continue
  free_kb=$(df -Pk "$SCR" | awk 'NR==2{print $4}')
  if [ "$free_kb" -lt 307200 ]; then echo "STOP: free disk ${free_kb} KB < 300 MB before $sess" >> "$OUT.stop"; exit 3; fi
  bsha=$(sha256sum "$ECB" | awk '{print $1}')
  [ "$bsha" = "$WANT_BIN" ] || { echo "STOP: binary sha $bsha" >> "$OUT.stop"; exit 4; }
  arc="$STUDY/sessions/$sess.tar.xz"
  want=$(jq -r --arg o "sessions/$sess" 'select(.status=="done" and .out==$o) | .sha256_tar_xz' "$STUDY/runs/progress.jsonl")
  got=$(sha256sum "$arc" | awk '{print $1}')
  dst="$SCR/replay/$sess"
  rm -rf "$dst"; mkdir -p "$dst"
  t0=$(date +%s)
  tar -xJf "$arc" -C "$dst"
  t1=$(date +%s)
  sdir=$(find "$dst" -mindepth 1 -maxdepth 1 -type d | head -1)
  mode="replay-all"; rcpt="$SCR/receipts/$sess.replay-all.json"
  timeout "$TMO" "$ECB" verify --dir "$sdir" --replay-all --out "$rcpt" >/dev/null 2>"$SCR/receipts/$sess.replay-all.stderr"
  rc=$?
  t2=$(date +%s)
  fallback_reason=null
  if [ "$rc" -eq 124 ]; then
    fallback_reason="\"replay-all exceeded $TMO s (killed at $((t2-t1)) s)\""
    rm -f "$rcpt"
    mode="replay-50"; rcpt="$SCR/receipts/$sess.replay-50.json"
    "$ECB" verify --dir "$sdir" --replay 50 --out "$rcpt" >/dev/null 2>"$SCR/receipts/$sess.replay-50.stderr"
    rc=$?
  fi
  t3=$(date +%s)
  rsha=$( [ -f "$rcpt" ] && sha256sum "$rcpt" | awk '{print $1}' || echo null)
  files_match=$(jq -n --slurpfile a "$rcpt" --slurpfile b "$STUDY/sessions/$sess.audit.json" '$a[0].files == $b[0].files and $a[0].session_id == $b[0].session_id')
  jq -c -n --arg sess "$sess" --arg want "$want" --arg got "$got" --arg mode "$mode" --arg sdir "$(basename "$sdir")" \
     --argjson rc "$rc" --argjson fb "$fallback_reason" --arg rsha "$rsha" --arg bsha "$bsha" \
     --argjson tx $((t1-t0)) --argjson tv $((t3-t1)) --argjson fm "$files_match" --argjson freekb "$free_kb" \
     --slurpfile r "$rcpt" '
    ($r[0]) as $r |
    {run:$sess, archive_top_dir:$sdir, archive_sha256:$got, progress_sha256:$want, archive_sha256_match:($got==$want),
     mode:$mode, fallback_reason:$fb, verify_rc:$rc, extract_seconds:$tx, verify_seconds:$tv, free_kb_before:$freekb,
     binary_sha256_checked:$bsha,
     session_id:$r.session_id, session_status:$r.session_status, audit_ok:$r.ok,
     problems:$r.problems, checks_all_true:([$r.checks[]]|all), checks:$r.checks,
     records:$r.records, verified_records:$r.verified_records,
     replays:($r.replays|length), replays_identical:([$r.replays[]|select(.reproduced and .detail=="identical")]|length),
     replay_details:([$r.replays[].detail]|group_by(.)|map({(.[0]):length})|add),
     auditor_binary_sha256:$r.auditor_binary_sha256, auditor_env_class_id:$r.auditor_env_class_id,
     auditor_hardware_class_id:$r.auditor_hardware_class_id,
     files_match_phase_b_audit:$fm, receipt_sha256:$rsha}' >> "$OUT"
  rm -rf "$dst"
  xz -9 -f "$rcpt"
done < "$SAMPLE"
echo "replay sample finished $(date -u +%FT%TZ)" >> "$OUT.done"
