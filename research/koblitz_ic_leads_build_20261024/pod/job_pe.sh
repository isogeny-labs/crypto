#!/usr/bin/env bash
# C1 with the incremental engine (PROTOCOL.md Amendments 7 and 7b), run on
# the pod from the uploaded, hash-checked files in /workspace/in.
# /workspace must be the pod volume: inputs, checkpoints and records survive
# a container restart and a pod stop/start, and the receiver resumes here.
#   - The job refuses to start without MIN_FREE_GB free on /workspace (a
#     checkpoint and its temp copy, about 2 x 73 GB at the peak).
#   - A draw whose record exists is skipped.
#   - A draw with a checkpoint resumes from it (pe_closure --checkpoint).
#   - A failed draw is retried from its checkpoint, up to RETRIES times,
#     before the job moves on; the record is renamed into place only on
#     success.
set -u
IN=/workspace/in; OUT=/workspace/out; R=$OUT/runs; CK=/workspace/ckpt
mkdir -p "$R" "$CK"
status() { echo "$(date -u +%FT%TZ) $1" >> "$OUT/STATUS"; }
log() { echo "$(date -u +%FT%TZ) $*" >> "$R/queue.log"; }
export F4_F2_MAX_ROWS=50000000 F4_F2_MAX_COLS=50000000 RAYON_NUM_THREADS=$(nproc)
B=$IN/pe_closure; chmod +x "$B"
DRAWS=${DRAWS:-"0 3"}
{ echo "date: $(date -u +%FT%TZ)"; echo "pod: ${RUNPOD_POD_ID:-?}"
  echo "pe_closure sha256: $(sha256sum "$B" | cut -d' ' -f1)"
  echo "draws sha256: $(sha256sum "$IN/m3-19-7.jsonl" | cut -d' ' -f1)"
  grep MemTotal /proc/meminfo; nproc; lscpu | grep 'Model name'
  df -h /workspace; grep ' /workspace ' /proc/mounts; } >> "$R/host.txt"
# Room for a checkpoint and its temp copy: free space plus the checkpoints
# already present (a resumed draw's own file is replaced, not added).
ck_kb=$(du -sk "$CK" | cut -f1)
free_gb=$(( ($(df -Pk /workspace | awk 'NR==2 {print $4}') + ck_kb) / 1048576 ))
if [ "$free_gb" -lt "${MIN_FREE_GB:-160}" ]; then
  status "WORKSPACE_TOO_SMALL free=${free_gb}G need=${MIN_FREE_GB:-160}G"; exit 1
fi
# Progress: checkpoint size (basis rows ≈ bytes / 95,016) every 5 minutes.
( while sleep 300; do { date -u +%FT%TZ; ls -la --time-style=+%FT%TZ "$CK"; } > "$OUT/progress.txt"; done ) &
status "PE5 C1 draws=$DRAWS (resume-capable)"
for k in $DRAWS; do
  rec="$R/pe5-m3-19-7.d$k.jsonl"
  if [ -s "$rec" ]; then log "pe5 d$k: record exists, skipped"; continue; fi
  for attempt in $(seq 1 "${RETRIES:-3}"); do
    [ -f "$CK/d$k.ckpt" ] && log "pe5 d$k: resuming from checkpoint ($(stat -c %s "$CK/d$k.ckpt") bytes)"
    log "pe5 d$k: start (attempt $attempt)"
    s=$(date +%s)
    (cd "$IN" && ./pe_closure --draws m3-19-7.jsonl --draw "$k" --degree 5 \
        --mode partial --batch 64 --max-cols 1200000 \
        --checkpoint "$CK/d$k.ckpt" --checkpoint-every "${CKPT_EVERY:-1800}") \
      > "$rec.tmp" 2>> "$R/pe5-m3-19-7.d$k.log"
    rc=$?
    log "pe5 d$k: exit $rc after $(( $(date +%s) - s )) s"
    if [ "$rc" = 0 ] && [ -s "$rec.tmp" ]; then mv "$rec.tmp" "$rec"; break; fi
    status "PE5 d$k FAILED rc=$rc (attempt $attempt)"
  done
  # A draw that still has no record stops the job: the next draw would
  # compete with this one's checkpoint for the disk.
  [ -s "$rec" ] || { status "PE5 d$k GAVE UP"; break; }
done
status DONE
# Serve the results for SERVE_SECONDS, then end the container (budget). The
# records and checkpoints stay on the volume.
sleep "${SERVE_SECONDS:-2700}"
echo "$(date -u +%FT%TZ) serving window over" >> "$OUT/STATUS"
kill -TERM 1
