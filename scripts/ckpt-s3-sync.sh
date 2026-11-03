#!/usr/bin/env bash
# Copy manifest-sha256-v1 checkpoint generations between a local directory and
# S3, for the native closure checkpoints (src/cryptanalysis/closure_checkpoint.rs).
#
#   ckpt-s3-sync.sh restore S3_URI LOCAL_DIR     newest two committed generations -> LOCAL_DIR
#   ckpt-s3-sync.sh push    LOCAL_DIR S3_URI     local committed generations not in S3 yet -> S3
#   ckpt-s3-sync.sh watch   LOCAL_DIR S3_URI [SECONDS]   push every SECONDS (default 120), once more on TERM
#
# A generation is immutable and its `checkpoint-manifest.json` is written last,
# so the same holds in S3: the manifest is uploaded after every other file of
# the generation, and a generation without it there is partial and ignored.
# Remote generations beyond the newest KEEP are deleted after a successful push.
# The process that reads a restored generation verifies every SHA-256 itself
# and falls back to the previous generation; this script never trusts S3 listing.
#
# S3_URI names one checkpoint directory, e.g. s3://BUCKET/dreg/m3-19-7.d0.
# Credentials come from the environment or the pod's IAM role; none is read here.
# CKPT_AWS may point at a different `aws` binary (tests use a directory-backed stub).
set -euo pipefail
AWS=${CKPT_AWS:-aws}
KEEP=${CKPT_KEEP:-2}

log() { echo "$(date -u +%FT%TZ) ckpt-s3-sync: $*" >&2; }

# Newest-first list of committed generation names under an S3 prefix.
remote_gens() {
  "$AWS" s3 ls "$1/" | awk '$1=="PRE" && $2 ~ /^gen-[0-9]+\/$/ {sub("/","",$2); print $2}' | sort -r | while read -r g; do
    if "$AWS" s3 ls "$1/$g/checkpoint-manifest.json" >/dev/null 2>&1; then echo "$g"; fi
  done
}

tune() {
  # Large files: parallel multipart with big parts (10000-part limit: 256 MB parts reach 2.5 TB).
  "$AWS" configure set default.s3.max_concurrent_requests "${CKPT_S3_CONCURRENCY:-32}" 2>/dev/null || true
  "$AWS" configure set default.s3.multipart_chunksize "${CKPT_S3_CHUNK:-256MB}" 2>/dev/null || true
}

restore() {
  local src=$1 dst=$2 n=0 g
  mkdir -p "$dst"
  while read -r g; do
    [ -n "$g" ] || continue
    if [ -f "$dst/$g/checkpoint-manifest.json" ]; then n=$((n + 1)); continue; fi
    log "restoring $g"
    rm -rf "$dst/$g.partial"
    "$AWS" s3 cp --only-show-errors --recursive "$src/$g/" "$dst/$g.partial/"
    mv "$dst/$g.partial" "$dst/$g"
    n=$((n + 1))
    [ "$n" -ge "$KEEP" ] && break
  done < <(remote_gens "$src")
  log "restore: $n committed generation(s) in $dst"
}

push() {
  local src=$1 dst=$2 g newest d latest=""
  # Only the newest committed local generation is uploaded, and only if it is
  # newer than the newest committed one in S3: older ones are obsolete, and a
  # generation can be hundreds of gigabytes.  The previous remote generation
  # stays as the fallback for a corrupt newest one.
  newest=$(remote_gens "$dst" | head -n 1 || true)
  for d in $(ls -d "$src"/gen-* 2>/dev/null | sort); do
    [ -f "$d/checkpoint-manifest.json" ] && latest=$d
  done
  if [ -n "$latest" ]; then
    g=$(basename "$latest")
    if [ -z "$newest" ] || [[ "$g" > "$newest" ]]; then
      log "uploading $g ($(du -sh "$latest" | cut -f1))"
      "$AWS" s3 cp --only-show-errors --recursive --exclude checkpoint-manifest.json "$latest/" "$dst/$g/"
      "$AWS" s3 cp --only-show-errors "$latest/checkpoint-manifest.json" "$dst/$g/checkpoint-manifest.json"
    fi
  fi
  # Orphans: a generation prefix without a manifest that is older than the newest
  # committed one is a push that died part-way; its objects only cost money.
  local top
  top=$(remote_gens "$dst" | head -n 1 || true)
  if [ -n "$top" ]; then
    while read -r g; do
      [ -n "$g" ] || continue
      if [[ "$g" < "$top" ]] && ! "$AWS" s3 ls "$dst/$g/checkpoint-manifest.json" >/dev/null 2>&1; then
        log "removing incomplete remote $g"
        "$AWS" s3 rm --only-show-errors --recursive "$dst/$g/"
      fi
    done < <("$AWS" s3 ls "$dst/" | awk '$1=="PRE" && $2 ~ /^gen-[0-9]+\/$/ {sub("/","",$2); print $2}')
  fi
  # Prune: keep the newest KEEP committed generations, and nothing newer is lost.
  local i=0
  while read -r g; do
    [ -n "$g" ] || continue
    i=$((i + 1))
    [ "$i" -le "$KEEP" ] && continue
    log "pruning remote $g"
    "$AWS" s3 rm --only-show-errors --recursive "$dst/$g/"
  done < <(remote_gens "$dst")
}

watch() {
  local src=$1 dst=$2 every=${3:-120} stop=0
  trap 'stop=1' TERM INT
  while [ "$stop" = 0 ]; do
    push "$src" "$dst" || log "push failed; will retry"
    sleep "$every" & wait $! || true
  done
  push "$src" "$dst"
}

tune
case ${1:-} in
  restore) restore "$2" "$3" ;;
  push) push "$2" "$3" ;;
  watch) watch "$2" "$3" "${4:-120}" ;;
  *) echo "usage: $0 restore S3_URI DIR | push DIR S3_URI | watch DIR S3_URI [SECONDS]" >&2; exit 2 ;;
esac
