#!/usr/bin/env bash
# Round-2 independent replay of the drawn sample of committed Phase B sessions
# (validator-written; adapted from the census validation's replay_sample.sh).
#
# For each sampled session (basename of the progress line's `out`): check the
# committed archive's SHA-256 against runs/progress.jsonl, check free disk,
# extract the archive into scratch, check the ecbench binary's sha256, run
# `ecbench verify --replay-all` (fallback `--replay 50` only if replay-all
# exceeds the timeout), compare the receipt's file map and session id with the
# committed Phase B audit receipt, then delete the extracted copy before the
# next session. One session at a time (free disk is small).
#
# Usage: replay_sample.sh STUDY_DIR ECBENCH WANT_SHA SCRATCH SAMPLE_FILE OUT_JSONL
set -u
STUDY=$1; ECB=$2; WANT_BIN=$3; SCR=$4; SAMPLE=$5; OUT=$6
TIMEOUT=${REPLAY_TIMEOUT:-1800}
mkdir -p "$SCR/replay" "$SCR/receipts"
while read -r sess; do
  [ -z "$sess" ] && continue
  arc="$STUDY/sessions/$sess.tar.xz"
  line=$(jq -c --arg o "sessions/$sess" 'select(.status=="done" and .out==$o)' "$STUDY/runs/progress.jsonl")
  want=$(echo "$line" | jq -r .sha256_tar_xz); raw=$(echo "$line" | jq -r .bytes_raw)
  got=$(sha256sum "$arc" | awk '{print $1}')
  free=$(df -B1 --output=avail "$SCR" | tail -1)
  if [ "$free" -lt $((raw + 200000000)) ]; then
    echo "{\"session\":\"$sess\",\"error\":\"disk: $free bytes free < raw $raw + 200 MB; stopped\"}" >> "$OUT"; exit 3
  fi
  bin=$(sha256sum "$ECB" | awk '{print $1}')
  [ "$bin" = "$WANT_BIN" ] || { echo "{\"session\":\"$sess\",\"error\":\"binary sha mismatch $bin\"}" >> "$OUT"; exit 4; }
  dst="$SCR/replay/$sess"; rm -rf "$dst"; mkdir -p "$dst"
  t0=$(date +%s); tar -xJf "$arc" -C "$dst"; t1=$(date +%s)
  mode="replay-all"; rcpt="$SCR/receipts/$sess.replay-all.json"
  timeout "$TIMEOUT" "$ECB" verify --dir "$dst/$sess" --replay-all --out "$rcpt" >/dev/null 2>"$SCR/receipts/$sess.replay-all.stderr"
  rc=$?; t2=$(date +%s); fb=null
  if [ "$rc" -eq 124 ]; then
    fb="\"replay-all exceeded $TIMEOUT s\""; rm -f "$rcpt"
    mode="replay-50"; rcpt="$SCR/receipts/$sess.replay-50.json"
    "$ECB" verify --dir "$dst/$sess" --replay 50 --out "$rcpt" >/dev/null 2>"$SCR/receipts/$sess.replay-50.stderr"; rc=$?
  fi
  t3=$(date +%s)
  rsha=$( [ -f "$rcpt" ] && sha256sum "$rcpt" | awk '{print $1}' || echo null)
  fm=$(jq -n --slurpfile a "$rcpt" --slurpfile b "$STUDY/sessions/$sess.audit.json" '$a[0].files == $b[0].files and $a[0].session_id == $b[0].session_id')
  jq -c -n --arg sess "$sess" --arg want "$want" --arg got "$got" --arg mode "$mode" --arg bin "$bin" \
     --argjson rc "$rc" --argjson fb "$fb" --arg rsha "$rsha" \
     --argjson tx $((t1-t0)) --argjson tv $((t3-t1)) --argjson fm "$fm" --argjson t0 "$t0" \
     --slurpfile r "$rcpt" '
    ($r[0]) as $r |
    {session:$sess, archive_sha256:$got, progress_sha256:$want, archive_sha256_match:($got==$want),
     binary_sha256_at_replay:$bin, mode:$mode, fallback_reason:$fb, verify_rc:$rc,
     started_unix:$t0, extract_s:$tx, verify_s:$tv,
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
echo "replay sample finished $(date -u +%FT%TZ)" >> "$OUT.done"
