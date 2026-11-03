#!/usr/bin/env bash
# C1 with the incremental engine (PROTOCOL.md Amendment 7), run on the pod
# from the uploaded, hash-checked files in /workspace/in. /workspace is a
# RunPod network volume: inputs, checkpoints and records survive the pod,
# and a replacement pod attached to the same volume resumes here.
#   - A draw whose record exists is skipped.
#   - A draw with a checkpoint resumes from it (pe_closure --checkpoint).
#   - The record is written to a temp file and renamed only on success.
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
  grep MemTotal /proc/meminfo; nproc; lscpu | grep 'Model name'; } >> "$R/host.txt"
# Progress: checkpoint size (basis rows ≈ bytes / 95,016) every 5 minutes.
( while sleep 300; do { date -u +%FT%TZ; ls -la --time-style=+%FT%TZ "$CK"; } > "$OUT/progress.txt"; done ) &
status "PE5 C1 draws=$DRAWS (resume-capable)"
for k in $DRAWS; do
  rec="$R/pe5-m3-19-7.d$k.jsonl"
  if [ -s "$rec" ]; then log "pe5 d$k: record exists, skipped"; continue; fi
  [ -f "$CK/d$k.ckpt" ] && log "pe5 d$k: resuming from checkpoint ($(stat -c %s "$CK/d$k.ckpt") bytes)"
  log "pe5 d$k: start"
  s=$(date +%s)
  (cd "$IN" && ./pe_closure --draws m3-19-7.jsonl --draw "$k" --degree 5 \
      --mode partial --batch 64 --max-cols 1200000 \
      --checkpoint "$CK/d$k.ckpt" --checkpoint-every "${CKPT_EVERY:-1800}") \
    > "$rec.tmp" 2>> "$R/pe5-m3-19-7.d$k.log"
  rc=$?
  log "pe5 d$k: exit $rc after $(( $(date +%s) - s )) s"
  if [ "$rc" = 0 ] && [ -s "$rec.tmp" ]; then mv "$rec.tmp" "$rec"; else status "PE5 d$k FAILED rc=$rc"; fi
done
status DONE
# Serve the results for SERVE_SECONDS, then end the container (budget). The
# records and checkpoints stay on the volume.
sleep "${SERVE_SECONDS:-2700}"
echo "$(date -u +%FT%TZ) serving window over" >> "$OUT/STATUS"
kill -TERM 1
