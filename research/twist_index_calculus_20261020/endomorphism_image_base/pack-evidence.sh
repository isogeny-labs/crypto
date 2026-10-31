#!/usr/bin/env bash
# Stage on local scratch, then pack once; avoid thousands of duplicate NFS writes.
# This copies evidence only. It never executes a cryptographic experiment.
set -euo pipefail
image_root=$(cd -- "$(dirname -- "$0")" && pwd)
image_scratch=${1:?new scratch directory, with room for the extracted tree}
image_mode=${2:-final}
[[ "$image_mode" == final || "$image_mode" == stage-only ]]
if test -d "$image_scratch"; then
  test "$(cat "$image_scratch/.study-owner")" = "$image_root"
else
  mkdir "$image_scratch"
  printf '%s\n' "$image_root" > "$image_scratch/.study-owner"
fi
image_scratch=$(cd -- "$image_scratch" && pwd)
mkdir -p "$image_scratch/runs/jobs" "$image_scratch/runs/source-bundles"
cd "$image_root"
if test "$image_mode" = final && test -f "$image_scratch/.refresh-before-final.txt"; then
  # A staging schedule change can leave partial local copies. Refresh these
  # complete immutable trees after the old copier has stopped.
  while IFS= read -r image_job; do
    rm -f "$image_scratch/.staged-$image_job"
  done < "$image_scratch/.refresh-before-final.txt"
  rm "$image_scratch/.refresh-before-final.txt"
fi
cp runs/cold-assembled-v1/provenance.json runs/reconciliation/cold-assembly-initial.json
cp runs/cold-assembled-final-v1/provenance.json runs/reconciliation/cold-assembly-final.json
tar --exclude='runs/jobs' --exclude='runs/cold-assembled-*' --exclude='target' \
    -cf - runs | tar -xf - -C "$image_scratch"
image_stage_job() {
  local image_job=$1 image_cache image_dest image_branch
  image_cache="/home/ec2-user/.cache/ephemeral/runs/$image_job"
  if ! rg -q '"phase"[[:space:]]*:[[:space:]]*"(done|failed|interrupted|timeout|error)"' "$image_cache/status.json"; then
    if test "$image_mode" = stage-only; then return 0; fi
    printf 'Refusing final archive with nonterminal job %s\n' "$image_job" >&2
    exit 1
  fi
  if test -f "$image_scratch/.staged-$image_job"; then return 0; fi
  image_dest="$image_scratch/runs/jobs/$image_job"
  mkdir -p "$image_dest"
  # Original source archives are retained below, so repeated unpacked sources
  # and recursively copied checkpoints do not multiply the evidence tree.
  tar --exclude='./env' --exclude='./env.*' --exclude='./out/source' \
      --exclude='./out/candidate-first' --exclude='./out/candidate-aa' \
      --exclude='./out/input/candidate' \
      --exclude='target' --exclude='.git' --exclude='previous-records' \
      -C "$image_cache" -cf - . | tar -xf - -C "$image_dest" || return 1
  # Each shard is immutable and has a separate destination. Parallel reads
  # avoid serial per-file network latency without changing any run output.
  for image_branch in out/candidate-first out/candidate-aa out/input/candidate; do
    if ! test -d "$image_cache/$image_branch"; then continue; fi
    mkdir -p "$image_dest/$image_branch"
    find "$image_cache/$image_branch" -mindepth 1 -maxdepth 1 -printf '%f\0' \
      | xargs -0 -r -P8 -I{} bash -euo pipefail -c \
        'tar --exclude=target --exclude=.git --exclude=previous-records -C "$1" -cf - -- "$3" | tar -xf - -C "$2"' \
        _ "$image_cache/$image_branch" "$image_dest/$image_branch" '{}' || return 1
  done
  if test -f "$image_cache/env" && rg -q '^IMAGE_BUNDLE_A=' "$image_cache/env"; then
    # Only the known source-bundle keys are read. No shell evaluation or
    # runtime credentials enter the evidence archive.
    awk '/^IMAGE_BUNDLE_[A-T]=/ {sub(/^[^=]*=/, ""); gsub(/\047/, ""); if ($0 !~ /^[A-Za-z0-9+\/=]*$/) exit 2; printf "%s", $0}' \
      "$image_cache/env" | base64 -d > "$image_scratch/runs/source-bundles/$image_job.tgz"
    gzip -t "$image_scratch/runs/source-bundles/$image_job.tgz" || return 1
  fi
  touch "$image_scratch/.staged-$image_job"
}
export image_scratch image_mode
export -f image_stage_job
# Independent immutable trees can be copied concurrently. Canonical final
# tar sorting fixes member order independently of this staging schedule.
jq -r '.jobs[].id' runs/reconciliation/job-registry.json \
  | xargs -r -P4 -I{} bash -euo pipefail -c 'image_stage_job "$1"' _ '{}'
if test "$image_mode" = stage-only; then
  printf 'Terminal job trees staged; no final archive emitted.\n'
  exit 0
fi
cd "$image_scratch"
sha256sum runs/source-bundles/*.tgz > runs/source-bundles/SHA256SUMS
tar --sort=name --mtime='2026-10-23 00:00:00Z' --owner=0 --group=0 \
    --numeric-owner --mode='u+rwX,go+rX,go-w' --format=gnu -cf - runs \
  | xz -6 -T1 > "$image_root/runs.tar.xz"
cd "$image_root"
sha256sum runs.tar.xz > runs.tar.xz.sha256
printf 'Archive staged at %s and saved with its digest.\n' "$image_scratch"
